use anyhow::Result;
use ratatui::{
  buffer::Buffer,
  crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
  layout::Rect,
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, List, ListItem, StatefulWidget, Widget},
};

use crate::state::{TypingStatus, WordItem, STATE};

const WRONG_STYLE: Style = Style::new().bg(Color::Red).fg(Color::Black);
const ACCEPTED_STYLE: Color = Color::Green;
const NOT_STARED_STYLE: Color = Color::DarkGray;
const COMMENT_STYLE: Style = Style::new()
  .fg(Color::DarkGray)
  .add_modifier(Modifier::ITALIC);
const SELECTED_STYLE: Style = Style::new().bg(Color::White).fg(Color::Black);

pub struct WordList {}

impl WordList {
  pub fn new() -> Self {
    Self {}
  }

  pub fn handle_keys(&self, key: KeyEvent) -> Result<bool> {
    if key.kind != KeyEventKind::Press {
      return Ok(false);
    }

    match key.modifiers {
      KeyModifiers::CONTROL => match key.code {
        KeyCode::Char('p') => self.select_previous(),
        KeyCode::Char('n') => self.select_next(),
        _ => return Ok(false),
      },
      KeyModifiers::NONE => match key.code {
        KeyCode::Up => self.select_previous(),
        KeyCode::Down => self.select_next(),
        _ => return Ok(false),
      },
      _ => return Ok(false),
    }

    Ok(true)
  }

  fn select_next(&self) {
    let mut state = STATE.get().unwrap().write().unwrap();
    state.list_state.select_next();
  }

  fn select_previous(&self) {
    let mut state = STATE.get().unwrap().write().unwrap();
    state.list_state.select_previous();
  }
}

impl Widget for &WordList {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let (selected_index, n_words, items) = {
      let state = STATE.get().unwrap().read().unwrap();
      (
        state.list_state.selected().unwrap(),
        state.words.len(),
        state.words.iter().map(ListItem::from).collect::<Vec<_>>(),
      )
    };

    let block = Block::default().borders(Borders::ALL).title(vec![
      Span::raw("Word List "),
      Span::styled(format!("{}/{}", selected_index + 1, n_words), COMMENT_STYLE),
    ]);

    let list = List::new(items)
      .block(block)
      .highlight_style(SELECTED_STYLE);

    {
      let mut state = STATE.get().unwrap().write().unwrap();
      StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
  }
}

impl From<&WordItem> for ListItem<'_> {
  fn from(value: &WordItem) -> Self {
    let status_icon = match value.word_status {
      TypingStatus::Wrong => Span::styled("[W]", WRONG_STYLE),
      TypingStatus::Accepted => Span::styled("[A]", ACCEPTED_STYLE),
      TypingStatus::NotStarted => Span::styled("[-]", NOT_STARED_STYLE),
    };

    let comment = Span::styled(value.word.kind.clone(), COMMENT_STYLE);

    let line = Line::from_iter([
      status_icon,
      Span::raw(" "),
      Span::raw(value.word.word.clone()),
      Span::raw(" "),
      comment,
    ]);

    ListItem::new(line)
  }
}
