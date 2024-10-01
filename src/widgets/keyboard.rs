use ratatui::{
  buffer::Buffer,
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::Line,
  widgets::{Block, Paragraph, Widget},
};

use crate::state::STATE;

const HIGHLIGHT_KEY_STYLE: Style = Style::new().bg(Color::DarkGray);
const HOME_ROW_KEY_STYLE: Style = Style::new().add_modifier(Modifier::UNDERLINED);

pub struct Keyboard {}

impl Keyboard {
  pub fn new() -> Self {
    Self {}
  }
}

impl Widget for &Keyboard {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered().title("Keyboard");
    let block_inner = block.inner(area);

    const KEYBOARD_LAYOUT: [&str; 3] = ["QDRWBJFUP;[", "ASHTGYNEOI", "ZXMCVKL,./"];

    let row_layout = Layout::new(
      Direction::Vertical,
      vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
      ],
    )
    .split(block_inner);

    let selected_word = {
      let state = STATE.get().unwrap().read().unwrap();
      state.words[state.list_state.selected().unwrap()]
        .word
        .word
        .to_ascii_uppercase()
    };

    for (row_idx, keys) in KEYBOARD_LAYOUT.iter().enumerate() {
      let col_layout = Layout::new(
        Direction::Horizontal,
        vec![
          match row_idx {
            0 => vec![Constraint::Length(0)],
            1 => vec![Constraint::Fill(1)],
            2 => vec![Constraint::Fill(3)],
            _ => panic!("Invalid row index"),
          },
          vec![Constraint::Fill(4); keys.chars().count()],
          match row_idx {
            0 => vec![],
            1 => vec![Constraint::Fill(3)],
            2 => vec![Constraint::Fill(1)],
            _ => panic!("Invalid row index"),
          },
        ]
        .concat(),
      )
      .split(row_layout[row_idx]);

      for (col_idx, key) in keys.chars().enumerate() {
        let mut line = Line::raw(key.to_string());

        if row_idx == 1 && (col_idx == 3 || col_idx == 6) {
          line = line.style(HOME_ROW_KEY_STYLE);
        }

        let mut paragraph = Paragraph::new(line).centered().block(Block::bordered());

        if selected_word.contains(key) {
          paragraph = paragraph.style(HIGHLIGHT_KEY_STYLE);
        }

        paragraph.render(col_layout[col_idx + 1], buf);
      }
    }

    block.render(area, buf);
  }
}
