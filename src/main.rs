use std::time::Instant;
// use glam::Vec3;
use glfw::Key;
// use glfw::ffi::GLFW_KEY_ESCAPE;
use settings::{HEIGHT, TITLE, WIDTH, SPAWNPOINT, FOV};
use loger::ProjectErrors;
use core::{Window, Events, Camera};
use graphics::{load_shader, load_texture_from_png};
use graphics::VoxelRenderer;
use graphics::{create_crosshair_mesh, create_wireframe_mesh};
use physics::update_time;
use controls::update_actions;
use world::raycast;
use world::draw_world;
use world::WorldController;
use crate::constant::{KEY_ESC, KEY_TAB, LCM};
use crate::mods::BlocksManager;
use crate::settings::{PERMISION_TEXTURE, RANGE};
use crate::voxels::BlockType;
use player::Player;
use ui::Button;
use graphics::create_ui_quad;
use std::sync::{Arc};
// use world::{ChunkRequest, ChunkResult};
// use world::chunk_loader_thread;
use crate::settings::Settings;
use std::path::Path;
// use world::save_world_info;
// use world::WorldInfo;
// use world::load_world_info;
use rand;
use std::fs;
use std::io;
use ui::draw_background;
use mods::MenuConfig;


mod files;
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
    WorldSelect,
    Settings,
    Playing,
}


fn list_worlds() -> Vec<String> {
    let worlds_dir = Path::new("res/worlds");
    if !worlds_dir.exists() {
        return Vec::new();
    }
    let mut worlds = Vec::new();
    if let Ok(entries) = std::fs::read_dir(worlds_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Проверяем, что внутри есть world.toml
                if path.join("world.toml").exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        worlds.push(name.to_string());
                    }
                }
            }
        }
    }
    worlds.sort(); // для стабильного порядка
    worlds
}


fn main() -> Result<(), ProjectErrors> {
    println!("Start Engine");

    // ЗАГРУЗКА РЕСУРСОВ ЯДРА
    println!("инициализация окна");
    let mut window = Window::init(TITLE, WIDTH, HEIGHT)?;
    window.set_swap_interval();
    println!("инициализация окна: ок");

    println!("Инициализация обработчика событий");
    let mut events = Events::init();
    events.setting(&mut window);
    println!("Инициализация обработчика событий завершена");

    println!("настройки");
    let mut settings = Settings::load();
    println!("настройки загружены");


    println!("инициализация менеджера блоков");
    let mut blocks_manager = BlocksManager::init("res/textures/atlas.png", "res/textures/blocks", PERMISION_TEXTURE)?;
    blocks_manager.build_atlas().expect("FATAL ERROR: текстурный атлас не собран");
    let blocks_manager = Arc::new(blocks_manager);
    println!("инициализация менеджера блоков завершена");

    println!("загрузка шейдеров меню");
    let ui_shader = load_shader("res/shaders/ui_vertex.glsl", "res/shaders/ui_fragment.glsl")?;
    println!("загрузка шейдеров завершена");

    println!("загрузка меню");
    let mut game_state = GameState::Menu;
    let ui_quad_vao = create_ui_quad();
    

    let button_worlds = Button::new("Worlds", 0.0, 0.4, 0.3, 0.1)?;
    let button_settings = Button::new("Settings", 0.0, 0.0, 0.3, 0.1)?;
    let button_exit = Button::new("Exit", 0.0, -0.4, 0.3, 0.1)?;

    //let texture = load_texture_from_png("res/textures/planks.png")?;
    //println!("Texture ID: {}", texture.id);
    let background_shader = load_shader("res/shaders/background_vertex.glsl", "res/shaders/background_fragment.glsl")?;
    let fullscreen_quad_vao = create_ui_quad();

    // Кнопки меню настроек
    let button_back = Button::new("Back", 0.0, -0.4, 0.3, 0.1)?;
    let button_sens_up = Button::new("Sens+", -0.2, 0.2, 0.2, 0.1)?;
    let button_sens_down = Button::new("Sens-", 0.2, 0.2, 0.2, 0.1)?;
    // let button_vol_up = Button::new("Vol+", -0.2, -0.1, 0.2, 0.1)?;
    // let button_vol_down = Button::new("Vol-", 0.2, -0.1, 0.2, 0.1)?;

    let mut active_world= String::new();


    let start_time = std::time::Instant::now();

    let menu_config = MenuConfig::load("res/menu_config.toml");
    let menu_background_texture = load_texture_from_png(&menu_config.background_texture)?;

    println!("Вход в меню");
    while window.is_open() {
        let elapsed = start_time.elapsed().as_secs_f32();

        // ЦИКЛ МЕНЮ
        while game_state == GameState::Menu {
            events.pull_events(&mut window);

            if events.cursor_locked {
                events.switch_cursor_mode(&mut window);
            }

            if events.j_pressed(KEY_ESC) {
                window.close();
                break;
            } else if events.j_pressed(Key::F11 as i32) {
                //window.switch_window_mode();
            }

            draw_background(&background_shader, &menu_background_texture, ui_quad_vao, elapsed);

            button_worlds.draw_button(&ui_shader, ui_quad_vao);
            button_settings.draw_button(&ui_shader, ui_quad_vao);
            button_exit.draw_button(&ui_shader, ui_quad_vao);

            if events.j_clicked(LCM) {
                println!("лкм нажата");
                let (mx, my) = (events.x as f32, events.y as f32);
                let (ww, wh) = (window.width as f32, window.height as f32);
                let ndc_x = 2.0 * mx / ww - 1.0;
                let ndc_y = 1.0 - 2.0 * my / wh;

                if button_worlds.contains(ndc_x, ndc_y) {
                    game_state = GameState::WorldSelect;
                    break
                } else if button_settings.contains(ndc_x, ndc_y) {
                    game_state = GameState::Settings;
                    break
                } else if button_exit.contains(ndc_x, ndc_y) {
                    println!("Выход из игры");
                    window.close();
                    break
                }
            }

            window.swap_buffers();
        }


        // ЦИКЛ НАСТРОЕК
        while game_state == GameState::Settings {
            events.pull_events(&mut window);

            if events.cursor_locked {
                events.switch_cursor_mode(&mut window);
            }

            if events.j_pressed(KEY_ESC) {
                game_state = GameState::Menu;
                break;
            }

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            button_back.draw_button(&ui_shader, ui_quad_vao);
            button_sens_up.draw_button(&ui_shader, ui_quad_vao);
            button_sens_down.draw_button(&ui_shader, ui_quad_vao);

            if events.j_clicked(LCM) {
                println!("нажата лкм");
                let (mx, my) = (events.x as f32, events.y as f32);
                let (ww, wh) = (window.width as f32, window.height as f32);
                let ndc_x = 2.0 * mx / ww - 1.0;
                let ndc_y = 1.0 - 2.0 * my / wh;


                if button_back.contains(ndc_x, ndc_y) {
                    game_state = GameState::Menu;
                    break;
                } else if button_sens_up.contains(ndc_x, ndc_y) {
                    settings.mouse_sensitivity = (settings.mouse_sensitivity + 0.05).min(2.0);
                    println!("чувствительность мыши {}", settings.mouse_sensitivity);
                    settings.save();
                } else if button_sens_down.contains(ndc_x, ndc_y) {
                    settings.mouse_sensitivity = (settings.mouse_sensitivity - 0.05).max(0.01);
                    println!("чувствительность мыши {}", settings.mouse_sensitivity);
                    settings.save();
                }
            }

            window.swap_buffers();
        }


        // Цикл выборки мира 
        while game_state == GameState::WorldSelect {
            //println!("Вход в меню выбора мира");
            events.pull_events(&mut window);

            if events.cursor_locked {
                events.switch_cursor_mode(&mut window);
            }

            if events.j_pressed(KEY_ESC) {
                game_state = GameState::Menu;
                break;
            }

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            let new_world_button = Button::new("Create New World", 0.0, -0.7, 0.5, 0.1)?;
            new_world_button.draw_button(&ui_shader, ui_quad_vao);

            let mut worlds = list_worlds();
            let mut world_buttons = Vec::new();

            if !worlds.is_empty() {
                for (i, world_name) in worlds.iter().enumerate() {
                    let btn = Button::new(world_name, 0.0, 0.4 - i as f32 * 0.15, 0.5, 0.1)?;
                    btn.draw_button(&ui_shader, ui_quad_vao);
                    world_buttons.push(btn);
                }

                if events.j_clicked(LCM) {
                    let (mx, my) = (events.x as f32, events.y as f32);
                    let (ww, wh) = (window.width as f32, window.height as f32);
                    let ndc_x = 2.0 * mx / ww - 1.0;
                    let ndc_y = 1.0 - 2.0 * my / wh;

                    if new_world_button.contains(ndc_x, ndc_y) {
                        println!("Создание нового мира");
                        
                        let mut world_name = String::new();
                        println!("Введите имя мира:");
                        io::stdin().read_line(&mut world_name).expect("Failed to read line");
                        let world_name = world_name.trim();
                        if world_name.is_empty() {
                            println!("Имя мира не может быть пустым");
                        } else if world_buttons.iter().any(|b| b.text == world_name) {
                            println!("Мир с таким именем уже существует");
                        } else {
                            active_world = world_name.to_string();
                            println!("Выбран мир '{}'", &active_world.clone());
                            game_state = GameState::Playing;
                            break;
                        }
                    }

                    for (i, btn) in world_buttons.iter().enumerate() {
                        if btn.contains(ndc_x, ndc_y) {
                            active_world = worlds[i].clone();
                            println!("Выбран мир '{}'", &active_world.clone());
                            game_state = GameState::Playing;
                            break;
                        }
                    }        
                }
            } else {
                println!("Нет миров для отображения");

                if events.j_clicked(LCM) {
                    let (mx, my) = (events.x as f32, events.y as f32);
                    let (ww, wh) = (window.width as f32, window.height as f32);
                    let ndc_x = 2.0 * mx / ww - 1.0;
                    let ndc_y = 1.0 - 2.0 * my / wh;

                    if new_world_button.contains(ndc_x, ndc_y) {
                        println!("Создание нового мира");
                        
                        let mut world_name = String::new();
                        println!("Введите имя мира:");
                        io::stdin().read_line(&mut world_name).expect("Failed to read line");
                        let world_name = world_name.trim();
                        if world_name.is_empty() {
                            println!("Имя мира не может быть пустым");
                        } else if world_buttons.iter().any(|b| b.text == world_name) {
                            println!("Мир с таким именем уже существует");
                        } else {
                            active_world = world_name.to_string();
                            println!("Выбран мир '{}'", &active_world.clone());
                            game_state = GameState::Playing;
                            break;
                        }
                    }      
                }
            }

            window.swap_buffers();
        }


        if game_state == GameState::Playing {
            // ПРОМЕЖУТОЧНЫЙ ЭТАП (ЗАГРУЗКА МИРА)
            if !events.cursor_locked {
                events.switch_cursor_mode(&mut window);
            }

            println!("загрузка шейдеров");
            let shader = load_shader("res/shaders/vertex_shader.glsl", "res/shaders/fragment_shader.glsl")?;
            let crosshair_shader = load_shader("res/shaders/crosshair_vertex.glsl", "res/shaders/crosshair_fragment.glsl")?;
            let line_shader = load_shader("res/shaders/line_vertex.glsl", "res/shaders/line_fragment.glsl")?;
            println!("загрузка шейдеров завершена");

            println!("загрузка текстуры");
            let texture = load_texture_from_png("res/textures/atlas.png")?;
            println!("загрузка текстуры: ок");

            println!("инициализация рендер движка");
            let mut renderer = VoxelRenderer::init();
            println!("инициализация рендер движка: ок");

            println!("Создание мира");
            let world_name = active_world.clone();
            let worlds_dir = Path::new("res/worlds");
            let world_path = worlds_dir.join(&world_name);
            let mut world_controller = WorldController::init();
            let mut world;
            // let mut world_info;

            if world_path.exists() {
                //world_info = load_world_info(&world_path).expect("Failed to load world info");
                world = world_controller.create_world(&mut renderer, &blocks_manager);

                //println!("Loaded world '{}' with seed {}", world_name, world_info.seed);
            } else {
                let seed = rand::random::<u32>();
                //world_info = WorldInfo { name: world_name.clone(), seed, player_position: SPAWNPOINT.into() };

                fs::create_dir_all(&world_path).expect("Failed to create world directory");
                //save_world_info(&world_path, &world_info).expect("Failed to save world info");

                world = world_controller.create_world(&mut renderer, &blocks_manager);
            }

            let crosshair_mesh = create_crosshair_mesh();
            let cube_mesh = create_wireframe_mesh();

            // НАСТРОЙКИ
            window.setting_open_gl();
            let mut last_frame = Instant::now();

            println!("инициализация камеры");
            //let spawnpoint = Vec3::new(world_info.player_position[0], world_info.player_position[1], world_info.player_position[2]);
            let mut camera = Camera::init(SPAWNPOINT, FOV);
            println!("инициализация камеры: ок");

            let mut player = Player::init(SPAWNPOINT);

            println!("Start main loop");

            // println!("Позиция игрока при загрузке: x{} y{} z{}", world_info.player_position[0], world_info.player_position[1], world_info.player_position[2]);
            while game_state == GameState::Playing {
                events.pull_events(&mut window);

                if events.j_pressed(KEY_ESC) {
                    //world_controller.save_world(&mut world_info, &world, &world_path, &camera);
                    game_state = GameState::WorldSelect;
                    break;
                } else if events.j_pressed(KEY_TAB) {
                    events.switch_cursor_mode(&mut window);
                }

                let (delta_time, now) = update_time(last_frame);
                last_frame = now;

                let hit = raycast(&world, camera.position, camera.front, RANGE as f32);
                update_actions(&mut events, &mut camera, &mut world, delta_time, &mut renderer, &hit, &mut player, &blocks_manager);

                world_controller.generate_world(&camera, &mut world, &blocks_manager, &world_path);

                draw_world(&mut window, &shader, &camera, &texture, &world.chunks_meshes, &crosshair_shader, &crosshair_mesh, &line_shader, &cube_mesh, &hit);
            }
        }
    }

    println!("Stop Engine");
    Ok(())
}