//! clap setup.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! We prefer clap using the `command!` macro, which runs at compile time.
//! We prefer clap using the builder pattern, which offers more capabilties.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we favor the separation of concerns.

use clap::{Arg, ArgAction};
use crate::args::Args;

pub fn clap() -> crate::Args {
    let matches = clap::command!()
    .name("markline")
    .version("1.1.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Markbox line picker for stdin line input")
    .arg(Arg::new("verbose")
        .help("Set the verbosity level")
        .short('v')
        .long("verbose")
        .action(ArgAction::Count))
    .get_matches();
    Args {
        verbose: matches.get_count("verbose"),
    }
}
