use crate::{graphics::VoxelRenderer, world::{World}};
use crate::graphics::Mesh;

pub struct WorldController {
    
}


impl WorldController {
    pub fn init() -> Self {
        Self {  }
    }


    pub fn create_world(&self, renderer: &mut VoxelRenderer) -> World {
        let mut world = World::create();
        world.generate_start_landscape();

        let len = world.chunks.len();
        println!("Чанков {len}");

        for (&(cx, cy, cz), chunk) in &world.chunks {
            let mesh = renderer.render(chunk, cx, cy, cz, &world);
            world.chunks_meshes.insert((cx, cy, cz), Mesh {
                vao: mesh.vao,
                vbo: mesh.vbo,
                vertex_count: mesh.vertex_count
            });
        }

        World { chunks: world.chunks, chunks_meshes: world.chunks_meshes, min_cx: 0, max_cx: 0, min_cz: 0, max_cz: 0 }
    }

}