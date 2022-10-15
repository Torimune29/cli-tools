use clap::{Parser, Subcommand};
use percent_encoding::{percent_decode, percent_encode, NON_ALPHANUMERIC};
use std::io::Read;

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
enum Kind {
    Shell,
    Json,
    Http,
}

fn get_message(s: &Option<String>) -> String {
    let mut buffer: String = String::new();
    match s {
        Some(message) => {
            buffer = message.clone();
        }
        None => {
            std::io::stdin().read_to_string(&mut buffer).unwrap();
        }
    }
    buffer
}

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct AppArg {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]

enum Commands {
    StrEscape {
        #[clap(value_enum, short = 'k', long = "kind")]
        kind: Kind,

        #[clap(help = "Input Message. If not, read STDIN")]
        message: Option<String>,
    },
    StrUnescape {
        #[clap(value_enum, short = 'k', long = "kind")]
        kind: Kind,

        #[clap(help = "Input Message. If not, read STDIN")]
        message: Option<String>,
    },
}

fn main() {
    let arg: AppArg = AppArg::parse();
    match arg.command {
        Commands::StrEscape { kind, message } => match kind {
            Kind::Shell => {
                println!("{}", snailquote::escape(&get_message(&message)));
            }
            Kind::Json => {
                println!("{}", escape8259::escape(&get_message(&message)));
            }
            Kind::Http => {
                println!(
                    "{}",
                    percent_encode(get_message(&message).as_bytes(), NON_ALPHANUMERIC)
                );
            }
        },
        Commands::StrUnescape { kind, message } => match kind {
            Kind::Shell => {
                println!("{}", snailquote::unescape(&get_message(&message)).unwrap());
            }
            Kind::Json => {
                println!("{}", escape8259::unescape(&get_message(&message)).unwrap());
            }
            Kind::Http => {
                println!(
                    "{}",
                    percent_decode(get_message(&message).as_bytes())
                        .decode_utf8()
                        .unwrap()
                );
            }
        },
    }
}
