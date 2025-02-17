use twilight_model::application::interaction::application_command::{
    CommandDataOption,
    CommandOptionValue,
};

use crate::extract::ExtractOption;

#[test]
fn test_extract_option() {
    let option_name = "option";
    let subcommand_name = "subcommand";
    let subcommand_group_name = "subcommand_group";
    let option = CommandOptionValue::Boolean(true);

    let options = vec![CommandDataOption {
        name: option_name.to_owned(),
        value: option.clone(),
    }];

    let subcommand_options = vec![CommandDataOption {
        name: subcommand_name.to_owned(),
        value: CommandOptionValue::SubCommand(options.clone()),
    }];

    let subcommand_group_options = vec![CommandDataOption {
        name: subcommand_group_name.to_owned(),
        value: CommandOptionValue::SubCommandGroup(subcommand_options.clone()),
    }];

    assert_eq!(options.clone().option(option_name), Some(option.clone()));
    assert_eq!(
        subcommand_options.option(&format!("{subcommand_name}/{option_name}")),
        Some(option.clone())
    );
    assert_eq!(
        subcommand_group_options.option(&format!(
            "{subcommand_group_name}/{subcommand_name}/{option_name}"
        )),
        Some(option)
    );

    assert_eq!(options.option("a"), None);
}
