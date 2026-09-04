use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(
    name = "rustdu",
    version = "0.2.0",
    about = "A fast terminal disk usage analyzer written in Rust",
    after_help = "Found a bug? Join our Telegram chat: t.me/MyRustDU-input"
)]
struct Args {
    /// Path to analyze disk usage for
    #[arg(default_value = ".")]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Running rustdu for path: {}", args.path);

    Ok(())
}