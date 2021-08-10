<h1>
  <img src="assets/Relm_logo_with_text.svg" height="65" alt="Relm4">
</h1>

[![CI](https://github.com/AaronErhardt/relm4/actions/workflows/rust.yml/badge.svg)](https://github.com/AaronErhardt/relm4/actions/workflows/rust.yml)
[![Matrix](https://img.shields.io/matrix/relm4:matrix.org?label=matrix%20chat)](https://matrix.to/#/#relm4:matrix.org)
[![Relm4 on crates.io](https://img.shields.io/crates/v/relm4.svg)](https://crates.io/crates/relm4)
[![Relm4 docs](https://img.shields.io/badge/rust-documentation-blue)](https://aaronerhardt.github.io/docs/relm4/relm4/)
![Miminum Rust version 1.53](https://img.shields.io/badge/rustc-1.53+-06a096.svg)

An idiomatic GUI library inspired by [Elm](https://elm-lang.org/) and based on [gtk4-rs](https://crates.io/crates/gtk4). 
Relm4 is a new version of [relm](https://github.com/antoyo/relm) that's built from scratch and is compatible with [GTK4](https://www.gtk.org/).

## Goals

+ ⏱️ **Productivity:** Writing an application should require as few overhead as possible
+ ⚡ **Flexibility:** Anything that's possible to do with GTK4 should be possible in Relm4 as well
+ ✨ **Simplicity:** Writing an application should be as easy and straight forward as possible
+ 🔧 **Maintainability**: The Elm programming model used by Relm4 provides a simple and clear structure for app development

## Dependencies

Relm4 only depends on GTK4: [How to install GTK4](https://www.gtk.org/docs/installations/)

## Ecosystem

Relm4 has two crates that extend the core functionality:

+ [relm4-macros](https://crates.io/crates/relm4-macros) provides a `widget` macro that simplifies UI creation
+ [relm4-components](https://crates.io/crates/relm4-components) is a collections of reusable components you can easily integrate into your application

Add this to your `Cargo.toml`:

```toml
relm4 = "0.1.0-beta.2"
relm4-macros = "0.1.0-beta.2"
relm4-components = "0.1.0-beta.2"
```

## Examples

Several example applications are available at [relm4-examples/](relm4-examples/).

**Feedback on the design and contributions are highly appreciated!**
