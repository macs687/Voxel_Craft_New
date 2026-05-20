use std::time::Instant;
use settings::{HEIGHT, TITLE, WIDTH, SPAWNPOINT, FOV};
use loger::ProjectErrors;
use core::{Window, Events, Camera};
use graphics::{load_shader, load_texture_from_png};
use graphics::VoxelRenderer;
use graphics::{create_crosshair_mesh, create_wireframe_mesh};
use physics::update_time;
use controls::update;
use controls::update_moving;
use world::raycast;
use world::draw_world;
use world::WorldController;
use crate::settings::RANGE;
use crate::voxels::BlockType;
use player::Player;


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

    println!("инициализация камеры");
    let mut camera = Camera::init(SPAWNPOINT, FOV);
    println!("инициализация камеры: ок");

    events.switch_cursor_mode(&mut window);

    println!("создание базовой шейдерной программы");
    let shader = load_shader("res/shaders/vertex_shader.glsl", "res/shaders/fragment_shader.glsl")?;
    let crosshair_shader = load_shader("res/shaders/crosshair_vertex.glsl", "res/shaders/crosshair_fragment.glsl")?;
    let line_shader = load_shader("res/shaders/line_vertex.glsl", "res/shaders/line_fragment.glsl")?;
    println!("создание базовой шейдерной программы завершено");

    println!("загрузка текстуры");
    let texture = load_texture_from_png("res/textures/block.png")?;
    println!("загрузка текстуры: ок");

    // ЗАГРУЗКА МИРА
    println!("инициализация рендер движка");
    let mut renderer = VoxelRenderer::init();
    println!("инициализация рендер движка: ок");

    println!("Создание мира");
    let mut world_controller = WorldController::init();
    let mut world = world_controller.create_world(&mut renderer);
    println!("Создание мира: ок");
    
    let crosshair_mesh = create_crosshair_mesh();
    let cube_mesh = create_wireframe_mesh();

    // НАСТРОЙКИ
    window.setting_open_gl();
    let mut last_frame = Instant::now();

    let mut player = Player::init(SPAWNPOINT);


    println!("Start main loop");
    while window.is_open() {
        // ОБНОВЛЕНИЕ СОБЫТИЙ
        let (delta_time, now) = update_time(last_frame);
        last_frame = now;
        events.pull_events(&mut window);

        // ИГРОВАЯ ЛОГИКА
        update(&mut events, &mut window);
        let hit = raycast(&world, camera.position, camera.front, RANGE as f32);
        update_moving(&mut events, &mut camera, &mut world, delta_time, &mut renderer, &hit, &mut player);

        // БЕСКОНЕЧНЫЙ МИР
        world_controller.generate_world(&camera, &mut world, &mut renderer);

        // РЕНДЕР МИРА
        draw_world(&mut window, &shader, &camera, &texture, &world.chunks_meshes, &crosshair_shader, &crosshair_mesh, &line_shader, &cube_mesh, &hit);
    }

    println!("Hello, world!");
    Ok(())
}