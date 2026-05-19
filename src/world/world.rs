use std::collections::HashMap;
use glam::{I16Vec3, Vec3};
use crate::{graphics::VoxelRenderer, settings::CHUNK_SIZE, voxels::{BlockType, Chunk}, world::RayHit};

use crate::graphics::Mesh;

pub type ChunkCoord = (i32, i32, i32);


pub struct World {
    pub chunks: HashMap<ChunkCoord, Chunk>
}


impl World {
    pub fn init() -> Self {
        let chunks = HashMap::new();
        Self { chunks }
    }


    pub fn load_chunk(&mut self, cx: i32, cy: i32, cz: i32) {
        println!("Попытка загрузить чанк ({}, {}, {})", cx, cy, cz);
        if !self.chunks.contains_key(&(cx, cy, cz)) {
            let mut chunk = Chunk::new();
            chunk.generate_test_terrain();
            self.chunks.insert((cx, cy, cz), chunk);
            println!("Чанк ({}, {}, {}) добавлен. Всего чанков: {}", cx, cy, cz, self.chunks.len());
        } else {
            println!("Чанк ({}, {}, {}) уже существует", cx, cy, cz);
        }
    }


    pub fn generate_start_landscape(&mut self) {
        for cx in -1..1 {
            for cz in -1..1 {
                self.load_chunk(cx, 0, cz);
            }
        }
    }


    pub fn get_block(&self, x: i32, y:i32, z:i32) -> Option<BlockType> {
        let cx = x.div_euclid(CHUNK_SIZE as i32);
        let cy = y.div_euclid(CHUNK_SIZE as i32);
        let cz = z.div_euclid(CHUNK_SIZE as i32);

        let block = self.chunks.get(&(cx, cy, cz)).and_then(|chunk| {
            let lx = x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let ly = y.rem_euclid(CHUNK_SIZE as i32) as usize;
            let lz = z.rem_euclid(CHUNK_SIZE as i32) as usize;
            chunk.get_block(lx, ly, lz)
        });

        block
    }


    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockType) {
        let cx = x.div_euclid(CHUNK_SIZE as i32);
        let cy = y.div_euclid(CHUNK_SIZE as i32);
        let cz = z.div_euclid(CHUNK_SIZE as i32);

        if let Some(chunk) = self.chunks.get_mut(&(cx, cy, cz)) {
            let lx = x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let ly = y.rem_euclid(CHUNK_SIZE as i32) as usize;
            let lz = z.rem_euclid(CHUNK_SIZE as i32) as usize;

            chunk.set_block(lx, ly, lz, block);
        };
    }    
}


pub fn rebuild_affected_meshes(world: &mut World, meshes: &mut HashMap<ChunkCoord, Mesh>, block_pos: (i32, i32, i32), renderer: &mut VoxelRenderer) {
        let cx = block_pos.0.div_euclid(CHUNK_SIZE as i32);
        let cy = block_pos.1.div_euclid(CHUNK_SIZE as i32);
        let cz = block_pos.2.div_euclid(CHUNK_SIZE as i32);

        let mut to_rebuild = vec![(cx, cy, cz)];
        for (dx, dy, dz) in [
            (1, 0, 0), (-1, 0, 0),
            (0, 1, 0), (0, -1, 0),
            (0, 0, 1), (0, 0, -1),
        ] {
            let nc = (cx + dx, cy + dy, cz + dz);
            if world.chunks.contains_key(&nc) {
                to_rebuild.push(nc);
            }
        }

        for coord in to_rebuild {
            if let Some(chunk) = world.chunks.get(&coord) {
                let new_mesh = renderer.render(chunk, coord.0, coord.1, coord.2, world);

                let new_chunk_mesh = Mesh {
                    vao: new_mesh.vao,
                    vbo: new_mesh.vbo,
                    vertex_count: new_mesh.vertex_count,
                };

                if let Some(old) = meshes.insert(coord, new_chunk_mesh) {
                    // Явно удаляем старые буферы (или полагаемся на Drop)
                    drop(old); // Drop освободит VAO и VBO
                }
            }
        }
    }


pub fn raycast(world: &World, origin: Vec3, derection: Vec3, max_dist: f32) -> Option<RayHit> {
        let dir = derection.normalize();
        let mut map_pos = I16Vec3::new(origin.x.floor() as i16, origin.y.floor() as i16, origin.z.floor() as i16);

        let delta_dist = Vec3::new(
            if dir.x == 0.0 { f32::MAX } else { (1.0 / dir.x).abs() },
            if dir.y == 0.0 { f32::MAX } else { (1.0 / dir.y).abs() },
            if dir.z == 0.0 { f32::MAX } else { (1.0 / dir.z).abs() },
        );

        let step = I16Vec3::new(        // (4)
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

        let mut last_normal = I16Vec3::ZERO;
        let max_steps = (max_dist * 2.0) as i32;

        for _ in 0..max_steps {
            if let Some(block) = world.get_block(map_pos.x as i32, map_pos.y as i32, map_pos.z as i32) {
                if block != BlockType::Air {
                    return Some(RayHit {
                        block_pos: (map_pos.x as i32,
                                    map_pos.y as i32,
                                    map_pos.z as i32),
                        normal: (last_normal.x as i32, 
                                last_normal.y as i32,
                                last_normal.z as i32) 
                    });
                }  
            }

            if side_dist.x < side_dist.y {
                if side_dist.x < side_dist.z {
                    // X – самая близкая граница
                    map_pos.x += step.x;
                    side_dist.x += delta_dist.x;
                    last_normal = I16Vec3::new(-step.x, 0, 0);
                } else {
                    // Z – самая близкая
                    map_pos.z += step.z;
                    side_dist.z += delta_dist.z;
                    last_normal = I16Vec3::new(0, 0, -step.z);
                }
            } else {
                if side_dist.y < side_dist.z {
                    // Y – самая близкая
                    map_pos.y += step.y;
                    side_dist.y += delta_dist.y;
                    last_normal = I16Vec3::new(0, -step.y, 0);
                } else {
                    // Z – самая близкая
                    map_pos.z += step.z;
                    side_dist.z += delta_dist.z;
                    last_normal = I16Vec3::new(0, 0, -step.z);
                }
            }

            if side_dist.x > max_dist && side_dist.y > max_dist && side_dist.z > max_dist {
                break;
            }
        }
        None
    }