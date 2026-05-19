use crate::core::{Window, Events, Camera};
use crate::constant::{KEY_A, KEY_D, KEY_ESC, KEY_LEFT_SHIFT, KEY_S, KEY_SPACE, KEY_TAB, KEY_W, LCM, PCM};
use crate::settings::{MOUSE_SENSITIVITY, MOVE_SPEED};
use glam::Vec3;
use crate::world::{RayHit, World};
use crate::voxels::BlockType;
use crate::graphics::{VoxelRenderer};


pub fn update(events: &mut Events, window: &mut Window) {
    if events.j_clicked(LCM) {
        window.gl_clear_color(0.3, 0.4, 0.5, 0.6);
    } else if events.j_pressed(KEY_TAB) && events.cursor_in_window {
        events.switch_cursor_mode(window);
    } else if events.j_pressed(KEY_ESC) {
        window.close();
    }
}


pub fn update_moving(events: &mut Events, camera: &mut Camera, world: &mut World, delta_time: f32, renderer: &mut VoxelRenderer, hit: &Option<RayHit>) {
    let mut direction = Vec3::ZERO;
    let pitch_delta = events.delta_y * MOUSE_SENSITIVITY;
    let yaw_delta = -events.delta_x * MOUSE_SENSITIVITY;

    if events.cursor_locked {
        camera.rotate(-pitch_delta, yaw_delta, 0.0);
    }

    if let Some(hit ) = hit {
        if events.j_clicked(LCM) {
            world.set_block(hit.block_pos.0 as i32, hit.block_pos.1 as i32, hit.block_pos.2 as i32, BlockType::Air);
            world.update(hit.block_pos, renderer);
            //println!("обноваление мира");
        } else if events.j_clicked(PCM) {
            world.set_block(hit.block_pos.0 as i32, hit.block_pos.1 as i32, hit.block_pos.2 as i32, BlockType::Planks);
            world.update(hit.block_pos, renderer);
        }
    }

    if events.pressed(KEY_W) {
        direction += camera.front;
    } else if events.pressed(KEY_A) {
        direction -= camera.right;
    } else if events.pressed(KEY_S) {
        direction -= camera.front;
    } else if events.pressed(KEY_D) {
        direction += camera.right;
    } else if events.pressed(KEY_SPACE) {
        direction += camera.up
    } else if events.pressed(KEY_LEFT_SHIFT) {
        direction -= camera.up;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        camera.position += direction * MOVE_SPEED * delta_time;
    }
}