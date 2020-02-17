
use super::{Error, Color, Font};
use image::DynamicImage;


pub struct Canvas
{
	w: u32,
	h: u32,
	data: Vec<u32>,
}


impl Canvas
{

	pub fn new(width: u32, height: u32, col: Color) -> Canvas
	{
		// Create an empty canvas
		if width==0 || height==0
			{ panic!("Canvas::new(): width and height can't be zero"); }

		let width = width as usize;
		let height = height as usize;

		// Allocate the data
		let mut data = Vec::with_capacity(width*height);

		unsafe
		{
			data.set_len(width*height);
		}

		// Create the canvas
		let mut cnv = Canvas
			{
				w: width as u32,
				h: height as u32,
				data,
			};

		// Clear it
		cnv.clear(col);

		cnv
	}

	pub fn from_image(img: DynamicImage) -> Result<Canvas, Error>
	{
		// Create a canvas from an image

		// Convert it to RGBA
		let img = img.to_rgba();

		// Get the size
		let w = img.width();
		let h = img.height();

		// Convert to a raw array
		let raw = img.into_raw();
		let ptr = raw.as_ptr() as *const u32;

		// Copy the data into a vector
		let size = (w*h) as usize;
		let mut vec = Vec::with_capacity(size);

		unsafe
		{
			vec.set_len(size);
			std::ptr::copy_nonoverlapping(ptr, vec.as_mut_ptr(), size);
		}

		let cnv = Canvas
			{
				w,
				h,
				data: vec,
			};

		Ok(cnv)
	}

	pub fn from_file(fname: &str) -> Result<Canvas, Error>
	{
		// Load an image file
		let img = match image::open(&std::path::Path::new(fname))
			{
				Ok (i) => i,
				Err (e) => return Err(Error::LoadImage(e.to_string())),
			};

		Canvas::from_image(img)
	}

	pub fn from_memory_file(buf: &[u8]) -> Result<Canvas, Error>
	{
		// Load an image
		let img = match image::load_from_memory(buf)
			{
				Ok (i) => i,
				Err (e) => return Err(Error::LoadImage(e.to_string())),
			};

		Canvas::from_image(img)
	}

	pub fn from_raw(w: u32, h: u32, data: Vec<u32>) -> Canvas
	{
		// Wrap raw data into a canvas
		if (w*h) as usize != data.len()
			{ panic!("Canvas::from_raw(): invalid dimensions"); }

		Canvas
		{
			w,
			h,
			data,
		}
	}

	pub fn clear(&mut self, col: Color)
	{
		// Clear the canvas with the given color
		unsafe
		{
			let sptr = self.data.as_ptr();
			let mut dptr = self.data.as_mut_ptr() as *mut u32;

			for x in 0..self.w as isize
				{ *(dptr.offset(x)) = col.0; }
				
			for _ in 1..self.h as isize
			{
				dptr = dptr.offset(self.w as isize);
				std::ptr::copy_nonoverlapping(sptr, dptr, self.w as usize);
			}
		}
	}

	pub fn into_data(self) -> Vec<u32>
	{
		// Extract the data
		self.data
	}

	pub fn size(&self) -> (u32, u32)
	{
		(self.w, self.h)
	}

	pub fn width(&self) -> u32
	{
		self.w
	}

	pub fn height(&self) -> u32
	{
		self.h
	}

	pub fn data(&self) -> &Vec<u32>
	{
		&self.data
	}

	pub fn sub(&self, x: u32, y: u32, w: u32, h: u32) -> Canvas
	{
		// Create a new sub-canvas
		if w==0 || h==0
			{ panic!("Canvas.sub(): invalid width or height"); }

		if (x+w)>self.w || (y+h)>self.h
			{ panic!("Canvas.sub(): invalid region"); }

		// Copy the data into a new vector
		let mut vec = Vec::with_capacity((w*h) as usize);
		unsafe { vec.set_len((w*h) as usize); }

		let mut sptr = unsafe { self.data.as_ptr().offset((y*self.w+x) as isize) };
		let mut dptr = vec.as_mut_ptr();

		// Copy the data row by row
		for _ in 0..h
		{
			unsafe
			{
				std::ptr::copy_nonoverlapping(sptr, dptr, w as usize);
				
				sptr = sptr.offset(self.w as isize);
				dptr = dptr.offset(w as isize);
			}
		}

		// Create and return the new canvas
		Canvas::from_raw(w, h, vec)
	}

	pub fn contains(&self, x: i32, y: i32) -> bool
	{
		// Check if the coordinates is contained in the canvas
		x>=0 && x<self.w as i32 && y>=0 && y<self.h as i32
	}

	pub fn set_pixel(&mut self, x: i32, y: i32, col: Color)
	{
		if self.contains(x, y)
			{ self.data[((y*self.w as i32)+x) as usize] = col.0; }
	}

	pub fn get_pixel(&self, x: i32, y: i32) -> Color
	{
		if self.contains(x, y)
			{ Color { 0:self.data[((y*self.w as i32)+x) as usize] } }
		else
			{ Color { 0:0 } }
	}

	pub fn draw_char(&mut self, font: &Font, x: i32, y: i32, col: Color, ch: char) -> i32
	{
		// Draw a character
		if let Some(glyph) = font.get_glyph(ch)
		{
			let gw = glyph.width() as i32;
			let data = glyph.data();

			for (i, v) in data.iter().enumerate()
			{
				let cx = i as i32%gw;
				let cy = i as i32/gw;

				if *v!=0
					{ self.set_pixel(x+cx, y+cy, col.blend(self.get_pixel(x+cx, y+cy))); }
			}

			gw as i32
		}
		else
		{
			0
		}
	}

	pub fn draw_text(&mut self, font: &Font, mut x: i32, y: i32, col: Color, text: &str)
	{
		// Draw a string of text
		for ch in text.chars()
		{
			x = x + self.draw_char(font, x, y, col, ch);
		}
	}

	pub fn blit(&mut self, mut dx: i32, mut dy: i32, o: &Canvas, mut sx: i32, mut sy: i32, w: u32, h: u32, alpha: bool)
	{
		// Clip the values
		if w==0 || h==0
			{ return; }

		let dw = self.w as i32;
		let dh = self.h as i32;
		let sw = o.w as i32;
		let sh = o.h as i32;
		let mut w = w as i32;
		let mut h = h as i32;

		if dx>=dw || (dx+w)<=0		{ return; }
		if dy>=dh || (dy+h)<=0		{ return; }

		if sx>=sw || (sx+w)<=0		{ return; }
		if sy>=sh || (sy+h)<=0		{ return; }

		if (dx+w)>dw
			{ w = dw-dx; }
		if (dy+h)>dh
			{ h = dh-dy; }

		if (sx+w)>sw
			{ w = sw-sx; }
		if (sy+h)>sh
			{ h = sh-sy; }

		if dx<0
		{
			w += dx;
			sx -= dx;
			dx = 0;
		}
		if dy<0
		{
			h += dy;
			sy -= dy;
			dy = 0;
		}

		if sx<0
		{
			w += sx;
			dx -= sx;
			sx = 0;
		}
		if sy<0
		{
			h += sy;
			dy -= sy;
			sy = 0;
		}

		if w<=0 || h<=0
			{ return; }

		// Perform the blit
		let mut sptr = unsafe { o.data.as_ptr().offset((sy*sw+sx) as isize) };
		let mut dptr = unsafe { self.data.as_mut_ptr().offset((dy*dw+dx) as isize) };

		if alpha
		{
			// Copy pixel by pixel
			for _ in 0..h
			{
				for x in 0..w
				{
					unsafe
					{
						let scol = Color { 0:*sptr.offset(x as isize) };
						let dcol = Color { 0:*dptr.offset(x as isize) };
						*dptr.offset(x as isize) = scol.blend(dcol).0;
					}
				}

				unsafe
				{
					sptr = sptr.offset(sw as isize);
					dptr = dptr.offset(dw as isize);
				}
			}
		}
		else
		{
			// Raw copy
			for _ in 0..h
			{
				unsafe
				{
					std::ptr::copy_nonoverlapping(sptr, dptr, w as usize);
					sptr = sptr.offset(sw as isize);
					dptr = dptr.offset(dw as isize);
				}
			}
		}
	}
}


