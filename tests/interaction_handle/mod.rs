mod followup;
mod response;
mod update_last;

use std::{env, sync::Arc, time::Duration};

use dotenvy::dotenv;
use sparkle_interactions::{
    builder::{
        component::{ButtonBuilder, ComponentsBuilder, TextInputBuilder},
        InteractionResponseBuilder,
    },
    InteractionHandle,
};
use tokio::time::sleep;
use twilight_gateway::{Intents, Shard};
use twilight_http::Client;
use twilight_model::{
    application::command::{Command, CommandType},
    channel::message::{component::ButtonStyle, Embed},
    gateway::{event::Event, ShardId},
    http::interaction::{InteractionResponse, InteractionResponseData},
    id::Id,
};
use twilight_util::builder::{
    command::CommandBuilder,
    embed::{EmbedBuilder, EmbedFooterBuilder},
    InteractionResponseDataBuilder,
};

fn progress_embed() -> EmbedBuilder {
    EmbedBuilder::new()
        .color(0x00FE_E75C)
        .title("Test is in Progress")
}

fn success_embed() -> EmbedBuilder {
    EmbedBuilder::new().color(0x0057_F287).title("Test Passed")
}

fn client() -> Result<Client, anyhow::Error> {
    dotenv()?;
    let token = env::var("BOT_TOKEN")?;

    Ok(Client::new(token))
}

async fn interaction_handle() -> Result<InteractionHandle, anyhow::Error> {
    let client = client()?;

    let application_id = client.current_user_application().await?.model().await?.id;

    client
        .interaction(application_id)
        .set_guild_commands(
            env::var("GUILD_ID")?.parse()?,
            &[CommandBuilder::new(
                "sparkle_interactions_test",
                "Command created by Sparkle Interactions for testing purposes",
                CommandType::ChatInput,
            )
            .build()],
        )
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

async fn send_component_message() -> Result<(), anyhow::Error> {
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

#[allow(clippy::tests_outside_test_module)]
#[tokio::test]
async fn test_respond_on_delay() -> Result<(), anyhow::Error> {
    let handle = interaction_handle().await?.respond_on_delay(
        InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([progress_embed()
                    .description(
                        "This response has been sent successfully on delay. Another response \
                         should be sent in a second.",
                    )
                    .build()])
                .build(),
        ),
        Duration::from_secs(2),
    );

    tokio::time::sleep(Duration::from_secs(3)).await;

    handle
        .respond(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The final response has been sent successfully. Sending a response on a \
                         delay is successful.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}
