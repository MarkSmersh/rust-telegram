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

#[derive(Subcommand, Debug, Eq, PartialEq)]
pub enum Commands {
    Echo(EchoArgs),
    Ping(PingArgs),
}

// impl TryInto<EchoArgs> for Commands {
//     type Error = Box<dyn Error>;
//
//     fn try_into(self) -> Result<EchoArgs, Self::Error> {
//         match self {
//             Self::Echo(args) => Ok(args),
//             _ => Err("")?,
//         }
//     }
// }

#[derive(Args, Debug, PartialEq, Eq)]
pub struct EchoArgs {
    #[arg(short, long)]
    pub bold: bool,

    #[arg(short, long)]
    pub uppercase: bool,

    #[arg(short, long)]
    pub italic: bool,

    pub string: String,
}

#[derive(Args, Debug, PartialEq, Eq)]
pub struct PingArgs {}
