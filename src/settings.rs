use glam::Vec3;

/// ширина чанка X
pub const CHUNK_W: usize = 16;  // ширина по X
/// высота чанка Y
pub const CHUNK_H: usize = 64;  // высота по Y (можно поставить 128, 256)
/// глубина чанка Z
pub const CHUNK_D: usize = 16;  // глубина по Z
/// чувствительность мыши
pub const MOUSE_SENSITIVITY: f32 = 0.001;
/// сид мира
pub const SEED: u32 = 1232;
/// спавнпоинт
pub const SPAWNPOINT: Vec3 = Vec3::new(0.0, 16.0, -10.0);
/// угол обзора
pub const FOV: f32 = 40.0_f32.to_radians();
/// скорость
pub const MOVE_SPEED: f32 = 5.0; 
/// название окна 
pub const TITLE: &str = "Voxel_Craft";
/// ширина окна
pub const WIDTH: u32 = 1920;
/// высота окна
pub const HEIGHT: u32 = 1080;
