

#[repr(C)]
pub struct Vertex
{
	pub x: f32,
	pub y: f32,
	pub col: u32,
	pub u: f32,
	pub v: f32,
	pub tx: f32,
	pub ty: f32,
	pub sx: f32,
	pub sy: f32,
	pub angle: f32,
	pub ox: f32,
	pub oy: f32,
}

const VERTEX_SIZE:usize = 48;


pub struct VertexBuffer
{
	vbo: u32,
	vao: u32,
	vbo_size: usize,
}

impl VertexBuffer
{

	pub fn new() -> VertexBuffer
	{
		let mut vbo = 0;
		let mut vao = 0;
		let vbo_size = 6000;

		unsafe
		{
			// Create the VBO and VAO
			gl::GenBuffers(1, &mut vbo);
			gl::GenVertexArrays(1, &mut vao);

			// Define the attributes
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

			// Position
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, VERTEX_SIZE as i32, 0 as *const std::os::raw::c_void);

			// Color
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(1, 4, gl::UNSIGNED_BYTE, gl::TRUE, VERTEX_SIZE as i32, 8 as *const std::os::raw::c_void);

			// TexCoord
			gl::EnableVertexAttribArray(2);
			gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, VERTEX_SIZE as i32, 12 as *const std::os::raw::c_void);

			// Translation
			gl::EnableVertexAttribArray(3);
			gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, VERTEX_SIZE as i32, 20 as *const std::os::raw::c_void);

			// Scale
			gl::EnableVertexAttribArray(4);
			gl::VertexAttribPointer(4, 2, gl::FLOAT, gl::FALSE, VERTEX_SIZE as i32, 28 as *const std::os::raw::c_void);

			// Angle
			gl::EnableVertexAttribArray(5);
			gl::VertexAttribPointer(5, 1, gl::FLOAT, gl::FALSE, VERTEX_SIZE as i32, 36 as *const std::os::raw::c_void);

			// Origin
			gl::EnableVertexAttribArray(6);
			gl::VertexAttribPointer(6, 2, gl::FLOAT, gl::FALSE, VERTEX_SIZE as i32, 40 as *const std::os::raw::c_void);

			// Allocate a decent initial buffer
			gl::BufferData(gl::ARRAY_BUFFER, (vbo_size*VERTEX_SIZE) as isize, std::ptr::null(), gl::STREAM_DRAW);

			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}

		VertexBuffer
			{
				vbo,
				vao,
				vbo_size,
			}
	}

	pub unsafe fn map(&mut self, first: usize, count: usize) -> *mut Vertex
	{
		// Bind the buffer
		gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			
		// Make sure the buffer is big enough
		if first+count > self.vbo_size
		{
			// Resize it
			self.vbo_size = first+count;

			gl::BufferData(gl::ARRAY_BUFFER, (self.vbo_size*VERTEX_SIZE) as isize, std::ptr::null(), gl::STREAM_DRAW);
		}

		// Map the region and return the pointer
		gl::MapBufferRange(gl::ARRAY_BUFFER, (first*VERTEX_SIZE) as isize, (count*VERTEX_SIZE) as isize, gl::MAP_WRITE_BIT+gl::MAP_INVALIDATE_RANGE_BIT) as *mut Vertex
	}

	pub unsafe fn unmap(&self)
	{
		// Unmap and unbind
		gl::UnmapBuffer(gl::ARRAY_BUFFER);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}

	pub fn draw_triangles(&self, first: u32, count: u32)
	{
		unsafe
		{
			// Bind the VAO
			gl::BindVertexArray(self.vao);

			// Draw the triangles
			gl::DrawArrays(gl::TRIANGLES, first as i32, count as i32);
			
			// Unbind
			gl::BindVertexArray(0);
		}
	}

}

impl Drop for VertexBuffer
{
	fn drop(&mut self)
	{
		// Drop the VAO and VBO
		unsafe
		{
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);

			gl::DeleteBuffers(1, &mut self.vbo);
			gl::DeleteVertexArrays(1, &mut self.vao);
		}
	}

}


