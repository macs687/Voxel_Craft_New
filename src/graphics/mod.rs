mod shader;
mod mesh;
mod texture;
mod voxel_renderer;

pub use shader::load_shader;
pub use texture::load_texture_from_png;
pub use voxel_renderer::VoxelRenderer;
const VERTEX_SIZE: usize = 5;