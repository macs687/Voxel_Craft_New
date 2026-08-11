use std::collections::HashMap;
use crate::core::{Camera, Window};
use crate::graphics::{Mesh, Shader, Texture};
use crate::settings::{CHUNK_D, CHUNK_H, CHUNK_W};
use crate::world::{ChunkCoord, RayHit};
use glam::{Mat4, Vec3};


pub fn draw_world(window: &mut Window, shader: &Shader, camera: &Camera, texture: &Texture, chunk_meshes: &HashMap<ChunkCoord, Mesh>, crosshair_shader: &Shader, crosshair_mesh: &Mesh, line_shader: &Shader, cube_mesh: &Mesh, hit: &Option<RayHit>) {
    window.gl_clear();
    
    let view = camera.get_view();
    let projection = camera.get_projection(window.width as f32, window.height as f32);
    shader.use_shader();

    shader.uniform_matrix("uView", view);
    shader.uniform_matrix("uProjection", projection);
        
    texture.bind(0);
    shader.uniform_texture("uTexture", 0);

    for (&(cx, cy, cz), mesh) in chunk_meshes {
        let model = Mat4::from_translation(Vec3::new(
                (cx * CHUNK_W as i32) as f32,
                (cy * CHUNK_H as i32) as f32,
                (cz * CHUNK_D as i32) as f32,
        ));

        shader.uniform_matrix("uModel", model);

        unsafe {
            gl::BindVertexArray(mesh.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, mesh.vertex_count as i32);
            gl::BindVertexArray(0);
        }
    }

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