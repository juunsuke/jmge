
mod color;
pub use color::Color;

mod window;
pub use window::Window;

mod shader;
pub use shader::{VertexShader, FragmentShader, ShaderProgram};

mod vbo;
pub use vbo::{VertexBuffer};

mod texture;
pub use texture::{Texture, RawTexture, TextureAtlas};

mod canvas;
pub use canvas::Canvas;

mod input;
pub use input::{Input, Mouse, Keyboard, Key};

mod font;
pub use font::{Font, Glyph};

mod renderer;
pub use renderer::{Renderer, Quad, Renderable};

mod ecs;
pub use ecs::{World, Component, Entity, System};
pub use jmge_derive::Component;

mod audio;
pub use audio::{Audio, Sound, SoundControl};

mod sprite;
pub use sprite::{SpriteSheet, Sprite, SpriteSystem};



#[derive(Debug)]
pub enum Error
{
	CompileShader (String),
	LoadImage (String),
	PackAtlas,
	LoadFont (String),
	NoAudioDevice,
	LoadSound,
	LoadSpriteSheet (String),
}


impl ToString for Error
{
	fn to_string(&self) -> String
	{
		match self
		{
			Error::CompileShader (s)		=> format!("Error compiling a shader: {}", s),
			Error::LoadImage (s)			=> format!("Error loading an image: {}", s),
			Error::PackAtlas				=> format!("Could not fit a canvas into a texture atlas"),
			Error::LoadFont (s)				=> format!("Error loading a font: {}", s),
			Error::NoAudioDevice			=> format!("No audio device found"),
			Error::LoadSound				=> format!("Error loading a sound file"),
			Error::LoadSpriteSheet (s)		=> format!("Error loading a sprite sheet: {}", s),
		}
	}
}

