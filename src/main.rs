use clap::Parser;
use lipsum::lipsum_words;
use arboard::Clipboard;

/// lor 1.0
/// Your Name <your.email@example.com>
/// A simple Lorem Ipsum text generator
#[derive(Parser)]
#[command(
    name = "lor",
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "A simple Lorem Ipsum text generator"
)]
struct Cli {
    /// Specify the number of words to generate (default: 500)
    #[arg(short = 'l', long = "length", default_value_t = 500)]
    length: usize,

    /// Copy the generated text to clipboard instead of printing it
    #[arg(short, long)]
    copy: bool,
}

fn main() {
    // 解析命令行参数
    let cli = Cli::parse();

    // 生成指定数量的 Lorem Ipsum 文本
    let text = lipsum_words(cli.length);

    if cli.copy {
        // 如果传入了 -c，复制到剪贴板
        let mut clipboard = Clipboard::new().unwrap_or_else(|err| {
            eprintln!("Error initializing clipboard: {}", err);
            std::process::exit(1);
        });
        if let Err(err) = clipboard.set_text(text) {
            eprintln!("Error copying text to clipboard: {}", err);
            std::process::exit(1);
        }
        println!("✔ Copied to clipboard!");
    } else {
        // 默认行为：打印到终端
        println!("{}", text);
    }
}
