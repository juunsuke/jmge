
use super::{Error, Canvas, Texture, TextureAtlas, Component, System, World, Renderable};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;


struct Frame
{
	tex: Rc<Texture>,
	dur: u32,
}

struct Tag
{
	from: usize,
	to: usize,
}


pub struct SpriteSheet
{
	frames: Vec<Frame>,
	tags: HashMap<String, Tag>,
}

impl SpriteSheet
{
	pub fn from_file(fname: &str, cnv: &Canvas, atlas: &mut TextureAtlas) -> Result<SpriteSheet, Error>
	{
		// Load the file
		let s = match std::fs::read_to_string(fname)
			{
				Ok (s) => s,
				Err (_) => return Err(Error::LoadSpriteSheet("Could not read the file".to_string())),
			};

		// Parse it
		let json = match json::parse(&s)
			{
				Ok (json) => json,
				Err (_) => return Err(Error::LoadSpriteSheet("Error parsing JSON data".to_string())),
			};

		// Extract the frames
		let mut frames = Vec::new();

		for f in json["frames"].members()
		{
			//let file = f["filename"].as_str().unwrap();
			let x = *&f["frame"]["x"].as_u32().unwrap();
			let y = *&f["frame"]["y"].as_u32().unwrap();
			let w = *&f["frame"]["w"].as_u32().unwrap();
			let h = *&f["frame"]["h"].as_u32().unwrap();
			let dur = *&f["duration"].as_u32().unwrap();

			// Create a sub-canvas and an atlas texture entry
			let sub = cnv.sub(x, y, w, h);
			let tex = Rc::new(atlas.add(sub)?);

			// Create a frame
			let frame = Frame
				{
					tex,
					dur,
				};

			frames.push(frame);
		}

		// Extract the tags
		let mut tags = HashMap::new();

		for t in json["meta"]["frameTags"].members()
		{
			let name = t["name"].as_str().unwrap();
			let from = t["from"].as_usize().unwrap();
			let to = t["to"].as_usize().unwrap();

			if from>to || to>=frames.len()
				{ return Err(Error::LoadSpriteSheet("Inconsistant tag info".to_string())); }

			// Create a tag
			let tag = Tag
				{
					from,
					to,
				};

			tags.insert(name.to_string(), tag);
		}


		Ok(SpriteSheet
			{
				frames,
				tags,
			}
		)
	}

	pub fn get_frame(&self, i: usize) -> (Rc<Texture>, u32)
	{
		// Get a reference to a frame
		(Rc::clone(&self.frames[i].tex), self.frames[i].dur)
	}

	pub fn get_tag(&self, name: &str) -> (usize, usize)
	{
		// Get a reference to a tag
		let tag = self.tags.get(name).expect("no such tag in the spritesheet");

		(tag.from, tag.to)
	}
}


//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------


#[derive(Component)]
pub struct Sprite
{
	ss: Rc<SpriteSheet>,
	cur_tag: String,
	from: usize,
	to: usize,
	pos: usize,
	cur_tex: Rc<Texture>,
	cur_dur: i64,
	last_time: i64,
	next_tag: Option<String>,
	rolled: bool,
}

impl Sprite
{
	pub fn new(ss: &Rc<SpriteSheet>, tag: &str) -> Sprite
	{
		// Create a new sprite based on the given spritesheet

		// Get the initial tag
		let (from, to) = ss.get_tag(tag);
		let (tex, dur) = ss.get_frame(from);

		Sprite
		{
			ss: Rc::clone(ss),
			cur_tag: String::from(tag),
			from,
			to,
			pos: from,
			cur_tex: tex,
			cur_dur: dur as i64,
			last_time: 0,
			next_tag: None,
			rolled: false,
		}
	}
	
	pub fn set_tag(&mut self, tag: &str)
	{
		// Change the current tag
		let (from, to) = self.ss.get_tag(tag);
		let (tex, dur) = self.ss.get_frame(from);

		self.from = from;
		self.to = to;
		self.pos = from;
		self.cur_tex = tex;
		self.cur_dur = dur as i64;
		self.last_time = 0;
		self.next_tag = None;
	}

	pub fn set_next_tag(&mut self, next_tag: &str)
	{
		// Set the next tag
		self.next_tag = Some(String::from(next_tag));
	}

	pub fn process(&mut self, time: i64) -> bool
	{
		// Process the animation
		self.rolled = false;
		let mut changed = false;

		// Advance to the next frame ?
		if (time-self.last_time) >= self.cur_dur
		{
			// Yes
			changed = true;

			if self.pos==self.to
			{
				// Roll over
				self.rolled = true;
				self.pos = self.from;

				// Change the tag ?
				if let Some(next_tag) = self.next_tag.take()
				{
					// Yes
					self.set_tag(&next_tag);
				}
			}
			else
			{
				self.pos += 1;
			}

			// Adjust the time
			let diff = (time-self.last_time) - self.cur_dur;

			if diff>self.cur_dur
			{
				// Probly the first process, or a super lag
				// Either way, reset to current time
				self.last_time = time;
			}
			else
			{
				// Adjust for proper timing
				self.last_time = time-diff;
			}
			
			// Get the new frame info
			let (tex, dur) = self.ss.get_frame(self.pos);
			self.cur_tex = tex;
			self.cur_dur = dur as i64;

		}

		changed
	}

	pub fn get_texture(&self) -> Rc<Texture>
	{
		// Get the current frame texture
		Rc::clone(&self.cur_tex)
	}

	pub fn cur_tag(&self) -> String
	{
		// Current tag name
		self.cur_tag.clone()
	}

	pub fn next_tag(&self) -> Option<String>
	{
		// Queued next tag
		match &self.next_tag
		{
			Some (s) => Some(s.clone()),
			None => None,
		}
	}

	pub fn rolled(&self) -> bool
	{
		// Rolled over flag
		self.rolled
	}
}


pub struct SpriteSystem
{
	time: Instant,
}

impl SpriteSystem
{
	pub fn new() -> SpriteSystem
	{
		SpriteSystem
		{
			time: Instant::now(),
		}
	}
}

impl System for SpriteSystem
{
	fn run(&mut self, world: &World)
	{
		// Get the time
		let time = self.time.elapsed().as_millis() as i64;

		// Process all the sprites
		for (e, mut sp) in world.iter_mut::<Sprite>()
		{
			// Process it
			let changed = sp.process(time);

			// Update the renderable if the texture changed
			if changed
			{
				if let Some(mut rend) = world.try_get_mut::<Renderable>(&e)
				{
					rend.texture = sp.get_texture();
				}
			}
		}
	}
}



