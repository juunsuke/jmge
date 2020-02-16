
#[derive(Clone, Copy)]
pub struct Color (pub u32);

fn f2u(v: f32) -> u8
{
	if v>=1.0
		{ 255 }
	else if v<=0.0
		{ 0 }
	else
		{ (v*255.0) as u8 }
}

fn shift(v: u8, c:u8) -> u32
{
	(v as u32)<<c
}


impl Color
{

	pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color
	{
		Color::rgba8(f2u(r), f2u(g), f2u(b), f2u(a))
	}

	pub fn rgb(r: f32, g: f32, b: f32) -> Color
	{
		Color::rgba(r, g, b, 1.0)
	}

	pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Color
	{
		Color { 0: shift(a, 24) | shift(b, 16) | shift(g, 8) | (r as u32) }
	}

	pub fn rgb8(r: u8, g: u8, b: u8) -> Color
	{
		Color::rgba8(r, g, b, 255)
	}

	pub fn a8(&self) -> u8 { ((self.0>>24) & 0xFF) as u8 }
	pub fn b8(&self) -> u8 { ((self.0>>16) & 0xFF) as u8 }
	pub fn g8(&self) -> u8 { ((self.0>>8) & 0xFF) as u8 }
	pub fn r8(&self) -> u8 { (self.0 & 0xFF) as u8 }

	pub fn a(&self) -> f32 { (self.a8() as f32) / 255.0 }
	pub fn b(&self) -> f32 { (self.b8() as f32) / 255.0 }
	pub fn g(&self) -> f32 { (self.g8() as f32) / 255.0 }
	pub fn r(&self) -> f32 { (self.r8() as f32) / 255.0 }

	pub fn blend(&self, o: Color) -> Color
	{
		// Blend this color and another one
		let sa = self.a8() as u32;

		if sa==255
			{ return *self; }
		else if sa==0
			{ return o; }

		let sr = self.r8() as u32;
		let sg = self.g8() as u32;
		let sb = self.b8() as u32;

		let dr = o.r8() as u32;
		let dg = o.g8() as u32;
		let db = o.b8() as u32;
		let da = o.a8() as u32;

		let ia = 255-sa;

		let r = (sr*sa)/255 + (dr*ia)/255;
		let g = (sg*sa)/255 + (dg*ia)/255;
		let b = (sb*sa)/255 + (db*ia)/255;
		let mut a = sa+da;
		if a > 255
			{ a = 255; }

		Color::rgba8(r as u8, g as u8, b as u8, a as u8)
	}
}

