use crate::core::{Camera, Events, Window};
use crate::graphics::{Mesh, Shader, Texture, VoxelRenderer};
use crate::voxels::Chunk;
use crate::world::RayHit;
use glam::{Mat4, Vec3};
use crate::constant::{self, *};
use crate::voxels::BlockType;

pub fn draw_world(window: &mut Window, shader: &Shader, camera: &Camera, texture: &Texture, mesh: &Mesh, crosshair_shader: &Shader, crosshair_mesh: &Mesh, chunk: &mut Chunk, line_shader: &Shader, cube_mesh: &Mesh, events: &Events, hit: &Option<RayHit>) {
    window.gl_clear();

    shader.use_shader();

    let view = camera.get_view();
    let projection = camera.get_projection(window.width as f32, window.height as f32);
    let model = Mat4::IDENTITY;

    shader.uniform_matrix("uModel", model);
    shader.uniform_matrix("uView", view);
    shader.uniform_matrix("uProjection", projection);
        
    texture.bind(0);
    shader.uniform_texture("uTexture", 0);

    unsafe {
        gl::BindVertexArray(mesh.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, mesh.vertex_count as i32);
        gl::BindVertexArray(0);
    }

    //println!("cam pos: {:?}, front: {:?}", camera.position, camera.front);

    //println!("ray cast normal");
    // Рисуем wireframe куб вокруг блока
    if let Some(hit) = hit {
        let model = Mat4::from_translation(Vec3::new(
            hit.block_pos.0 as f32,
            hit.block_pos.1 as f32,
            hit.block_pos.2 as f32,
        )) * Mat4::from_scale(Vec3::splat(1.005)); // чуть больше, чтобы не застревать

        line_shader.use_shader();
        line_shader.uniform_matrix("uModel", model);
        line_shader.uniform_matrix("uView", view);
        line_shader.uniform_matrix("uProjection", projection);
        line_shader.uniform_vec4("uColor", 0.0, 0.0, 0.0, 1.0); // чёрный контур

        unsafe {
            // Линии рисуются без отсечения граней, и с тестом глубины
            gl::BindVertexArray(cube_mesh.vao);
            gl::DrawArrays(gl::LINES, 0, cube_mesh.vertex_count as i32);
            gl::BindVertexArray(0);
        }
    }
    

    crosshair_shader.use_shader();
    crosshair_shader.uniform_color("uColor", 1.0, 1.0, 1.0, 0.8);

    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    // рисуем прицел
    
    unsafe {
        gl::BindVertexArray(crosshair_mesh.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, crosshair_mesh.vertex_count as i32);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Disable(gl::BLEND);
    }

    window.swap_buffers();
}