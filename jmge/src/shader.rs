
use std::ffi::{CString};
use super::Error;
use nalgebra::base::{Matrix4};


fn space_cstring(len: usize) -> CString
{
	// Create a C string filled with spaces
	let mut buffer: Vec<u8> = Vec::with_capacity(len+1);
	buffer.extend([b' '].iter().cycle().take(len));
	unsafe { CString::from_vec_unchecked(buffer) }
}

struct Shader (u32);

impl Shader
{
	fn from_bytes(source: &[u8], stype: u32) -> Result<Shader, Error>
	{
		let id = unsafe { gl::CreateShader(stype) };
		let shader = Shader (id);

		unsafe
		{
			// Compile the shader
			let len = source.len() as i32;
			gl::ShaderSource(id, 1, &(source.as_ptr() as *const i8), &len);
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

	pub fn id(&self) -> u32
	{
		self.0
	}
}

impl Drop for Shader
{
	fn drop(&mut self)
	{
		// Delete the shader
		unsafe { gl::DeleteShader(self.0); }
	}
}

pub struct VertexShader (Shader);

impl VertexShader
{
	pub fn from_bytes(source: &[u8]) -> Result<VertexShader, Error>
	{
		let shader = Shader::from_bytes(source, gl::VERTEX_SHADER)?;
		Ok(VertexShader (shader))
	}

	pub fn id(&self) -> u32
	{
		self.0.id()
	}
}


pub struct FragmentShader (Shader);

impl FragmentShader
{
	pub fn from_bytes(source: &[u8]) -> Result<FragmentShader, Error>
	{
		let shader = Shader::from_bytes(source, gl::FRAGMENT_SHADER)?;
		Ok(FragmentShader (shader))
	}

	pub fn id(&self) -> u32
	{
		self.0.id()
	}
}


pub struct ShaderProgram (u32);

impl ShaderProgram
{
	pub fn new(vert: VertexShader, frag: FragmentShader) -> Result<ShaderProgram, Error>
	{
		// Create the program
		let id = unsafe { gl::CreateProgram() };
		let prg = ShaderProgram (id);

		unsafe
		{
			// Link the program
			gl::AttachShader(id, vert.id());
			gl::AttachShader(id, frag.id());
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

		Ok(prg)
	}

	pub fn from_str(vert: &str, frag: &str) -> Result<ShaderProgram, Error>
	{
		// Create and compile both shader parts
		let vert = VertexShader::from_bytes(vert.as_bytes())?;
		let frag = FragmentShader::from_bytes(frag.as_bytes())?;

		ShaderProgram::new(vert, frag)
	}

	pub fn new_default() -> Result<ShaderProgram, Error>
	{
		ShaderProgram::from_str(include_str!("../shaders/def.vert"), include_str!("../shaders/def.frag"))
	}
	
	pub fn id(&self) -> u32
	{
		self.0
	}

	pub fn enable(&self)
	{
		// Enable the shader program
		unsafe
		{
			gl::UseProgram(self.0);
		}
	}

	pub fn set_uniform_matrix(&self, name: &str, mat: &Matrix4<f32>)
	{
		// Find the uniform and set it
		unsafe
		{
			let uni = gl::GetUniformLocation(self.0, CString::new(name).unwrap().as_bytes_with_nul().as_ptr() as *const i8);

			if uni<0
				{ panic!("set_uniform_matrix(): Uniform '{}' not found", name); }

			gl::UniformMatrix4fv(uni, 1, 0, mat.as_ptr());
		}
	}
}

impl Drop for ShaderProgram
{
	fn drop(&mut self)
	{
		if self.0!=0
		{
			unsafe
			{
				gl::DeleteProgram(self.0);
			}
		}
	}
}



