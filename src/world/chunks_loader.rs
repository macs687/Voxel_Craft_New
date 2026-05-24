use std::path::PathBuf;
use std::sync::{Arc, mpsc::Sender, mpsc::Receiver};
use crate::graphics::VoxelRenderer;
use crate::mods::BlocksManager;
use crate::world::ChunkCoord;
use std::collections::HashMap;
use crate::voxels::{BlockType, Chunk};
use crate::settings::{CHUNK_D, CHUNK_H, CHUNK_W};
use crate::world::world_files;
use crate::world::region::REGION_SIZE;
use crate::world::world_files::save_chunk;


pub struct ChunkRequest {
    pub coord: ChunkCoord,
    pub neighbors: HashMap<ChunkCoord, Box<[[[BlockType; CHUNK_W]; CHUNK_D]; CHUNK_H]>>,
    pub seed: u32,
    pub blocks_manager: Arc<BlocksManager>,
    pub world_path: Option<PathBuf>
}


pub struct ChunkResult {
    pub coord: ChunkCoord,
    pub chunk: Chunk,
    pub vertices: Vec<f32>,
}


pub struct SaveRequest {
    pub world_path: PathBuf,
    pub coord: ChunkCoord,
    pub chunk: Chunk,   // владеющий чанк
}


pub fn chunk_loader_thread(request_rx: Receiver<ChunkRequest>, result_tx: Sender<ChunkResult>) {
    let mut renderer = VoxelRenderer::init();

    
    loop {
        match request_rx.recv() {
            Ok(req) => {
                let chunk = if let Some(ref path) = req.world_path {
                    world_files::load_chunk(path, req.coord.0, req.coord.1, req.coord.2)
                        .unwrap_or_else(|| {
                            let mut chunk_new = Chunk::new();
                            chunk_new.generate_terrain(req.coord.0, req.coord.1, req.coord.2, req.seed, &req.blocks_manager);
                            chunk_new
                        })
                } else {
                    let mut chunk_new = Chunk::new();
                    chunk_new.generate_terrain(req.coord.0, req.coord.1, req.coord.2, req.seed, &req.blocks_manager);
                    chunk_new
                };

                renderer.buffer.clear();

                let get_block = &|wx: i32, wy: i32, wz: i32| -> Option<BlockType> {
                    let cx = wx.div_euclid(CHUNK_W as i32);
                    let cy = wy.div_euclid(CHUNK_H as i32);
                    let cz = wz.div_euclid(CHUNK_D as i32);
                    if (cx, cy, cz) == req.coord {
                        // Блок из нового чанка
                        let lx = wx.rem_euclid(CHUNK_W as i32) as usize;
                        let ly = wy.rem_euclid(CHUNK_H as i32) as usize;
                        let lz = wz.rem_euclid(CHUNK_D as i32) as usize;
                        chunk.get_block(lx, ly, lz)
                    } else if let Some(neighbor_blocks) = req.neighbors.get(&(cx, cy, cz)) {
                        let lx = wx.rem_euclid(CHUNK_W as i32) as usize;
                        let ly = wy.rem_euclid(CHUNK_H as i32) as usize;
                        let lz = wz.rem_euclid(CHUNK_D as i32) as usize;
                        Some(neighbor_blocks[ly][lz][lx])
                    } else {
                        None
                    }
                };

                let get_uv = &|name: &str| {
                    req.blocks_manager.get_uv_by_name(name).unwrap_or([0.0; 4])
                };


                renderer.render_to_buffer(&chunk, req.coord.0, req.coord.1, req.coord.2, &req.blocks_manager, get_block, get_uv);

                let result = ChunkResult {
                    coord: req.coord,
                    chunk,
                    vertices: std::mem::take(&mut renderer.buffer)
                };

                if result_tx.send(result).is_err() {
                    break;
                };
            },
            Err(_) => break,
        }
    }
}


/// поток для сохранения чанков, который слушает канал `save_rx` и сохраняет чанки в файлы при получении запросов
pub fn chunk_saver_thread(request_rx: Receiver<SaveRequest>) {
    loop {
        match request_rx.recv() {
            Ok(req) => {
                if let Err(e) = save_chunk(&req.world_path, req.coord.0, req.coord.1, req.coord.2, &req.chunk) {
                    eprintln!("Failed to save chunk {:?}: {}", req.coord, e);
                }
            },
            Err(_) => break,
        }
    }
}