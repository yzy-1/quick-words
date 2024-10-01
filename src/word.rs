use rand::prelude::*;
use serde::Deserialize;

use crate::opts::Args;

#[derive(Clone, Debug, Deserialize)]
pub struct Word {
  pub word: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub phonetics: String,
  pub definition: Vec<String>,
}

pub struct WordList {
  words: Vec<Word>,
}

impl WordList {
  pub fn new(config: &WordListConfig) -> WordList {
    let mut words = Self::get_words_from_file(&config.path);

    if let Some(pattern) = &config.filter {
      words.retain(|word| pattern.is_match(&word.word));
    }

    if config.shuffle {
      words.shuffle(&mut rand::thread_rng());
    }

    if words.is_empty() {
      panic!("Word list is empty");
    }

    WordList { words }
  }

  fn get_words_from_file(path: &str) -> Vec<Word> {
    let mut words: Vec<Word> =
      serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();

    words.sort_by(|a, b| {
      a.word
        .to_ascii_lowercase()
        .cmp(&b.word.to_ascii_lowercase())
    });

    words
  }

  pub fn as_slice(&self) -> &[Word] {
    &self.words
  }
}

pub struct WordListConfig {
  pub path: String,
  pub shuffle: bool,
  pub filter: Option<regex::Regex>,
}

impl WordListConfig {
  pub fn from_opts(opts: &Args) -> WordListConfig {
    WordListConfig {
      path: opts.wordlist.clone(),
      shuffle: opts.shuffle,
      filter: opts
        .filter
        .as_ref()
        .map(|filter| regex::Regex::new(filter).unwrap()),
    }
  }
}
