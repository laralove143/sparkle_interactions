#[allow(clippy::tests_outside_test_module)]
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

#[allow(clippy::tests_outside_test_module)]
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

#[allow(clippy::tests_outside_test_module)]
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
