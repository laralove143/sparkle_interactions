<div align="center">
   <a href="https://github.com/laralove143/sparkle_interactions/issues">
      <img alt="Stars Badge" height="20" src="https://m3-markdown-badges.vercel.app/stars/2/1/laralove143/sparkle_interactions"/>
   </a>
   <a href="https://github.com/laralove143/sparkle_interactions/stargazers">
      <img alt="Issues Badge" height="20" src="https://m3-markdown-badges.vercel.app/issues/2/1/laralove143/sparkle_interactions"/>
   </a>
   <a href="https://www.rust-lang.org">
      <img alt="Rust" height="20" src="https://ziadoua.github.io/m3-Markdown-Badges/badges/Rust/rust1.svg"/>
   </a>
   <a href="https://github.com/laralove143/sparkle_interactions/tree/main?tab=MIT-1-ov-file">
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
   </a><br>
   <a href="https://docs.rs/sparkle_interactions">
      <img alt="Documentation" height="25" src="https://img.shields.io/docsrs/sparkle_interactions?logo=docsdotrs&label=Documentation&labelColor=555F6F&color=77DD77">
   </a>
   <a href="https://crates.io/crates/sparkle_interactions">
      <img alt="Downloads" height="25" src="https://img.shields.io/crates/d/sparkle-interactions?logo=rust&label=Downloads&labelColor=555F6F&color=77DD77">
   </a>
</div>

# ✨📄 Sparkle Interactions

Safe, concise Discord interaction handling for [Twilight](https://api.twilight.rs)

## ✨ Features

### Responding to Interactions

An interaction handle is provided to respond to interactions, which holds the state of the interaction.

The handle has a `respond` method which determines whether to create an interaction response or a followup message using the state.

See module documentation of `handle` module for more.

### Tracking Last Response

Optionally, the interaction handle can track the last message sent. Methods are then provided to return or update the last message.

See module documentation of `handle` module for more.

### Extracting Interaction Data

This library provides methods to extract interaction data, command options, and modal components concisely.

See module documentation of the `extract` module for more.

### Builders

Builders for components and interaction responses are given by the library, allowing you to create these structs concisely and safely.

See module documentation of the `builder` module for more.

## 🏷️ Versioning

Since this crate is a third party crate for Twilight, its minor version follows the minor version of Twilight that this crate supports.

For example, for a version of the crate that supports Twilight `0.15.4`, this crate's version will be `0.15.x`, where `x` can be any number.

## 🧪 Testing

This crate makes use of unit tests and integration tests. Unit tests can be run as usual, while integration tests require some setup.

Since integration tests receive Discord events and make requests to Discord, you should first set these environment variables in a `.env` file located in the root of this repository:

- `BOT_TOKEN`: The token of the bot to use for testing
- `GUILD_ID`: The ID of the server where the test command will be created
- `CHANNEL_ID`: The ID of the channel where the test message with a component will be created

Each test runs on an interaction. Since bots can't create interactions, when running the test, the bot will wait for an `InteractionCreate` event to use for testing.

For tests that don't run on component interactions, you should first run the test then send the `sparkle_interactions_test` command manually after which point the bot runs the test on the interaction you created.

For tests that run on component interactions, the bot will send a message with the component upon running the test. Once you click on the component, testing will continue.

## 🙋 Wanted: Issues

While this crate is tested heavily, there may still be some bugs; or you might have some awesome ideas. Please to create issues or PRs for bug reports, suggestions, and additions.
