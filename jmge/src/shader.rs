
use std::ffi::{CString, CStr};
use super::Error;
use nalgebra::base::{Matrix4};


fn space_cstring(len: usize) -> CString
{
	// Create a C string filled with spaces
	let mut buffer: Vec<u8> = Vec::with_capacity(len+1);
	buffer.extend([b' '].iter().cycle().take(len));
	unsafe { CString::from_vec_unchecked(buffer) }
}

enum ShaderType
{
	Vertex,
	Fragment,
}

struct ShaderPart
{
	id: u32
}

impl ShaderPart
{
	pub fn from_string(source: &str, stype: ShaderType) -> Result<ShaderPart, Error>
	{
		// Convert the string to a C string
		let source: &CStr = &CString::new(source).unwrap();

		// Create a new shader
		let stype = match stype
			{
				ShaderType::Vertex => gl::VERTEX_SHADER,
				ShaderType::Fragment => gl::FRAGMENT_SHADER
			};
		let id = unsafe { gl::CreateShader(stype) };
		let shader = ShaderPart { id };

		unsafe
		{
			// Compile the shader
			gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
			gl::CompileShader(id);
		
			// Check success
			let mut success: i32 = 1;
			gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

			if success==0
			{
				// Failure, extract the error message
				let mut len: i32 = 0;
				gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

				let error: CString = space_cstring(len as usize);
				gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut i8);

				return Err(Error::CompileShader(error.to_string_lossy().into_owned()));
			}
		}

		Ok(shader)
	}

	pub fn from_string_vertex(source: &str) -> Result<ShaderPart, Error>
	{
		ShaderPart::from_string(source, ShaderType::Vertex)
	}

	pub fn from_string_fragment(source: &str) -> Result<ShaderPart, Error>
	{
		ShaderPart::from_string(source, ShaderType::Fragment)
	}
}

impl Drop for ShaderPart
{
	fn drop(&mut self)
	{
		// Delete the shader
		unsafe { gl::DeleteShader(self.id); }
	}
}


pub struct Shader
{
	id: u32,
}

impl Shader
{
	pub fn new(vert: &str, frag: &str) -> Result<Shader, Error>
	{
		// Create and compile both shader parts
		let vert = ShaderPart::from_string_vertex(vert)?;
		let frag = ShaderPart::from_string_fragment(frag)?;

		// Create the program
		let id = unsafe { gl::CreateProgram() };
		let shader = Shader { id };

		unsafe
		{
			// Link the program
			gl::AttachShader(id, vert.id);
			gl::AttachShader(id, frag.id);
			gl::LinkProgram(id);

			// Check success
			let mut success: i32 = 1;
			gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

			if success==0
			{
				// Failure, extract the error message
				let mut len: i32 = 0;
				gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);

				let error = space_cstring(len as usize);
				gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut i8);
		
				return Err(Error::CompileShader(error.to_string_lossy().into_owned()));
			}

		}

		Ok(shader)
	}

	pub fn new_default() -> Result<Shader, Error>
	{
		Shader::new(include_str!("../shaders/def.vert"), include_str!("../shaders/def.frag"))
	}

	pub fn enable(&self)
	{
		// Enable the shader program
		unsafe
		{
			gl::UseProgram(self.id);
		}
	}

	pub fn set_uniform_matrix(&self, name: &str, mat: &Matrix4<f32>)
	{
		// Find the uniform and set it
		unsafe
		{
			let uni = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_bytes_with_nul().as_ptr() as *const i8);

			if uni<0
				{ panic!("set_uniform_matrix(): Uniform '{}' not found", name); }

			gl::UniformMatrix4fv(uni, 1, 0, mat.as_ptr());
		}
	}
}

impl Drop for Shader
{
	fn drop(&mut self)
	{
		if self.id!=0
		{
			unsafe
			{
				gl::DeleteProgram(self.id);
			}
		}
	}
}



