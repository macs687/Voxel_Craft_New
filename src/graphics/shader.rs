use gl::types::*;
use std::{fs::File, io::Read};
use std::ffi::CString;
use crate::loger::ProjectErrors;
use std::ffi::CStr;


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
    let mut vertex_code = read_shader_file(vertex_file)?;
    let mut fragment_code = read_shader_file(fragment_file)?;

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