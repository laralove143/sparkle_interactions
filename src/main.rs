mod interaction;

use std::{env, ops::Deref, sync::Arc};

use anyhow::Result;
use dotenvy::dotenv;
use futures_util::StreamExt;
use tracing::{error, info};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{stream::ShardEventStream, Shard};
use twilight_http::Client;
use twilight_model::{
    gateway::{event::Event, Intents},
    id::{marker::ApplicationMarker, Id},
};

struct Config {
    token: String,
}

impl Config {
    fn new() -> Result<Self> {
        dotenv()?;

        Ok(Self {
            token: env::var("TOKEN")?,
        })
    }
}

struct ContextInner {
    application_id: Id<ApplicationMarker>,
    cache: InMemoryCache,
    client: Client,
    config: Config,
}

#[derive(Clone)]
struct Context(Arc<ContextInner>);

impl Deref for Context {
    type Target = Arc<ContextInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Context {
    async fn new() -> Result<Self> {
        let cache = InMemoryCache::builder()
            .resource_types(ResourceType::empty())
            .build();
        let config = Config::new()?;
        let client = Client::new(config.token.clone());

        let application_id = client.current_user_application().await?.model().await?.id;

        Ok(Self(Arc::new(ContextInner {
            application_id,
            cache,
            client,
            config,
        })))
    }

    async fn shards(&self) -> Result<Vec<Shard>> {
        Ok(twilight_gateway::stream::create_recommended(
            &self.client,
            twilight_gateway::Config::new(self.config.token.clone(), Intents::empty()),
            |_, builder| builder.build(),
        )
        .await?
        .collect())
    }

    async fn handle_event(self, event: Event) {
        let event_handle_res: Result<()> = match event {
            Event::Ready(_) => {
                info!("ready");
                Ok(())
            }
            Event::InteractionCreate(interaction) => self.handle_interaction(interaction.0).await,
            _ => Ok(()),
        };

        if let Err(err) = event_handle_res {
            error!(?err);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    LogTracer::init()?;
    let fmt_tracing_layer = tracing_subscriber::fmt::layer().without_time().pretty();
    tracing_subscriber::registry()
        .with(fmt_tracing_layer)
        .with(tracing_journald::layer()?)
        .with(EnvFilter::try_from_default_env()?)
        .try_init()?;

    let ctx = Context::new().await?;

    ctx.set_commands().await?;

    let mut shards = ctx.shards().await?;
    let mut event_stream = ShardEventStream::new(shards.iter_mut());

    while let Some((_, event_res)) = event_stream.next().await {
        match event_res {
            Ok(event) => {
                ctx.cache.update(&event);

                let ctx_clone = ctx.clone();
                tokio::spawn(ctx_clone.handle_event(event));
            }
            Err(err) => {
                error!(?err, "error receiving event");

                if err.is_fatal() {
                    error!("received fatal error while receiving event, exiting");
                    break;
                }

                continue;
            }
        };
    }

    Ok(())
}
