use std::{collections::{HashMap, HashSet}, sync::Arc};
use glam::{I16Vec3, Vec3, IVec3};
use crate::{graphics::VoxelRenderer, mods::BlocksManager, settings::{CHUNK_D, CHUNK_H, CHUNK_W, MAX_STEPS, RENDER_DIST, SEED}, voxels::{BlockType, Chunk}, world::chunks_loader::ChunkRequest};
use std::sync::mpsc::Sender;

use crate::graphics::Mesh;

pub type ChunkCoord = (i32, i32, i32);


pub struct World {
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub chunks_meshes: HashMap<ChunkCoord, Mesh>,
    pub min_cx: i32,
    pub max_cx: i32,
    pub min_cz: i32,
    pub max_cz: i32,
}


#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    pub block_pos: (i32, i32, i32), // координаты блока в мировом пространстве (целые)
    pub normal: (i32, i32, i32), // нормаль грани, в которую попал луч
}


impl World {
    pub fn create() -> Self {
        let chunks = HashMap::new();
        let chunks_meshes = HashMap::new();
        Self {
            chunks, 
            chunks_meshes,
            min_cx: 0,
            max_cx: 0,
            min_cz: 0,
            max_cz: 0 
        }
    }


    pub fn load_chunk(&mut self, cx: i32, cy: i32, cz: i32, blocks_manager: &BlocksManager) {
        println!("Попытка загрузить чанк ({}, {}, {})", cx, cy, cz);
        if !self.chunks.contains_key(&(cx, cy, cz)) {
            let mut chunk = Chunk::new();
            chunk.generate_terrain(cx, 0, cz, SEED, blocks_manager);
            self.chunks.insert((cx, cy, cz), chunk);
            println!("Чанк ({}, {}, {}) добавлен. Всего чанков: {}", cx, cy, cz, self.chunks.len());
        } else {
            println!("Чанк ({}, {}, {}) уже существует", cx, cy, cz);
        }
    }


    pub fn generate_start_landscape(&mut self, blocks_manager: &BlocksManager) {
        for cx in -1..1 {
            for cz in -1..1 {
                self.load_chunk(cx, 0, cz, blocks_manager);
            }
        }
    }


    pub fn get_block(&self, x: i32, y:i32, z:i32) -> Option<BlockType> {
        let cx = x.div_euclid(CHUNK_W as i32);
        let cy = y.div_euclid(CHUNK_H as i32);
        let cz = z.div_euclid(CHUNK_D as i32);

        let block = self.chunks.get(&(cx, cy, cz)).and_then(|chunk| {
            let lx = x.rem_euclid(CHUNK_W as i32) as usize;
            let ly = y.rem_euclid(CHUNK_H as i32) as usize;
            let lz = z.rem_euclid(CHUNK_D as i32) as usize;
            chunk.get_block(lx, ly, lz)
        });

        block
    }


    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockType) {
        let cx = x.div_euclid(CHUNK_W as i32);
        let cy = y.div_euclid(CHUNK_H as i32);
        let cz = z.div_euclid(CHUNK_D as i32);

        if let Some(chunk) = self.chunks.get_mut(&(cx, cy, cz)) {
            let lx = x.rem_euclid(CHUNK_W as i32) as usize;
            let ly = y.rem_euclid(CHUNK_H as i32) as usize;
            let lz = z.rem_euclid(CHUNK_D as i32) as usize;

            chunk.set_block(lx, ly, lz, block);
        }
    }


    /// Устанавливает блок по имени. Если имя не найдено в реестре, ничего не делает.
    pub fn set_block_by_name(&mut self, x: i32, y: i32, z: i32, name: &String, blocks_manager: &BlocksManager) {
        if let Some(id) = blocks_manager.get_id(name) {
            self.set_block(x, y, z, BlockType::Custom(id));
        } else {
            println!("блока {name} не существует")
        }
    }


    fn calculete_meshes(&self, block_pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
        let cx = block_pos.0.div_euclid(CHUNK_W as i32);
        let cy = block_pos.1.div_euclid(CHUNK_H as i32);
        let cz = block_pos.2.div_euclid(CHUNK_D as i32);

        let mut to_rebuild = vec![(cx, cy, cz)];
        for (dx, dy, dz) in [
            (1, 0, 0), (-1, 0, 0),
            (0, 1, 0), (0, -1, 0),
            (0, 0, 1), (0, 0, -1),
        ] {
            let nc = (cx + dx, cy + dy, cz + dz);
            if self.chunks.contains_key(&nc) {
                to_rebuild.push(nc);
            }
        }

        to_rebuild
    }  


    pub fn update(&mut self, block_pos: (i32, i32, i32), renderer: &mut VoxelRenderer, blocks_manager: &BlocksManager) {
        let to_rebuild = self.calculete_meshes(block_pos);
        for coord in to_rebuild {
            if let Some(chunk) = self.chunks.get(&coord) {
                let new_mesh = renderer.render(chunk, coord.0, coord.1, coord.2, &self, blocks_manager);

                let new_chunk_mesh = Mesh {
                    vao: new_mesh.vao,
                    vbo: new_mesh.vbo,
                    vertex_count: new_mesh.vertex_count,
                };

                if let Some(old) = self.chunks_meshes.insert(coord, new_chunk_mesh) {
                    //println!("Chunk {:?} new vertex count: {}", coord, new_mesh.vertex_count);
                    // Явно удаляем старые буферы (или полагаемся на Drop)
                    //drop(old); // Drop освободит VAO и VBO

                    unsafe {
                        gl::DeleteVertexArrays(1, &old.vao);
                        gl::DeleteBuffers(1, &old.vbo);
                    }
                }
            }
        }

        self.min_cx = self.chunks.keys().map(|&(cx, _, _)| cx).min().unwrap_or(0);
        self.max_cx = self.chunks.keys().map(|&(cx, _, _)| cx).max().unwrap_or(0);
        self.min_cz = self.chunks.keys().map(|&(_, _, cz)| cz).min().unwrap_or(0);
        self.max_cz = self.chunks.keys().map(|&(_, _, cz)| cz).max().unwrap_or(0);

        println!("update_chunks: границы X=[{}..{}], Z=[{}..{}]", self.min_cx, self.max_cx, self.min_cz, self.max_cz);
    }


    pub fn update_world(&mut self, player_cx: i32, player_cz: i32, request_tx: &Sender<ChunkRequest>, blocks_manager: &Arc<BlocksManager>,) {
        let mut required = HashSet::new();

        for dx in -RENDER_DIST..RENDER_DIST {
            for dz in -RENDER_DIST..RENDER_DIST {
                required.insert((player_cx + dx, 0, player_cz + dz));
            }
        }

        self.chunks.retain(|&(cx, cy, cz), _| required.contains(&(cx, cy, cz)));

        for coord in &required {
            if !self.chunks.contains_key(coord) {
                let mut neighbors = HashMap::new();

                for (dx, dy, dz) in &[(1,0,0), (-1,0,0), (0,1,0), (0,-1,0), (0,0,1), (0,0,-1)] {
                    let neighbor_coord = (coord.0 + dx, coord.1 + dy, coord.2 + dz);
                    if let Some(chunk) = self.chunks.get(&neighbor_coord) {
                        // Копируем массив блоков
                        let blocks_copy = chunk.blocks.clone();
                        neighbors.insert(neighbor_coord, blocks_copy);
                    }
                }

                let request = ChunkRequest {
                    coord: *coord,
                    neighbors,
                    seed: SEED,
                    blocks_manager: blocks_manager.clone()
                };

                request_tx.send(request).expect("Failed to send chunk");
            }
        }

        if self.chunks.is_empty() {
            self.min_cx = 0; self.max_cx = 0;
            self.min_cz = 0; self.max_cz = 0;
        } else {
            self.min_cx = self.chunks.keys().map(|&(cx, _, _)| cx).min().unwrap();
            self.max_cx = self.chunks.keys().map(|&(cx, _, _)| cx).max().unwrap();
            self.min_cz = self.chunks.keys().map(|&(_, _, cz)| cz).min().unwrap();
            self.max_cz = self.chunks.keys().map(|&(_, _, cz)| cz).max().unwrap();
        }
    }
}


pub fn raycast(world: &World, origin: Vec3, derection: Vec3, max_dist: f32) -> Option<RayHit> {
        let dir = derection.normalize();
        let mut map_pos = IVec3::new(origin.x.floor() as i32, origin.y.floor() as i32, origin.z.floor() as i32);

        let margin = 1.0;
        let min_wx = (world.min_cx * CHUNK_W as i32) as f32 - margin;
        let max_wx = ((world.max_cx + 1) * CHUNK_W as i32) as f32 + margin;
        let min_wz = (world.min_cz * CHUNK_D as i32) as f32 - margin;
        let max_wz = ((world.max_cz + 1) * CHUNK_D as i32) as f32 + margin;

        let delta_dist = Vec3::new(
            if dir.x == 0.0 { f32::MAX } else { (1.0 / dir.x).abs() },
            if dir.y == 0.0 { f32::MAX } else { (1.0 / dir.y).abs() },
            if dir.z == 0.0 { f32::MAX } else { (1.0 / dir.z).abs() },
        );

        let step = IVec3::new(
            if dir.x > 0.0 { 1 } else { -1 },
            if dir.y > 0.0 { 1 } else { -1 },
            if dir.z > 0.0 { 1 } else { -1 },
        );

        let mut side_dist = Vec3::new(
            if dir.x > 0.0 {
                (map_pos.x as f32 + 1.0 - origin.x) * delta_dist.x
            } else {
                (origin.x - map_pos.x as f32) * delta_dist.x
            },
            if dir.y > 0.0 {
                (map_pos.y as f32 + 1.0 - origin.y) * delta_dist.y
            } else {
                (origin.y - map_pos.y as f32) * delta_dist.y
            },
            if dir.z > 0.0 {
                (map_pos.z as f32 + 1.0 - origin.z) * delta_dist.z
            } else {
                (origin.z - map_pos.z as f32) * delta_dist.z
            },
        );

        let mut last_normal = IVec3::ZERO;
        let max_steps = MAX_STEPS;

        for _ in 0..max_steps {
            let wx = map_pos.x as f32;
            let wz = map_pos.z as f32;
            // if wx < min_wx || wx > max_wx || wz < min_wz || wz > max_wz {
            //     break;
            // }

            if let Some(block) = world.get_block(map_pos.x as i32, map_pos.y as i32, map_pos.z as i32) {
                if block != BlockType::Air {
                    return Some(RayHit {
                        block_pos: (map_pos.x as i32, map_pos.y as i32, map_pos.z as i32),
                        normal: (last_normal.x as i32, last_normal.y as i32, last_normal.z as i32),
                    });
                }
            }

            if side_dist.x < side_dist.y {
                if side_dist.x < side_dist.z {
                    map_pos.x += step.x;
                    side_dist.x += delta_dist.x;
                    last_normal = IVec3::new(-step.x, 0, 0);
                } else {
                    map_pos.z += step.z;
                    side_dist.z += delta_dist.z;
                    last_normal = IVec3::new(0, 0, -step.z);
                }
            } else {
                if side_dist.y < side_dist.z {
                    map_pos.y += step.y;
                    side_dist.y += delta_dist.y;
                    last_normal = IVec3::new(0, -step.y, 0);
                } else {
                    map_pos.z += step.z;
                    side_dist.z += delta_dist.z;
                    last_normal = IVec3::new(0, 0, -step.z);
                }
            }

            if side_dist.x > max_dist && side_dist.y > max_dist && side_dist.z > max_dist {
                break;
            }
        }
        None
    }