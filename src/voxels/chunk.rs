use crate::settings::*;
use super::block::BlockType;
use crate::settings::{CHUNK_D, CHUNK_H, CHUNK_W, CHUNK_SIZE};


pub struct Chunk {
    pub blocks: [[[BlockType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}


impl Chunk {
    pub fn new() -> Self {
        let blocks = [[[BlockType::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        Chunk { blocks }
    }


    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<BlockType> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            Some(self.blocks[y][z][x])
        } else {
            None
        }
    }


    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            self.blocks[y][z][x] = block;
        }
    }


    pub fn generate_test_terrain(&mut self) {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let block = if y <= 3 {
                        BlockType::Planks
                    } else if y <= 6 {
                        BlockType::Planks
                    } else {
                        BlockType::Air
                    };
                    self.blocks[y][z][x] = block;
                }
            }
        }
    }


    pub fn build_mesh(&self) -> (u32, usize) {
        // Заглушка: вернуть пустой VAO (0) и 0 индексов
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }
        (vao, 0)
    }

    
}


