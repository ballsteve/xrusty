use clap::Parser;
/// Read an XML document, an XSL stylesheet and perform the transformation
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use url::Url;
#[allow(unused_imports)]
use xrust::item::{Item, Node, SequenceTrait};
use xrust::output::OutputDefinition;
use xrust::parser::ParseError;
use xrust::parser::xml::parse as xmlparse;
use xrust::parser::xpath::parse as xpathparse;
use xrust::transform::context::{ContextBuilder, StaticContextBuilder};
use xrust::trees::smite::RNode;
use xrust::xdmerror::{Error, ErrorKind};
use xrust::xslt::from_document;
use xrust_net::resolve;

use xrust_md::md::parse as mdparse;

fn make_from_str(s: &str) -> Result<RNode, Error> {
    let d = RNode::new_document();
    xmlparse(
        d.clone(),
        s.trim(),
        Some(|_: &_| Err(ParseError::Notimplemented)),
    )
}

fn main() {
    #[derive(Parser, Debug)]
    #[command(version, about = "", long_about = None)]
    struct Args {
        /// Transform source documents using a XSLT stylesheet
        #[arg(short, long)]
        transform: Option<String>,
        /// Documents
        docs: Vec<String>,
        // TODO: output (with % substitutions)
        // TODO: validate
        // TODO: security policies
    }
    let args = Args::parse();

    // Default output method is XML.
    // If a stylesheet is specified then it may set a different method.
    let mut output_method = String::from("xml");

    let mut stctxt = StaticContextBuilder::new()
        .message(|m| {
            println!("{}", m);
            Ok(())
        })
        .fetcher(|_url| {
            Err(Error::new(
                ErrorKind::Unknown,
                String::from("loading external resources not implemented"),
            ))
        })
        .parser(|_s| {
            Err(Error::new(
                ErrorKind::Unknown,
                String::from("loading resources not implemented"),
            ))
        })
        .build();

    // Set up the stylesheet, if specified.
    let mut xform = None;
    let style = RNode::new_document();
    if let Some(transform) = args.transform {
        let stylepath = Path::new(&transform);
        let mut stylefile = File::open(&stylepath).unwrap_or_else(|why| {
            eprintln!(
                "unable to open stylesheet \"{}\" due to \"{}\"",
                transform, why
            );
            exit(1)
        });
        let mut stylexml = String::new();
        stylefile
            .read_to_string(&mut stylexml)
            .unwrap_or_else(|why| {
                eprintln!("unable to read from \"{}\" due to \"{}\"", transform, why);
                exit(2)
            });

        let _: Result<RNode, Error> = xmlparse(
            style.clone(),
            stylexml.trim(),
            Some(|_: &_| Err(ParseError::Notimplemented)),
        )
        .or_else(|why| {
            eprintln!("failed to parse XSL stylesheet due to {}", why);
            exit(3)
        });

        let pwd = std::env::current_dir().expect("unable to get current directory");
        let pwds = pwd
            .into_os_string()
            .into_string()
            .expect("unable to convert pwd");
        xform = Some(
            from_document(
                style.clone(),
                Some(
                    Url::parse(format!("file://{}/{}", pwds.as_str(), transform.as_str()).as_str())
                        .expect("unable to parse stylesheet URL"),
                ),
                |s| make_from_str(s),
                |url| resolve(url),
            )
            .unwrap_or_else(|why| {
                eprintln!("failed to compile XSL stylesheet due to {}", why);
                exit(4)
            }),
        );

        match ContextBuilder::new()
            .context(vec![Item::Node(style.clone())])
            .build()
            .dispatch(
                &mut stctxt,
                &xpathparse(
                    "/xsl:stylesheet/xsl:output/@method",
                    Some(style.clone()),
                    None,
                )
                .expect("unable to parse output method XPath"),
            ) {
            Ok(m) => {
                if m.len() > 0 {
                    output_method = m[0].to_string().to_lowercase()
                }
            }
            _ => (), // just use the default
        }
    }

    // TODO: define command-line option for indentation
    // TODO: extract indentation from stylesheet
    let mut od = OutputDefinition::new();
    od.set_indent(true);

    args.docs.iter().for_each(|src| {
        let srcpath = Path::new(src);
        let mut srcfile = File::open(&srcpath).unwrap_or_else(|why| {
            eprintln!(
                "unable to open source document \"{}\" due to \"{}\"",
                &src, why
            );
            exit(5)
        });
        let mut srcraw = String::new();
        srcfile.read_to_string(&mut srcraw).unwrap_or_else(|why| {
            eprintln!("unable to read from \"{}\" due to \"{}\"", src, why);
            exit(6)
        });

        let mut sourcedoc = RNode::new_document();
        // Parse as XML or Markdown
        match srcpath.extension().map(|o| o.to_str()) {
            Some(Some("xml")) => {
                xmlparse(
                    sourcedoc.clone(),
                    srcraw.as_str().trim(),
                    Some(|_: &_| Err(ParseError::Notimplemented)),
                )
                .unwrap_or_else(|why| {
                    eprintln!("failed to parse XML due to {}", why);
                    exit(7)
                });
            }
            Some(Some("md")) => {
                sourcedoc = mdparse(srcraw.as_str()).unwrap_or_else(|why| {
                    eprintln!("unable to parse markdown due to {}", why);
                    exit(8)
                });
            }
            _ => {
                // Try XML, if that fails try MD, otherwise fail
                if let Err(why) = xmlparse(
                    sourcedoc.clone(),
                    srcraw.as_str().trim(),
                    Some(|_: &_| Err(ParseError::Notimplemented)),
                ) {
                    eprintln!("failed to parse XML due to {}", why);
                    exit(9)
                } else {
                    match mdparse(srcraw.as_str()) {
                        Ok(d) => {
                            sourcedoc = d;
                        }
                        Err(why) => {
                            eprintln!("unable to parse markdown due to {}", why);
                            exit(10)
                        }
                    }
                }
            }
        };

        let result = xform
            .clone()
            .map_or(vec![Item::Node(sourcedoc.clone())], |mut ctxt| {
                let rd = RNode::new_document();
                ctxt.context(vec![Item::Node(sourcedoc.clone())], 0);
                ctxt.result_document(rd.clone());

                ctxt.populate_key_values(&mut stctxt, sourcedoc)
                    .unwrap_or_else(|why| {
                        eprintln!("unable to populate key values due to {}", why);
                        exit(11)
                    });

                let resultseq = ctxt.evaluate(&mut stctxt).unwrap_or_else(|why| {
                    eprintln!("failed to evaluate stylesheet due to {}", why);
                    exit(12)
                });
                resultseq
            });
        result.iter().for_each(|i| {
            match output_method.as_str() {
                "xml" => {
                    // Default is XML
                    println!("{}", i.to_xml_with_options(&od));
                }
                "text" => {
                    println!("{}", i.to_string())
                }
                "html" => {
                    eprintln!("HTML output method is not supported");
                    exit(13)
                }
                _ => {
                    eprintln!("output method {} is unknown", output_method);
                    exit(14)
                }
            }
        });
    });
}
