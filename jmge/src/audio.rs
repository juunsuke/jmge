
use super::{Error};
use std::io::{Seek, SeekFrom, Read};
use std::sync::Arc;

use rodio::{Decoder, Sink, Source};


pub struct Audio
{
	device: rodio::Device,
}

impl Audio
{
	pub fn new() -> Result<Audio, Error>
	{
		// Setup rodio
		let device = match rodio::default_output_device()
			{
				Some(v) => v,
				None => return Err(Error::NoAudioDevice),
			};
		
		Ok(Audio
		{
			device,
		})
	}

	pub fn play(&self, snd: &Sound) -> SoundControl
	{
		// Create a sound reader
		let reader = SoundReader::new(&snd.data);

		// Create a decoder
		let source = Decoder::new(reader).unwrap().buffered();

		// Create a sink
		let sink = Sink::new(&self.device);

		// Play the sound
		sink.append(source);

		SoundControl { sink }
	}

	pub fn play_detached(&self, snd: &Sound)
	{
		// Play and detach
		self.play(snd).detach();
	}
}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct SoundControl
{
	sink: Sink,
}

impl SoundControl
{
	pub fn detach(self)
	{
		self.sink.detach();
	}

	pub fn volume(&self) -> f32
	{
		self.sink.volume()
	}

	pub fn set_volume(&self, vol: f32)
	{
		self.sink.set_volume(vol);
	}
}


//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct SoundReader
{
	data: Arc<Vec<u8>>,
	pos: i64,
}

impl SoundReader
{
	fn new(data: &Arc<Vec<u8>>) -> SoundReader
	{
		SoundReader
		{
			data: Arc::clone(data),
			pos: 0,
		}
	}
}

impl Seek for SoundReader
{
	fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64>
	{
		match pos
		{
			SeekFrom::Start (x) => self.pos = x as i64,
			SeekFrom::End (x) => self.pos = self.data.len() as i64 - x,
			SeekFrom::Current (x) => self.pos += x,
		}

		if self.pos<0
			{ self.pos = 0; }

		if self.pos>self.data.len() as i64
			{ self.pos = self.data.len() as i64 }

		Ok(self.pos as u64)
	}
}

impl Read for SoundReader
{
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
	{
		// Calc the range to copy
		let mut want = buf.len() as i64;
		let len = self.data.len() as i64;

		if self.pos==len || want==0
			{ return Ok(0); }

		if (self.pos+want)>len
			{ want = len-self.pos; }

		// Copy the data
		buf.copy_from_slice(&self.data[self.pos as usize..(self.pos+want) as usize]);
		self.pos += want;

		Ok(want as usize)
	}
}


//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

pub struct Sound
{
	data: Arc<Vec<u8>>,
}

impl Sound
{
	pub fn from_file(fname: &str) -> Result<Sound, Error>
	{
		// Read the file
		let data = match std::fs::read(fname)
			{
				Ok(data) => data,
				Err(_) => return Err(Error::LoadSound),
			};

		Ok(Sound
		{
			data: Arc::new(data),
		})
	}
}


