use std::{env, sync::Arc};

use dotenvy::dotenv;
use sparkle_interactions::{
    InteractionHandle,
    builder::component::{ButtonBuilder, ComponentsBuilder},
};
use twilight_gateway::{Intents, Shard};
use twilight_http::Client;
use twilight_model::{
    application::command::CommandType,
    channel::message::component::ButtonStyle,
    gateway::{ShardId, event::Event},
};
use twilight_util::builder::{command::CommandBuilder, embed::EmbedBuilder};
use twilight_validate as _;

pub(crate) fn client() -> Result<Client, anyhow::Error> {
    dotenv()?;
    let token = env::var("BOT_TOKEN")?;

    Ok(Client::new(token))
}

pub(crate) async fn interaction_handle() -> Result<InteractionHandle, anyhow::Error> {
    let client = client()?;

    let application_id = client.current_user_application().await?.model().await?.id;

    client
        .interaction(application_id)
        .set_guild_commands(env::var("GUILD_ID")?.parse()?, &[CommandBuilder::new(
            "sparkle_interactions_test",
            "Command created by Sparkle Interactions for testing purposes",
            CommandType::ChatInput,
        )
        .build()])
        .await?;

    let mut shard = Shard::new(
        ShardId::ONE,
        client.token().unwrap().to_owned(),
        Intents::empty(),
    );

    loop {
        let event_res = shard.next_event().await;

        if let Event::InteractionCreate(interaction) = event_res? {
            return Ok(InteractionHandle::new(
                Arc::new(client),
                application_id,
                interaction.id,
                interaction.0.token,
            ));
        }
    }
}

pub(crate) fn progress_embed() -> EmbedBuilder {
    EmbedBuilder::new()
        .color(0x00FE_E75C)
        .title("Test is in Progress")
}

#[allow(clippy::allow_attributes, dead_code, reason = "false-positive")]
pub(crate) async fn send_component_message() -> Result<(), anyhow::Error> {
    client()?
        .create_message(env::var("CHANNEL_ID")?.parse()?)
        .embeds(&[progress_embed()
            .description("Click on the button to continue.")
            .build()])?
        .components(
            &ComponentsBuilder::new()
                .buttons(vec![
                    ButtonBuilder::with_custom_id(
                        "sparkly_button".to_owned(),
                        "Continue".to_owned(),
                        ButtonStyle::Primary,
                    )
                    .unicode_emoji("➡️".to_owned())
                    .build(),
                ])
                .build(),
        )?
        .await?;

    Ok(())
}

pub(crate) fn success_embed() -> EmbedBuilder {
    EmbedBuilder::new().color(0x0057_F287).title("Test Passed")
}
