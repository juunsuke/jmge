
use std::ops::{Deref, DerefMut};


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


pub struct MapGuard<T>
{
	ptr: *mut T,
	len: usize,
}

impl<T> Drop for MapGuard<T>
{
	fn drop(&mut self)
	{
		// Unmap and unbind
		unsafe
		{
			gl::UnmapBuffer(gl::ARRAY_BUFFER);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}
}

impl<T> MapGuard<T>
{
	pub fn ptr(&self) -> *mut T
	{
		self.ptr
	}
/*
	pub fn get(&self, index: usize) -> &mut T
	{
		unsafe
		{
			let ptr = self.ptr.offset(index as isize);
			let r: &mut T = &mut *ptr;

			r
		}
	}

	pub fn slice(&self, start: usize, count: usize) -> &mut [T]
	{
		unsafe
		{
			let ptr = self.ptr.offset(start as isize);
			std::slice::from_raw_parts_mut(ptr, count)
		}
	}*/
}

impl<T> Deref for MapGuard<T>
{
	type Target = [T];

	fn deref(&self) -> &[T]
	{
		unsafe
		{
			std::slice::from_raw_parts(self.ptr, self.len)
		}
	}
}

impl<T> DerefMut for MapGuard<T>
{
	fn deref_mut(&mut self) -> &mut [T]
	{
		unsafe
		{
			std::slice::from_raw_parts_mut(self.ptr, self.len)
		}
	}
}


pub struct VertexBuffer<T>
{
	vbo: u32,
	vao: u32,
	vbo_size: usize,
	phantom: std::marker::PhantomData<T>,
}

impl<T> VertexBuffer<T>
{

	pub fn new() -> VertexBuffer<T>
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
				phantom: std::marker::PhantomData,
			}
	}

	pub fn map(&mut self, count: usize) -> MapGuard<T>
	{
		// Bind the buffer
		unsafe
		{
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
		}
			
		// Make sure the buffer is big enough
		if count > self.vbo_size
		{
			// Resize it
			self.vbo_size = count;

			unsafe
			{
				gl::BufferData(gl::ARRAY_BUFFER, (self.vbo_size*VERTEX_SIZE) as isize, std::ptr::null(), gl::STREAM_DRAW);
			}
		}

		// Map the region
		let flags = gl::MAP_WRITE_BIT+gl::MAP_INVALIDATE_RANGE_BIT;
		let ptr = unsafe { gl::MapBufferRange(gl::ARRAY_BUFFER, 0, (count*VERTEX_SIZE) as isize, flags) };

		MapGuard
		{
			ptr: ptr as *mut T,
			len: count,
		}
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

impl<T> Drop for VertexBuffer<T>
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


