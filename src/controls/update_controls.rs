use crate::GameState;
use crate::core::{Window, Events, Camera};
use crate::settings::{KEY_A, KEY_D, KEY_ESC, KEY_LEFT_SHIFT, KEY_S, KEY_SPACE, KEY_TAB, KEY_W, KEY_LCM, KEY_PCM, KEY_F1};
use crate::mods::BlocksManager;
use crate::player::Player;
use crate::settings::{BLOCK_TYPES_NUMBER, CREATIVE_VERTICAL_MOVE, JUMP_FORCE, MOUSE_SENSITIVITY, MOVE_SPEED};
use glam::{Vec3, Quat};
use glfw::Key;
use crate::world::{RayHit, World};
use crate::voxels::BlockType;
use crate::graphics::{VoxelRenderer};



pub fn update_actions(events: &mut Events, camera: &mut Camera, world: &mut World, delta_time: f32, renderer: &mut VoxelRenderer, hit: &Option<RayHit>, player: &mut Player, blocks_manager: &BlocksManager) {
    let mut direction = Vec3::ZERO;
    //camera.rotation = Quat::IDENTITY;



    // СЛУЖЕБНЫЕ СИСТЕМЫ (ИНВЕНТАРЬ)
    if events.j_pressed(Key::F1 as i32) { player.selected_block = "grass".to_string(); }
    if events.j_pressed(Key::F2 as i32) { player.selected_block = "dirt".to_string(); }
    if events.j_pressed(Key::F3 as i32) { player.selected_block = "planks".to_string(); }
    // if events.j_pressed(Key::F4 as i32) { player.selected_block_id = 4; }
    // if events.j_pressed(Key::F5 as i32) { player.selected_block_id = 5; }
    // if events.j_pressed(Key::F6 as i32) { player.selected_block_id = 6; }


    // МЫШКА
    if events.cursor_locked {
        let mut pitch_delta = events.delta_y * MOUSE_SENSITIVITY;
        let yaw_delta = -events.delta_x * MOUSE_SENSITIVITY;

        //println!("Мышь x {}, мышь y {}", yaw_delta, pitch_delta);

        // if pitch_delta < -90.0_f32.to_radians() {
        //     pitch_delta = -90.0_f32.to_radians();
        // } 
        
        if pitch_delta > 89.0_f32.to_radians() {
            pitch_delta = 89.0_f32.to_radians();
        }

        // camera.rotation = Quat::IDENTITY;
        camera.rotate(-pitch_delta, yaw_delta, 0.0);
    }


    // ДВИЖЕНИЕ
    if events.pressed(KEY_W) { direction += camera.front; }
    if events.pressed(KEY_S) { direction -= camera.front; }
    if events.pressed(KEY_A) { direction -= camera.right; }
    if events.pressed(KEY_D) { direction += camera.right; }

    direction.y = 0.0;

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    if events.j_pressed(Key::F as i32) {
        player.fly = !player.fly;
    }

    if events.pressed(KEY_SPACE) {
        if player.fly {
            player.position += Vec3::new(0.0, CREATIVE_VERTICAL_MOVE, 0.0);
        } else {
            player.velocity.y = JUMP_FORCE;
        }
    } else if events.pressed(KEY_LEFT_SHIFT) {
        if player.fly {
            player.position += Vec3::new(0.0, -CREATIVE_VERTICAL_MOVE, 0.0);
        } else {
            // включить приседание
        }
    }




    let jump = events.j_pressed(KEY_SPACE);
    //println!("{}", player.on_ground);
    let cam_pos = player.update_moving(world, direction, jump, delta_time);
    camera.position = cam_pos;


    if let Some(hit ) = hit {
        if events.j_clicked(KEY_LCM) {
            //println!("Нажата ЛКМ");
            world.set_block(hit.block_pos.0 as i32, hit.block_pos.1 as i32, hit.block_pos.2 as i32, BlockType::Air);
            world.update(hit.block_pos, renderer, blocks_manager);
            //println!("обноваление мира");
        } else if events.j_clicked(KEY_PCM) {
            //println!("ПКМ");
            let nx = hit.block_pos.0 + hit.normal.0;
            let ny = hit.block_pos.1 + hit.normal.1;
            let nz = hit.block_pos.2 + hit.normal.2;
            //println!("Sum: {} + {} = {}", hit.block_pos.2, hit.normal.2, nz);
            world.set_block_by_name(nx, ny, nz, &player.selected_block, blocks_manager);
            world.update(hit.block_pos, renderer, blocks_manager);
        }
    }


    // if events.pressed(KEY_W) {
    //     direction += camera.front;
    // } else if events.pressed(KEY_A) {
    //     direction -= camera.right;
    // } else if events.pressed(KEY_S) {
    //     direction -= camera.front;
    // } else if events.pressed(KEY_D) {
    //     direction += camera.right;
    // } else if events.pressed(KEY_SPACE) {
    //     direction += camera.up
    // } else if events.pressed(KEY_LEFT_SHIFT) {
    //     direction -= camera.up;
    // }
}