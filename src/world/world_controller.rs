use crate::mods::BlocksManager;
use crate::{core::Camera, graphics::VoxelRenderer, world::World};
use crate::graphics::Mesh;
use crate::settings::{CHUNK_D, CHUNK_H, CHUNK_W};
use super::world::ChunkCoord;
use crate::world::{ChunkRequest, ChunkResult};
use std::result;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use super::chunk_loader_thread;
use std::path::Path;
// use super::world_files::{WorldInfo, save_world_info};
use super::chunks_loader::SaveRequest;
// use super::chunks_loader::chunk_saver_thread;



pub struct WorldController {
    request_tx: Sender<ChunkRequest>,
    result_rx: Receiver<ChunkResult>,
    save_tx: Sender<SaveRequest>,
    last_player_chunk: (i32, i32)
}


impl WorldController {
    pub fn init() -> Self {
        let (request_tx, request_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();

        thread::spawn(move || {
            chunk_loader_thread(request_rx, result_tx);
        });

        let (save_tx, save_rx) = mpsc::channel();

        // thread::spawn(move || {
        //     chunk_saver_thread(save_rx);
        // });

        Self {
            request_tx,
            result_rx,
            save_tx,
            last_player_chunk: (0, 0) 
        }
    }


    pub fn create_world(&self, renderer: &mut VoxelRenderer, blocks_manager: &BlocksManager) -> World {
        let mut world = World::create();
        world.generate_start_landscape(blocks_manager);

        let len = world.chunks.len();
        println!("Чанков {len}");

        for (&(cx, cy, cz), chunk) in &world.chunks {
            let mesh = renderer.render(chunk, cx, cy, cz, &world, blocks_manager);
            world.chunks_meshes.insert((cx, cy, cz), Mesh {
                vao: mesh.vao,
                vbo: mesh.vbo,
                vertex_count: mesh.vertex_count
            });
        }

        World { chunks: world.chunks, chunks_meshes: world.chunks_meshes, min_cx: 0, max_cx: 0, min_cz: 0, max_cz: 0, }
    }


    pub fn generate_world(&mut self, camera: &Camera, world: &mut World, arc_blocks_manager: &Arc<BlocksManager>, world_path: &Path) {
        let player_cx = (camera.position.x / CHUNK_W as f32).floor() as i32;
        let player_cz = (camera.position.z / CHUNK_D as f32).floor() as i32;

        if player_cx != self.last_player_chunk.0 || player_cz != self.last_player_chunk.1 {
            self.last_player_chunk = (player_cx, player_cz);
            world.update_world(player_cx, player_cz, &self.request_tx, &self.save_tx, arc_blocks_manager, world_path);
        }

        while let Ok(result) = self.result_rx.try_recv() {
            let coord = result.coord;
            world.chunks.insert(coord, result.chunk);
            let mesh = Mesh::new(&result.vertices);
            if let Some(old) = world.chunks_meshes.insert(coord, mesh) {
                unsafe {
                    gl::DeleteVertexArrays(1, &old.vao);
                    gl::DeleteBuffers(1, &old.vbo);
                }
            }
        }


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
    }


    // pub fn save_world(&self, world_info: &mut WorldInfo, world: &World, world_path: &Path, camera: &Camera) {
    //     // Сохраняем позицию игрока
    //     world_info.player_position = [camera.position.x, camera.position.y, camera.position.z];
    //     save_world_info(world_path, world_info).expect("НЕВОЗМОЖНО СОХРАНИТЬСЯ!!!");

    //     // Сохраняем все чанки
    //     world.save_all_chunks(world_path);
    // }
}