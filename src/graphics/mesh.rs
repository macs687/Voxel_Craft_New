use gl::types::{GLsizei, GLsizeiptr, GLuint};

pub fn create_mesh_cube() -> (GLuint, usize) {
    let vertices: [f32; 24] = [
        // позиции (x, y, z)
        0.0, 0.0, 0.0, // 0
        1.0, 0.0, 0.0, // 1
        1.0, 1.0, 0.0, // 2
        0.0, 1.0, 0.0, // 3
        0.0, 0.0, 1.0, // 4
        1.0, 0.0, 1.0, // 5
        1.0, 1.0, 1.0, // 6
        0.0, 1.0, 1.0, // 7
    ];

    let indices: [u32; 36] = [
        0, 1, 2, 0, 2, 3, // передняя грань (z=0)
        1, 5, 6, 1, 6, 2, // правая
        5, 4, 7, 5, 7, 6, // задняя
        4, 0, 3, 4, 3, 7, // левая
        3, 2, 6, 3, 6, 7, // верхняя
        4, 5, 1, 4, 1, 0, // нижняя
    ];

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
        vertices.as_ptr() as *const _,
        gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as GLsizei,
            std::ptr::null()
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    (vao, indices.len())
}