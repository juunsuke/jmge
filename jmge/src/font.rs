
use super::{Error, Canvas};
use std::collections::HashMap;


pub struct Glyph
{
	w: u32,
	h: u32,
	data: Vec<u32>,
}


impl Glyph
{
	fn new(w: u32, h: u32, data: Vec<u32>) -> Glyph
	{
		Glyph
		{
			w,
			h,
			data,
		}
	}

	fn _empty() -> Glyph
	{
		Glyph
		{
			w: 0,
			h: 0,
			data: Vec::new(),
		}
	}

	pub fn width(&self) -> u32			{ self.w }
	pub fn height(&self) -> u32			{ self.h }
	pub fn data(&self) -> &Vec<u32>		{ &self.data }
}


pub struct Font
{
	char_map: HashMap<u32, u32>,
	glyphs: Vec<Glyph>,
	height: u32,
}

impl Font
{
/*	pub fn new(size: u32) -> Result<Font, Error>
	{
		// Load the font
		let font_data: &[u8] = include_bytes!("../DejaVuSansMono.ttf");
		let font = match rusttype::Font::from_bytes(font_data)
			{
				Ok(f) => f,
				Err(_) => return Err(Error::LoadFont("Couldn't load the built-in font".to_string())),
			};

		// Build the character to glyph map
		let mut char_map = HashMap::new();

		for c in 0..0x2FFFF
		{
			// Lookup the glyph for that codepoint
			let glyph = font.glyph(rusttype::Codepoint{0:c});
			let id = glyph.id().0;

			if id!=0
			{
				// There's a glyph
				char_map.insert(c, id);
			}
		}

		let mut glyphs = Vec::new();

		println!("Rendering {} glyphs", font.glyph_count());

		// Render all the glyphs
		for c in 0..font.glyph_count()
		{
			// Scale and position the glyph
			let glyph = font.glyph(rusttype::GlyphId{ 0:c as u32 });
			let glyph = glyph.scaled(rusttype::Scale { x: size as f32, y: size as f32 });
			let glyph = glyph.positioned(rusttype::Point { x: 0.0, y: 0.0 });

			if let Some(bb) = glyph.pixel_bounding_box()
			{
				// Render the glyph
				let w = bb.width() as u32;
				let h = bb.height() as u32;

				let mut vec = vec![0.0; (w*h) as usize];

				//println!("{},{}  -  {},{}     {}x{}", bb.min.x, bb.min.y, bb.max.x, bb.max.y, w, h);

				glyph.draw(|x, y, v|
					{
						vec[(y*w as u32+x) as usize] = v;
					}
				);

				// Add it
				glyphs.push(Glyph::new(w, h, vec));
			}
			else
			{
				// Add an empty glyph
				glyphs.push(Glyph::empty());
			}
		}

		println!("Done, {}", glyphs.len());

		let fnt = Font
		{
			char_map,
			glyphs,
		};

		Ok(fnt)
	}
*/
	pub fn from_canvas(cnv: &Canvas) -> Result<Font, Error>
	{
		// Create a bitmap font from a canvas

		// Validate the size
		let (w, h) = cnv.size();
		if (w%16)!=0 || (h%16)!=0
			{ return Err(Error::LoadFont(String::from("Font::from_canvas(): the canvas size isn't evenly divisible by 16"))); }

		// Character size
		let cw = w/16;
		let ch = h/16;

		// Build a simple character map
		let mut char_map = HashMap::new();

		for c in 0..255
		{
			char_map.insert(c, c);
		}

		// Build the glyphs
		let mut glyphs = Vec::new();

		for c in 0..255
		{
			// Top-left pixel location in the canvas
			let cy = (c/16)*ch;
			let cx = (c%16)*cw;

			// Create a sub-canvas
			let sub = cnv.sub(cx, cy, cw, ch);

			// Create and add the glyph
			glyphs.push(Glyph::new(cw, ch, sub.into_data()));
		}

		let fnt = Font
		{
			char_map,
			glyphs,
			height: ch,
		};

		Ok(fnt)
	}

	pub fn get_glyph(&self, ch: char) -> Option<&Glyph>
	{
		// Check if the character is mapped
		if let Some(i) = self.char_map.get(&(ch as u32))
		{
			Some(&self.glyphs[*i as usize])
		}
		else
		{
			// Not found
			None
		}
	}

	pub fn char_width(&self, ch: char) -> u32
	{
		// Get the size of a character
		match self.get_glyph(ch)
		{
			Some(g) => g.width(),
			_ => 0,
		}
	}

	pub fn str_width(&self, s: &str) -> u32
	{
		// Get the size of a string
		s.chars().map(|ch| self.char_width(ch)).sum()
	}

	pub fn height(&self) -> u32
	{
		// Font height
		self.height
	}
}
