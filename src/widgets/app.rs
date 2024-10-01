use anyhow::Result;
use ratatui::{
  buffer::Buffer,
  crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
  layout::{Constraint, Direction, Flex, Layout, Rect},
  widgets::Widget,
  DefaultTerminal, Frame,
};

use crate::state::STATE;

use super::{input, keyboard, selected, word_list};

pub struct App {
  input: input::Input,
  selected: selected::Selected,
  word_list: word_list::WordList,
  keyboard: keyboard::Keyboard,
}

impl App {
  pub fn new() -> Self {
    Self {
      input: input::Input::new(),
      selected: selected::Selected::new(),
      word_list: word_list::WordList::new(),
      keyboard: keyboard::Keyboard::new(),
    }
  }

  pub fn run(self, mut terminal: DefaultTerminal) -> Result<()> {
    while !STATE.get().unwrap().read().unwrap().should_exit {
      terminal.draw(|frame| self.draw(frame))?;

      if let Event::Key(key) = event::read()? {
        if self.handle_keys(key)? {
          continue;
        }

        if self.word_list.handle_keys(key)? {
          continue;
        }

        if self.input.handle_keys(key)? {
          continue;
        }
      }
    }

    Ok(())
  }

  fn handle_keys(&self, key: KeyEvent) -> Result<bool> {
    if key.kind != KeyEventKind::Press {
      return Ok(false);
    }

    if key.modifiers != KeyModifiers::CONTROL {
      return Ok(false);
    }

    if key.code == KeyCode::Char('q') {
      let mut state = STATE.get().unwrap().write().unwrap();
      state.should_exit = true;
      return Ok(true);
    }

    Ok(false)
  }

  fn draw(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());

    let cursor_position = {
      let state = STATE.get().unwrap().read().unwrap();
      state.cursor_position.clone()
    };

    frame.set_cursor_position(cursor_position);
  }
}

impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(11),
      ])
      .split(area);

    let content_chucks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(50), Constraint::Fill(1)])
      .split(chunks[1]);

    let keyboard_chucks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Max(102)])
      .flex(Flex::Center)
      .split(chunks[2]);

    self.input.render(chunks[0], buf);
    self.selected.render(content_chucks[0], buf);
    self.word_list.render(content_chucks[1], buf);
    self.keyboard.render(keyboard_chucks[0], buf);
  }
}
