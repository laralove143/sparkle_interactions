//! Builders for components
//!
//! The entrypoint of the builders for buttons and select menus is
//! [`ComponentsBuilder`].
//! Although this is not enforced, it provides abstractions over managing action
//! rows, making your code safer.
//!
//! Similar abstractions exist for modals at
//! [`InteractionResponseBuilder::show_modal`].

use twilight_model::{
    channel::message::{
        component::{
            ActionRow, Button, ButtonStyle, SelectMenu, SelectMenuOption, TextInput, TextInputStyle,
        },
        Component, ReactionType,
    },
    id::{marker::EmojiMarker, Id},
};

#[cfg(doc)]
use crate::builder::InteractionResponseBuilder;

/// Create a [`Button`] with a builder.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ButtonBuilder {
    custom_id: Option<String>,
    disabled: bool,
    emoji: Option<ReactionType>,
    label: Option<String>,
    style: ButtonStyle,
    url: Option<String>,
}

impl ButtonBuilder {
    /// Create a new builder for buttons with a given custom ID.
    #[must_use]
    pub const fn with_custom_id(custom_id: String, label: String, style: ButtonStyle) -> Self {
        Self {
            custom_id: Some(custom_id),
            disabled: false,
            emoji: None,
            label: Some(label),
            style,
            url: None,
        }
    }

    /// Create a new builder for buttons with a given URL.
    #[must_use]
    pub const fn with_url(url: String, label: String) -> Self {
        Self {
            custom_id: None,
            disabled: false,
            emoji: None,
            label: Some(label),
            style: ButtonStyle::Link,
            url: Some(url),
        }
    }

    /// Make this button disabled.
    #[must_use]
    pub const fn disable(mut self) -> Self {
        self.disabled = true;

        self
    }

    // noinspection DuplicatedCode
    /// Set a custom emoji for this button.
    #[must_use]
    pub fn custom_emoji(mut self, name: String, id: Id<EmojiMarker>, animated: bool) -> Self {
        self.emoji = Some(ReactionType::Custom {
            animated,
            id,
            name: Some(name),
        });

        self
    }

    /// Set a unicode emoji for this button.
    #[must_use]
    pub fn unicode_emoji(mut self, emoji: String) -> Self {
        self.emoji = Some(ReactionType::Unicode { name: emoji });

        self
    }

    /// Consume this builder and return the configured [`Button`]
    #[must_use]
    pub fn build(self) -> Button {
        Button {
            custom_id: self.custom_id,
            disabled: self.disabled,
            emoji: self.emoji,
            label: self.label,
            style: self.style,
            url: self.url,
        }
    }
}

/// Create a [`SelectMenuOption`] with a builder.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SelectMenuOptionBuilder {
    default: bool,
    description: Option<String>,
    emoji: Option<ReactionType>,
    label: String,
    value: String,
}

impl SelectMenuOptionBuilder {
    /// Create a new builder for select menu options.
    #[must_use]
    pub const fn new(label: String, value: String) -> Self {
        Self {
            default: false,
            description: None,
            emoji: None,
            label,
            value,
        }
    }

    /// Set this select menu option to be selected by default.
    #[must_use]
    pub const fn default(mut self) -> Self {
        self.default = true;

        self
    }

    /// Set a description for this select menu option.
    #[must_use]
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);

        self
    }

    // noinspection DuplicatedCode
    /// Set a custom emoji for this select menu option.
    #[must_use]
    pub fn custom_emoji(mut self, name: String, id: Id<EmojiMarker>, animated: bool) -> Self {
        self.emoji = Some(ReactionType::Custom {
            animated,
            id,
            name: Some(name),
        });

        self
    }

    /// Set a unicode emoji for this select menu option.
    #[must_use]
    pub fn unicode_emoji(mut self, emoji: String) -> Self {
        self.emoji = Some(ReactionType::Unicode { name: emoji });

        self
    }

    /// Consume this builder and return the configured [`SelectMenuOption`]
    #[must_use]
    pub fn build(self) -> SelectMenuOption {
        SelectMenuOption {
            default: self.default,
            description: self.description,
            emoji: self.emoji,
            label: self.label,
            value: self.value,
        }
    }
}

/// Create a [`SelectMenu`] with a builder.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SelectMenuBuilder {
    custom_id: String,
    disabled: bool,
    max_values: Option<u8>,
    min_values: Option<u8>,
    options: Vec<SelectMenuOption>,
    placeholder: Option<String>,
}

impl SelectMenuBuilder {
    /// Create a new builder for select menus.
    #[must_use]
    pub fn new(custom_id: String, options: Vec<SelectMenuOption>) -> Self {
        Self {
            custom_id,
            disabled: false,
            max_values: None,
            min_values: None,
            options,
            placeholder: None,
        }
    }

    /// Make this select menu disabled.
    #[must_use]
    pub const fn disable(mut self) -> Self {
        self.disabled = true;

        self
    }

    /// Set the maximum number of options that can be chosen for this select
    /// menu.
    #[must_use]
    pub const fn max_values(mut self, max_values: u8) -> Self {
        self.max_values = Some(max_values);

        self
    }

    /// Set the minimum number of options that can be chosen for this select
    /// menu.
    #[must_use]
    pub const fn min_values(mut self, min_values: u8) -> Self {
        self.min_values = Some(min_values);

        self
    }

    /// Set the text to be shown when no options are selected.
    #[must_use]
    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);

        self
    }

    /// Consume this builder and return the configured [`SelectMenuOption`]
    #[must_use]
    pub fn build(self) -> SelectMenu {
        SelectMenu {
            custom_id: self.custom_id,
            disabled: self.disabled,
            max_values: self.max_values,
            min_values: self.min_values,
            options: self.options,
            placeholder: self.placeholder,
        }
    }
}

/// Create [`TextInput`] with a builder.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextInputBuilder {
    custom_id: String,
    label: String,
    max_length: Option<u16>,
    min_length: Option<u16>,
    placeholder: Option<String>,
    required: bool,
    style: TextInputStyle,
    value: Option<String>,
}

impl TextInputBuilder {
    /// Create a new builder for modals.
    #[must_use]
    pub const fn new(label: String, custom_id: String) -> Self {
        Self {
            custom_id,
            label,
            max_length: None,
            min_length: None,
            placeholder: None,
            required: false,
            style: TextInputStyle::Short,
            value: None,
        }
    }

    /// Set the maximum number of characters allowed to be entered into this
    /// text input.
    #[must_use]
    pub const fn max_length(mut self, max_length: u16) -> Self {
        self.max_length = Some(max_length);

        self
    }

    /// Set the minimum number of characters allowed to be entered into this
    /// text input.
    #[must_use]
    pub const fn min_length(mut self, min_length: u16) -> Self {
        self.min_length = Some(min_length);

        self
    }

    /// Set the placeholder of this text input.
    #[must_use]
    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);

        self
    }

    /// Make this text input required.
    #[must_use]
    pub const fn require(mut self) -> Self {
        self.required = true;

        self
    }

    /// Make this text input's style paragraph.
    #[must_use]
    pub const fn paragraph(mut self) -> Self {
        self.style = TextInputStyle::Paragraph;

        self
    }

    /// Set the prefilled value of this text input.
    #[must_use]
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);

        self
    }

    /// Consume this builder and return the configured [`TextInput`].
    #[must_use]
    pub fn build(self) -> TextInput {
        TextInput {
            custom_id: self.custom_id,
            label: self.label,
            max_length: self.max_length,
            min_length: self.min_length,
            placeholder: self.placeholder,
            required: Some(self.required),
            style: self.style,
            value: self.value,
        }
    }
}

/// Create a vector of [`Component`]s with a builder.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ComponentsBuilder {
    action_rows: Vec<ActionRow>,
}

impl Default for ComponentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentsBuilder {
    /// Create a new builder for components.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            action_rows: vec![],
        }
    }

    /// Add an action row of buttons.
    #[must_use]
    pub fn buttons(mut self, buttons: Vec<Button>) -> Self {
        self.action_rows.push(ActionRow {
            components: buttons.into_iter().map(Component::Button).collect(),
        });

        self
    }

    /// Add a select menu.
    ///
    /// Since an action row can't have multiple select menus or mix select menus
    /// and buttons, this is the only method for adding select menus.
    #[must_use]
    pub fn select_menu(mut self, select_menu: SelectMenu) -> Self {
        self.action_rows.push(ActionRow {
            components: vec![Component::SelectMenu(select_menu)],
        });

        self
    }

    /// Consume this builder and return the configured [`Component`]s.
    pub fn build(self) -> Vec<Component> {
        self.action_rows
            .into_iter()
            .map(Component::ActionRow)
            .collect()
    }
}
