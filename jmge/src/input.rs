
pub use glfw::Key;


pub struct Mouse
{
	ox: i32,
	oy: i32,
	x: i32,
	y: i32,
	obuts: u8,
	buts: u8,
}

impl Mouse
{

	pub fn new() -> Mouse
	{
		Mouse
		{
			ox: 0,
			oy: 0,
			x: 0,
			y: 0,
			obuts: 0,
			buts: 0,
		}
	}

	pub fn reset(&mut self)
	{
		// Reset the state
		self.ox = self.x;
		self.oy = self.y;
		self.obuts = self.buts;
	}

	pub fn set_pos(&mut self, x: i32, y: i32)
	{
		// Set a new mouse position
		self.x = x;
		self.y = y;
	}

	pub fn pos(&self) -> (i32, i32)					{ (self.x, self.y) }
	pub fn pos_old(&self) -> (i32, i32)				{ (self.ox, self.oy) }
	pub fn pos_delta(&self) -> (i32, i32)			{ (self.x-self.ox, self.y-self.oy) }
	pub fn moved(&self) -> bool						{ (self.x!=self.ox) || (self.y!=self.oy) }

	pub fn set_but(&mut self, but: u8, down: bool)
	{
		// Set/clear a button state
		let but = 1<<but;

		if down
		{
			self.buts |= but;
		}
		else
		{
			self.buts &= 255-but;
		}
	}

	pub fn buts(&self) -> u8						{ self.buts }
	pub fn buts_old(&self) -> u8					{ self.obuts }

	pub fn but_down(&self, but: u8) -> bool
	{
		// Get the down/up state of a specific button
		if but>7
			{ panic!("Mouse.but_down(): invalid button index"); }

		(self.buts & (1<<but)) > 0
	}

	pub fn but_down_only(&self, but: u8) -> bool
	{
		// Check if a button is the only button down
		if but>7
			{ panic!("Mouse.but_down(): invalid button index"); }

		self.buts == (1<<but)
	}

	pub fn but_pressed(&self, but: u8) -> bool
	{
		// Check if a button was pressed on this frame
		if but>7
			{ panic!("Mouse.but_down(): invalid button index"); }
		
		((self.buts & (1<<but)) > 0) && ((self.obuts & (1<<but)) == 0)
	}

	pub fn but_released(&self, but: u8) -> bool
	{
		// Check if a button was released on this frame
		if but>7
			{ panic!("Mouse.but_down(): invalid button index"); }
		
		((self.buts & (1<<but)) == 0) && ((self.obuts & (1<<but)) > 0)
	}

}

//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------

const NUM_KEYS: usize = glfw::ffi::KEY_LAST as usize + 1;


pub struct Keyboard
{
	keys: Vec<u32>,
	okeys: Vec<u32>,
}

impl Keyboard
{
	pub fn new() -> Keyboard
	{
		// Setup the key vectors
		Keyboard
		{
			keys: vec![0; NUM_KEYS],
			okeys: vec![0; NUM_KEYS],
		}
	}

	pub fn reset(&mut self)
	{
		// Reset the state
		for c in 0..NUM_KEYS
		{
			// Copy the current state
			self.okeys[c] = self.keys[c];

			// Increase the counter if it's down
			if self.keys[c]>0
				{ self.keys[c] += 1; }
		}
	}

	pub fn set_key(&mut self, key: Key, down: bool)
	{
		// Skip the unknown key
		if key==Key::Unknown
			{ return; }

		// Change the state of a key
		self.keys[key as usize] = match down { true => 1, false => 0};
	}

	pub fn key_down(&self, key: Key) -> bool
	{
		// Check wether a key is currently marked as down
		self.keys[key as usize] > 0
	}

	pub fn key_pressed(&self, key: Key) -> bool
	{
		// Check wether a key was pressed on this frame
		self.keys[key as usize] == 1
	}

	pub fn key_released(&self, key: Key) -> bool
	{
		// Check wether a key was released on this frame
		(self.keys[key as usize] == 0) && (self.okeys[key as usize] > 0)
	}
}


//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------------------


pub struct Input
{
	mouse: Mouse,
	keyboard: Keyboard,
}

impl Input
{

	pub fn new() -> Input
	{
		Input
		{
			mouse: Mouse::new(),
			keyboard: Keyboard::new(),
		}
	}

	pub fn reset(&mut self)
	{
		// Reset the input state for a new frame
		self.mouse.reset();
		self.keyboard.reset();
	}

	pub fn mouse(&self) -> &Mouse							{ &self.mouse }
	pub fn mouse_mut(&mut self) -> &mut Mouse				{ &mut self.mouse }

	pub fn keyboard(&self) -> &Keyboard						{ &self.keyboard }
	pub fn keyboard_mut(&mut self) -> &mut Keyboard			{ &mut self.keyboard }

}

