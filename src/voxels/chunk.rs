use crate::{mods::BlocksManager, voxels::CHUNK_VOLUME};
use super::block::BlockType;
use crate::settings::{CHUNK_D, CHUNK_H, CHUNK_W};
use noise::{NoiseFn, Perlin};


pub struct Chunk {
    pub blocks: Box<[[[BlockType; CHUNK_W]; CHUNK_D]; CHUNK_H]>,
}


impl Chunk {
    pub fn new() -> Self {
        let mut v: Vec<BlockType> = Vec::with_capacity(CHUNK_VOLUME);
        v.resize(CHUNK_VOLUME, BlockType::Air);
        let boxed_slice: Box<[BlockType]> = v.into_boxed_slice();
        let ptr = Box::into_raw(boxed_slice) as *mut [[[BlockType; CHUNK_W]; CHUNK_D]; CHUNK_H];
        let boxed = unsafe { Box::from_raw(ptr) };

        println!("Chunk dimensions: W={}, H={}, D={}", CHUNK_W, CHUNK_H, CHUNK_D);

        Chunk { blocks: boxed }
    }


    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<BlockType> {
        if x < CHUNK_W && y < CHUNK_H && z < CHUNK_D {
            Some(self.blocks[y][z][x])
        } else {
            None
        }
    }


    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        if x < CHUNK_W && y < CHUNK_H && z < CHUNK_D {
            self.blocks[y][z][x] = block;
        }
    }


    pub fn generate_terrain(&mut self, chunk_x: i32, chunk_y: i32, chunk_z: i32, seed: u32, blocks_manager: &BlocksManager) {


        let perlin = Perlin::new(seed);

        let scale = 30.0;          // масштаб шума (чем меньше, тем круче холмы)
        let height_amp = 15.0;     // амплитуда высоты // to do ВЫНЕСТИИ В НАСТРОЙКИ
        let base_height = (CHUNK_H as f32 * 0.35) as i32; // ~22 при высоте 64

        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let wx = (chunk_x * CHUNK_W as i32 + x as i32) as f64;
                let wz = (chunk_z * CHUNK_D as i32 + z as i32) as f64;
                let point = [wx / scale as f64, wz / scale as f64];

                let noise_val = perlin.get(point);
                let surface_y = base_height + (noise_val * height_amp) as i32;

                for y in 0..CHUNK_H {
                    let block = if (y as i32) < surface_y {
                        let id = blocks_manager.get_id(&"dirt".to_string());
                        BlockType::Custom(id.unwrap_or(0))
                    } else if (y as i32) == surface_y {
                        let id = blocks_manager.get_id(&"grass".to_string());
                        BlockType::Custom(id.unwrap_or(0))
                    } else {
                        BlockType::Air
                    };

                    self.blocks[y][z][x] = block
                }
            }
        }
    }
}