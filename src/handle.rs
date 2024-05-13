//! Responding to interactions concisely
//!
//! [`InteractionHandle::respond`] centralizes creating the first response and
//! creating a followup by tracking whether the interaction was responded to.
//!
//! # Example
//!
//! ```rust
//! # use std::sync::Arc;
//! # use std::time::Duration;
//! # use sparkle_interactions::{builder::InteractionResponseBuilder, InteractionHandle};
//! # use twilight_http::Client;
//! # use twilight_model::{
//! #     application::interaction::{Interaction, InteractionType},
//! #     http::interaction::InteractionResponseData,
//! #     id::Id,
//! # };
//! #
//! # async fn foo() {
//! # let interaction = Interaction {
//! #     app_permissions: None,
//! #     application_id: Id::new(1),
//! #     channel: None,
//! #     #[allow(deprecated)]
//! #     channel_id: None,
//! #     data: None,
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
//! # let response_data = InteractionResponseData {
//! #     allowed_mentions: None,
//! #     attachments: None,
//! #     choices: None,
//! #     components: None,
//! #     content: None,
//! #     custom_id: None,
//! #     embeds: None,
//! #     flags: None,
//! #     title: None,
//! #     tts: None,
//! # };
//! #
//! # let client = Arc::new(Client::new("a".to_owned()));
//! # let application_id = Id::new(1);
//! let handle = InteractionHandle::new(client, application_id, interaction.id, interaction.token)
//!     .track_last_message()
//!     .respond_on_delay(
//!         InteractionResponseBuilder::defer_send_message().build(),
//!         Duration::from_secs(2),
//!     );
//!
//! tokio::time::sleep(Duration::from_secs(3)).await;
//! // interaction is deferred here
//!
//! handle
//!     .respond(InteractionResponseBuilder::send_message(
//!         response_data.clone(),
//!     ))
//!     .await?;
//! // interaction response is created here
//!
//! handle
//!     .respond(InteractionResponseBuilder::send_message(
//!         response_data.clone(),
//!     ))
//!     .await?;
//! // followup message is created here
//!
//! handle.update_last(InteractionResponseBuilder::send_message(
//!     response_data.clone(),
//! ))
//! // followup message is updated here
//! # }
//! ```

use std::{
    fmt::{Display, Formatter},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
};

use twilight_http::{client::InteractionClient, response::DeserializeBodyError, Client, Response};
use twilight_model::{
    channel::Message,
    http::interaction::InteractionResponse,
    id::{
        marker::{ApplicationMarker, InteractionMarker, MessageMarker},
        Id,
    },
};
use twilight_validate::message::MessageValidationError;

/// Errors returned while responding to interactions
#[derive(Debug)]
pub enum Error {
    /// A [`DeserializeBodyError`] was returned
    DeserializeBody(DeserializeBodyError),
    /// An error was returned by [`twilight_http`]
    Http(twilight_http::Error),
    /// A [`MessageValidationError`] was returned
    MessageValidation(MessageValidationError),
    /// Tried to return the last message when it isn't tracked
    LastMessageNotTracked,
}

impl Display for Error {
    fn fmt(&self, #[allow(clippy::min_ident_chars)] f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeserializeBody(err) => err.fmt(f),
            Self::Http(err) => err.fmt(f),
            Self::MessageValidation(err) => err.fmt(f),
            Self::LastMessageNotTracked => {
                f.write_str("tried to return the last message when it isn't tracked")
            }
        }
    }
}

impl From<DeserializeBodyError> for Error {
    fn from(err: DeserializeBodyError) -> Self {
        Self::DeserializeBody(err)
    }
}

impl From<twilight_http::Error> for Error {
    fn from(err: twilight_http::Error) -> Self {
        Self::Http(err)
    }
}

impl From<MessageValidationError> for Error {
    fn from(err: MessageValidationError) -> Self {
        Self::MessageValidation(err)
    }
}

impl std::error::Error for Error {}

/// Response returned from responding to an interaction
#[derive(Debug)]
pub enum FollowupResponse {
    /// The response is already deserialized
    Deserialized(Message),
    /// The response is not deserialized
    NotDeserialized(Response<Message>),
    /// The response is empty
    None,
}

impl FollowupResponse {
    /// Returns the response's message.
    ///
    /// Deserializes the message if it's not already deserialized.
    ///
    /// Returns `None` if the response is of variant [`FollowupResponse::None`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::DeserializeBody`] if the response is of variant
    /// [`FollowupResponse::NotDeserialized`] and the response couldn't be
    /// deserialized.
    pub async fn model(self) -> Result<Option<Message>, Error> {
        match self {
            Self::Deserialized(message) => Ok(Some(message)),
            Self::NotDeserialized(message) => Ok(Some(message.model().await?)),
            Self::None => Ok(None),
        }
    }
}

/// Struct for responding to interactions
///
/// Holds stateful data to create a valid response to an interaction.
///
/// It can be cloned safely without losing stateful data, it is also
/// thread-safe.
#[derive(Clone, Debug)]
pub struct InteractionHandle {
    application_id: Id<ApplicationMarker>,
    client: Arc<Client>,
    id: Id<InteractionMarker>,
    token: String,
    is_responded: Arc<AtomicBool>,
    last_message_id: Arc<AtomicU64>,
    is_last_message_tracked: bool,
}

impl InteractionHandle {
    const LOAD_ORDERING: Ordering = Ordering::Acquire;
    const STORE_ORDERING: Ordering = Ordering::Release;

    /// Create a new handle for an interaction
    ///
    /// # Warning
    ///
    /// Create only one handle per interaction. Otherwise, the interaction's
    /// state will be lost.
    #[must_use]
    pub fn new(
        client: Arc<Client>,
        application_id: Id<ApplicationMarker>,
        interaction_id: Id<InteractionMarker>,
        token: String,
    ) -> Self {
        Self {
            application_id,
            client,
            id: interaction_id,
            token,
            is_responded: Arc::new(AtomicBool::new(false)),
            last_message_id: Arc::new(AtomicU64::new(0)),
            is_last_message_tracked: false,
        }
    }

    /// Set the handle to track the last message to be able to use
    /// [`InteractionHandle::update_last`] and
    /// [`InteractionHandle::last_message`]
    ///
    /// This makes [`InteractionHandle::respond`] deserialize every request.
    ///
    /// # Warning
    ///
    /// This must be called before any method except [`InteractionHandle::new`]
    /// is called.
    #[must_use]
    pub const fn track_last_message(mut self) -> Self {
        self.is_last_message_tracked = true;
        self
    }

    /// Return the [`InteractionClient`] for this handle.
    #[must_use]
    pub fn client(&self) -> InteractionClient<'_> {
        self.client.interaction(self.application_id)
    }

    /// Send a response if no other response has been sent in the given timeout
    /// period
    ///
    /// This can be used to defer interactions if they aren't responded to
    /// within the 3-second period Discord allows for the first response.
    /// The response can be a defer response or a custom response.
    ///
    /// # Warnings
    ///
    /// This must be called as soon as possible after an `InteractionCreate`
    /// event was received to ensure the accuracy of the wait-period.
    ///
    /// Due to complexities involving error handling in a non-blocking method,
    /// if sending the response fails, it is ignored.
    /// Though, if an error did occur, it can be detected in a later response to
    /// the interaction.
    #[cfg(feature = "respond_on_delay")]
    #[must_use]
    pub fn respond_on_delay(
        self,
        response: InteractionResponse,
        delay: std::time::Duration,
    ) -> Self {
        let handle = self.clone();

        tokio::spawn(async move {
            tokio::time::sleep(delay).await;

            if !self.is_responded() {
                drop(self.respond(response).await);
            }
        });

        handle
    }

    /// Respond to the interaction with the given response.
    ///
    /// If this is the first response, it creates a new response, otherwise
    /// it creates a followup response.
    ///
    /// Returns [`FollowupResponse::None`] if this is the first response,
    /// [`FollowupResponse::NotDeserialized`] if it isn't,
    /// [`FollowupResponse::Deserialized`] if it isn't and
    /// [`InteractionHandle::track_last_message`] was called.
    ///
    /// There is a builder for [`InteractionResponse`] at
    /// [`InteractionResponseBuilder`]
    ///
    /// # Errors
    ///
    /// Returns [`Error::MessageValidation`] if response isn't a valid followup
    /// message.
    ///
    /// Returns [`Error::DeserializeBody`] if
    /// [`InteractionHandle::track_last_message`] was called and the response
    /// couldn't be deserialized.
    ///
    /// Returns [`Error::Http`] if a request failed.
    ///
    /// [`InteractionResponseBuilder`]: crate::builder::InteractionResponseBuilder
    pub async fn respond(&self, response: InteractionResponse) -> Result<FollowupResponse, Error> {
        if self.is_responded() {
            let followup_response = self.create_followup(response).await?;

            if self.is_last_message_tracked {
                let message = followup_response.model().await?;

                self.set_last_message_id(message.id.get());

                Ok(FollowupResponse::Deserialized(message))
            } else {
                Ok(FollowupResponse::NotDeserialized(followup_response))
            }
        } else {
            self.client()
                .create_response(self.id, &self.token, &response)
                .await?;

            self.set_is_responded(true);

            Ok(FollowupResponse::None)
        }
    }

    /// Update the last response to the interaction
    ///
    /// There is a builder for [`InteractionResponse`] at
    /// [`InteractionResponseBuilder`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::LastMessageNotTracked`] if
    /// [`InteractionHandle::track_last_message`] wasn't called before.
    ///
    /// Returns [`Error::MessageValidation`] if response isn't a valid followup
    /// message.
    ///
    /// Returns [`Error::Http`] if a request failed.
    pub async fn update_last(
        &self,
        response: InteractionResponse,
    ) -> Result<Response<Message>, Error> {
        let interaction_client = self.client();

        if let Some(last_message_id) = self.last_message_id()? {
            let mut update_followup =
                interaction_client.update_followup(&self.token, last_message_id);

            let Some(data) = response.data else {
                return Ok(update_followup.await?);
            };

            if let Some(attachments) = &data.attachments {
                update_followup = update_followup.attachments(attachments)?;
            }

            update_followup = update_followup.components(data.components.as_deref())?;
            update_followup = update_followup.content(data.content.as_deref())?;
            update_followup = update_followup.embeds(data.embeds.as_deref())?;

            Ok(update_followup.await?)
        } else {
            let mut update_response = interaction_client.update_response(&self.token);

            let Some(data) = response.data else {
                return Ok(update_response.await?);
            };

            if let Some(attachments) = &data.attachments {
                update_response = update_response.attachments(attachments)?;
            }

            update_response = update_response.components(data.components.as_deref())?;
            update_response = update_response.content(data.content.as_deref())?;
            update_response = update_response.embeds(data.embeds.as_deref())?;

            Ok(update_response.await?)
        }
    }

    /// Return the last followup message sent to the interaction.
    ///
    /// Returns `None` if no followup message has been sent yet.
    ///
    /// # Errors
    ///
    /// Returns [`Error::LastMessageNotTracked`] if
    /// [`InteractionHandle::track_last_message`] wasn't called before.
    pub fn last_message_id(&self) -> Result<Option<Id<MessageMarker>>, Error> {
        if !self.is_last_message_tracked {
            return Err(Error::LastMessageNotTracked);
        }

        let message_id = self.last_message_id.load(Self::LOAD_ORDERING);

        if message_id == 0 {
            Ok(None)
        } else {
            Ok(Some(Id::new(message_id)))
        }
    }

    async fn create_followup(
        &self,
        response: InteractionResponse,
    ) -> Result<Response<Message>, Error> {
        let interaction_client = self.client();
        let mut create_followup = interaction_client.create_followup(&self.token);

        let Some(data) = response.data else {
            return Ok(create_followup.await?);
        };

        if let Some(attachments) = &data.attachments {
            create_followup = create_followup.attachments(attachments)?;
        }

        if let Some(components) = &data.components {
            create_followup = create_followup.components(components)?;
        }

        if let Some(content) = &data.content {
            create_followup = create_followup.content(content)?;
        }

        if let Some(embeds) = &data.embeds {
            create_followup = create_followup.embeds(embeds)?;
        }

        if let Some(flags) = data.flags {
            create_followup = create_followup.flags(flags);
        }

        if let Some(tts) = data.tts {
            create_followup = create_followup.tts(tts);
        }

        Ok(create_followup.await?)
    }

    fn is_responded(&self) -> bool {
        self.is_responded.load(Self::LOAD_ORDERING)
    }

    fn set_is_responded(&self, value: bool) {
        self.is_responded.store(value, Self::STORE_ORDERING);
    }

    fn set_last_message_id(&self, value: u64) {
        self.last_message_id.store(value, Self::STORE_ORDERING);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use twilight_http::Client;
    use twilight_model::id::Id;

    use crate::InteractionHandle;

    #[test]
    fn test_data_integrity() {
        let client = Client::new("a".to_owned());
        let handle =
            InteractionHandle::new(Arc::new(client), Id::new(1), Id::new(1), "a".to_owned());
        #[allow(clippy::redundant_clone)]
        let handle_clone = handle.clone();

        handle.set_is_responded(true);
        assert_eq!(handle.is_responded(), handle_clone.is_responded());
    }
}
