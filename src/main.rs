use std::time::Instant;
use glfw::ffi::GLFW_KEY_ESCAPE;
use settings::{HEIGHT, TITLE, WIDTH, SPAWNPOINT, FOV};
use loger::ProjectErrors;
use core::{Window, Events, Camera};
use graphics::{load_shader, load_texture_from_png};
use graphics::VoxelRenderer;
use graphics::{create_crosshair_mesh, create_wireframe_mesh};
use physics::update_time;
use controls::update_moving;
use world::raycast;
use world::draw_world;
use world::WorldController;
use crate::constant::{KEY_ESC, LCM};
use crate::mods::BlocksManager;
use crate::settings::{PERMISION_TEXTURE, RANGE};
use crate::voxels::BlockType;
use player::Player;
use ui::Button;
use graphics::create_ui_quad;
use std::sync::{Arc, mpsc};
use world::{ChunkRequest, ChunkResult};
use world::chunk_loader_thread;



mod mods;
mod assets;
mod constant;
mod settings;
mod loger;
mod core;
mod graphics;
mod world;
mod controls;
mod physics;
mod voxels;
mod player;
mod ui;


#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
}


fn main() -> Result<(), ProjectErrors> {
    // ЗАГРУЗКА РЕСУРСОВ ЯДРА
    println!("инициализация окна");
    let mut window = Window::init(TITLE, WIDTH, HEIGHT)?;
    window.set_swap_interval();
    println!("инициализация окна: ок");

    println!("Инициализация обработчика событий");
    let mut events = Events::init();
    events.setting(&mut window);
    println!("Инициализация обработчика событий завершена");

    println!("инициализация менеджера блоков");
    let mut blocks_manager = BlocksManager::init("res/textures/atlas.png", "res/textures/blocks", PERMISION_TEXTURE)?;
    blocks_manager.build_atlas();
    let blocks_manager = Arc::new(blocks_manager);
    println!("инициализация менеджера блоков завершена");

    println!("загрузка шейдеров");
    let shader = load_shader("res/shaders/vertex_shader.glsl", "res/shaders/fragment_shader.glsl")?;
    let crosshair_shader = load_shader("res/shaders/crosshair_vertex.glsl", "res/shaders/crosshair_fragment.glsl")?;
    let line_shader = load_shader("res/shaders/line_vertex.glsl", "res/shaders/line_fragment.glsl")?;
    let ui_shader = load_shader("res/shaders/ui_vertex.glsl", "res/shaders/ui_fragment.glsl")?;
    println!("загрузка шейдеров завершена");

    println!("загрузка меню");
    let mut game_state = GameState::Menu;
    let ui_quad_vao = create_ui_quad();

    let button_play = Button::new("Play", 0.0, 0.4, 0.3, 0.1)?;
    let button_settings = Button::new("Settings", 0.0, 0.0, 0.3, 0.1)?;
    let button_exit = Button::new("Exit", 0.0, -0.4, 0.3, 0.1)?;

    println!("Вход в меню");
    while window.is_open() {

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            //gl::Disable(gl::CULL_FACE);
        }

        // ЦИКЛ МЕНЮ
        while game_state == GameState::Menu {
            events.pull_events(&mut window);

            if events.cursor_locked {
                events.switch_cursor_mode(&mut window);
            }

            if events.j_pressed(KEY_ESC) {
                window.close();
                break;
            }

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            button_play.draw_button(&ui_shader, ui_quad_vao);
            button_settings.draw_button(&ui_shader, ui_quad_vao);
            button_exit.draw_button(&ui_shader, ui_quad_vao);

            if events.j_clicked(LCM) {
                let (mx, my) = (events.x as f32, events.y as f32);
                let (ww, wh) = (window.width as f32, window.height as f32);
                let ndc_x = 2.0 * mx / ww - 1.0;
                let ndc_y = 1.0 - 2.0 * my / wh;

                if button_play.contains(ndc_x, ndc_y) {
                    game_state = GameState::Playing;
                    break
                } else if button_exit.contains(ndc_x, ndc_y) {
                    window.close();
                    break
                }
            }
            window.swap_buffers();
        }


        // ПРОМЕЖУТОЧНЫЙ ЭТАП (ЗАГРУЗКА МИРА)
        events.switch_cursor_mode(&mut window);

        println!("загрузка текстуры");
        let texture = load_texture_from_png("res/textures/atlas.png")?;
        println!("загрузка текстуры: ок");

        println!("инициализация рендер движка");
        let mut renderer = VoxelRenderer::init();
        println!("инициализация рендер движка: ок");

        println!("Создание мира");
        let mut world_controller = WorldController::init();
        let mut world = world_controller.create_world(&mut renderer, &blocks_manager);
        println!("Создание мира: ок");

        let crosshair_mesh = create_crosshair_mesh();
        let cube_mesh = create_wireframe_mesh();

        // НАСТРОЙКИ
        window.setting_open_gl();
        let mut last_frame = Instant::now();

        println!("инициализация камеры");
        let mut camera = Camera::init(SPAWNPOINT, FOV);
        println!("инициализация камеры: ок");

        let mut player = Player::init(SPAWNPOINT);

        println!("Start main loop");
        while game_state == GameState::Playing {
            events.pull_events(&mut window);

            if events.j_pressed(KEY_ESC) {
                game_state = GameState::Menu;
                break;
            }

            let (delta_time, now) = update_time(last_frame);
            last_frame = now;

            let hit = raycast(&world, camera.position, camera.front, RANGE as f32);
            update_moving(&mut events, &mut camera, &mut world, delta_time, &mut renderer, &hit, &mut player, &blocks_manager);

            world_controller.generate_world(&camera, &mut world, &blocks_manager, &blocks_manager);

            draw_world(&mut window, &shader, &camera, &texture, &world.chunks_meshes, &crosshair_shader, &crosshair_mesh, &line_shader, &cube_mesh, &hit);
        }
    }

    println!("Stop Engine");
    Ok(())
}