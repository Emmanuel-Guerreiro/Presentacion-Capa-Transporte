use std::fmt::Display;

use clap::Parser;
mod client;
mod server;
#[derive(Parser, PartialEq, Eq, Debug)]
enum ClientCommand {
    #[command(about = "Ping random server")]
    Ping,
    #[command(about = "Generate bcrypt hash from plaintext")]
    Hash { plain: String },
    #[command(about = "Run in server mode")]
    Server,
}

impl ClientCommand {
    fn to_string(&self) -> String {
        match self {
            Self::Ping => String::from("PING"),
            Self::Hash { plain } => format!("HASH {plain}"),
            Self::Server => String::from("SERVER"),
        }
    }
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Emmanuel Guerreiro - Lautaro Fernandez")]
pub struct Cli {
    #[clap(subcommand)]
    cmd: ClientCommand,
}

fn main() {
    let args = Cli::parse();
    if args.cmd == ClientCommand::Server {
        server::init_server().unwrap()
    } else {
        client::init_client(args)
    }
}
