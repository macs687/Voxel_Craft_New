// use std::fs;
// use std::path::{Path, PathBuf};
// use crate::loger::ProjectErrors;
// use crate::voxels::{BlockType, Chunk};
// use crate::settings::{CHUNK_W, CHUNK_H, CHUNK_D};
// use crate::world::region::Region;
// use serde::{Serialize, Deserialize};
// use crate::world::region::REGION_SIZE;


// /// Метаинформация о мире
// #[derive(Serialize, Deserialize)]
// pub struct WorldInfo {
//     pub name: String,
//     pub seed: u32,
//     pub player_position: [f32; 3],
// }


// /// Сохраняет чанк в aрхив региона. Если регион не существует, он будет создан.
// pub fn save_chunk(world_path: &Path, cx: i32, cy: i32, cz: i32, chunk: &Chunk) -> Result<(), ProjectErrors> {
//     let region_x = cx.div_euclid(REGION_SIZE);
//     let region_z = cz.div_euclid(REGION_SIZE);

//     let region_dir = world_path.join("regions");
//     fs::create_dir_all(&region_dir)?;
//     let region_path = region_dir.join(format!("r.{}.{}.dat", region_x, region_z));

//     let mut region = Region::open(&region_path)?;
//     region.save_chunk(cx, cz, chunk)?;

//     Ok(())
// }


// /// Загружает чанк из файла. Возвращает `None`, если файл не существует.
// pub fn load_chunk(world_path: &Path, cx: i32, cy: i32, cz: i32) -> Option<Chunk> {
//     let region_x = cx.div_euclid(REGION_SIZE);
//     let region_z = cz.div_euclid(REGION_SIZE);

//     let region_path = world_path.join("regions").join(format!("r.{}.{}.dat", region_x, region_z));

//     if !region_path.exists() {
//         return None;
//     }

//     let mut region = Region::open(&region_path).ok()?;
//     region.load_chunk(cx, cz, cy)
// }


// /// Сохраняет информацию о мире в `world_path/world.toml`
// pub fn save_world_info(world_path: &Path, info: &WorldInfo) -> std::io::Result<()> {
//     let toml = toml::to_string_pretty(info).expect("Failed to serialize WorldInfo");
//     fs::write(world_path.join("world.toml"), toml)
// }


// /// Загружает информацию о мире. Возвращает `None`, если файла нет.
// pub fn load_world_info(world_path: &Path) -> Option<WorldInfo> {
//     let file_path = world_path.join("world.toml");
//     if file_path.exists() {
//         let data = fs::read_to_string(file_path).ok()?;
//         toml::from_str(&data).ok()
//     } else {
//         None
//     }
// }