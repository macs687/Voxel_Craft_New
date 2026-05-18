use gl::types::{GLsizei, GLsizeiptr, GLuint};
use super::VERTEX_SIZE;

pub struct Mesh {
    pub vao: GLuint,
    pub vertex_count: usize
}


impl Mesh {
    pub fn new(buffer: &[f32]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                (buffer.len() * std::mem::size_of::<f32>()) as isize, 
                buffer.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            let stride = (VERTEX_SIZE * std::mem::size_of::<f32>()) as i32;

            gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as GLsizei,
            std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            let uv_offset = (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void;

            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, uv_offset);
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let vertex_count = buffer.len() / VERTEX_SIZE;

        Self {
            vao,
            vertex_count: vertex_count
        }
    }
}