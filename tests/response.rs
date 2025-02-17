#![expect(missing_docs, reason = "this is a test file")]
#![cfg(test)]

mod common;

use std::time::Duration;

use common::{interaction_handle, progress_embed, send_component_message, success_embed};
use sparkle_interactions::builder::{InteractionResponseBuilder, component::TextInputBuilder};
use tokio::time::sleep;
use twilight_util::builder::InteractionResponseDataBuilder;

#[tokio::test]
async fn test_modal() -> Result<(), anyhow::Error> {
    send_component_message().await?;
    let handle = interaction_handle().await?;

    handle
        .respond(
            InteractionResponseBuilder::show_modal(
                "Test Modal".to_owned(),
                "sparkly_modal".to_owned(),
            )
            .text_input(
                TextInputBuilder::new(
                    "Test Text Input".to_owned(),
                    "sparkly_text_input".to_owned(),
                )
                .placeholder("It's normal for the submit button to fail.".to_owned())
                .build(),
            )
            .build(),
        )
        .await?;

    Ok(())
}

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

    sleep(Duration::from_secs(3)).await;

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

#[tokio::test]
async fn test_send_message() -> Result<(), anyhow::Error> {
    let handle = interaction_handle().await?;

    handle
        .respond(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The response has been sent successfully. Responding with a send message \
                         is successful.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_update_message() -> Result<(), anyhow::Error> {
    send_component_message().await?;
    let handle = interaction_handle().await?;

    handle
        .respond(InteractionResponseBuilder::update_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The response has been sent successfully. Responding to a component with \
                         an update message response is successful.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}
