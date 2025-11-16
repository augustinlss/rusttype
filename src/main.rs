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
}

fn main() -> Result<()> {
    let args = Args::parse();

    enable_raw_mode()?;

    let words_count = args.word_count;

    let mut game: Game = create_game(words_count)?;

    game.start()?;

    disable_raw_mode()?;

    Ok(())
}
