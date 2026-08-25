pub mod echo;
pub mod ping;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(no_binary_name(true), name = "")]
pub struct Cli {
    // #[command(flatten)]
    // command: Echo,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Echo(EchoArgs),
    Ping(PingArgs),
}

#[derive(Args, Debug)]
pub struct EchoArgs {
    #[arg(short, long)]
    pub bold: bool,

    #[arg(short, long)]
    pub uppercase: bool,

    #[arg(short, long)]
    pub italic: bool,

    pub string: String,
}

#[derive(Args, Debug)]
pub struct PingArgs {}
