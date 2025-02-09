use clap::Parser;
use lipsum::lipsum_words;
use arboard::Clipboard;

/// 🚀 A fast Lorem Ipsum text generator
#[derive(Parser)]
#[command(
    name = "lor",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "Generates Lorem Ipsum text and optionally copies it to clipboard."
)]
struct Cli {
    /// Number of words to generate (default: 500)
    #[arg(short = 'l', long = "length", default_value_t = 500)]
    length: usize,

    /// Copy the generated text to clipboard instead of printing
    #[arg(short, long)]
    copy: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let text = lipsum_words(cli.length);

    if cli.copy {
        Clipboard::new()?.set_text(text.clone())?;
        println!("✔ Copied to clipboard!");
    } else {
        println!("{}", text);
    }

    Ok(())
}
