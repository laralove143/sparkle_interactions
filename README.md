<!--suppress HtmlDeprecatedAttribute -->
<div align="center">
   <a href="https://github.com/laralove143/sparkle_template/issues">
      <img alt="Stars Badge" height="20" src="https://m3-markdown-badges.vercel.app/stars/2/1/laralove143/sparkle_template"/>
   </a>
   <a href="https://github.com/laralove143/sparkle_template/stargazers">
      <img alt="Issues Badge" height="20" src="https://m3-markdown-badges.vercel.app/issues/2/1/laralove143/sparkle_template"/>
   </a>
   <a href="https://www.rust-lang.org">
      <img alt="Rust" height="20" src="https://ziadoua.github.io/m3-Markdown-Badges/badges/Rust/rust1.svg"/>
   </a>
   <a href="https://github.com/laralove143/sparkle_template/tree/main?tab=MIT-1-ov-file">
      <img alt="MIT License" height="20" src="https://ziadoua.github.io/m3-Markdown-Badges/badges/LicenceMIT/licencemit1.svg"/>
   </a><br>   
   <a href="mailto:me@lara.lv">
      <img alt="Mail" height="30" src="https://ziadoua.github.io/m3-Markdown-Badges/badges/Mail/mail1.svg"/>
   </a>
   <a href="https://discord.lara.lv">
      <img alt="Discord" height="30" src="https://ziadoua.github.io/m3-Markdown-Badges/badges/Discord/discord1.svg"/>
   </a>
   <a href="https://github.com/sponsors/laralove143">
      <img alt="Sponsor" height="30" src="https://ziadoua.github.io/m3-Markdown-Badges/badges/Sponsor/sponsor1.svg"/>
   </a>
</div>

# ✨📄 Sparkle Template

Opinionated template for [Rust](https://www.rust-lang.org) projects using [Twilight](https://api.twilight.rs)

## ✨ Features

### Common Twilight Crates Included

Boilerplate for every Twilight crate with every feature enabled is in this template, you can simply remove the things
you don't need

For Twilight Gateway and Twilight HTTP, `rustls-webpki-roots`, `simd-json`, `zlib-simd` and `trust-dns` are enabled

These crates aren't included because they aren't used very often:

- `twilight-gateway-queue`
- `twilight-http-ratelimiting`
- `twilight-lavalink`

### Context

A `Context` struct is given to share stateful data throughout the program, such as `twilight_http::Client`

The struct is a wrapper around `Arc<ContextInner>` and implements `Clone`, and `Deref` to `ContextInner`, this provides
an abstraction over the `Arc` management while making `Context` cheap to clone and safe to use in multi-threads

### Event Handling

Each event is handled in a separate `tokio` task spawned with `tokio::spawn`

`handle_event` method on `Context` is called on each event, which handles the `Ready` and `InteractionCreate` events

### Error Handling with Anyhow

[`anyhow`](https://docs.rs/anyhow) is used to handle errors in a polymorphic way

The executable uses `anyhow` to create ad-hoc errors

Panics and errors also include backtraces

### Configuration with Environment Variables

Environment variables are used to set info such as the bot's token

A `Config` struct is provided to centralize all the configuration, which has a constructor that loads it from
environment variables

Support for `.env` files is added with the [`dotenvy`](https://docs.rs/dotenvy) crate

### Conventional Interaction Handling

`InteractionContext` is given to create responses with less boilerplate

The trait `CreateCommand` defines a method to build the `Command`, the trait `RunInteraction` defines methods to run the
interaction

Each interaction should have its own module under the `interaction` module, the module should define a struct with the
data required while running the interaction, including `InteractionContext`

The traits `CreateCommand` and `RunInteraction` are meant to be defined on the interaction's struct, they define methods
to build and run the interaction

Some methods are implemented on `Context` to streamline managing interactions:

- **`interaction_client`**: Given to easily access `InteractionClient`
- **`set_commands`**: Set the commands of the bot, this is called in the `main` function

Additionally, `Context` has a `handle_interaction` method that runs a given `Interaction`, which extracts `custom_id`
and matches on it to run the appropriate command

A sample implementation of an interaction can be found at `interaction/mock.rs`

### Tracing

[`tracing-subscriber`](https://docs.rs/tracing-subscriber) crate is configured to use the environment
variable `RUST_LOG` to print [`tracing`](https://docs.rs/tracing) messages, the messages are formatted with the `pretty`
configuration

[`tracing-journald`](https://docs.rs/tracing-journald) crate is used to log `tracing` messages
to [`journald`](https://www.freedesktop.org/software/systemd/man/latest/systemd-journald)

`tracing` is used for the messages generated in the executable too

Compatibility between [`log`](https://docs.rs/log) and `tracing` is provided
with [`tracing-log`](https://docs.rs/tracing-log)

### This `README`

You can use this `README` as a template for your own `README`. It includes badges and a structure of headings.

## 🤔 Usage

1. Click on _Use this template_ in the GitHub web UI
    - Or copy-paste the code in some other way
2. Replace occurrences of `FILLME` with the actual values
3. Update name on the `LICENSE` file
4. Remove lines with `suppress` if you don't want compatibility with JetBrains IDEs

## 🙋 Wanted: Issues

Feel free to create an issue or PR for suggestions
