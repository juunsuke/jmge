
use super::{Color, Shader, Error, Input, Mouse, Keyboard};
use glfw::{Context, WindowEvent};
use nalgebra::base::Matrix4;
use std::time::{Instant};

pub struct Window
{
	proj_mat: Matrix4<f32>,
	shader: Shader,

	input: Input,

	frame: u32,
	frame_tot: u64,
	last_time: Instant,
	fps: f32,
	
	window: glfw::Window,
	events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
	glfw: glfw::Glfw,
}


fn calc_proj(w: f32, h: f32) -> Matrix4<f32>
{
	// Calc a projection matrix
	Matrix4::new_orthographic(0.0, w, h, 0.0, -1.0, 1.0)
}

impl Window
{

	pub fn new() -> Result<Window, Error>
	{
		// Initialize GLFW
		let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		// Create the default window
		glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
		glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
		glfw.window_hint(glfw::WindowHint::Resizable(false));

		let (mut window, events) = glfw.create_window(1920, 1080, "JMGE", glfw::WindowMode::Windowed).unwrap();

		window.make_current();
		window.set_all_polling(true);

		// Setup OpenGL
		gl::load_with(|s| glfw.get_proc_address_raw(s));

		//glfw.set_swap_interval(glfw::SwapInterval::None);

		unsafe
		{
			gl::Disable(gl::DEPTH_TEST);
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}

		// Set the viewport
		let (w, h) = window.get_size();

		unsafe { gl::Viewport(0, 0, w, h); }

		// Prepare the projection matrix
		let proj_mat = calc_proj(w as f32, h as f32);

		let shader = Shader::new_default()?;
		shader.enable();

		let wnd = Window
		{
			glfw,
			window,
			events,
			proj_mat,
			shader,
			frame: 0,
			frame_tot: 0,
			last_time: Instant::now(),
			fps: 0.0,
			input: Input::new(),
		};

		Ok(wnd)
	}

	pub fn set_projection(&self)
	{
		// Set the projection matrix
		self.shader.set_uniform_matrix("Projection", &self.proj_mat);
	}

	pub fn swap(&mut self)
	{
		// Swap the display buffers
		self.window.swap_buffers();

		// Perform FPS calculations
		self.frame += 1;
		self.frame_tot += 1;

		let now = Instant::now();
		let ms = now.duration_since(self.last_time).as_millis();

		if ms>=1000
		{
			// Calc the new FPS
			self.fps = (self.frame as f32) / (ms as f32 / 1000.0);
			self.frame = 0;
			self.last_time = now;

			// Update the title
			self.window.set_title(&format!("JMGE - {} fps", self.fps));
		}
	}

	pub fn poll_events(&mut self)
	{
		// Reset the input state
		self.input.reset();

		// Poll the queued events
		self.glfw.poll_events();

		for (_, event) in glfw::flush_messages(&self.events)
		{
			//println!("{:?}", event);

			match event
			{
				WindowEvent::CursorPos (x, y) =>
					{
						// Update the mouse position
						self.input.mouse_mut().set_pos(x as i32, y as i32);
					},

				WindowEvent::MouseButton (but, glfw::Action::Press, _) =>
					{
						self.input.mouse_mut().set_but(but as u8, true);
					},

				WindowEvent::MouseButton (but, glfw::Action::Release, _) =>
					{
						self.input.mouse_mut().set_but(but as u8, false);
					},

				WindowEvent::Key(key, _, glfw::Action::Press, _) =>
					{
						self.input.keyboard_mut().set_key(key, true);
					},

				WindowEvent::Key(key, _, glfw::Action::Release, _) =>
					{
						self.input.keyboard_mut().set_key(key, false);
					},


				_ => (),
			}
		}
	}

	pub fn should_close(&self) -> bool
	{
		self.window.should_close()
	}

	pub fn clear(&self, col: Color)
	{
		// Clear the window
		unsafe
		{
			gl::ClearColor(col.r(), col.g(), col.b(), col.a());
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}

	pub fn fps(&self) -> f32
	{
		self.fps
	}

	pub fn frames(&self) -> u64
	{
		self.frame_tot
	}

	pub fn input(&self) -> &Input
	{
		&self.input
	}

	pub fn mouse(&self) -> &Mouse
	{
		self.input.mouse()
	}

	pub fn keyboard(&self) -> &Keyboard
	{
		self.input.keyboard()
	}
}


