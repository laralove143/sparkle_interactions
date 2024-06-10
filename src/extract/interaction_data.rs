use twilight_model::application::interaction::{
    application_command::CommandData, message_component::MessageComponentInteractionData,
    modal::ModalInteractionData, InteractionData,
};

/// Trait implemented on [`InteractionData`] to extract its variants without
/// using pattern matching
pub trait ExtractInteractionData {
    /// Extract the name or custom ID of an interaction
    ///
    /// For [`InteractionData::ApplicationCommand`], this returns the name, for
    /// other kinds, it returns the custom ID
    ///
    /// Returns `None` if the interaction is not a command, component or modal.
    /// This is because [`InteractionData`] is marked non-exhaustive.
    fn custom_id(&self) -> Option<&str>;

    /// Extract [`CommandData`] from an interaction
    ///
    /// Returns `None` if the interaction is not an application command
    fn command_data(self) -> Option<CommandData>;

    /// Extract [`MessageComponentInteractionData`] from an interaction
    ///
    /// Returns `None` if the interaction is not a message component
    fn component_data(self) -> Option<MessageComponentInteractionData>;

    /// Extract [`ModalInteractionData`] from an interaction
    ///
    /// Returns `None` if the interaction is not a modal submit interaction.
    fn modal_data(self) -> Option<ModalInteractionData>;
}

impl ExtractInteractionData for InteractionData {
    fn custom_id(&self) -> Option<&str> {
        match self {
            InteractionData::ApplicationCommand(command) => Some(&command.name),
            InteractionData::MessageComponent(component) => Some(&component.custom_id),
            InteractionData::ModalSubmit(modal) => Some(&modal.custom_id),
            _ => None,
        }
    }

    fn command_data(self) -> Option<CommandData> {
        if let Self::ApplicationCommand(data) = self {
            Some(*data)
        } else {
            None
        }
    }

    fn component_data(self) -> Option<MessageComponentInteractionData> {
        if let Self::MessageComponent(data) = self {
            Some(data)
        } else {
            None
        }
    }

    fn modal_data(self) -> Option<ModalInteractionData> {
        if let Self::ModalSubmit(data) = self {
            Some(data)
        } else {
            None
        }
    }
}
