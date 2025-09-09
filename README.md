# χrusty

[![crates.io](https://img.shields.io/crates/v/xrusty.svg)](https://crates.io/crates/xrusty)
[![Released API docs](https://docs.rs/xrusty/badge.svg)](https://docs.rs/xrusty)
[![ALv2 licensed](https://img.shields.io/badge/license-ALv2-blue.svg)](./LICENSE)

A command line interface for the [χrust crate](https://gitlab.gnome.org/World/Rust/markup-rs/xrust/).

χrusty allows you to transform a document - XML, JSON, or Markdown - using an XSL stylesheet (XSLT) to produce a result document.

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
  -h, --help                   Print help
  -V, --version                Print version

```

For example:

```
xrusty --transform xsl/style.xsl xml/source.xml
```

This command will read the XML document ```xml/source.xml``` and transform it using the XSL stylesheet ```xsl/style.xsl```. The result of the transformation will be output to stdout.
