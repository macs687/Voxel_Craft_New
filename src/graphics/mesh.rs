use gl::types::{GLsizei, GLsizeiptr, GLuint};

pub fn create_mesh_cube() -> (GLuint, usize) {
    let vertices: [f32; 120] = [
        // Передняя грань (z = -0.5)  — направлена на -Z
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,

        // Задняя грань (z = 0.5)    — направлена на +Z
         0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  0.0, 1.0,

        // Левая грань (x = -0.5)
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,

        // Правая грань (x = 0.5)
         0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  0.0, 1.0,

        // Верхняя грань (y = 0.5)
        -0.5,  0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,

        // Нижняя грань (y = -0.5)
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 1.0,
    ];

    let indices: [u32; 36] = [
        0,  1,  2,  0,  2,  3, // перед
        4,  5,  6,  4,  6,  7, // зад
        8,  9, 10,  8, 10, 11, // лево
        12, 13, 14, 12, 14, 15, // право
        16, 17, 18, 16, 18, 19, // верх
        20, 21, 22, 20, 22, 23, // низ
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

        let stride = 5 * std::mem::size_of::<f32>() as i32; // 20 байт между вершинами

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

    (vao, indices.len())
}