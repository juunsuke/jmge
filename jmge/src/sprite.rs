
use super::{Color, Vertex, Texture, VertexBuffer};
use nalgebra::base::{Matrix4, Vector3};
use std::rc::{Rc, Weak};
use std::cell::RefCell;


pub struct SpriteMetrics
{
	pub x: i32,
	pub y: i32,
	pub w: u32,
	pub h: u32,
	pub col: Color,
	pub sx: f32,
	pub sy: f32,
	pub angle: f32,
	pub ox: i32,
	pub oy: i32,
}

fn write_vertex(vtx: &mut Vertex, sm: &SpriteMetrics, x: f32, y: f32, u: f32, v: f32)
{
	// Write vertex data
	vtx.x = x;
	vtx.y = y;
	vtx.col = sm.col.0;
	vtx.u = u;
	vtx.v = v;
	vtx.tx = sm.x as f32;
	vtx.ty = sm.y as f32;
	vtx.sx = sm.sx;
	vtx.sy = sm.sy;
	vtx.angle = sm.angle;
	vtx.ox = -sm.ox as f32;
	vtx.oy = -sm.oy as f32;
}

impl SpriteMetrics
{

	pub fn new() -> SpriteMetrics
	{
		// Default values
		SpriteMetrics
		{
			x: 0,
			y: 0,
			w: 0,
			h: 0,
			col: Color::rgba(1.0, 1.0, 1.0, 1.0),
			sx: 1.0,
			sy: 1.0,
			angle: 0.0,
			ox: 0,
			oy: 0,
		}
	}

	pub fn write_vertices(&self, v: &mut [Vertex], uv: (f32, f32, f32, f32))
	{
		let x1 = 0.0;
		let y1 = 0.0;
		let x2 = self.w as f32;
		let y2 = self.h as f32;
		let (u1, v1, u2, v2) = uv;

		// First triangle
		write_vertex(&mut v[0], self, x1, y1, u1, v1);
		write_vertex(&mut v[1], self, x2, y1, u2, v1);
		write_vertex(&mut v[2], self, x1, y2, u1, v2);

		// Second triangle
		write_vertex(&mut v[3], self, x2, y1, u2, v1);
		write_vertex(&mut v[4], self, x2, y2, u2, v2);
		write_vertex(&mut v[5], self, x1, y2, u1, v2);
	}

	pub fn calc_transform_matrix(&self) -> Matrix4<f32>
	{
		// Build a transformation matrix using the metrics

		// Translation
		let mut mat = Matrix4::new_translation(&Vector3::new(self.x as f32, self.y as f32, 0.0));

		// Rotation
		if self.angle!=0.0
			{ mat *= Matrix4::from_axis_angle(&Vector3::z_axis(), self.angle); }

		// Scale
		if self.sx!=1.0 || self.sy!=1.0
			{ mat *= Matrix4::new_nonuniform_scaling(&Vector3::new(self.sx, self.sy, 1.0)); }

		// Origin translation
		mat *= Matrix4::new_translation(&Vector3::new(-self.ox as f32, -self.oy as f32, 0.0));

		mat
	}
}


//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct Sprite
{
	metrics: SpriteMetrics,
	tex: Rc<Texture>,
	visible: bool,
}

impl Sprite
{
	pub fn new(tex: &Rc<Texture>) -> Sprite
	{
		// Setup the metrics
		let (w, h) = tex.size();

		let mut metrics = SpriteMetrics::new();
		metrics.w = w;
		metrics.h = h;
		
		Sprite
		{
			metrics,
			tex: Rc::clone(tex),
			visible: true,
		}
	}

	pub fn set_pos(&mut self, x: i32, y: i32)
	{
		self.metrics.x = x;
		self.metrics.y = y;
	}

	pub fn set_color(&mut self, col: Color)
	{
		self.metrics.col = col;
	}

	pub fn set_scale(&mut self, sx: f32, sy: f32)
	{
		self.metrics.sx = sx;
		self.metrics.sy = sy;
	}

	pub fn set_angle(&mut self, angle: f32)
	{
		self.metrics.angle = angle;
	}

	pub fn set_origin(&mut self, ox: i32, oy: i32)
	{
		self.metrics.ox = ox;
		self.metrics.oy = oy;
	}

	pub fn set_texture(&mut self, tex: &Rc<Texture>)
	{
		let (w, h) = tex.size();
		self.metrics.w = w;
		self.metrics.h = h;
		self.tex = Rc::clone(tex);
	}

	pub fn set_visible(&mut self, vis: bool)
	{
		self.visible = vis;
	}

	pub fn show(&mut self)
	{
		self.set_visible(true);
	}

	pub fn hide(&mut self)
	{
		self.set_visible(false);
	}

	pub fn pos(&self) -> (i32, i32)				{ (self.metrics.x, self.metrics.y) }
	pub fn size(&self) -> (u32, u32)			{ (self.metrics.w, self.metrics.h) }
	pub fn color(&self) -> Color				{ self.metrics.col }
	pub fn scale(&self) -> (f32, f32)			{ (self.metrics.sx, self.metrics.sy) }
	pub fn angle(&self) -> f32					{ self.metrics.angle }
	pub fn origin(&self) -> (i32, i32)			{ (self.metrics.ox, self.metrics.oy) }
	pub fn texture(&self) -> Rc<Texture>		{ Rc::clone(&self.tex) }
	pub fn visible(&self) -> bool				{ self.visible }
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct SpriteBatch
{
	sprites: Vec<Weak<RefCell<Sprite>>>,
	vbo: VertexBuffer<Vertex>,
}

struct DrawBatch
{
	tex: Rc<Texture>,
	first: u32,
	count: u32,
}

impl SpriteBatch
{

	pub fn new() -> SpriteBatch
	{
		SpriteBatch
		{
			sprites: Vec::new(),
			vbo: VertexBuffer::new(),
		}
	}

	pub fn add(&mut self, sp: Sprite) -> Rc<RefCell<Sprite>>
	{
		// Add a sprite to the vector
		let sp = Rc::new(RefCell::new(sp));
		self.sprites.push(Rc::downgrade(&sp));

		// Return an Rc copy
		sp
	}

	pub fn draw(&mut self)
	{
		// Remove all the dead sprites
		self.sprites.retain(|sp| sp.strong_count()>0);

		// Prepare batching
		let mut last_tex: Option<Rc<Texture>> = None;
		let mut batches = Vec::new();
		let mut pos = 0;
		let mut start = 0;

		{
			// Map the VBO, large enough for all the sprites
			//let mut vtx = unsafe { self.vbo.map(0, self.sprites.len()*6) };
			let mut map = self.vbo.map(self.sprites.len()*6);

			// Iterate through all the sprites
			for sp in self.sprites.iter()
			{
				let spcell = sp.upgrade().unwrap();
				let sp = spcell.borrow();

				if sp.visible
				{
					// Create a new draw batch if the texture is different
					if let Some(ref tex) = last_tex
					{
						if !tex.is_same(&sp.tex)
						{
							let batch = DrawBatch
								{
									tex: Rc::clone(tex),
									first: start,
									count: pos-start,
								};

							batches.push(batch);

							// Start a new one
							last_tex = Some(Rc::clone(&sp.tex));
							start = pos;
						}
					}
					else
					{
						// First one
						last_tex = Some(Rc::clone(&sp.tex));
					}

					// Write the vertices
					//sp.metrics.write_vertices(map.slice(pos as usize*6, 6), sp.tex.uv());
					sp.metrics.write_vertices(&mut map[pos as usize*6..(pos+1) as usize*6], sp.tex.uv());

					// Move on
					pos += 1;
				}
			}

			// Create the final batch
			if let Some(ref tex) = last_tex
			{
				let batch = DrawBatch
					{
						tex: Rc::clone(tex),
						first: start,
						count: pos-start,
					};

				batches.push(batch);
			}
		}

		// Draw all the batches
		for batch in batches
		{
			//println!("Batch : first: {}  count: {}", batch.first, batch.count);
			batch.tex.enable();
			self.vbo.draw_triangles(batch.first*6, batch.count*6);
		}
	}
}



