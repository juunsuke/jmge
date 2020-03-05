
use super::{Error, Canvas};
use rect_packer::Packer;
use std::rc::{Rc, Weak};
use std::cell::RefCell;


pub enum Texture
{
	Raw (Rc<RefCell<RawTexture>>),
	AtlasEntry (Rc<RefCell<AtlasEntry>>),
}

impl Texture
{
	pub fn from_canvas(cnv: &Canvas, smooth: bool) -> Texture
	{
		// Shortcut for creating a raw texture
		Texture::Raw(Rc::new(RefCell::new(RawTexture::from_canvas(cnv, smooth))))
	}

	pub fn from_file(fname: &str, smooth: bool) -> Result<Texture, Error>
	{
		Ok(Texture::Raw(Rc::new(RefCell::new(RawTexture::from_file(fname, smooth)?))))
	}

	pub fn size(&self) -> (u32, u32)
	{
		// Return the texture size
		match *self
		{
			Texture::Raw (ref raw) => raw.borrow().size(),
			Texture::AtlasEntry (ref entry) => entry.borrow().cnv.size(),
		}
	}

	fn base_raw_id(&self) -> u32
	{
		// Return the underlying raw texture
		match *self
		{
			Texture::Raw (ref raw) => raw.borrow().id,
			Texture::AtlasEntry (ref entry) => entry.borrow().raw_tex.id,
		}
	}
	
	pub fn uv(&self) -> (f32, f32, f32, f32)
	{
		// Return the UV values
		match *self
		{
			Texture::Raw (_) => (0.0, 0.0, 1.0, 1.0),
			Texture::AtlasEntry (ref entry) => entry.borrow().uv,
		}
	}

	pub fn is_same(&self, other: &Texture) -> bool
	{
		// Check if both textures have the same underlying raw texture
		self.base_raw_id() == other.base_raw_id()
	}

	pub fn enable(&self)
	{
		// Enable the underyling raw texture
		match *self
		{
			Texture::Raw (ref raw) => raw.borrow().enable(),
			Texture::AtlasEntry (ref entry) => entry.borrow().raw_tex.enable(),
		}
	}

	pub fn update(&self, cnv: &Canvas)
	{
		// Update the texture
		match *self
		{
			Texture::Raw (ref raw) => raw.borrow_mut().update(cnv),
			Texture::AtlasEntry (_) => panic!("Texture.update(): Entries in texture a texture atlas cannot be updated"),
		}
	}
}


pub struct RawTexture
{
	id: u32,
	w: u32,
	h: u32,
}

impl RawTexture
{
	pub fn new(w: u32, h: u32, smooth: bool) -> RawTexture
	{
		// Create an OpenGL texture
		let mut id = 0;

		unsafe
		{
			// Create the texture
			gl::GenTextures(1, &mut id);

			// Bind it and set its parameters
			gl::BindTexture(gl::TEXTURE_2D, id);

			let v = if smooth { gl::LINEAR } else { gl::NEAREST } as i32;
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, v);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, v);
			
			// Create the store
			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, w as i32, h as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
		}

		RawTexture
		{
			id,
			w,
			h,
		}
	}

	pub fn from_canvas(cnv: &Canvas, smooth: bool) -> RawTexture
	{
		// Get the size
		let (w, h) = cnv.size();

		// Create the OpenGL texture
		let tex = RawTexture::new(w, h, smooth);

		// Upload the data to it
		unsafe
		{
			let ptr = cnv.data().as_ptr();
			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, w as i32, h as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr as *const std::os::raw::c_void);
		}

		tex
	}

	pub fn from_file(fname: &str, smooth: bool) -> Result<RawTexture, Error>
	{
		// Load the file into a canvas and create a texture from it
		let cnv = Canvas::from_file(fname)?;

		Ok(RawTexture::from_canvas(&cnv, smooth))
	}

	pub fn size(&self) -> (u32, u32)
	{
		(self.w, self.h)
	}

	pub fn enable(&self)
	{
		// Bind the texture
		unsafe
		{
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}

	pub fn update(&mut self, cnv: &Canvas)
	{
		// Bind the texture
		self.enable();

		// Update the data
		let (w, h) = cnv.size();
		
		unsafe
		{
			let ptr = cnv.data().as_ptr();
			gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, w as i32, h as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr as *const std::os::raw::c_void);
		}

		self.w = w;
		self.h = h;
	}
}

impl Drop for RawTexture
{
	fn drop(&mut self)
	{
		// Delete the texture
		unsafe
		{
			gl::DeleteTextures(1, &self.id);
		}
	}
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct AtlasEntry
{
	cnv: Canvas,
	raw_tex: Rc<RawTexture>,
	x: i32,
	y: i32,
	uv: (f32, f32, f32, f32),
}

pub struct TextureAtlas
{
	tex: Rc<RawTexture>,
	size: u32,
	packer: Packer,
	entries: Vec<Weak<RefCell<AtlasEntry>>>,
}

fn create_packer(size: u32) -> Packer
{
	// Setup a packer
	let config = rect_packer::Config
		{
			width: size as i32,
			height: size as i32,
			border_padding: 2,
			rectangle_padding: 2,
		};

	Packer::new(config)
}

impl TextureAtlas
{
	pub fn new(size: u32, smooth: bool) -> TextureAtlas
	{
		// Create a new raw texture
		let raw_tex = Rc::new(RawTexture::new(size, size, smooth));

		// Create the atlas
		TextureAtlas
		{
			tex: raw_tex,
			size,
			packer : create_packer(size),
			entries: Vec::new(),
		}
	}

	fn upload_data(&self, entry: &AtlasEntry)
	{
		// Upload the entry's canvas
		let (w, h) = entry.cnv.size();

		unsafe
		{
			gl::TexSubImage2D(gl::TEXTURE_2D, 0, entry.x, entry.y, w as i32, h as i32, gl::RGBA, gl::UNSIGNED_BYTE, entry.cnv.data().as_ptr() as *const std::os::raw::c_void);
		}
	}

	fn fix_uv(&self, entry: &mut AtlasEntry)
	{
		// Fix the UV values for the entry
		let (w, h) = entry.cnv.size();

		let u1 = entry.x as f32 / self.size as f32;
		let v1 = entry.y as f32 / self.size as f32;
		let u2 = (entry.x+w as i32) as f32 / self.size as f32;
		let v2 = (entry.y+h as i32) as f32 / self.size as f32;

		entry.uv = (u1, v1, u2, v2);
	}

	pub fn add(&mut self, cnv: Canvas) -> Result<Texture, Error>
	{
		// Try to pack 
		if let Some(rect) = self.packer.pack(cnv.width() as i32, cnv.height() as i32, false)
		{
			// Create an atlas entry
			let mut entry = AtlasEntry
				{
					cnv,
					raw_tex: Rc::clone(&self.tex),
					x: rect.x,
					y: rect.y,
					uv: (0.0, 0.0, 0.0, 0.0),
				};
			
			// Upload the canvas data
			self.upload_data(&entry);

			// Calc the UV values
			self.fix_uv(&mut entry);

			// Keep track of the entry
			let entry = Rc::new(RefCell::new(entry));
			self.entries.push(Rc::downgrade(&entry));

			Ok(Texture::AtlasEntry(entry))
		}
		else
		{
			// Did not fit
			Err(Error::PackAtlas)
		}
	}

	pub fn resize(&mut self, size: u32, smooth: bool) -> Result<(), Error>
	{
		// Create a new texture with the new size
		self.tex = Rc::new(RawTexture::new(size, size, smooth));
		self.size = size;

		// Re-create the packer
		self.packer = create_packer(size);

		// Cleanup dead entries
		self.entries.retain(|e| e.strong_count()>0);

		// Go through all the entries
		for entry in self.entries.iter()
		{
			// Borrow the entry
			let entry_cell = entry.upgrade().unwrap();
			let mut entry = entry_cell.borrow_mut();

			// Try to pack it
			if let Some(rect) = self.packer.pack(entry.cnv.width() as i32, entry.cnv.height() as i32, false)
			{
				// Adjust the entry
				entry.raw_tex = Rc::clone(&self.tex);
				entry.x = rect.x;
				entry.y = rect.y;

				// Re-upload the data
				self.upload_data(&entry);

				// Recalc the UV values
				self.fix_uv(&mut entry);
			}
			else
			{
				// Failure
				return Err(Error::PackAtlas);
			}
		}

		// All good
		Ok(())
	}
}



