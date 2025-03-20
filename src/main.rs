use anyhow::{Context, Ok, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// 検索するパターン
    pattern: String,
    /// 読み込むファイルのパス
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
