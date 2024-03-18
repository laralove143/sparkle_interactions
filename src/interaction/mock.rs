use anyhow::Result;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::Interaction,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::command::CommandBuilder;

use crate::interaction::{CreateCommand, InteractionContext, RunInteraction};

pub struct Mock {
    ctx: InteractionContext,
    is_guild: bool,
}

impl CreateCommand for Mock {
    fn command() -> Result<Command> {
        Ok(
            CommandBuilder::new(Self::CUSTOM_ID, "mock", CommandType::ChatInput)
                .validate()?
                .build(),
        )
    }
}

impl RunInteraction for Mock {
    const CUSTOM_ID: &'static str = "mock";

    async fn new(interaction: Interaction, ctx: InteractionContext) -> Result<Self> {
        Ok(Self {
            ctx,
            is_guild: interaction.is_guild(),
        })
    }

    async fn run(self) -> Result<()> {
        self.ctx
            .create_response(&InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: None,
            })
            .await?;

        Ok(())
    }
}
