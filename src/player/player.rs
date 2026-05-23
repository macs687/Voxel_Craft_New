use glam::Vec3;
use crate::settings::{BASE_PLAYER_EYE_HEIGHT, CREATIVE_HORIZONTAL_SPEED, GRAVITY, JUMP_FORCE, MOVE_SPEED, PLAYER_HEIGHT, PLAYER_WIDTH};
use crate::world::World;
use crate::physics::Hitbox;
use crate::BlockType;


pub struct Player {
    pub hitbox: Hitbox,
    /// Позиция нижней точки игрока (центр низа AABB)
    pub position: Vec3,
    /// Скорость (гравитация и движение)
    pub velocity: Vec3,
    /// Ширина AABB (игрок - прямоугольник)
    pub width: f32,
    /// Высота AABB
    pub height: f32,
    /// Высота глаз относительно position.y
    pub eye_height: f32,
    /// На земле ли игрок (определяется при проверке коллизий)
    pub on_ground: bool,
    /// текущий блок в руке
    pub selected_block: String,
    /// полёт
    pub fly: bool
}


impl Player {
    pub fn init(position: Vec3) -> Self {
        Self {
            hitbox: Hitbox::from_player(position, PLAYER_WIDTH, PLAYER_HEIGHT),
            position: position,
            velocity: Vec3::ZERO,
            width: PLAYER_WIDTH, 
            height: PLAYER_HEIGHT,
            eye_height: BASE_PLAYER_EYE_HEIGHT,
            on_ground: true,
            selected_block: "Planks".to_string(),
            fly: false
        }
    }


    fn check_ground(&mut self, world: &World) {
        let cx = self.position.x.floor() as i32;
        let cz = self.position.z.floor() as i32;
        let by = self.position.y.floor() as i32 - 1;

        if let Some(block) = world.get_block(cx, by, cz) {
            if block != BlockType::Air {
                let ground_level = by as f32 + 1.0;

                if self.position.y <= ground_level + 0.001 {
                    self.position.y = ground_level;
                    self.velocity.y = 0.0;
                    self.on_ground = true;
                }

                self.on_ground = false;
            }
        }
    }


    fn get_colliding_blocks(&self, world: &World) -> Vec<(i32, i32, i32)> {
        let min_x = self.hitbox.min.x.floor() as i32;
        let min_y = self.hitbox.min.y.floor() as i32;
        let min_z = self.hitbox.min.z.floor() as i32;
        let max_x = self.hitbox.max.x.ceil() as i32;
        let max_y = self.hitbox.max.y.ceil() as i32;
        let max_z = self.hitbox.max.z.ceil() as i32;

        let mut blocks = Vec::new();

        for y in min_y..max_y {
            for z in min_z..max_z {
                for x in min_x..max_x {
                    if let Some(block) = world.get_block(x, y, z) {
                        if block != BlockType::Air {
                            blocks.push((x, y, z));
                        }
                    }
                }
            }
        }

        blocks
    }


    fn try_move(&mut self, offset: Vec3, world: &World) {
        let new_pos = self.position + offset;
        self.hitbox = Hitbox::from_player(new_pos, self.width, self.height);
        let colliding_blocks = self.get_colliding_blocks(world);

        if colliding_blocks.is_empty() {
            self.position = new_pos;
            return;
        }

        let axis = if offset.x != 0.0 {
            Vec3::X
        } else if offset.y != 0.0 {
            Vec3::Y
        } else {
            Vec3::Z
        };

        let mut best_penetration = 0.0f32;
        for &(bx, by, bz) in &colliding_blocks {
            let block_aabb = Hitbox::from_block(bx, by, bz);
            if let Some(pen) = self.hitbox.penetration(&block_aabb, axis) {
                if pen.abs() > best_penetration.abs() {
                    best_penetration = pen;
                }
            }
        }

        self.position = new_pos + axis * best_penetration;
    }


    pub fn update_moving(&mut self, world: &World, move_dir: Vec3, jump: bool, delta: f32) -> Vec3 {
        if !self.fly {
            self.velocity.y += GRAVITY * delta;
            self.velocity.x = move_dir.x * MOVE_SPEED;
            self.velocity.z = move_dir.z * MOVE_SPEED;
        } else {
            self.velocity.x = move_dir.x * CREATIVE_HORIZONTAL_SPEED;
            self.velocity.z = move_dir.z * CREATIVE_HORIZONTAL_SPEED;
        }
        
        
        if jump && self.on_ground {
            self.velocity.y = JUMP_FORCE;
            self.on_ground = false;
        }
        
        self.try_move(Vec3::new(self.velocity.x * delta, 0.0, 0.0), world);
        self.try_move(Vec3::new(0.0, self.velocity.y * delta, 0.0), world);
        self.try_move(Vec3::new(0.0, 0.0, self.velocity.z * delta), world);

        self.check_ground(world);

        self.position + Vec3::new(0.0, BASE_PLAYER_EYE_HEIGHT, 0.0)
    }
}