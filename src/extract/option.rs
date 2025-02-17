#[cfg(test)]
mod tests;

use twilight_model::{
    application::{
        command::CommandOptionType,
        interaction::application_command::{CommandDataOption, CommandOptionValue},
    },
    id::{
        Id,
        marker::{AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker},
    },
};

impl ExtractOptionValue for CommandOptionValue {
    fn attachment(&self) -> Option<Id<AttachmentMarker>> {
        if let Self::Attachment(attachment_id) = self {
            Some(*attachment_id)
        } else {
            None
        }
    }

    fn boolean(&self) -> Option<bool> {
        if let Self::Boolean(val) = self {
            Some(*val)
        } else {
            None
        }
    }

    fn channel(&self) -> Option<Id<ChannelMarker>> {
        if let Self::Channel(val) = self {
            Some(*val)
        } else {
            None
        }
    }

    fn focused(self) -> Option<(String, CommandOptionType)> {
        if let Self::Focused(name, option_type) = self {
            Some((name, option_type))
        } else {
            None
        }
    }

    fn integer(&self) -> Option<i64> {
        if let Self::Integer(data) = self {
            Some(*data)
        } else {
            None
        }
    }

    fn mentionable(&self) -> Option<Id<GenericMarker>> {
        if let Self::Mentionable(data) = self {
            Some(*data)
        } else {
            None
        }
    }

    fn number(&self) -> Option<f64> {
        if let Self::Number(data) = self {
            Some(*data)
        } else {
            None
        }
    }

    fn role(&self) -> Option<Id<RoleMarker>> {
        if let Self::Role(data) = self {
            Some(*data)
        } else {
            None
        }
    }

    fn string(self) -> Option<String> {
        if let Self::String(data) = self {
            Some(data)
        } else {
            None
        }
    }

    fn subcommand(self) -> Option<Vec<CommandDataOption>> {
        if let Self::SubCommand(data) = self {
            Some(data)
        } else {
            None
        }
    }

    fn subcommand_group(self) -> Option<Vec<CommandDataOption>> {
        if let Self::SubCommandGroup(data) = self {
            Some(data)
        } else {
            None
        }
    }

    fn user(&self) -> Option<Id<UserMarker>> {
        if let Self::User(data) = self {
            Some(*data)
        } else {
            None
        }
    }
}

/// Trait implemented on [`Vec<CommandDataOption>`] extract options
pub trait ExtractOption {
    /// Extract the value of the option with the given name
    ///
    /// Returns `None` if not found
    ///
    /// # Subcommands
    ///
    /// To extract the option from a subcommand or subcommand group, use the
    /// syntax `subcommand_name/option_name` or
    /// `subcommand_group_name/subcommand_name/option_name` in the name argument
    ///
    /// You can still use just `option_name` to extract the subcommand or
    /// subcommand group values
    fn option(self, name: &str) -> Option<CommandOptionValue>;
}

/// Trait implemented on [`CommandOptionValue`] to extract its variants without
/// using pattern matching
pub trait ExtractOptionValue {
    /// Extract [`Id<AttachmentMarker>`] from an option
    ///
    /// Returns `None` if the option is not an attachment option
    fn attachment(&self) -> Option<Id<AttachmentMarker>>;
    /// Extract `bool` from an option
    ///
    /// Returns `None` if the option is not a bool option
    fn boolean(&self) -> Option<bool>;
    /// Extract [`Id<ChannelMarker>`] from an option
    ///
    /// Returns `None` if the option is not a channel option
    fn channel(&self) -> Option<Id<ChannelMarker>>;
    /// Extract the value of the focused option
    ///
    /// Returns `None` if the option is not focused
    fn focused(self) -> Option<(String, CommandOptionType)>;
    /// Extract `i64` from an option
    ///
    /// Returns `None` if the option is not an integer option
    fn integer(&self) -> Option<i64>;
    /// Extract a mentionable value's ID from an option
    ///
    /// Returns `None` if the option is not a mentionable value option
    fn mentionable(&self) -> Option<Id<GenericMarker>>;
    /// Extract `f64` from an option
    ///
    /// Returns `None` if the option is not a number option
    fn number(&self) -> Option<f64>;
    /// Extract [`Id<RoleMarker>`] from an option
    ///
    /// Returns `None` if the option is not a role option
    fn role(&self) -> Option<Id<RoleMarker>>;
    /// Extract `String` from an option
    ///
    /// Returns `None` if the option is not a string option
    fn string(self) -> Option<String>;
    /// Extract a subcommand's options from an option
    ///
    /// Returns `None` if the option is not a subcommand option
    fn subcommand(self) -> Option<Vec<CommandDataOption>>;
    /// Extract a subcommand group's options from an option
    ///
    /// Returns `None` if the option is not a subcommand group option
    fn subcommand_group(self) -> Option<Vec<CommandDataOption>>;
    /// Extract [`Id<UserMarker>`] from an option
    ///
    /// Returns `None` if the option is not a user option
    fn user(&self) -> Option<Id<UserMarker>>;
}

impl ExtractOption for Vec<CommandDataOption> {
    fn option(self, name: &str) -> Option<CommandOptionValue> {
        let (option_name, subcommand_name_opt, subcommand_group_name_opt) = parse_option_name(name);

        if let (Some(subcommand_group_name), Some(subcommand_name)) =
            (subcommand_group_name_opt, subcommand_name_opt)
        {
            let subcommand_group_options =
                find_option(self, subcommand_group_name)?.subcommand_group()?;
            let subcommand_options =
                find_option(subcommand_group_options, subcommand_name)?.subcommand()?;

            find_option(subcommand_options, option_name)
        } else if let Some(subcommand_name) = subcommand_name_opt {
            let subcommand_options = find_option(self, subcommand_name)?.subcommand()?;

            find_option(subcommand_options, option_name)
        } else {
            find_option(self, option_name)
        }
    }
}

fn find_option(options: Vec<CommandDataOption>, name: &str) -> Option<CommandOptionValue> {
    options
        .into_iter()
        .find_map(|option| (option.name == name).then_some(option.value))
}

fn parse_option_name(name: &str) -> (&str, Option<&str>, Option<&str>) {
    let mut name_parts = name.split('/').rev();
    // option_name
    let option_name = name_parts.next().unwrap_or(name);
    // subcommand_name/option_name
    let subcommand_name_opt = name_parts.next();
    // subcommand_group_name/subcommand_name/option_name
    let subcommand_group_name_opt = name_parts.next();
    // .filter(|s| s.is_empty())
    (option_name, subcommand_name_opt, subcommand_group_name_opt)
}
