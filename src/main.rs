use anyhow::Result;
use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use game::game::{create_game, Game};

mod game;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of words for test
    #[arg(short = 'w', long = "words", default_value_t = 10)]
    word_count: usize,

    /// Show the menu
    #[arg(short = 'm', long = "menu", default_value_t = false)]
    menu: bool,
}

use game::menu::{Menu, MenuAction};

use crate::game::game::GameState;

fn main() -> Result<()> {
    let args = Args::parse();
    enable_raw_mode()?;

    if args.menu {
        loop {
            let mut menu = Menu::new();
            match menu.run()? {
                MenuAction::StartGame(word_count) => {
                    let mut game: Game = create_game(word_count)?;
                    game.start()?;
                }
                MenuAction::Quit => break,
            }
        }
    } else {
        loop {
            let mut game: Game = create_game(args.word_count)?;
            game.start()?;
            match game.state {
                GameState::Quit => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}
