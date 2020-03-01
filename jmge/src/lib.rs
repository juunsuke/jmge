
mod color;
pub use color::Color;

mod window;
pub use window::Window;

mod shader;
pub use shader::{VertexShader, FragmentShader, ShaderProgram};

mod vbo;
pub use vbo::{VertexBuffer, Vertex};

mod sprite;
pub use sprite::{SpriteMetrics, Sprite, SpriteBatch};

mod texture;
pub use texture::{Texture, RawTexture, TextureAtlas};

mod canvas;
pub use canvas::Canvas;

mod input;
pub use input::{Input, Mouse, Keyboard, Key};

mod font;
pub use font::{Font, Glyph};

mod renderer;
pub use renderer::{Renderer, Quad};

pub mod ecs;

//pub mod gui;


#[derive(Debug)]
pub enum Error
{
	CompileShader (String),
	LoadImage (String),
	PackAtlas,
	LoadFont (String),
}


impl ToString for Error
{
	fn to_string(&self) -> String
	{
		match self
		{
			Error::CompileShader (s)	=> format!("Error compiling a shader: {}", s),
			Error::LoadImage (s)		=> format!("Error loading an image: {}", s),
			Error::PackAtlas			=> format!("Could not fit a canvas into a texture atlas"),
			Error::LoadFont (s)			=> format!("Error loading a font: {}", s),
		}
	}
}

