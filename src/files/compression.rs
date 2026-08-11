use crate::settings::{CHUNK_W, CHUNK_H, CHUNK_D};
use crate::voxels::{Chunk, BlockType};




/// Преобразует чанк в плоский вектор u16
fn chunk_to_u16(chunk: &Chunk) -> Vec<u16> {
    let total = CHUNK_W * CHUNK_H * CHUNK_D;
    let mut data = Vec::with_capacity(total);
    for y in 0..CHUNK_H {
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                data.push(chunk.blocks[y][z][x].id());
            }
        }
    }
    data
}


/// RLE сжатие (аналогично compressRLE из C++). Принимает сырые данные (слайс u16).
/// Возвращает сжатый вектор байт.
pub fn compress_rle(data: &[u16]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new()
    }

    let mut compressed = Vec::with_capacity(data.len() * 2);
    let mut counter = 1u8;
    let mut current = data[0];

    for &next in data.iter().skip(1) {
        if next == current && counter < 255 {
            counter += 1;
        } else {
            compressed.push(counter - 1);
            compressed.extend_from_slice(&current.to_le_bytes());
            current = next;
            counter = 1;
        }
    }

    compressed.push(counter - 1);
    compressed.extend_from_slice(&current.to_le_bytes());
    compressed
}


/// RLE распаковка (аналогично decompressRLE). Возвращает распакованные u16.
fn decompress_rle(data: &[u8]) -> Vec<u16> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < data.len() - 2 {
        let counter = data[i] as usize + 1;
        let value = u16::from_le_bytes([data[i+1], data[i+2]]);
        out.extend(std::iter::repeat(value).take(counter));
        i += 3;
    }
    out
}


/// Восстанавливает чанк из плоского вектора u16
fn u16_to_chunk(data: &[u16]) -> Chunk {
    let mut chunk = Chunk::new();
    let mut idx = 0;
    for y in 0..CHUNK_H {
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let id = data[idx];
                chunk.blocks[y][z][x] = if id == 0 { BlockType::Air } else { BlockType::Custom(id) };
                idx += 1;
            }
        }
    }
    chunk
}