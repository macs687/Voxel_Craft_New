use gl::types::*;
use std::{fs::File, io::Read};
use std::ffi::CString;
use crate::loger::ProjectErrors;
use std::ffi::CStr;
use glam::Mat4;


pub struct Shader{
    id: GLuint,
}


impl Shader {
    pub fn new(id: GLuint) -> Self {
        Self { id }
    }


    pub fn use_shader(&self) {
        unsafe { gl::UseProgram(self.id); }
    }


    pub fn uniform_matrix(&self, name: &str, matrix: Mat4) {
        unsafe {
            let c_name = CString::new(name).expect("CString::new failed");
            let transform_loc = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, matrix.as_ref().as_ptr());
        }
    }


    pub fn uniform_texture(&self, name: &str, slot: i32) {
        unsafe {
            let c_name = code_to_cstring(name.to_string(), "uniform_texture").expect("ошибка конвертации в С строку");
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            if location != -1 {
                gl::Uniform1i(location, slot);
            }
        }
    }


    pub fn uniform_color(&self, name: &str, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            if location != -1 {
                gl::Uniform4f(location, r, g, b, a);
            }
        }
    }


    pub fn uniform_vec4(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            if location != -1 {
                gl::Uniform4f(location, x, y, z, w);
            }
        }
    }


    pub fn uniform_vec2(&self, name: &str, x: f32, y: f32) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            if location != -1 {
                gl::Uniform2f(location, x, y);
            }
        }
    }


    pub fn uniform_float(&self, name: &str, value: f32) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let loc = gl::GetUniformLocation(self.id, c_name.as_ptr());
            if loc != -1 { gl::Uniform1f(loc, value); }
        }
    }
}


impl Drop for Shader {
    fn drop(&mut self){
        unsafe { gl::DeleteProgram(self.id) }
    }
}


fn read_shader_file(path: &str) -> Result<String, ProjectErrors> {
    let mut file = File::open(path).map_err(|e| ProjectErrors::FileOpen {
        path: path.to_string(),
        source: e,
    })?;
    let mut code = String::new();
    file.read_to_string(&mut code).map_err(|e| ProjectErrors::FileRead {
        path: path.to_string(),
        source: e,
    })?;
    Ok(code)
}


fn code_to_cstring(code: String, context: &str) -> Result<CString, ProjectErrors> {
    CString::new(code).map_err(|e| ProjectErrors::CStringConvert {
        context: context.to_string(),
        source: e,
    })
}


fn compile_shader(source_code: &CStr, shader_type: GLenum, stage: &'static str,) -> Result<GLuint, ProjectErrors> {
    unsafe {
        let shader = gl::CreateShader(shader_type);

        gl::ShaderSource(shader, 1, &source_code.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success: GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        
        if success == 0 {
            let mut log_buf: [GLchar; 512] = [0; 512];
            gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), log_buf.as_mut_ptr());
            
            let log = CStr::from_ptr(log_buf.as_ptr()).to_string_lossy().into_owned();
            gl::DeleteShader(shader);

            return Err(ProjectErrors::ShaderCompilation { stage, log });
        }

        Ok(shader)
    }
}


pub fn load_shader(vertex_file: &str, fragment_file: &str) -> Result<Shader, ProjectErrors> {
    let vertex_code = read_shader_file(vertex_file)?;
    let fragment_code = read_shader_file(fragment_file)?;

    let v_shader_code = code_to_cstring(vertex_code, "vertex_code")?;
    let f_shader_code = code_to_cstring(fragment_code, "fragment_code")?;

    let vertex = compile_shader(&v_shader_code, gl::VERTEX_SHADER, "Vertex")?;
    let fragment = compile_shader(&f_shader_code, gl::FRAGMENT_SHADER, "Fragment")?;

    let id = unsafe {
        let program: GLuint = gl::CreateProgram();

        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);
        gl::LinkProgram(program);

        let mut success: GLint = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut log_buf: [GLchar; 512] = [0; 512];
            gl::GetProgramInfoLog(program, 512, std::ptr::null_mut(), log_buf.as_mut_ptr());

            // Безопасное извлечение лога через CStr
            let log = CStr::from_ptr(log_buf.as_ptr()).to_string_lossy().into_owned();

            // Удаляем всё, что создали: шейдеры и неудачную программу
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            gl::DeleteProgram(program);

            return Err(ProjectErrors::ShaderLinking { log });
        }

        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);

        program
    };

    Ok(Shader::new(id))
}