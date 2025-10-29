mod game;

use clap::Parser;
use tetra::ContextBuilder;
use crate::game::{
	defaults::{ DEFAULT_SIZE, VERSION, TITLE },
	state::GameState
};

// Argument parsing.
#[derive(Parser, Debug)]
#[command(author = "lazypwny751", version = VERSION, propagate_version = true)]
pub struct Opt {
	#[arg(short, long, default_value = "en")]
	pub language: String,
}

// main block here.
fn main() -> tetra::Result {
	let _options = Opt::parse();

	let mut ctx = ContextBuilder::new(
		format!("{TITLE} - {VERSION}"), 
		DEFAULT_SIZE.0, // x 
		DEFAULT_SIZE.1	// y
	).quit_on_escape(true).build()?;
	
	ctx.run(GameState::new)
}
