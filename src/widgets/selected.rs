use ratatui::{
  layout::{Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, List, Widget},
};

use crate::state::{TypingStatus, STATE};

const WRONG_STYLE: Style = Style::new().bg(Color::Red).fg(Color::Black);
const ACCEPTED_STYLE: Color = Color::Green;
const WORD_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);
const COMMENT_STYLE: Style = Style::new().fg(Color::DarkGray);

pub struct Selected {}

impl Selected {
  pub fn new() -> Self {
    Self {}
  }
}

impl Widget for &Selected {
  fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
    let selected_word = {
      let state = STATE.get().unwrap().read().unwrap();
      state.words[state.list_state.selected().unwrap()].clone()
    };

    let block = Block::bordered().title("Selected");
    let block_inner = block.inner(area);

    block.render(area, buf);

    let layout = Layout::new(
      Direction::Vertical,
      vec![
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(1),
      ],
    )
    .split(block_inner);

    let word = selected_word
      .word
      .word
      .chars()
      .zip(selected_word.char_status.iter())
      .map(|(c, s)| {
        Span::styled(
          c.to_string(),
          match s {
            TypingStatus::Wrong => WRONG_STYLE,
            TypingStatus::Accepted => ACCEPTED_STYLE.into(),
            TypingStatus::NotStarted => Style::default(),
          }
          .patch(WORD_STYLE),
        )
      })
      .collect();

    let line = Line::from(
      vec![
        word,
        vec![
          Span::raw(" "),
          Span::styled(selected_word.word.kind.clone(), COMMENT_STYLE),
          Span::raw(" "),
          Span::styled(selected_word.word.phonetics.clone(), COMMENT_STYLE),
        ],
      ]
      .concat(),
    );
    let definition = List::new(selected_word.word.definition.clone());

    line.render(layout[0], buf);
    Widget::render(definition, layout[2], buf);
  }
}
