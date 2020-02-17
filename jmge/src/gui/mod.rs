
mod button;
pub use button::Button;


use super::{Input};


pub struct Widget
{
}



pub struct Gui
{
}

impl Gui
{
	pub fn new() -> Gui
	{
		Gui
		{
		}
	}

	pub fn process_input(&mut self, input: &Input)
	{
	}
}


