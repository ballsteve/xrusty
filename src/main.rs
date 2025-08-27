/// Read an XML document, an XSL stylesheet and perform the transformation
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use url::Url;
use xrust::item::{Item, Node, SequenceTrait};
use xrust::output::OutputDefinition;
use xrust::parser::xml::parse as xmlparse;
use xrust::parser::xpath::parse as xpathparse;
use xrust::transform::context::{ContextBuilder, StaticContextBuilder};
use xrust::trees::smite::RNode;
use xrust::xdmerror::{Error, ErrorKind};
use xrust::xslt::from_document;
//use xrust_net::resolver;

use xrust_md::parse as mdparse;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} stylesheet source", args[0]);
        return;
    }

    let stylepath = Path::new(&args[1]);
    let mut stylefile = match File::open(&stylepath) {
        Err(why) => {
            panic!(
                "unable to open stylesheet \"{}\" due to \"{}\"",
                &args[1], why
            )
        }
        Ok(f) => f,
    };
    let mut stylexml = String::new();
    match stylefile.read_to_string(&mut stylexml) {
        Err(why) => panic!("unable to read from \"{}\" due to \"{}\"", &args[1], why),
        Ok(_) => {}
    };

    let style = RNode::new_document();
    let _ = xmlparse(style.clone(), stylexml.trim(), None).expect("failed to parse XSL stylesheet");

    let srcpath = Path::new(&args[2]);
    let mut srcfile = match File::open(&srcpath) {
        Err(why) => {
            panic!(
                "unable to open source document \"{}\" due to \"{}\"",
                &args[2], why
            )
        }
        Ok(f) => f,
    };
    let mut srcraw = String::new();
    match srcfile.read_to_string(&mut srcraw) {
        Err(why) => panic!("unable to read from \"{}\" due to \"{}\"", &args[2], why),
        Ok(_) => {}
    };

    // Parse as XML or Markdown
    let sourcedoc = match srcpath.extension().map(|o| o.to_str()) {
        Some(Some("xml")) => xmlparse(RNode::new_document(), srcraw.as_str().trim(), None)
            .expect("failed to parse XML"),
        Some(Some("md")) => mdparse(srcraw.as_str()).expect("unable to parse markdown"),
        _ => {
            // Try XML, if that fails try MD, otherwise fail
            RNode::new_document()
        }
    };

    let mut od = OutputDefinition::new();
    od.set_indent(true);

    let pwd = std::env::current_dir().expect("unable to get current directory");
    let pwds = pwd
        .into_os_string()
        .into_string()
        .expect("unable to convert pwd");
    let mut ctxt = from_document(
        style.clone(),
        Some(
            Url::parse(format!("file://{}/{}", pwds, &args[1]).as_str())
                .expect("unable to parse stylesheet URL"),
        ),
        |_| {
            Err(Error::new(
                ErrorKind::Unknown,
                String::from("loading resources not implemented"),
            ))
        },
        |_| {
            Err(Error::new(
                ErrorKind::Unknown,
                String::from("loading external resources not implemented"),
            ))
        },
    )
    .expect("failed to compile XSL stylesheet");

    let rd = RNode::new_document();
    ctxt.context(vec![Item::Node(sourcedoc.clone())], 0);
    ctxt.result_document(rd.clone());
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
    ctxt.populate_key_values(&mut stctxt, sourcedoc)
        .expect("unable to populate key values");

    let resultdoc = ctxt
        .evaluate(&mut stctxt)
        .expect("failed to evaluate stylesheet");

    let output_method = ContextBuilder::new()
        .context(vec![Item::Node(style.clone())])
        .build()
        .dispatch(
            &mut stctxt,
            &xpathparse("/xsl:stylesheet/xsl:output/@method", Some(style.clone()))
                .expect("unable to parse output method XPath"),
        )
        .expect("unable to find output method");
    match output_method.len() {
        0 => {
            // Default is XML
            println!("{}", resultdoc.to_xml_with_options(&od));
        }
        1 => match output_method.to_string().as_str() {
            "xml" => println!("{}", resultdoc.to_xml_with_options(&od)),
            "text" => println!("{}", resultdoc.to_string()),
            "html" => panic!("html output method not supported"),
            _ => panic!("unknown output method"),
        },
        _ => panic!("type error while trying to find ouput method"),
    }
}
