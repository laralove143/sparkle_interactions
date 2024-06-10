//! Extracting data from [`Interaction`]
//!
//! # General Usage Flow
//!
//! Extracting a value from an interaction involves the use of one, two or three
//! traits, depending on the kind of interaction.
//!
//! ## Extracting Data from [`InteractionData`]
//!
//! The [`Interaction`] struct contains an enum, [`InteractionData`], for the
//! data in each [`InteractionType`]. [`ExtractInteractionData`] can be used to
//! concisely extract the wrapped value in this enum. It can also be used to
//! extract the custom ID or name of the interaction without pattern matching.
//!
//! For message component interactions, the extracted data is all you need to
//! access the values.
//!
//! ## Extracting Options from Application Commands
//!
//! [`InteractionData`] contains [`CommandData`] with the options in an
//! application command. [`ExtractOption`] can be used to find
//! [`CommandOptionValue`]s, including ones in subcommands and subcommand
//! groups.
//!
//! [`CommandOptionValue`] is an enum for each kind of option.
//! [`ExtractOptionValue`] can be used to concisely extract the wrapped value in
//! this enum.
//!
//! ## Extracting Components from Modal Interactions
//!
//! The last type of interactions is modal interactions. [`InteractionData`]
//! contains [`ModalInteractionData`] with the components in a modal. The
//! components are nested vectors for each action row, which can complicate
//! finding the value of components. [`ExtractModalComponent`] can be used to
//! find the value of a component.
//!
//! ## Examples
//!
//! ### Application Commands
//!
//! Say we have an application command with a subcommand group named "fun", with
//! a subcommand named "annoy", which has an option named "user". We can get the
//! selected user in the options with this example.
//!
//! ```rust
//! # use sparkle_interactions::extract::{ExtractInteractionData, ExtractOption};
//! # use twilight_model::{
//! #     application::{
//! #         command::CommandType,
//! #         interaction::{
//! #             application_command::{CommandData, CommandDataOption, CommandOptionValue},
//! #             Interaction, InteractionData, InteractionType,
//! #         },
//! #     },
//! #     id::Id,
//! # };
//! #
//! # let interaction = Interaction {
//! #     app_permissions: None,
//! #     application_id: Id::new(1),
//! #     channel: None,
//! #     #[allow(deprecated)]
//! #     channel_id: None,
//! #     data: Some(InteractionData::ApplicationCommand(Box::new(CommandData {
//! #         guild_id: None,
//! #         id: Id::new(1),
//! #         name: String::new(),
//! #         kind: CommandType::ChatInput,
//! #         options: vec![CommandDataOption {
//! #             name: "fun".to_owned(),
//! #             value: CommandOptionValue::SubCommandGroup(vec![CommandDataOption {
//! #                 name: "annoy".to_owned(),
//! #                 value: CommandOptionValue::SubCommand(vec![CommandDataOption {
//! #                     name: "user".to_owned(),
//! #                     value: CommandOptionValue::User(Id::new(1)),
//! #                 }]),
//! #             }]),
//! #         }],
//! #         resolved: None,
//! #         target_id: None,
//! #     }))),
//! #     guild_id: None,
//! #     guild_locale: None,
//! #     id: Id::new(1),
//! #     kind: InteractionType::ApplicationCommand,
//! #     locale: None,
//! #     member: None,
//! #     message: None,
//! #     token: String::new(),
//! #     user: None,
//! # };
//! #
//! interaction
//!     .data?
//!     .command_data()?
//!     .options
//!     .option("fun/annoy/user")?;
//! ```
//!
//! ### Message Component Interactions
//!
//! Say we have a select menu component with the possible values "Rust", "C++"
//! and "C". We can find which values the user picked with this example.
//!
//! ```rust
//! # use twilight_model::{
//! #     application::interaction::{
//! #         message_component::MessageComponentInteractionData, Interaction, InteractionData,
//! #         InteractionType,
//! #     },
//! #     channel::message::component::ComponentType,
//! #     id::Id,
//! # };
//! # use sparkle_interactions::extract::ExtractInteractionData;
//! #
//! # let interaction = Interaction {
//! #     app_permissions: None,
//! #     application_id: Id::new(1),
//! #     channel: None,
//! #     #[allow(deprecated)]
//! #     channel_id: None,
//! #     data: Some(InteractionData::MessageComponent(
//! #         MessageComponentInteractionData {
//! #             custom_id: "a".to_owned(),
//! #             component_type: ComponentType::SelectMenu,
//! #             values: vec!["Rust".to_owned(), "C++".to_owned(), "C".to_owned()],
//! #         },
//! #     )),
//! #     guild_id: None,
//! #     guild_locale: None,
//! #     id: Id::new(1),
//! #     kind: InteractionType::MessageComponent,
//! #     locale: None,
//! #     member: None,
//! #     message: None,
//! #     token: String::new(),
//! #     user: None,
//! # };
//! #
//! interaction.data?.component_data()?.values;
//! ```
//!
//! ### Modal Interactions
//!
//! Say we have a modal interaction that has a text input component with the
//! custom ID `name_surname`. We can get the input value of this component with
//! this example.
//!
//! ```rust
//! # use sparkle_interactions::extract::{
//! #     ExtractInteractionData, ExtractModalComponent, ExtractOption,
//! # };
//! # use twilight_model::{
//! #     application::interaction::{
//! #         modal::{
//! #             ModalInteractionData, ModalInteractionDataActionRow,
//! #              ModalInteractionDataComponent,
//! #         },
//! #         Interaction, InteractionData, InteractionType,
//! #     },
//! #     channel::message::component::ComponentType,
//! #     id::Id,
//! # };
//! #
//! # let interaction = Interaction {
//! #     app_permissions: None,
//! #     application_id: Id::new(1),
//! #     channel: None,
//! #     #[allow(deprecated)]
//! #     channel_id: None,
//! #     data: Some(InteractionData::ModalSubmit(ModalInteractionData {
//! #         components: vec![ModalInteractionDataActionRow {
//! #             components: vec![ModalInteractionDataComponent {
//! #                 custom_id: "name_surname".to_owned(),
//! #                 kind: ComponentType::TextInput,
//! #                 value: Some("a".to_owned()),
//! #             }],
//! #         }],
//! #         custom_id: "".to_string(),
//! #     })),
//! #     guild_id: None,
//! #     guild_locale: None,
//! #     id: Id::new(1),
//! #     kind: InteractionType::ModalSubmit,
//! #     locale: None,
//! #     member: None,
//! #     message: None,
//! #     token: String::new(),
//! #     user: None,
//! # };
//! #
//! interaction
//!     .data?
//!     .modal_data()?
//!     .components
//!     .component("name_surname")?;
//! ```
//!
//! # Differences from Twilight Interactions
//!
//! The [`twilight_interactions`](https://docs.rs/twilight-interactions) crate also provides a way
//! to extract command options.
//!
//! The main difference between this crate's approach and Twilight Interactions'
//! approach is that this crate doesn't rely on macros. This means you have more
//! control over how the extraction happens, but the difference mostly comes
//! down to preference.
//!
//! This crate also provides additional features like extracting modal
//! components. Conversely, Twilight Interactions provides a way to create
//! commands with macros, which is not a goal of this crate. So, these two
//! crates can be used together to have the features of both.

#[cfg(doc)]
use twilight_model::application::interaction::{
    application_command::{CommandData, CommandDataOption, CommandOptionValue},
    modal::ModalInteractionData,
    Interaction, InteractionData, InteractionType,
};

mod interaction_data;
mod modal_component;
mod option;

pub use interaction_data::ExtractInteractionData;
pub use modal_component::ExtractModalComponent;
pub use option::{ExtractOption, ExtractOptionValue};
