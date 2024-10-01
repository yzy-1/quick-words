use anyhow::Result;
use ratatui::{
  buffer::Buffer,
  crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
  layout::{Position, Rect},
  widgets::{Block, Paragraph, Widget},
};

use crate::state::STATE;

pub struct Input {}

impl Input {
  pub fn new() -> Self {
    Self {}
  }

  fn move_cursor_left(&self) {
    let cursor_moved_left = {
      let state = STATE.get().unwrap().read().unwrap();
      state.input.character_index.saturating_sub(1)
    };

    let clamped_cursor = self.clamp_cursor(cursor_moved_left);

    {
      let mut state = STATE.get().unwrap().write().unwrap();
      state.input.character_index = clamped_cursor;
    }
  }

  fn move_cursor_right(&self) {
    let cursor_moved_right = {
      let state = STATE.get().unwrap().read().unwrap();
      state.input.character_index.saturating_add(1)
    };

    let clamped_cursor = self.clamp_cursor(cursor_moved_right);

    {
      let mut state = STATE.get().unwrap().write().unwrap();
      state.input.character_index = clamped_cursor;
    }
  }

  fn enter_char(&self, new_char: char) {
    let index = self.byte_index();

    {
      let mut state = STATE.get().unwrap().write().unwrap();
      state.input.text.insert(index, new_char);
    }

    self.move_cursor_right();
  }

  /// Returns the byte index based on the character position.
  ///
  /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
  /// the byte index based on the index of the character.
  fn byte_index(&self) -> usize {
    let state = STATE.get().unwrap().read().unwrap();

    state
      .input
      .text
      .char_indices()
      .map(|(i, _)| i)
      .nth(state.input.character_index)
      .unwrap_or(state.input.text.len())
  }

  fn delete_char(&self) {
    let input = {
      let state = STATE.get().unwrap().read().unwrap();
      state.input.clone()
    };

    let is_not_cursor_leftmost = input.character_index != 0;
    if is_not_cursor_leftmost {
      // Method "remove" is not used on the saved text for deleting the selected char.
      // Reason: Using remove on String works on bytes instead of the chars.
      // Using remove would require special care because of char boundaries.

      let current_index = input.character_index;
      let from_left_to_current_index = current_index - 1;

      // Getting all characters before the selected character.
      let before_char_to_delete = input.text.chars().take(from_left_to_current_index);
      // Getting all characters after selected character.
      let after_char_to_delete = input.text.chars().skip(current_index);

      // Put all characters together except the selected one.
      // By leaving the selected one out, it is forgotten and therefore deleted.
      {
        let mut state = STATE.get().unwrap().write().unwrap();
        state.input.text = before_char_to_delete.chain(after_char_to_delete).collect();
      }

      self.move_cursor_left();
    }
  }

  fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
    let state = STATE.get().unwrap().read().unwrap();
    new_cursor_pos.clamp(0, state.input.text.chars().count())
  }

  fn reset_cursor(&self) {
    let mut state = STATE.get().unwrap().write().unwrap();
    state.input.character_index = 0;
  }

  fn clear_input(&self) {
    {
      let mut state = STATE.get().unwrap().write().unwrap();
      state.input.text.clear();
    }

    self.reset_cursor();
  }

  fn submit_input(&self) {
    let (input, selected_index) = {
      let state = STATE.get().unwrap().read().unwrap();
      (
        state.input.text.clone(),
        state.list_state.selected().unwrap(),
      )
    };

    {
      let mut state = STATE.get().unwrap().write().unwrap();
      let selected_word = &mut state.words[selected_index];
      let matched = selected_word.validate_word(&input);
      if matched {
        state.list_state.select_next();
      }
      state.input.text.clear();
    }

    self.reset_cursor();
  }

  pub fn handle_keys(&self, key: KeyEvent) -> Result<bool> {
    if key.kind != KeyEventKind::Press {
      return Ok(false);
    }

    match key.modifiers {
      KeyModifiers::CONTROL => match key.code {
        KeyCode::Char('c') => self.clear_input(),
        KeyCode::Char('m') => self.submit_input(),
        _ => return Ok(false),
      },
      KeyModifiers::NONE => match key.code {
        KeyCode::Enter => self.submit_input(),
        KeyCode::Char(to_insert) => self.enter_char(to_insert),
        KeyCode::Backspace => self.delete_char(),
        KeyCode::Left => self.move_cursor_left(),
        KeyCode::Right => self.move_cursor_right(),
        KeyCode::Esc => self.clear_input(),
        _ => return Ok(false),
      },
      _ => return Ok(false),
    }

    Ok(true)
  }
}

impl Widget for &Input {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let input = {
      let state = STATE.get().unwrap().read().unwrap();
      state.input.clone()
    };

    let widget = Paragraph::new(input.text.clone()).block(Block::bordered().title("Input"));

    let new_cursor_position = Position::new(
      // Draw the cursor at the current position in the input field.
      // This position is can be controlled via the left and right arrow key
      area.x + input.character_index as u16 + 1,
      // Move one line down, from the border to the input line
      area.y + 1,
    );

    {
      let mut state = STATE.get().unwrap().write().unwrap();
      state.cursor_position = new_cursor_position;
    }

    widget.render(area, buf);
  }
}
