use clap::Parser;

use crate::error::Result;

pub mod dev;

#[derive(Debug, Parser)]
pub enum Subcommands {
    Dev(dev::Dev),
}

impl Subcommands {
    pub(crate) fn run(&self) -> Result {
        match self {
            Self::Dev(x) => x.run(),
        }
    }
}
