use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
enum Kind {
    Json,
}

fn get_message (s: &Option<String>) -> String {
    let mut buffer: String = String::new();
    match s {
        Some(message) => {
            buffer = message.clone();
        }
        None => {
            std::io::stdin().read_line(&mut buffer).unwrap();
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
    #[clap(value_enum, short = 'k', long = "kind")]
    kind: Kind,

    #[clap(help = "Input Message. If not, read STDIN")]
    message: Option<String>,
}

fn main() {
    let arg: AppArg = AppArg::parse();
    let buffer: String = get_message(&arg.message);
    println!(
        "{:?}: {}",
        arg.kind,
        buffer
    );
}
