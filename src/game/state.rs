use tetra::{graphics::{ self, Color, text::Text, text::Font }, State, Context };
use crate::game::defaults::TEXT_OFFSET;
use crate::TTableOne;

pub struct GameState {
	text: Text,
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.text.draw(ctx, TEXT_OFFSET);

        Ok(())
    }
}

impl GameState {
	pub fn new(ctx: &mut Context, language: &str) -> tetra::Result<GameState> {
		#[allow(clippy::useless_format)]
		let text = Text::new(format!("{}", TTableOne::localize("hello world", language)),
			Font::vector(ctx, "assets/SourceCodePro-Regular.ttf", 16.0)?,
		);

		Ok(GameState { text })
	}
}
