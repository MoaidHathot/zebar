use std::process;

use clap::{Args, Parser, Subcommand};

const VERSION: &'static str = env!("VERSION_NUMBER");

#[derive(Parser, Debug)]
#[clap(author, version = VERSION, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
  /// Open a window by its ID (eg. `zebar open bar`).
  Open(OpenWindowArgs),
  /// Output available monitors.
  Monitors(OutputMonitorsArgs),
}

#[derive(Args, Debug)]
pub struct OpenWindowArgs {
  /// ID of the window to open (eg. `bar`).
  pub window_id: String,

  /// Arguments to pass to the window.
  ///
  /// These become available via the `self` provider.
  #[clap(short, long, num_args = 1.., value_parser=parse_open_args)]
  pub args: Option<Vec<(String, String)>>,
}

#[derive(Args, Debug)]
pub struct OutputMonitorsArgs {
  /// Use ASCII NUL character (character code 0) instead of newlines
  /// for delimiting monitors.
  ///
  /// Useful for piping to `xargs -0`.
  #[clap(short, long)]
  pub print0: bool,
}

/// Print to stdout/stderror and exit the process.
pub fn print_and_exit(output: anyhow::Result<String>) {
  match output {
    Ok(output) => {
      print!("{}", output);
      process::exit(0);
    }
    Err(err) => {
      eprintln!("Error: {}", err);
      process::exit(1);
    }
  }
}

/// Parses arguments passed to the `open` CLI command into a string tuple.
fn parse_open_args(
  input: &str,
) -> anyhow::Result<(String, String), String> {
  let mut parts = input.split('=');

  match (parts.next(), parts.next()) {
    (Some(key), Some(value)) => Ok((key.into(), value.into())),
    _ => Err("Arguments must be of format KEY1=VAL1".into()),
  }
}
