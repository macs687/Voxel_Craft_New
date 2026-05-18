mod draw_world;
use glam::*;
use crate::voxels::Chunk;
pub use draw_world::draw_world;
use crate::settings::*;
use crate::voxels::BlockType;

#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    pub block_pos: (i32, i32, i32), // координаты блока в мировом пространстве (целые)
    pub normal: (i32, i32, i32),    // нормаль грани, в которую попал луч
}

pub fn raycast(chunk: &Chunk, origin: glam::Vec3, direction: glam::Vec3, max_dist: f32) -> Option<RayHit> {
    // Переводим в координаты блоков (float)
    let mut t = 0.0;
    let step = 0.01; // маленький шаг для простоты (не самый быстрый, но надёжный)
    while t < max_dist {
        let pos = origin + direction * t;
        let bx = pos.x.floor() as i32;
        let by = pos.y.floor() as i32;
        let bz = pos.z.floor() as i32;

        //println!("t={:.2}, pos=({:.2},{:.2},{:.2}), bx={}, by={}, bz={}", t, pos.x, pos.y, pos.z, bx, by, bz);


        // Проверяем, что координаты в пределах чанка
        if bx >= 0 && bx < CHUNK_SIZE as i32 && by >= 0 && by < CHUNK_SIZE as i32 && bz >= 0 && bz < CHUNK_SIZE as i32 {
            let block = chunk.blocks[by as usize][bz as usize][bx as usize];
            if block != BlockType::Air {
                // Нашли блок. Определим нормаль (с какой стороны пришли)
                let normal = find_normal(origin, direction, t);
                return Some(RayHit {
                    block_pos: (bx, by, bz),
                    normal,
                });
            }
        }
        t += step;
    }
    None
}

fn find_normal(origin: glam::Vec3, direction: glam::Vec3, t: f32) -> (i32, i32, i32) {
    let hit_point = origin + direction * t;
    let bx = hit_point.x.floor() as i32;
    let by = hit_point.y.floor() as i32;
    let bz = hit_point.z.floor() as i32;

    let cx = bx as f32 + 0.5;
    let cy = by as f32 + 0.5;
    let cz = bz as f32 + 0.5;

    let rel = hit_point - glam::Vec3::new(cx, cy, cz);
    // Определяем, к какой грани ближе
    let abs = glam::Vec3::new(rel.x.abs(), rel.y.abs(), rel.z.abs());
    if abs.x > abs.y && abs.x > abs.z {
        (if rel.x > 0.0 { 1 } else { -1 }, 0, 0)
    } else if abs.y > abs.x && abs.y > abs.z {
        (0, if rel.y > 0.0 { 1 } else { -1 }, 0)
    } else {
        (0, 0, if rel.z > 0.0 { 1 } else { -1 })
    }
}