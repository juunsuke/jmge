
use super::{Texture, VertexBuffer, ShaderProgram, Error};
use std::rc::Rc;

#[repr(C)]
struct Vertex
{
	pub x: f32,
	pub y: f32,
	pub col: u32,
	pub u: f32,
	pub v: f32,
}


pub struct TileMap
{
	w: u32,
	h: u32,
	tw: u32,
	th: u32,
	tex: Rc<Texture>,
}

impl TileMap
{
	pub fn new(w: u32, h: u32, tw: u32, th: u32, tex: &Rc<Texture>) -> TileMap
	{
		// Do some simple validations
		if w==0 || h==0
			{ panic!("invalid map dimensions"); }

		if tw==0 || th==0
			{ panic!("invalid tile size"); }

		let (tsw, tsh) = tex.size();
		if (tsw%tw)!=0 || (tsh%th)!=0
			{ panic!("tileset texture size not even divisible by tile size"); }

		TileMap
		{
			w,
			h,
			tw,
			th,
			tex: Rc::clone(tex),
		}
	}

	pub fn size(&self) -> (u32, u32)
	{
		(self.w, self.h)
	}

	pub fn tile_size(&self) -> (u32, u32)
	{
		(self.tw, self.th)
	}

	pub fn texture(&self) -> Rc<Texture>
	{
		Rc::clone(&self.tex)
	}
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct TileMapRenderer
{
	shader: ShaderProgram,
	vb: VertexBuffer<Vertex>,
}

impl TileMapRenderer
{
	pub fn new() -> Result<TileMapRenderer, Error>
	{
		// Create the shader and VBO
		let shader = ShaderProgram::from_str(
			include_str!("../shaders/tilemap.vert"),
			include_str!("../shaders/tilemap.frag")
		)?;

		let vb = VertexBuffer::new();

		Ok(TileMapRenderer
		{
			shader,
			vb,
		})
	}
}




