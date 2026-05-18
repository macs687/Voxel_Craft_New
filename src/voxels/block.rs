use gl::types::*;


pub struct Block {
    id: u8
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockType {
    Air,
    Planks 
}