use glam::Vec3;

pub struct Hitbox {
    pub min: Vec3,
    pub max: Vec3
}


impl Hitbox {
    pub fn from_player(center_bottom: Vec3, width: f32, height: f32) -> Self {
        let half_w = width / 2.0;

        Self {
            min: center_bottom - Vec3::new(half_w, 0.0, half_w),
            max: center_bottom + Vec3::new(half_w, height, half_w)
        }
    }


    pub fn from_block(x: i32, y: i32, z: i32) -> Self {
        Self { 
            min: Vec3::new(x as f32, y as f32, z as f32), 
            max: Vec3::new((x + 1) as f32, (y + 1) as f32, (z + 1) as f32)
        }
    }


    pub fn penetration(&self, block: &Hitbox, axis: Vec3) -> Option<f32> {
        if axis.x != 0.0 {
            let overlap_left = self.max.x - block.min.x;
            let overlap_right = block.max.x - self.min.x;
            if overlap_left > 0.0 && overlap_right > 0.0 {
                // Выбираем минимальное смещение, чтобы разойтись
                if overlap_left < overlap_right {
                    Some(-overlap_left)
                } else {
                    Some(overlap_right)
                }
            } else {
                None
            }
        } else if axis.y != 0.0 {
            let overlap_bottom = self.max.y - block.min.y;
            let overlap_top = block.max.y - self.min.y;
            if overlap_bottom > 0.0 && overlap_top > 0.0 {
                if overlap_bottom < overlap_top {
                    Some(-overlap_bottom)
                } else {
                    Some(overlap_top)
                }
            } else {
                None
            }
        } else {
            let overlap_back = self.max.z - block.min.z;
            let overlap_front = block.max.z - self.min.z;
            if overlap_back > 0.0 && overlap_front > 0.0 {
                if overlap_back < overlap_front {
                    Some(-overlap_back)
                } else {
                    Some(overlap_front)
                }
            } else {
                None
            }
        }
    }
}