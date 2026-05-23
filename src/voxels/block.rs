use crate::mods::BlocksManager;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum BlockType {
    Air = 0,
    Custom(u16),
}


impl BlockType {
    pub fn name<'a>(&self, blocks_manager: &'a BlocksManager) -> &'a str {
        match self {
            BlockType::Air => "air",
            BlockType::Custom(id) => &blocks_manager.blocks[*id as usize].name
        }
    }


    pub fn id(&self) -> u16 {
        match self {
            BlockType::Air => 0,
            BlockType::Custom(id) => *id,
        }
    }
}