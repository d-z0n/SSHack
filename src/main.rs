mod app;
mod cli;
mod conf;
mod database;
mod screens;
mod server;
mod theme;

use std::{error::Error, time::Duration};

use clap::Parser;
use ratatui::crossterm::{self};

use crate::{
    cli::{Args, Commands},
    conf::Conf,
    screens::{home::HomeScreen, leaderboard::LeaderboardScreen},
};

// inspired by: https://github.com/Eugeny/russh/blob/main/russh/examples/ratatui_app.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    database::create_missing_db();
    let args = Args::parse();
    match args.command {
        Commands::Run { local, leaderboard } => {
            let conf = Conf::get();
            if !local {
                let mut server = server::AppServer::new(conf, args);
                server.run().await.expect("Failed running server");
            } else {
                let mut term = ratatui::init();
                let res = app::app(&mut term, conf, leaderboard);
                ratatui::restore();
                return res;
            }
        }
        Commands::Flags { command } => command.run(),
    }

    Ok(())
}
