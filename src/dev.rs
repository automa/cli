use clap::Parser;
use tracing::instrument;

use crate::error::Result;

/// Tools for local development
#[derive(Debug, Parser)]
pub struct Dev {}

impl Dev {
    #[instrument(name = "dev", skip_all)]
    pub fn run(self) -> Result {
        Ok(())
    }
}
