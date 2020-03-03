
use super::{ShaderProgram, VertexBuffer, Error, Color, Texture, Component, World};
use std::rc::Rc;
use nalgebra::base::Matrix4;


#[repr(C)]
struct Vertex
{
	pub x: f32,
	pub y: f32,
	pub col: u32,
	pub u: f32,
	pub v: f32,
	pub tx: f32,
	pub ty: f32,
	pub sx: f32,
	pub sy: f32,
	pub angle: f32,
	pub ox: f32,
	pub oy: f32,
}


#[derive(Component)]
pub struct Renderable
{
	// Texture to render
	pub texture: Rc<Texture>,

	// Position
	pub x: i32,
	pub y: i32,

	// Color tint
	pub color: Color,

	// Visibility flag
	pub visible: bool,

	// Scale
	pub x_scale: f32,
	pub y_scale: f32,

	// Origin
	pub x_origin: i32,
	pub y_origin: i32,

	// Rotation angle
	pub angle: f32,
}

impl Renderable
{
	pub fn new(tex: &Rc<Texture>, x: i32, y: i32) -> Renderable
	{
		// Create a new renderable with mostly default values
		Renderable
		{
			texture: Rc::clone(tex),
			x,
			y,
			color: Color::rgb(1.0, 1.0, 1.0),
			visible: true,
			x_scale: 1.0,
			y_scale: 1.0,
			x_origin: 0,
			y_origin: 0,
			angle: 0.0,
		}
	}
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct Quad
{
	// Position
	pub x: f32,
	pub y: f32,

	// Size
	pub w: f32,
	pub h: f32,

	// Color
	pub col: u32,

	// Scale
	pub sx: f32,
	pub sy: f32,

	// Rotation
	pub angle: f32,
	
	// Origin
	pub ox: f32,
	pub oy: f32,

	// Texture
	pub tex: Rc<Texture>,
}

impl Quad
{
	pub fn new(tex: &Rc<Texture>) -> Quad
	{
		let (w, h) = tex.size();

		Quad
		{
			x: 0.0,
			y: 0.0,
			w: w as f32,
			h: h as f32,
			col: 0xFFFFFFFF,
			sx: 1.0,
			sy: 1.0,
			angle: 0.0,
			ox: 0.0,
			oy: 0.0,
			tex: Rc::clone(tex),
		}
	}

	pub fn with_pos(mut self, x: f32, y: f32) -> Quad
	{
		self.x = x;
		self.y = y;
		self
	}

	pub fn with_size(mut self, w: f32, h: f32) -> Quad
	{
		self.w = w;
		self.h = h;
		self
	}

	pub fn with_color(mut self, col: Color) -> Quad
	{
		self.col = col.as_u32();
		self
	}

	pub fn with_scale(mut self, sx: f32, sy: f32) -> Quad
	{
		self.sx = sx;
		self.sy = sy;
		self
	}

	pub fn with_angle(mut self, angle: f32) -> Quad
	{
		self.angle = angle;
		self
	}

	pub fn with_origin(mut self, ox: f32, oy: f32) -> Quad
	{
		self.ox = ox;
		self.oy = oy;
		self
	}

	fn write_vertex(&self, vtx: &mut Vertex, x: f32, y: f32, u: f32, v: f32)
	{
		// Write vertex data
		vtx.x = x;
		vtx.y = y;
		vtx.col = self.col;
		vtx.u = u;
		vtx.v = v;
		vtx.tx = self.x;
		vtx.ty = self.y;
		vtx.sx = self.sx;
		vtx.sy = self.sy;
		vtx.angle = self.angle;
		vtx.ox = -self.ox;
		vtx.oy = -self.oy;
	}

	fn write_vertices(&self, v: &mut [Vertex])
	{
		let x1 = 0.0;
		let y1 = 0.0;
		let x2 = self.w as f32;
		let y2 = self.h as f32;
		let (u1, v1, u2, v2) = self.tex.uv();

		// First triangle
		self.write_vertex(&mut v[0], x1, y1, u1, v1);
		self.write_vertex(&mut v[1], x2, y1, u2, v1);
		self.write_vertex(&mut v[2], x1, y2, u1, v2);

		// Second triangle
		self.write_vertex(&mut v[3], x2, y1, u2, v1);
		self.write_vertex(&mut v[4], x2, y2, u2, v2);
		self.write_vertex(&mut v[5], x1, y2, u1, v2);
	}

}


pub struct Renderer
{
	shader: ShaderProgram,
	vb: VertexBuffer<Vertex>,
	quads: Vec<Quad>,
}


impl Renderer
{
	pub fn new() -> Result<Renderer, Error>
	{
		// Create the shader and VBO
		let shader = ShaderProgram::from_str(
			include_str!("../shaders/renderable.vert"),
			include_str!("../shaders/renderable.frag")
		)?;

		let vb = VertexBuffer::new();

		// Create the renderer
		let rend = Renderer
			{
				shader,
				vb,
				quads: Vec::new(),
			};

		Ok(rend)
	}

	pub fn add_quad(&mut self, quad: Quad)
	{
		// Add a quad to the queue
		self.quads.push(quad);
	}

	pub fn add_world(&mut self, world: &World)
	{
		// Add all the renderables from the provided world
		for (_, rend) in world.iter::<Renderable>()
		{
			// Create a quad for the renderable
			let (w, h) = rend.texture.size();

			let quad = Quad::new(&rend.texture)
				.with_pos(rend.x as f32, rend.y as f32)
				.with_size(w as f32, h as f32)
				.with_color(rend.color)
				.with_scale(rend.x_scale, rend.y_scale)
				.with_angle(rend.angle)
				.with_origin(rend.x_origin as f32, rend.y_origin as f32);

			self.add_quad(quad);
		}
	}

	fn write_vb(&mut self)
	{
		// Map enough vertices for all the quads
		let mut map = self.vb.map(self.quads.len()*6);
		let mut pos = 0;

		// Write all the vertices
		for quad in self.quads.iter()
		{
			quad.write_vertices(&mut map[pos..pos+6]);
			pos += 6;
		}
	}

	fn count_similar(&self, start: usize) -> usize
	{
		// Count how many quads use the same texture as the start one
		let mut count = 1;
		let len = self.quads.len();
		let orig = &self.quads[start].tex;

		for c in start+1..len
		{
			// Stop if it differs
			if !orig.is_same(&self.quads[c].tex)
				{ break; }

			count += 1;
		}

		count
	}

	pub fn render(&mut self, proj_mat: &Matrix4<f32>)
	{
		// Build the vertex buffer data
		self.write_vb();

		// Set the projection matrix
		self.shader.enable();
		self.shader.set_uniform_matrix("Projection", proj_mat);


		// Draw the quads
		let mut pos = 0;
		let len = self.quads.len();

		while pos<len
		{
			// Count how many use the same texture
			let count = self.count_similar(pos);

			//println!("Start: {}, Count: {}", pos, count);

			// Draw them
			self.quads[pos].tex.enable();
			self.vb.draw_triangles(pos as u32*6, count as u32*6);

			pos += count;
		}

		// Clear the queued quads
		self.quads.clear();
	}
}

