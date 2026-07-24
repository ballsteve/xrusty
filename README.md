# χrusty

[![crates.io](https://img.shields.io/crates/v/xrusty.svg)](https://crates.io/crates/xrusty)
[![Released API docs](https://docs.rs/xrusty/badge.svg)](https://docs.rs/xrusty)
[![ALv2 licensed](https://img.shields.io/badge/license-ALv2-blue.svg)](./LICENSE)

A command line interface for the [χrust crate](https://gitlab.gnome.org/World/Rust/markup-rs/xrust/).

χrusty allows you to transform a document - XML, JSON, or Markdown - using an XSL stylesheet (XSLT) to produce a result document.

## Security

χrusty allows you to set a security policy. A security policy is used to control whether χrusty can access local or remote resources, and whether stylesheets can use scarce resources. The policy should be chosen carefully, especially if χrusty is to be used with any untrusted, unknown, or unverified XML documents or XSL stylesheets.

NB. security policies apply to any resources loaded by an XML document or XSL stylesheet. Any documents or stylesheets specified on the command line are not limited by the security policy in force.

χrust and χrusty-net provide a number of security features, as follows:

* χrust: *maximum-depth* limits how deeply a stylesheet can evaluate templates. Setting a limit prevents infinite loops, which is a denial-of-service attack.
* χrust-net: *access* controls whether a resource (a file or URL) may be accessed at all. The default is to not allow access to any resources. This may be used to prevent exfiltration of data.
* χrust-net: *maximum-size* limits the maximum size of a resource. Te default is to not allow any resource. This may be used to prevent very large documents being downloaded, which is a denial-of-service attack.

χrusty has two security policies:

* *none* - No external resources may be accessed
* *full* - No limits on resource usage

Contact the maintainer for more fine-grained security policies.

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone the [χrusty project](https://gitlab.gnome.org/World/Rust/markup-rs/xrust/) from [Gitlab](https://gitlab.gnome.org/)
```
git clone https://gitlab.gnome.org/World/Rust/markup-rs/xrusty.git
```
3. Use [cargo](https://doc.rust-lang.org/cargo/) to install χrusty
```
cd xrusty
cargo install --path .
```

This will install ```xrusty``` into $HOME/.cargo/bin. To install into a different location use the --root option, see ```cargo help install```.

## Usage

χrusty takes one or more documents and performs a series of operations on them. The result of processing is then sent to the standard output. The documents may be XML or [Markdown](https://gitlab.gnome.org/World/Rust/markup-rs/xrust-md/).

The operations performed are specified with command line arguments. Two operations don't need to be explicitly specified: parsing the document and serialising the result.

The ```--help``` option displays the command usage:

```
xrusty --help

Usage: xrusty [OPTIONS] [DOCS]...

Arguments:
  [DOCS]...  Documents

Options:
  -t, --transform <TRANSFORM>  Transform source documents using a XSLT stylesheet
  -p, --policy <POLICY>        The security policy to use
      --parameter <PARAMETER>  A stylesheet parameter in the form name=value. The value will be a string literal
  -h, --help                   Print help
  -V, --version                Print version

```

For example:

```
xrusty --policy full --transform xsl/style.xsl xml/source.xml
```

This command will read the XML document ```xml/source.xml``` and transform it using the XSL stylesheet ```xsl/style.xsl```. The result of the transformation will be output to stdout. The _full_ security policy will be in force.

Stylesheet parameters may be specified using the ```--parameter``` option. There can be more than one parameter specified. The parameter name is separated from the value with an '=' character. The parameter name must be a valid XML Name.

## Change Log

| Release | Notes |
|---------|-------|
| 0.4.0   | Add stylesheet parameters |
| 0.3.0   | Add security policies |
| 0.2.1   | Synchronise with χrust 2.0.1 |
| 0.2.0   | Synchronise with χrust 1.3.0 |
| 0.1.0   | Initial release |
