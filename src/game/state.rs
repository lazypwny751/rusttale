use tetra::{graphics::{ 
	self, Color, text::Text, text::Font }, 
	State, Context 
};
use crate::game::defaults::{TEXT_OFFSET, DEFAULT_FONT, DEFAULT_FONT_SIZE};

pub struct GameState {
	text: Text,
}

impl GameState {
	pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
		let sample_text = "Welcome to rusttale (Initramfs)!\n\n[ OK ]    Loading tinyllm.\n[ OK ]    Loading resources.\n[ OK ]    Loading dialogs.\n[ OK ]    Loading helpers.\n[ FAIL ] Init rusttale.\n>  Failed to init rusttale, executing fallback shell...\n\nrusttale rescue> ";
		let text = Text::new(sample_text, Font::vector(ctx, DEFAULT_FONT, DEFAULT_FONT_SIZE)?);

		Ok(GameState { text })
	}
}

// State implementation.
impl State for GameState {
	fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
		Ok(())
	}

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition.
        graphics::clear(ctx, Color::rgb(0.07, 0.07, 0.07));

        self.text.draw(ctx, TEXT_OFFSET);

        Ok(())
    }
}
