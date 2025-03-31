mod game;
mod locales;

use clap::Parser;
use tetra::ContextBuilder;
use crate::locales::part_one::TTableOne;
use crate::game::{
	defaults::{ DEFAULT_SIZE, VERSION, TITLE },
	state::GameState
};

#[derive(Parser, Debug)]
#[command(author = "lazypwny751", version = VERSION, propagate_version = true)]
pub struct Opt {
	#[arg(short, long, default_value = "en")]
	pub language: String,
}

fn main() -> tetra::Result {
	let options = Opt::parse();

	let mut ctx = ContextBuilder::new(
		format!("{} - {}", TITLE, VERSION), 
		DEFAULT_SIZE.0, // x 
		DEFAULT_SIZE.1	// y
	).quit_on_escape(true).build()?;
	
	ctx.run(|ctx| GameState::new(ctx, &options.language))
}
