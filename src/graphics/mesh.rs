use gl::types::{GLsizei, GLuint};
use super::VERTEX_SIZE;

pub struct Mesh {
    pub vao: GLuint,
    pub vbo: GLuint,
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
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.len() * std::mem::size_of::<f32>()) as _,
                buffer.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            // location = 0

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

            // uv location = 1

            let uv_offset = (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void;

            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, uv_offset);
            gl::EnableVertexAttribArray(1);

            // texture location 2

            let texture_offset = (5 * std::mem::size_of::<f32>()) as *const std::ffi::c_void;

            gl::VertexAttribPointer(2, 1, gl::FLOAT, gl::FALSE, stride, texture_offset);
            gl::EnableVertexAttribArray(2);


            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let vertex_count = buffer.len() / VERTEX_SIZE;

        Self {
            vao,
            vbo,
            vertex_count: vertex_count
        }
    }
}


pub fn create_crosshair_mesh() -> Mesh {
    // Крестик: два прямоугольника, пересекающихся в центре
    // Координаты в NDC: x, y от -1 до 1
    // Вертикальная линия: x от -0.005 до 0.005, y от -0.03 до 0.03
    // Горизонтальная линия: x от -0.03 до 0.03, y от -0.005 до 0.005
    let vertices: [f32; 24] = [
        // Вертикальная линия (два треугольника)
        -0.005, -0.03,  0.005, -0.03,  0.005, 0.03,
        -0.005, -0.03,  0.005, 0.03,  -0.005, 0.03,
        // Горизонтальная линия (два треугольника)
        -0.03, -0.005,  0.03, -0.005,  0.03, 0.005,
        -0.03, -0.005,  0.03, 0.005,  -0.03, 0.005,
    ];

    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    
    Mesh { vao, vbo, vertex_count: vertices.len() / 2 } // количество вершин = 12, по 2 координаты
}


pub fn create_wireframe_mesh() -> Mesh {
    // 12 ребер, каждое ребро - 2 вершины
    let vertices: [f32; 72] = [
        // передняя грань (z = 0.5)
        -0.5, -0.5, 0.5,   0.5, -0.5, 0.5,
        -0.5, 0.5, 0.5,   0.5, 0.5, 0.5,
        -0.5, -0.5, 0.5,  -0.5, 0.5, 0.5,
         0.5, -0.5, 0.5,   0.5, 0.5, 0.5,
        // задняя грань (z = -0.5)
        -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,
        -0.5, 0.5, -0.5,  0.5, 0.5, -0.5,
        -0.5, -0.5, -0.5, -0.5, 0.5, -0.5,
         0.5, -0.5, -0.5,  0.5, 0.5, -0.5,
        // соединяющие рёбра
        -0.5, -0.5, -0.5, -0.5, -0.5, 0.5,
         0.5, -0.5, -0.5,  0.5, -0.5, 0.5,
        -0.5, 0.5, -0.5, -0.5, 0.5, 0.5,
         0.5, 0.5, -0.5,  0.5, 0.5, 0.5,
    ];

    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    
    Mesh { vao, vbo, vertex_count: vertices.len() / 3 } // количество вершин
}


// impl Drop for Mesh {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteVertexArrays(1, &self.vao);
//             gl::DeleteBuffers(1, &self.vbo);
//         }
//     }
// }



pub fn create_ui_quad() -> GLuint {
    let vertices: [f32; 16] = [
        // x, y, u, v
        -1.0, -1.0, 0.0, 0.0,
         1.0, -1.0, 1.0, 0.0,
         1.0,  1.0, 1.0, 1.0,
        -1.0,  1.0, 0.0, 1.0,
    ];
    let indices: [u32; 6] = [0,1,2, 0,2,3];

    let (mut vao, mut vbo, mut ebo) = (0,0,0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as _,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let stride = 4 * std::mem::size_of::<f32>() as i32;
        // Позиция (location = 0) – 2 float
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        // UV (location = 1) – 2 float, смещение 2 float
        let uv_offset = (2 * std::mem::size_of::<f32>()) as *const std::ffi::c_void;
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, uv_offset);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    vao
}