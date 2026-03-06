mod cli;
mod database;
use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{KeyCode, KeyModifiers},
    },
};

use crate::cli::{Args, Commands};

mod screens;

fn main() -> Result<(), Box<dyn Error>> {
    database::create_missing_db();
    let args = Args::parse();
    match args.command {
        Commands::Run => {
            ratatui::run(app)?;
            ratatui::restore();
        }
        Commands::Flags { command } => command.run(),
    }

    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let mut app = App {
        screen: Box::new(screens::home::HomeScreen::default()),
    };
    loop {
        terminal.draw(|f| app.render(f))?;
        if let Some(k) = crossterm::event::read()?.as_key_press_event() {
            match (k.code, k.modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => return Ok(()),
                k => {
                    if let Some(t) = app.screen.handle_input(k) {
                        app.screen = t;
                    }
                }
            }
        }
    }
}

struct App {
    screen: Box<dyn screens::screen::Screen>,
}

impl App {
    fn render(&mut self, f: &mut Frame) {
        self.screen.render(f)
    }
}
