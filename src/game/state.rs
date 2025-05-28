// imports.
use tetra::audio::{self, Sound, SoundInstance};
use tetra::{graphics::{ 
	self, Color, text::Text, text::Font }, 
	State, Context 
};
use crate::game::defaults::TEXT_OFFSET;
use crate::TTableOne;

// Game State.
pub struct GameState {
	text: Text,
	channel: SoundInstance
}

// And the gamestate.
impl GameState {
	pub fn new(ctx: &mut Context, language: &str) -> tetra::Result<Self> {
		audio::set_master_volume(ctx, 0.4);

        let sound = Sound::new("resources/music/songfordenise.mp3")?;
        let channel = sound.spawn(ctx)?;

		#[allow(clippy::useless_format)]
		let text = Text::new(format!("{}", TTableOne::localize("hello world", language)),
			Font::vector(ctx, "resources/font/SourceCodePro-Regular.ttf", 16.0)?,
		);

		Ok(GameState { text, channel })
	}
}

// State implementation.
impl State for GameState {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result {
		self.channel.play();

		Ok(())
	}

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.text.draw(ctx, TEXT_OFFSET);

        Ok(())
    }
}
