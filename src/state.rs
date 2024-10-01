use std::sync::{OnceLock, RwLock};

use ratatui::{layout::Position, widgets::ListState};

use crate::{
  opts,
  word::{Word, WordList, WordListConfig},
};

pub static STATE: OnceLock<RwLock<State>> = OnceLock::new();

#[derive(Clone)]
pub struct State {
  pub should_exit: bool,
  pub input: Input,
  pub words: Vec<WordItem>,
  pub list_state: ListState,
  pub cursor_position: Position,
}

impl State {
  pub fn new(opts: &opts::Args) -> Self {
    let word_list = WordList::new(&WordListConfig::from_opts(opts));

    Self {
      should_exit: false,
      input: Input {
        text: String::new(),
        character_index: 0,
      },
      words: word_list
        .as_slice()
        .into_iter()
        .cloned()
        .map(WordItem::new)
        .collect(),
      list_state: ListState::default().with_selected(Some(0)),
      cursor_position: Position::default(),
    }
  }
}

#[derive(Clone)]
pub struct Input {
  pub text: String,
  pub character_index: usize,
}

#[derive(Clone)]
pub struct WordItem {
  pub word: Word,
  pub word_status: TypingStatus,
  pub char_status: Vec<TypingStatus>,
}

impl WordItem {
  pub fn new(word: Word) -> Self {
    let char_len = word.word.chars().count();

    Self {
      word,
      word_status: TypingStatus::NotStarted,
      char_status: vec![TypingStatus::NotStarted; char_len],
    }
  }

  pub fn validate_word(&mut self, input: &str) -> bool {
    let char_len = self.word.word.chars().count();

    if input == self.word.word {
      self.word_status = TypingStatus::Accepted;
      self.char_status = vec![TypingStatus::Accepted; char_len];
      return true;
    }

    self.word_status = TypingStatus::Wrong;
    for (i, (input_char, word_char)) in input
      .chars()
      .map(Some)
      .chain(None)
      .zip(self.word.word.chars())
      .enumerate()
    {
      self.char_status[i] = if input_char == Some(word_char) {
        TypingStatus::Accepted
      } else {
        TypingStatus::Wrong
      };
    }

    false
  }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TypingStatus {
  Wrong,
  Accepted,
  NotStarted,
}
