use crate::core::{Window, Events, Camera};
use crate::constant::{KEY_A, KEY_D, KEY_ESC, KEY_LEFT_SHIFT, KEY_S, KEY_SPACE, KEY_TAB, KEY_W, LCM, PCM, KEY_F1};
use crate::settings::{MOUSE_SENSITIVITY, MOVE_SPEED, BLOCK_TYPES_NUMBER};
use glam::Vec3;
use glfw::Key;
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



pub fn update_moving(events: &mut Events, camera: &mut Camera, world: &mut World, delta_time: f32, renderer: &mut VoxelRenderer, hit: &Option<RayHit>, mut selected_block: BlockType) {
    let mut direction = Vec3::ZERO;
    let pitch_delta = events.delta_y * MOUSE_SENSITIVITY;
    let yaw_delta = -events.delta_x * MOUSE_SENSITIVITY;

    if events.cursor_locked {
        camera.rotate(-pitch_delta, yaw_delta, 0.0);
    }

    //let mut selected_block_id = *selected_block as i32;

    if events.j_pressed(Key::F1 as i32) { selected_block = BlockType::Dirt; }
    if events.j_pressed(Key::F2 as i32) { selected_block = BlockType::Planks; }
    if events.j_pressed(Key::F3 as i32) { selected_block = BlockType::Grass; }
    if events.j_pressed(Key::F4 as i32) { selected_block = BlockType::Stone; }
    if events.j_pressed(Key::F5 as i32) { selected_block = BlockType::Sand; }
    if events.j_pressed(Key::F6 as i32) { selected_block = BlockType::Wood; }

    if let Some(hit ) = hit {
        if events.j_clicked(LCM) {
            println!("Нажата ЛКМ");
            world.set_block(hit.block_pos.0 as i32, hit.block_pos.1 as i32, hit.block_pos.2 as i32, BlockType::Air);
            world.update(hit.block_pos, renderer);
            //println!("обноваление мира");
        } else if events.j_clicked(PCM) {
            println!("ПКМ");
            let nx = hit.block_pos.0 + hit.normal.0;
            let ny = hit.block_pos.1 + hit.normal.1;
            let nz = hit.block_pos.2 + hit.normal.2;
            //println!("Sum: {} + {} = {}", hit.block_pos.2, hit.normal.2, nz);
            world.set_block(nx, ny, nz, selected_block);
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