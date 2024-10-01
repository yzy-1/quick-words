use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
  /// Wordlist file path
  pub wordlist: String,

  /// Whether to shuffle the word list
  #[arg(short, long)]
  pub shuffle: bool,

  /// Regex filter for the word
  #[arg(short, long)]
  pub filter: Option<String>,
}
