use std::time::Duration;

#[allow(clippy::tests_outside_test_module)]
#[tokio::test]
async fn test_update_last_initial() -> Result<(), anyhow::Error> {
    let handle = interaction_handle().await?.track_last_message();

    handle
        .update_last(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The response has been sent successfully. Updating the last response when \
                         no previous response has been sent is successful.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}

#[allow(clippy::tests_outside_test_module)]
#[tokio::test]
async fn test_update_last_after_defer() -> Result<(), anyhow::Error> {
    let handle = interaction_handle().await?.track_last_message();

    handle
        .respond(InteractionResponseBuilder::defer_send_message().build())
        .await?;

    handle
        .update_last(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The response has been sent successfully. Updating the last response \
                         after a defer response is successful.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}

#[allow(clippy::tests_outside_test_module)]
#[tokio::test]
async fn test_update_last_after_message() -> Result<(), anyhow::Error> {
    let handle = interaction_handle().await?.track_last_message();

    handle
        .respond(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([progress_embed()
                    .description(
                        "The first response has been sent. This should be edited in a second.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    sleep(Duration::from_secs(1)).await;

    handle
        .update_last(InteractionResponseBuilder::send_message(
            InteractionResponseDataBuilder::new()
                .embeds([success_embed()
                    .description(
                        "The response has been edited successfully. Updating the last response \
                         after a message response is successful.",
                    )
                    .build()])
                .build(),
        ))
        .await?;

    Ok(())
}
