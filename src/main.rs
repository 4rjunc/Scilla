use crate::{config::ScillaConfig, context::ScillaContext};

pub mod commands;
pub mod config;
pub mod constants;
pub mod context;
pub mod error;
pub mod misc;
pub mod prompt;
pub mod tui;
pub mod ui;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let config = ScillaConfig::load()?;
    let ctx = ScillaContext::try_from(config)?;

    tui::run(ctx).await?;

    Ok(())
}
