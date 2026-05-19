mod block;
mod chunk;

pub use block::BlockType;
pub use chunk::Chunk;

use crate::settings::CHUNK_D;
use crate::settings::CHUNK_H;
use crate::settings::CHUNK_W;

pub const CHUNK_VOLUME: usize = CHUNK_D * CHUNK_W * CHUNK_H;
