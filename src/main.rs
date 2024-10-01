use std::sync::RwLock;

use anyhow::Result;
use clap::Parser;
use state::{State, STATE};

pub mod opts;
pub mod state;
pub mod widgets;
pub mod word;

fn main() -> Result<()> {
  let opts = opts::Args::parse();

  STATE.get_or_init(|| RwLock::new(State::new(&opts)));

  let mut terminal = ratatui::init();
  terminal.clear()?;
  let app = widgets::app::App::new();
  let app_result = app.run(terminal);
  ratatui::restore();
  app_result
}
