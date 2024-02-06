use std::path::PathBuf;

use clap::{Subcommand};
use clap_builder::Parser;

#[allow(clippy::option_option)]
#[derive(clap::Parser, Debug, Clone, PartialEq)]
#[command(author, version, about, subcommand_precedence_over_arg = true)]
pub struct Cli {
    /// Generate a file instead of printing to stdout    
    #[arg(short, long, group = "file_out")]
    file: Option<Option<PathBuf>>,
}