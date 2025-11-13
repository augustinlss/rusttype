use std::env;

use anyhow::Result;
use clap::Parser;
use game::state::{start_game, Game};

mod game;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    word_count: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let words_count = args.word_count;

    let game: Game = start_game(words_count)?;

    Ok(())
}
