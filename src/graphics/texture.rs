use std::path::Path;

use gl::types::GLuint;
use crate::{loger::ProjectErrors};
use image::{GenericImageView};

pub struct Texture {
    id: GLuint
}


impl Texture {
    pub fn new(id: GLuint) -> Self {
        Self { id }
    }


    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}


impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}


pub fn load_texture_from_png(path: &str) -> Result<Texture, ProjectErrors> {
    let img = image::open(&Path::new(path)).map_err(|e| ProjectErrors::TextureLoadError {
        path: path.to_string(),
        source: e 
    })?;

    let (width, height) = img.dimensions();
    let img = img.flipv();
    let data = img.to_rgba8().into_raw();

    let mut id: GLuint = 0;

    unsafe {
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );
    };

    let texture = Texture::new(id);
    Ok(texture)
}