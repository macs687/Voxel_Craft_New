// use image::{RgbaImage, Rgba};

// use crate::loger::ProjectErrors;

// pub const TOTAL_BLOCKS: u16 = 7;
// pub const TILE_SIZE: u8 = 16;




// pub fn build_texture_atlas() -> Result<(), ProjectErrors> {
//     let _total_blocks = TOTAL_BLOCKS as u32; // TODO СДЕЛАТЬ АВТОМАТИЧЕСКИЙ ПОДСЧЁТ КОЛ-ВО БЛОКОВ В ДВИЖКЕ
//     let rows = (_total_blocks as f64).sqrt().ceil() as u32;
//     let columns = (_total_blocks as f64 / rows as f64).ceil() as u32;

//     let atlas_width = rows * TILE_SIZE as u32;
//     let atlas_height = columns * TILE_SIZE as u32;

//     let mut atlas = RgbaImage::from_pixel(atlas_width, atlas_height, Rgba([0u8, 0, 0, 0]));

//     for id in 0.._total_blocks {
//         let (col, row) = (id % columns, id / rows);

//     };


//     Ok(())
// }