use crate::{graphics::{Shader, Texture, create_ui_quad, load_shader, load_texture_from_png}, loger::ProjectErrors};
use gl::types::GLuint;


pub struct Background {
    shader: Shader,
    texture: Texture,
    vao: GLuint
}


impl Background {
    pub fn init(vertex_shader: &str, fragment_shader: &str, texture: &str) -> Result<Self, ProjectErrors> {
        let shader = load_shader(vertex_shader, fragment_shader)?;
        let texture = load_texture_from_png(texture)?;

        let vao = create_ui_quad();

        Ok(Self { 
            shader, 
            texture, 
            vao 
        })
    }


    pub fn draw(&self, elapsed: f32) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.shader.use_shader();
        self.texture.bind(0);
        self.shader.uniform_texture("uTexture", 0);
        self.shader.uniform_float("uTime", elapsed);
        self.shader.uniform_vec2("uScrollSpeed", 0.02, 0.01);

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}