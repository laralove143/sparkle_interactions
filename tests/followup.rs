#![expect(missing_docs, reason = "this is a test file")]
#![cfg(test)]

mod common;

use std::time::Duration;

use common::{interaction_handle, progress_embed, send_component_message, success_embed};
use sparkle_interactions::builder::InteractionResponseBuilder;
use tokio::time::sleep;
use twilight_util::builder::{InteractionResponseDataBuilder, embed::EmbedFooterBuilder};

#[tokio::test]
async fn test_followup_response_to_defer() -> Result<(), anyhow::Error> {
    let handle = interaction_handle().await?;

    handle
        .respond(InteractionResponseBuilder::defer_send_message().build())
        .await?;

    sleep(Duration::from_secs(1)).await;

    handle
        .respond(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The response has been sent successfully. Responding after a defer \
                         response successful.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_followup_response_to_defer_update_message() -> Result<(), anyhow::Error> {
    send_component_message().await?;
    let handle = interaction_handle().await?;

    handle
        .respond(InteractionResponseBuilder::defer_update_message().build())
        .await?;

    sleep(Duration::from_secs(1)).await;

    handle
        .respond(InteractionResponseBuilder::update_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The response has been sent successfully. Responding to a component after \
                         a defer update message response is successful.",
                    )
                    .footer(
                        EmbedFooterBuilder::new(
                            "Sending a new message after a defer update message response seems to \
                             be the expected Discord behavior.",
                        )
                        .build(),
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_followup_response_to_message() -> Result<(), anyhow::Error> {
    let handle = interaction_handle().await?;

    handle
        .respond(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([progress_embed()
                    .description(
                        "The first response has been sent. The second response should be sent in \
                         a second.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    sleep(Duration::from_secs(1)).await;

    handle
        .respond(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The response has been sent successfully. Responding after a message \
                         response is successful.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}
