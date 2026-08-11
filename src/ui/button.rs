use crate::graphics::{Shader, Texture};
use crate::assets::fonts;
use crate::graphics::load_texture_from_image_data;
use crate::loger::ProjectErrors;
use gl::types::GLuint;
use image::imageops;


pub struct Button {
    pub text: String,
    pub texture: Texture,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}


impl Button {
    pub fn new(text: &str, x: f32, y: f32, width: f32, height: f32) -> Result<Self, ProjectErrors> {
        let img = fonts::rasterize_text(text);
        let invert_img = imageops::flip_vertical(&img);
        //println!("Rasterized '{}' size: {}x{}", text, img.width(), img.height());
        let texture = load_texture_from_image_data(&invert_img)?;
        //println!("Texture id for '{}': {}", text, texture.id);

        Ok(Self { 
            text: text.to_string(), 
            texture: texture, 
            x,
            y,
            width, 
            height 
        })
    }


    pub fn contains(&self, ndc_x: f32, ndc_y: f32) -> bool {
        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        ndc_x >= self.x - half_w && ndc_x <= self.x + half_w && ndc_y >= self.y - half_h && ndc_y <= self.y + half_h
    }


    pub fn draw_button(&self, shader: &Shader, quad_vao: GLuint) {
        shader.use_shader();
        self.texture.bind(0);
        shader.uniform_texture("uTexture", 0);

        // Исходный квад занимает [-1, 1] по обеим осям.
        // Чтобы получить прямоугольник шириной W и высотой H в NDC,
        // нужно умножить координаты на W/2 и H/2, а затем сдвинуть на позицию кнопки.
        let scale_x = self.width / 2.0;
        let scale_y = self.height / 2.0;
        shader.uniform_vec2("uScale", scale_x, scale_y);
        shader.uniform_vec2("uOffset", self.x, self.y);

        unsafe {
            gl::BindVertexArray(quad_vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}