use crate::{core::Camera, graphics::VoxelRenderer, world::World};
use crate::graphics::Mesh;
use crate::settings::{CHUNK_D, CHUNK_H, CHUNK_W};
use super::world::ChunkCoord;


pub struct WorldController {
    last_player_chunk: (i32, i32)
}


impl WorldController {
    pub fn init() -> Self {
        Self { last_player_chunk: (0, 0) }
    }


    pub fn create_world(&self, renderer: &mut VoxelRenderer) -> World {
        let mut world = World::create();
        world.generate_start_landscape();

        let len = world.chunks.len();
        println!("Чанков {len}");

        for (&(cx, cy, cz), chunk) in &world.chunks {
            let mesh = renderer.render(chunk, cx, cy, cz, &world);
            world.chunks_meshes.insert((cx, cy, cz), Mesh {
                vao: mesh.vao,
                vbo: mesh.vbo,
                vertex_count: mesh.vertex_count
            });
        }

        World { chunks: world.chunks, chunks_meshes: world.chunks_meshes, min_cx: 0, max_cx: 0, min_cz: 0, max_cz: 0 }
    }


    pub fn generate_world(&mut self, camera: &Camera, world: &mut World, renderer: &mut VoxelRenderer) {
        let player_cx = (camera.position.x / CHUNK_W as f32).floor() as i32;
        let player_cz = (camera.position.z / CHUNK_D as f32).floor() as i32;

        if player_cx != self.last_player_chunk.0 || player_cz != self.last_player_chunk.1 {
            self.last_player_chunk = (player_cx, player_cz);
            world.update_world(player_cx, player_cz);

            // УДАЛЯЕМ СТАРЫЙ МЕШ
            let to_remove: Vec<ChunkCoord> = world.chunks_meshes.keys()
                .filter(|k| !world.chunks.contains_key(k))
                .copied()
                .collect();

            for key in to_remove {
                if let Some(mesh) = world.chunks_meshes.remove(&key) {
                    unsafe {
                        gl::DeleteVertexArrays(1, &mesh.vao);
                        gl::DeleteBuffers(1, &mesh.vbo);
                    }
                }
            }


            for (&(cx, cy, cz), chunk) in &world.chunks {
                if !world.chunks_meshes.contains_key(&(cx, cy, cz)) {
                    let mesh = renderer.render(chunk, cx, cy, cz, &world);
                    world.chunks_meshes.insert((cx, cy, cz), mesh);
                }
            }
        }
    }
}