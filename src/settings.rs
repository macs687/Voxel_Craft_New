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
pub const SPAWNPOINT: Vec3 = Vec3::new(0.0, 30.0, -10.0);
/// угол обзора
pub const FOV: f32 = 70.0_f32.to_radians();
/// скорость
pub const MOVE_SPEED: f32 = 15.0; 
/// название окна 
pub const TITLE: &str = "Voxel_Craft";
/// ширина окна
pub const WIDTH: u32 = 1920;
/// высота окна
pub const HEIGHT: u32 = 1080;
/// кол во типов блоков
pub const BLOCK_TYPES_NUMBER: i32 = 6;

/// НАСТРОЙКИ РЭЙКАСТА
pub const MAX_STEPS: i32 = 30;
pub const RANGE: i32 = 7;

/// дальность прорисовки
pub const RENDER_DIST: i32 = 16;


pub const PLAYER_WIDTH: f32 = 0.6;
pub const PLAYER_HEIGHT: f32 = 1.8;
pub const BASE_PLAYER_EYE_HEIGHT: f32 = 2.0;
pub const GRAVITY: f32 = -25.0;
pub const JUMP_FORCE: f32 = 8.0;


pub const CREATIVE_VERTICAL_MOVE: f32 = 0.1;
pub const CREATIVE_HORIZONTAL_SPEED: f32 = 70.0;
pub const PERMISION_TEXTURE: u32 = 32;


use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub mouse_sensitivity: f32,
    pub volume: f32,
}


impl Default for Settings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.1,
            volume: 1.0,
        }
    }
}


impl Settings {
    const PATH: &str = "res/settings.toml";   // <-- обновлённый путь

    pub fn load() -> Self {
        if Path::new(Self::PATH).exists() {
            let data = fs::read_to_string(Self::PATH).expect("Unable to read settings file");
            toml::from_str(&data).unwrap_or_default()
        } else {
            Settings::default()
        }
    }

    pub fn save(&self) {
        let data = toml::to_string_pretty(self).expect("Failed to serialize settings");
        fs::write(Self::PATH, data).expect("Unable to write settings file");
    }
}
