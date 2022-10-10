use clap::{Parser, Subcommand};
use percent_encoding::{percent_encode, percent_decode, NON_ALPHANUMERIC};
use std::io::Read;


#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
enum Kind {
    Shell,
    Json,
    HTTP,
}


fn get_message (s: &Option<String>) -> String {
    let mut buffer:String = String::new();
    match s {
        Some(message)=> {
            buffer = message.clone();
        }
        None => {
            std::io::stdin().read_to_string(&mut buffer).unwrap();
        }
    }
    return buffer;
}

// Parserを継承した構造体はArgの代わりに使用することが可能。
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

    #[clap(value_enum, short = 'k', long = "kind")]
    kind: Kind,
}

#[derive(Debug, Subcommand)]

enum Commands {
    Escape {
        #[clap(help = "Input Message. If not, read STDIN")]
        message: Option<String>,
    } ,
    Unescape {
        #[clap(help = "Input Message. If not, read STDIN")]
        message: Option<String>,
    },
}

fn main() {
    let arg: AppArg = AppArg::parse();
    match arg.kind {
        Kind::Shell => {
            match arg.command {
                Commands::Escape { message } => println!("{}", snailquote::escape(&get_message(&message))),
                Commands::Unescape { message } => println!("{}", snailquote::unescape(&get_message(&message)).unwrap()),
            }
        },
        Kind::Json => {
            match arg.command {
                Commands::Escape { message } => println!("{}", escape8259::escape(&get_message(&message))),
                Commands::Unescape { message } => println!("{}", escape8259::unescape(&get_message(&message)).unwrap()),
            }
        }
        Kind::HTTP => {
            match arg.command {
                Commands::Escape { message } => println!("{}", percent_encode(&get_message(&message).as_bytes(), NON_ALPHANUMERIC).to_string()),
                Commands::Unescape { message } => println!("{}", percent_decode(&get_message(&message).as_bytes()).decode_utf8().unwrap()),
            }
        }
    }
}
