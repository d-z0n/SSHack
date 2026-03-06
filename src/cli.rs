use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::database;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the server (WIP)
    Run,
    /// Add/Delete/List flags (WIP)
    Flags {
        #[command(subcommand)]
        command: FlagCommands,
    },
}

/// Commands to modify flags
#[derive(Subcommand)]
pub enum FlagCommands {
    /// list flags in a tidy list
    List,
    /// delete a flag
    Delete {
        #[arg(short)]
        id: i32,
    },
    /// load json file with flags into database
    Load {
        /// Path to read from (default stdin)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// print flags as json to file
    Write {
        /// Path to write to (default stdout)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// add a new flag
    Add {
        /// name of the flag
        #[arg(short, long)]
        name: String,
        /// description of the flag (including links to files / services)
        #[arg(short, long)]
        description: String,
        /// points for solving the flag
        #[arg(short, long)]
        points: i32,
        /// solution string
        #[arg(short, long)]
        flag: String,
    },
}
impl FlagCommands {
    pub fn run(&self) {
        match &self {
            FlagCommands::Delete { id } => database::delete_flag(*id).unwrap(),
            FlagCommands::List => {
                for flag in database::Flag::get_all().unwrap() {
                    println!(
                        "{} - {} - {} - {} - {}",
                        flag.id(),
                        flag.name(),
                        &flag.description()[..10.min(flag.description().len())],
                        flag.points(),
                        flag.flag()
                    )
                }
            }
            FlagCommands::Add {
                name,
                description,
                points,
                flag,
            } => database::create_flag(&name, &description, *points, &flag),
            _ => todo!(),
        }
    }
}
