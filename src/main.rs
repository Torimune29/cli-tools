mod budget;
mod utility;
use clap::Parser;
// Parserを継承した構造体はArgの代わりに使用することが可能。
#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct AppArg {
    // 任意のオプション
    #[clap(short, long)]
    name: Option<String>,

    // 必須のオプション
    #[clap(short = 'c', long = "count")]
    count: i32,

    // 位置引数
    message: String,
}

fn main() {
    budget::budget();
    utility::file::read();
    let arg: AppArg = AppArg::parse();
    for _ in 0..arg.count {
        println!(
            "{}: {}",
            arg.name.clone().unwrap_or_else(|| String::from("Alice")),
            arg.message
        );
    }
}
