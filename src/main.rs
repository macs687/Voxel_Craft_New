use core::{Window, Events, Camera};
use std::collections::HashMap;
use std::time::Instant;
use glam::Vec3;
use glfw::Key::*;
use crate::constant::*;
use crate::loger::ProjectErrors;
use graphics::load_shader;
use graphics::VoxelRenderer;
use graphics::load_texture_from_png;
use settings::{MOUSE_SENSITIVITY};
use world::draw_world;
use graphics::create_crosshair_mesh;
use graphics::create_wireframe_mesh;
use world::raycast;
use world::World;
use world::ChunkCoord;
use graphics::Mesh;
use crate::world::rebuild_affected_meshes;

mod world;
mod voxels;
mod constant;
mod loger;
mod core;
mod graphics;
mod settings;

fn main() -> Result<(), ProjectErrors> {
    // ЗАГРУЗКА РЕСУРСОВ ЯДРА

    println!("инициализация окна");
    let mut window = Window::init("Voxel Craft", 1920, 1080)?;
    window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    println!("Инициализация обработчика событий");
    let mut events = Events::init();
    events.setting(&mut window);
    println!("Инициализация обработчика событий завершена");

    println!("инициализация камеры");
    let mut camera = Camera::init(Vec3::new(0.0, 0.0, -10.0), 40.0_f32.to_radians());
    println!("инициализация камеры: ок");

    events.switch_cursor_mode(&mut window);

    println!("создание базовой шейдерной программы");
    let shader = load_shader("res/shaders/vertex_shader.glsl", "res/shaders/fragment_shader.glsl")?;
    let crosshair_shader = load_shader("res/shaders/crosshair_vertex.glsl", "res/shaders/crosshair_fragment.glsl")?;
    let line_shader = load_shader("res/shaders/line_vertex.glsl", "res/shaders/line_fragment.glsl")?;
    println!("создание базовой шейдерной программы завершено");

    println!("загрузка текстуры");
    let mut texture = load_texture_from_png("res/textures/planks.jpg")?;
    println!("загрузка текстуры: ок");


    // ЗАГРУЗКА МИРА
    println!("инициализация рендер движка");
    let mut renderer = VoxelRenderer::init();
    println!("инициализация рендер движка: ок");

    println!("Создание мира");
    let mut world = World::init();
    world.generate_start_landscape();

    let len = world.chunks.len();

    println!("{len}");
    println!("Создание мира: ок");

    let mut chunk_meshes: HashMap<ChunkCoord, Mesh> = HashMap::new();
    for (&(cx, cy, cz), chunk) in &world.chunks {
        let mesh = renderer.render(chunk, cx, cy, cz, &world);
        chunk_meshes.insert((cx, cy, cz), Mesh {
            vao: mesh.vao,
            vbo: mesh.vbo,
            vertex_count: mesh.vertex_count
        });
    }

    
    let crosshair_mesh = create_crosshair_mesh();
    let cube_mesh = create_wireframe_mesh();

    // НАСТРОЙКИ
    window.setting_openGL();
    let mut last_frame = Instant::now();

    println!("Start main loop");
    while window.is_open() {
        let now = Instant::now();
        let delta_time = (now - last_frame).as_secs_f32();
        last_frame = now;
        let delta_time = delta_time.min(0.05);

        //println!("дельта {delta_time}");

        // прослушивание всех устройств и обработка событий 
        events.pull_events(&mut window);

        if events.j_clicked(LCM) {
            //println!("ЛКМ нажата");
            window.gl_clear_color(0.3, 0.4, 0.5, 0.6);
        } else if events.j_pressed(KEY_TAB) && events.cursor_in_window {
            events.switch_cursor_mode(&mut window);
        }else if events.j_pressed(Escape as i32) {
            window.close(); 
        }

        // игровая логика
        let mut direction = Vec3::ZERO;

        let pitch_delta = events.delta_y * MOUSE_SENSITIVITY;
        let yaw_delta = -events.delta_x * MOUSE_SENSITIVITY;

        if events.cursor_locked {
            camera.rotate(-pitch_delta, yaw_delta, 0.0);
        }



        //println!("front: {:?}, right: {:?}", camera.front, camera.right);
        let hit = raycast(&world, camera.position, camera.front, 8.0);

        if let Some(ref hit ) = hit {
            //let hitbox = Some(hit).unwrap();

            if events.j_clicked(LCM) {
                world.set_block(hit.block_pos.0 as i32, hit.block_pos.1 as i32, hit.block_pos.2 as i32, voxels::BlockType::Air);
                rebuild_affected_meshes(&mut world, &mut chunk_meshes, hit.block_pos, &mut renderer);
            }

            if events.j_clicked(PCM) {
                world.set_block(hit.block_pos.0 as i32, hit.block_pos.1 as i32, hit.block_pos.2 as i32, voxels::BlockType::Planks);
                rebuild_affected_meshes(&mut world, &mut chunk_meshes, hit.block_pos, &mut renderer);
            }
        }


        if events.pressed(KEY_W) {
            direction += camera.front;
            // println!("W нажата");
            // println!("W pressed, direction = {:?}", direction);
        }
        if events.pressed(KEY_S) {
            direction -= camera.front; // назад
            // println!("S нажата");
            // println!("S pressed, direction = {:?}", direction);
        }
        if events.pressed(KEY_A) {
            // println!("A нажата");
            direction -= camera.right; // влево
            // println!("A pressed, direction = {:?}", direction);
        }
        if events.pressed(KEY_D) {
            direction += camera.right; // вправо
                // println!("D нажата");
                // println!("D pressed, direction = {:?}", direction);
        }
        if events.pressed(KEY_SPACE) {
            direction += Vec3::Y; // вверх (мировая ось)
        }
        if events.pressed(KEY_LEFT_SHIFT) {
            direction -= Vec3::Y; // вниз
        }

        //println!("final direction: {:?}", direction);

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            camera.position += direction * MOVE_SPEED * delta_time;
        }


        // РЕНДЕР МИРА
        draw_world(&mut window, &shader, &camera, &texture, &chunk_meshes, &crosshair_shader, &crosshair_mesh, &line_shader, &cube_mesh, &hit);
    }

    println!("Hello, world!");
    Ok(())
}