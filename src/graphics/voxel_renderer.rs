use crate::{graphics::mesh::Mesh, settings::CHUNK_SIZE, voxels::{BlockType, Chunk}, world::World};

pub struct VoxelRenderer {
    buffer: Vec<f32>
}


impl VoxelRenderer {
    pub fn init() -> Self {
        Self { buffer: Vec::new() }
    }
    

    pub fn render(&mut self, chunk: &Chunk, chunk_x: i32, chunk_y: i32, chunk_z: i32, world: &World) -> Mesh {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let block = chunk.blocks[y][z][x];

                    if block == BlockType::Air {
                        continue;
                    }

                    let gx = chunk_x * CHUNK_SIZE as i32 + x as i32;
                    let gy = chunk_y * CHUNK_SIZE as i32 + y as i32;
                    let gz = chunk_z * CHUNK_SIZE as i32 + z as i32;

                    let cx = x as f32;
                    let cy = y as f32;
                    let cz = z as f32;
                    let s = 0.5; // половина размера блока

                    let faces = [
                        ( 1,  0,  0), // право
                        (-1,  0,  0), // лево
                        ( 0,  1,  0), // верх
                        ( 0, -1,  0), // низ
                        ( 0,  0,  1), // перед (z+)
                        ( 0,  0, -1), // зад (z-)
                    ];

                    for &(dx, dy, dz) in &faces {
                        let ngx = gx as isize + dx as isize;
                        let ngy = gy as isize + dy as isize;
                        let ngz = gz as isize + dz as isize;

                        let neighbor = world.get_block(ngx as i32, ngy as i32, ngz as i32);
                        let visible = neighbor.map_or(true, |b| b == BlockType::Air);
                        
                        if visible {
                            add_face(&mut self.buffer, cx, cy, cz, s, (dx, dy, dz));
                        }
                    }
                }   
            }
        }

        Mesh::new(&self.buffer)
    }
}


fn add_face(buffer: &mut Vec<f32>, cx: f32, cy: f32, cz: f32, s: f32, dir: (i32, i32, i32)) {
    let uv00 = (0.0f32, 0.0f32);
    let uv10 = (1.0, 0.0);
    let uv11 = (1.0, 1.0);
    let uv01 = (0.0, 1.0);

    let (v0, v1, v2, v3): ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32)) = match dir {
        ( 1,  0,  0) => ( // право (+x)
            (cx + s, cy - s, cz - s),
            (cx + s, cy - s, cz + s),
            (cx + s, cy + s, cz + s),
            (cx + s, cy + s, cz - s),
        ),
        (-1,  0,  0) => ( // лево (-x)
            (cx - s, cy - s, cz + s),
            (cx - s, cy - s, cz - s),
            (cx - s, cy + s, cz - s),
            (cx - s, cy + s, cz + s),
        ),
        ( 0,  1,  0) => ( // верх (+y)
            (cx - s, cy + s, cz - s),
            (cx + s, cy + s, cz - s),
            (cx + s, cy + s, cz + s),
            (cx - s, cy + s, cz + s),
        ),
        ( 0, -1,  0) => ( // низ (-y)
            (cx - s, cy - s, cz + s),
            (cx + s, cy - s, cz + s),
            (cx + s, cy - s, cz - s),
            (cx - s, cy - s, cz - s),
        ),
        ( 0,  0,  1) => ( // перед (+z)
            (cx - s, cy - s, cz + s),
            (cx + s, cy - s, cz + s),
            (cx + s, cy + s, cz + s),
            (cx - s, cy + s, cz + s),
        ),
        ( 0,  0, -1) => ( // зад (-z)
            (cx + s, cy - s, cz - s),
            (cx - s, cy - s, cz - s),
            (cx - s, cy + s, cz - s),
            (cx + s, cy + s, cz - s),
        ),
        _ => return,
    };

    // Первый треугольник: v0, v1, v2
    push_vertex(buffer, v0.0, v0.1, v0.2, uv00.0, uv00.1);
    push_vertex(buffer, v1.0, v1.1, v1.2, uv10.0, uv10.1);
    push_vertex(buffer, v2.0, v2.1, v2.2, uv11.0, uv11.1);
    // Второй треугольник: v0, v2, v3
    push_vertex(buffer, v0.0, v0.1, v0.2, uv00.0, uv00.1);
    push_vertex(buffer, v2.0, v2.1, v2.2, uv11.0, uv11.1);
    push_vertex(buffer, v3.0, v3.1, v3.2, uv01.0, uv01.1);
}


fn push_vertex(buffer: &mut Vec<f32>, x: f32, y: f32, z: f32, u: f32, v: f32) {
    buffer.push(x);
    buffer.push(y);
    buffer.push(z);
    buffer.push(u);
    buffer.push(v);
}