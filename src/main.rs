use std::io::stdout;

use anstream::{AutoStream, ColorChoice};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use colorchoice_clap::Color;
use tracing_log::AsTrace;
use tracing_subscriber::prelude::*;

mod error;
mod styles;

mod dev;

/// CLI for Automa
#[derive(Debug, Parser)]
#[clap(name = "automa", version)]
#[command(styles = styles::styles())]
struct App {
    #[command(subcommand)]
    cmd: Subcommands,

    #[command(flatten)]
    color: Color,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Debug, Parser)]
enum Subcommands {
    Dev(dev::Dev),
}

fn main() {
    let program = App::parse();

    program.color.write_global();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_target(false)
                .with_ansi(!matches!(AutoStream::choice(&stdout()), ColorChoice::Never))
                .with_filter(program.verbose.log_level_filter().as_trace()),
        )
        .init();

    let result = match program.cmd {
        Subcommands::Dev(x) => x.run(),
    };

    error::finish(result);
}

#[cfg(test)]
mod test {
    use super::*;

    use clap::CommandFactory;

    #[test]
    fn verify_app() {
        App::command().debug_assert();
    }
}
