use gl::types::GLuint;

use crate::graphics::{Shader, Texture};

pub fn draw_background(shader: &Shader, texture: &Texture, vao: GLuint, elapsed: f32) {
    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    shader.use_shader();
    texture.bind(0);
    shader.uniform_texture("uTexture", 0);
    shader.uniform_float("uTime", elapsed);
    shader.uniform_vec2("uScrollSpeed", 0.02, 0.01);

    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::BindVertexArray(vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        gl::BindVertexArray(0);
    }
}