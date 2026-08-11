// use std::{fs::{File, OpenOptions}, path::Path};
// use std::io::{Read, Write, Seek, SeekFrom};
// use crate::voxels::Chunk;
// use crate::loger::ProjectErrors;
// use crate::settings::{CHUNK_W, CHUNK_H, CHUNK_D};
// use crate::voxels::BlockType;
// use std::collections::HashMap;

// pub const REGION_SIZE: i32 = 32; // 32x32 чанка в регионе
// pub const HEADER_SIZE: u64 = 4096;
// pub const REGION_VOLUME: usize = REGION_SIZE as usize * REGION_SIZE as usize;


// /// один сжатый регион
// pub struct RegionMemory {
//     compressed_chunks: [Option<Vec<u8>>; REGION_VOLUME],
// }


// impl RegionMemory {
//     fn new() -> Self {
//         Self {
//             compressed_chunks: std::array::from_fn(|_| None),
//         }
//     }
// }


// /// Основной кэш регионов.
// pub struct RegionCache {
//     /// Ключ: координаты региона (rx, rz), значение: память региона
//     regions: HashMap<(i32, i32), RegionMemory>,
// }


// impl RegionCache {
//     pub fn init() -> Self {
//         Self {
//             regions: HashMap::new(),
//         }
//     }


//     pub fn cache_chunk(&mut self, cx: i32, cz: i32, chunk_data: &[u16]) {
//         let region_x = cx.div_euclid(REGION_SIZE);
//         let region_z = cz.div_euclid(REGION_SIZE);
//         let local_x = cx.rem_euclid(REGION_SIZE) as usize;
//         let local_z = cz.rem_euclid(REGION_SIZE) as usize;
//         let index = local_z * REGION_SIZE as usize + local_x;
        
//         let region = self.regions.entry((region_x, region_z)).or_insert_with(|| {
//             load_region_file(&Self::region_path(world_path, region_x, region_z))
//         });
//     }
    
// }


// fn load_region_file(path: &Path) -> Result<HashMap<usize, Vec<u8>>, ProjectErrors> {
//     let mut file = match File::open(path) {
//         Ok(f) => f,
//         Err(_) => return Ok(HashMap::new()),
//     };

//     let len = file.metadata()?.len();
//     if len < HEADER_SIZE {
//         return Ok(HashMap::new());
//     }

//     // Читаем заголовок (1024 смещения по 4 байта, big-endian)
//     let mut offsets = [0u32; REGION_VOLUME];
//     for i in 0..REGION_VOLUME {
//         let mut buf = [0u8; 4];
//         file.seek(SeekFrom::Start(i as u64 * 4))?;
//         file.read_exact(&mut buf)?;
//         offsets[i] = u32::from_be_bytes(buf);
//     }

//     let mut result = HashMap::new();
//     for (i, &offset) in offsets.iter().enumerate() {
//         if offset == 0 {
//             continue;
//         }
//         // Переходим к смещению offset
//         file.seek(SeekFrom::Start(offset as u64))?;
//         // Читаем размер сжатых данных (u32 be)
//         let mut size_buf = [0u8; 4];
//         file.read_exact(&mut size_buf)?;
//         let compressed_size = u32::from_be_bytes(size_buf) as usize;
//         // Читаем сами сжатые данные
//         let mut compressed = vec![0u8; compressed_size];
//         file.read_exact(&mut compressed)?;
//         result.insert(i, compressed);
//     }

//     Ok(result)
// }




// pub struct Region {
//     file: File,
//     offsets: [u32; REGION_SIZE as usize * REGION_SIZE as usize], // 1024 чанка
// }


// impl Region {
//     pub fn open(region_path: &Path) -> Result<Self, ProjectErrors> {
//         let mut file = OpenOptions::new()
//             .read(true)
//             .write(true)
//             .create(true)
//             .open(region_path)?;

//         let mut offsets = [0u32; (REGION_SIZE * REGION_SIZE) as usize];
//         let metadata = file.metadata()?;

//         if metadata.len() >= HEADER_SIZE {
//             let mut buf = [0u8; 4];
//             for (i, offset) in offsets.iter_mut().enumerate() {
//                 file.seek(SeekFrom::Start(i as u64 * 4))?;
//                 file.read_exact(&mut buf)?;
//                 *offset = u32::from_be_bytes(buf);
//             }
//         } else {
//             file.set_len(HEADER_SIZE)?; // Инициализируем файл с нуля
//         }

//         Ok( Self { file, offsets })
//     }


//     fn index(cx: i32, cz: i32) -> usize {
//         let x = cx.rem_euclid(REGION_SIZE) as usize;
//         let z = cz.rem_euclid(REGION_SIZE) as usize;
//         x + z * REGION_SIZE as usize
//     }


//     pub fn load_chunk(&mut self, cx: i32, cz: i32, cy: i32) -> Option<Chunk> {
//         let idx = Self::index(cx, cz);
//         let offset = self.offsets[idx] as u64;
//         if offset == 0 || offset < HEADER_SIZE {
//             return None;
//         }

//         // Читаем данные чанка
//         let mut data = vec![0u16; CHUNK_W * CHUNK_H * CHUNK_D];
//         let bytes: &mut [u8] = unsafe {
//             std::slice::from_raw_parts_mut(
//                 data.as_mut_ptr() as *mut u8,
//                 data.len() * 2,
//             )
//         };

//         self.file.seek(SeekFrom::Start(offset)).ok()?;
//         self.file.read_exact(bytes).ok()?;

//         let mut chunk = Chunk::new();
//         let mut idx = 0;

//         for y in 0..CHUNK_H {
//             for z in 0..CHUNK_D {
//                 for x in 0..CHUNK_W {
//                     let id = data[idx];
//                     let block = if id == 0 { BlockType::Air } else { BlockType::Custom(id) };
//                     chunk.blocks[y][z][x] = block;
//                     idx += 1;
//                 }
//             }
//         }
//         Some(chunk)
//     }


//     pub fn save_chunk(&mut self, cx: i32, cz: i32, chunk: &Chunk) -> Result<(), ProjectErrors> {
//         let idx = Self::index(cx, cz);
//         let old_offset = self.offsets[idx] as u64;

//         // Сериализуем чанк
//         let mut data: Vec<u16> = Vec::with_capacity(CHUNK_W * CHUNK_H * CHUNK_D);

//         for y in 0..CHUNK_H {
//             for z in 0..CHUNK_D {
//                 for x in 0..CHUNK_W {
//                     data.push(chunk.blocks[y][z][x].id());
//                 }
//             }
//         }

//         let bytes: &[u8] = unsafe {
//             std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 2)
//         };

//         // Если чанк полностью из воздуха, удаляем его из файла
//         if data.iter().all(|id| *id == 0) {
//             if old_offset != 0 {
//                 // Помечаем как удалённый, не сдвигая данные (для простоты)
//                 self.offsets[idx] = 0;
//                 self.write_header()?;
//             }
//             return Ok(());
//         }

//         // Пишем в конец файла (или перезаписываем, если старый блок подходит по размеру)
//         let offset = self.file.seek(SeekFrom::End(0))?;
//         self.file.write_all(bytes)?;
//         self.offsets[idx] = offset as u32;
//         self.write_header()?;
//         Ok(())
//     }
    

//     fn write_header(&mut self) -> Result<(), ProjectErrors> {
//         self.file.seek(SeekFrom::Start(0))?;
//         for offset in &self.offsets {
//             let buf = offset.to_be_bytes();
//             self.file.write_all(&buf)?;
//         }
//         Ok(())
//     }
// }