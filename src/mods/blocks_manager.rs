use std::path::Path;
use image::{Rgba, RgbaImage};
use crate::loger::ProjectErrors;


pub struct BlockInfo {
    pub name: String,
    pub id: u16,
    pub uv: [f32; 4]
}


pub struct BlocksManager {
    pub blocks: Vec<BlockInfo>,
    pub atlas_columns: u32,
    pub atlas_rows: u32,
    pub tile_size: u32,
    pub atlas_path: String,
    pub blocks_dir: String,
}


impl BlocksManager {
    pub fn init(atlas_path: &str, blocks_dir: &str, tile_size: u32) -> Result<Self, ProjectErrors> {
        let mut blocks = vec![BlockInfo {
            name: "air".into(),
            id: 0,
            uv: [0.0; 4]
        }];

        let directory = Path::new(blocks_dir);

        let mut png_files: Vec<_> = std::fs::read_dir(directory)
            .map_err(ProjectErrors::Io)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
            .collect();

        png_files.sort_by_key(|e| e.file_name().to_string_lossy().to_lowercase());

        for (index, entry) in png_files.iter().enumerate() {
            let path = entry.path();

            let stem = path.file_stem()
                .ok_or_else(|| ProjectErrors::InvalidFileName(format!(
                    "Файл {} имеет недопустимое имя",
                    path.display()
                )))?;

            let stem = stem.to_string_lossy().to_string();
            let id = (index + 1) as u16;

            blocks.push(BlockInfo {
                name: stem, 
                id, 
                uv: [0.0; 4] 
            });
        }

        let total_blocks = blocks.len();
        let columns = (total_blocks as f64).sqrt().ceil() as u32;
        let rows = (total_blocks as f64 / columns as f64).ceil() as u32;

        Ok( Self { 
            blocks, 
            atlas_columns: columns, 
            atlas_rows: rows, 
            tile_size,
            atlas_path: atlas_path.to_string(),
            blocks_dir: blocks_dir.to_string()
        })
    }


    pub fn build_atlas(&mut self) -> Result<(), ProjectErrors> {
        let width = self.atlas_columns * self.tile_size;
        let height = self.atlas_rows * self.tile_size;
        let mut atlas = RgbaImage::from_pixel(width, height, Rgba([0u8; 4]));

        for id in 1..self.blocks.len() as u16 {
            let info = &self.blocks[id as usize];
            let file_path = Path::new(&self.blocks_dir).join(format!("{}.png", info.name));

            if !file_path.exists() {
                eprintln!("Файл {} не найден, пропускаем", file_path.display());
                continue;
            }

            let img = image::open(&file_path).map_err(|e| ProjectErrors::TextureLoadError {
                path: file_path.to_string_lossy().into_owned(),
                source: e,
            })?;

            let img = img.resize_exact(
                self.tile_size,
                self.tile_size,
                image::imageops::FilterType::Nearest,
            );

            let rgba = img.to_rgba8();
            let col = id % self.atlas_columns as u16;
            let row = id / self.atlas_columns as u16;
            let x_offset = col as u32 * self.tile_size;
            let y_offset = row as u32 * self.tile_size;

            for y in 0..self.tile_size {
                for x in 0..self.tile_size {
                    atlas.put_pixel(x_offset + x, y_offset + y, *rgba.get_pixel(x, y));
                }
            }

            // UV с переворотом по вертикали (OpenGL)
            let u_min = col as f32 / self.atlas_columns as f32;
            let v_min = 1.0 - (row as f32 + 1.0) / self.atlas_rows as f32;
            let u_max = u_min + 1.0 / self.atlas_columns as f32;
            let v_max = v_min + 1.0 / self.atlas_rows as f32;
            self.blocks[id as usize].uv = [u_min, v_min, u_max, v_max];
        }

        atlas.save(Path::new(&self.atlas_path)).map_err(|e| ProjectErrors::Io);
        Ok(())
    }


    pub fn get_id(&self, block_name: &String) -> Option<u16> {
        self.blocks.iter()
            .find(|info| info.name.eq_ignore_ascii_case(block_name))
            .map(|info| info.id)
    }


    pub fn get_uv_by_name(&self, name: &str) -> Option<[f32; 4]> {
        self.blocks.iter()
            .find(|info| info.name.eq_ignore_ascii_case(name))
            .map(|info| info.uv)
    }
}