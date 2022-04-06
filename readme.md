# Ruquotes

[![Documentation](https://docs.rs/ruquotes/badge.svg)](https://docs.rs/ruquotes)
[![Code Coverage](https://codecov.io/gh/UltiRequiem/ruquotes/branch/main/graph/badge.svg)](https://codecov.io/gh/UltiRequiem/ruquotes)

API Wrapper and CLI tool for [Quotable](https://github.com/lukePeavey/quotable)
📣

## Installation

```sh
cargo install ruquotes
```

## Usage

```rust
use ruquotes::quote;

let custom_quote = quote().await?;

println!(
    "{} \n - {}",
    custom_quote.content.green(),
    custom_quote.author.cyan()
);
```

## CLI

### Install

The same way as the library.

Or use a binary from
[releases](https://github.com/UltiRequiem/ruquotes/releases/latest).

### Usage

```sh
quotable
```

[Video Showcase](https://youtu.be/IXKOg6IxP3Y) 📹

## Standing on the shoulders of giants

- 🗼 [tokio-rs](https://github.com/tokio-rs/tokio): A runtime for writing
  reliable asynchronous applications with Rust.

- 🤗 [colored](https://github.com/mackwic/colored): The easier way to have text
  on your term!

- 👏 [clap](https://github.com/clap-rs/clap): A full featured, fast Command Line
  Argument Parser for Rust

- ⚡ [reqwest](https://github.com/seanmonstar/reqwest): An easy and powerful Rust
  HTTP Client

## Authors

[Eliaz Bobadilla](https://ultirequiem.com) - Creator and Maintainer 💪

See also the full list of
[contributors](https://github.com/UltiRequiem/ruquotes/contributors) who
participated in this project ✨

## Versioning

We use [Semantic Versioning](http://semver.org). For the versions available, see
the [tags](https://github.com/UltiRequiem/ruquotes/tags) 🏷️

## Other Art

I also made this on [JavaScript](https://github.com/UltiRequiem/ranmess),
[Python](https://github.com/UltiRequiem/quoteran),[A website](https://github.com/UltiRequiem/ulti-random-quotes),
and [Go](https://github.com/UltiRequiem/quotable).

## Licence

Licensed under the MIT License 📄
