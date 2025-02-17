#![doc = include_str!("../README.md")]

#[cfg(test)]
use {anyhow as _, dotenvy as _, tokio as _, twilight_gateway as _, twilight_util as _};

pub mod builder;
pub mod extract;
pub mod handle;

pub use handle::InteractionHandle;
