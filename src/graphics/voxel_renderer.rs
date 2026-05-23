use crate::{graphics::mesh::Mesh, mods::BlocksManager, settings::{CHUNK_D, CHUNK_H, CHUNK_W}, voxels::{BlockType, Chunk}, world::World};

const ATLAS_COLS: f32 = 16.0;   // количество столбцов в атласе
const ATLAS_ROWS: f32 = 16.0;   // количество строк
const TILE_SIZE: f32 = 1.0 / 16.0; // размер одного тайла в UV‑координатах (0..1)


pub struct VoxelRenderer {
    pub buffer: Vec<f32>
}


impl VoxelRenderer {
    pub fn init() -> Self {
        Self { buffer: Vec::new() }
    }


    pub fn render(&mut self, chunk: &Chunk, chunk_x: i32, chunk_y: i32, chunk_z: i32, world: &World, blocks_manager: &BlocksManager) -> Mesh {
        let get_block = &|wx, wy, wz| world.get_block(wx, wy, wz);
        let get_uv = &|name: &str| blocks_manager.get_uv_by_name(name).unwrap_or([0.0; 4]);

        self.render_to_buffer(chunk, chunk_x, chunk_y, chunk_z, blocks_manager, get_block, get_uv);
        Mesh::new(&self.buffer)
    }


    pub fn render_to_buffer<F, G>(&mut self, chunk: &Chunk, chunk_x: i32, chunk_y: i32, chunk_z: i32, blocks_manager: &BlocksManager, get_block: &F, get_uv: &G) where 
    F: Fn(i32, i32, i32) -> Option<BlockType>,
    G: Fn(&str) -> [f32; 4]
    {
        self.buffer.clear();

        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                for x in 0..CHUNK_W {
                    let block = chunk.blocks[y][z][x];
                    if block == BlockType::Air {
                        continue;
                    }

                    let gx = chunk_x * CHUNK_W as i32  + x as i32;
                    let gy = chunk_y * CHUNK_H as i32  + y as i32;
                    let gz = chunk_z * CHUNK_D as i32  + z as i32;

                    let lx = x as f32;
                    let ly = y as f32;
                    let lz = z as f32;
                    let s = 0.5;

                    let faces = [
                        (1, 0, 0),
                        (-1, 0, 0),
                        (0, 1, 0),
                        (0, -1, 0),
                        (0, 0, 1),
                        (0, 0, -1),
                    ];

                    for &(dx, dy, dz) in &faces {
                        let ngx = (gx as isize + dx as isize) as i32;
                        let ngy = (gy as isize + dy as isize) as i32;
                        let ngz = (gz as isize + dz as isize) as i32;

                        let neighbor = get_block(ngx, ngy, ngz);
                        let visible = neighbor.map_or(true, |b| b == BlockType::Air);

                        if visible {
                            let uv = get_uv(block.name(blocks_manager));
                            let block_id = block.id();
                            add_face(&mut self.buffer, lx, ly, lz, s, (dx, dy, dz), uv, block_id);
                        }
                    }
                }
            }
        }
    }
}


fn add_face(buffer: &mut Vec<f32>, lx: f32, ly: f32, lz: f32, s: f32, dir: (i32, i32, i32), uv: [f32; 4], block_id: u16) {
    let [u_min, v_min, u_max, v_max] = uv;
    let uv00 = (u_min, v_min);
    let uv10 = (u_max, v_min);
    let uv11 = (u_max, v_max);
    let uv01 = (u_min, v_max);

    let (v0, v1, v2, v3): ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32)) = match dir {
        ( 1,  0,  0 ) => ( // право (+x)
            (lx + s, ly - s, lz - s),
            (lx + s, ly - s, lz + s),
            (lx + s, ly + s, lz + s),
            (lx + s, ly + s, lz - s),
        ),
        ( -1,  0,  0 ) => ( // лево (-x)
            (lx - s, ly - s, lz + s),
            (lx - s, ly - s, lz - s),
            (lx - s, ly + s, lz - s),
            (lx - s, ly + s, lz + s),
        ),
        ( 0,  1,  0 ) => ( // верх (+y)
            (lx - s, ly + s, lz - s),
            (lx + s, ly + s, lz - s),
            (lx + s, ly + s, lz + s),
            (lx - s, ly + s, lz + s),
        ),
        ( 0, -1,  0 ) => ( // низ (-y)
            (lx - s, ly - s, lz + s),
            (lx + s, ly - s, lz + s),
            (lx + s, ly - s, lz - s),
            (lx - s, ly - s, lz - s),
        ),
        ( 0,  0,  1 ) => ( // перед (+z)
            (lx - s, ly - s, lz + s),
            (lx + s, ly - s, lz + s),
            (lx + s, ly + s, lz + s),
            (lx - s, ly + s, lz + s),
        ),
        ( 0,  0, -1 ) => ( // зад (-z)
            (lx + s, ly - s, lz - s),
            (lx - s, ly - s, lz - s),
            (lx - s, ly + s, lz - s),
            (lx + s, ly + s, lz - s),
        ),
        _ => return,
    };

    // Первый треугольник: v0, v1, v2
    vertex(buffer, v0.0, v0.1, v0.2, uv00.0, uv00.1, block_id as f32);
    vertex(buffer, v1.0, v1.1, v1.2, uv10.0, uv10.1, block_id as f32);
    vertex(buffer, v2.0, v2.1, v2.2, uv11.0, uv11.1, block_id as f32);
    // Второй треугольник: v0, v2, v3
    vertex(buffer, v0.0, v0.1, v0.2, uv00.0, uv00.1, block_id as f32);
    vertex(buffer, v2.0, v2.1, v2.2, uv11.0, uv11.1, block_id as f32);
    vertex(buffer, v3.0, v3.1, v3.2, uv01.0, uv01.1, block_id as f32);
}


fn vertex(buffer: &mut Vec<f32>, x: f32, y: f32, z: f32, u: f32, v: f32, block_id: f32) {
    buffer.push(x);
    buffer.push(y);
    buffer.push(z);
    buffer.push(u);
    buffer.push(v);
    buffer.push(block_id);
}