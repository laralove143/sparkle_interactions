//! Builders for [`InteractionResponse`]
//!
//! The entrypoint of this module is [`InteractionResponseBuilder`].
//! All the other builders can be created using methods on it.
use twilight_model::{
    channel::message::{
        Component,
        MessageFlags,
        component::{ActionRow, TextInput},
    },
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

/// Create an [`InteractionResponse`] to defer an interaction with a builder.
///
/// This is created with [`InteractionResponseBuilder::defer_send_message`] or
/// [`InteractionResponseBuilder::defer_update_message`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DeferInteractionResponseBuilder {
    is_ephemeral: bool,
    response_type: InteractionResponseType,
    suppress_embeds: bool,
}

impl DeferInteractionResponseBuilder {
    /// Consume this builder and return the configured [`InteractionResponse`]
    #[must_use]
    pub fn build(self) -> InteractionResponse {
        let mut flags = MessageFlags::empty();
        if self.is_ephemeral {
            flags.insert(MessageFlags::EPHEMERAL);
        }
        if self.suppress_embeds {
            flags.insert(MessageFlags::SUPPRESS_EMBEDS);
        }

        InteractionResponse {
            kind: self.response_type,
            data: Some(InteractionResponseData {
                allowed_mentions: None,
                attachments: None,
                choices: None,
                components: None,
                content: None,
                custom_id: None,
                embeds: None,
                flags: (!flags.is_empty()).then_some(flags),
                title: None,
                tts: None,
            }),
        }
    }

    /// Set the response to be ephemeral.
    ///
    /// This makes the response only visible to the user that created the
    /// interaction.
    #[must_use]
    pub const fn ephemeral(mut self) -> Self {
        self.is_ephemeral = true;
        self
    }

    /// Set the response to not show embeds.
    #[must_use]
    pub const fn suppress_embeds(mut self) -> Self {
        self.suppress_embeds = true;
        self
    }
}

/// Create an [`InteractionResponse`] with a builder.
///
/// Twilight Util has an [`InteractionResponseDataBuilder`] struct to build
/// interaction response data.
/// This builder creates [`InteractionResponse`]s from
/// [`InteractionResponseData`], except for
/// [`InteractionResponseBuilder::show_modal`]
///
/// [`InteractionResponseDataBuilder`]:
/// https://api.twilight.rs/twilight_util/builder/struct.InteractionResponseDataBuilder
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct InteractionResponseBuilder;

impl InteractionResponseBuilder {
    /// Defer an interaction to follow up with a message later.
    ///
    /// This method should be used when the message is expected to be sent after
    /// 3 seconds, as Discord requires a response in the first 3 seconds
    /// after an interaction is created.
    ///
    /// This uses [`InteractionResponseType::DeferredChannelMessageWithSource`]
    /// as the response type.
    #[must_use]
    pub const fn defer_send_message() -> DeferInteractionResponseBuilder {
        DeferInteractionResponseBuilder {
            response_type: InteractionResponseType::DeferredChannelMessageWithSource,
            is_ephemeral: false,
            suppress_embeds: false,
        }
    }

    /// Defer a message component or modal submit interaction.
    ///
    /// While [`InteractionResponseBuilder::defer_send_message`] sends a new
    /// message showing a loading state, this method shows the loading state on
    /// the button for a few seconds.
    /// Followups still behave like
    /// [`InteractionResponseBuilder::defer_send_message`].
    ///
    /// This method should be used when the message is expected to be sent after
    /// 3 seconds, as Discord requires a response in the first 3 seconds
    /// after an interaction is created.
    ///
    /// This uses [`InteractionResponseType::DeferredUpdateMessage`]
    /// as the response type.
    #[must_use]
    pub const fn defer_update_message() -> DeferInteractionResponseBuilder {
        DeferInteractionResponseBuilder {
            response_type: InteractionResponseType::DeferredUpdateMessage,
            is_ephemeral: false,
            suppress_embeds: false,
        }
    }

    /// Create a response to a ping from Discord.
    ///
    /// This uses [`InteractionResponseType::Pong`] as the response type.
    #[must_use]
    pub const fn pong() -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None,
        }
    }

    /// Respond to an interaction with a message.
    ///
    /// This uses [`InteractionResponseType::ChannelMessageWithSource`] as the
    /// response type.
    #[must_use]
    pub const fn send_message(data: InteractionResponseData) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data),
        }
    }

    /// Respond to an interaction with a modal.
    ///
    /// This uses [`InteractionResponseType::Modal`] as the
    /// response type.
    #[must_use]
    pub const fn show_modal(title: String, custom_id: String) -> ModalInteractionResponseBuilder {
        ModalInteractionResponseBuilder {
            action_rows: vec![],
            custom_id,
            title,
        }
    }

    /// Respond to a message component or modal interaction to edit the message
    /// with the component.
    ///
    /// This uses [`InteractionResponseType::UpdateMessage`] as the
    /// response type.
    #[must_use]
    pub const fn update_message(data: InteractionResponseData) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::UpdateMessage,
            data: Some(data),
        }
    }
}

/// Create an [`InteractionResponse`] to show a modal with a builder.
///
/// This is created with [`InteractionResponseBuilder::show_modal`].
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ModalInteractionResponseBuilder {
    action_rows: Vec<ActionRow>,
    custom_id: String,
    title: String,
}

impl ModalInteractionResponseBuilder {
    /// Consume this builder and return the configured
    /// [`InteractionResponse`].
    pub fn build(self) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::Modal,
            data: Some(InteractionResponseData {
                allowed_mentions: None,
                attachments: None,
                choices: None,
                components: Some(
                    self.action_rows
                        .into_iter()
                        .map(Component::ActionRow)
                        .collect(),
                ),
                content: None,
                custom_id: Some(self.custom_id),
                embeds: None,
                flags: None,
                title: Some(self.title),
                tts: None,
            }),
        }
    }

    /// Add a text input component to this modal.
    #[must_use]
    pub fn text_input(mut self, text_input: TextInput) -> Self {
        self.action_rows.push(ActionRow {
            components: vec![Component::TextInput(text_input)],
        });

        self
    }
}
