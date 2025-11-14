use std::io::stdout;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::state::{create_game, Game};

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

    // prepare terminal
    let mut stdout = stdout();
    // execute!(stdout, EnterAlternateScreen)?;
    // enable_raw_mode()?;

    let words_count = args.word_count;

    let mut game: Game = create_game(words_count)?;

    // TODO: add main game loop
    //
    println!("Target words: {}", game.target_words.join(" "));

    // disable_raw_mode()?;
    // execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
