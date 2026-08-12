use settings::{HEIGHT, TITLE, WIDTH, FOV, SPAWNPOINT, PATH_SETTINGS, RAYCAST_DIST};
use loger::ProjectErrors;
use engine::{GameState, Engine, Game};
use menu::Menu;

mod loger;
mod menu;
mod engine;



mod files;
mod mods;
mod assets;
mod settings;

mod core;
mod graphics;
mod world;
mod controls;
mod physics;
mod voxels;
mod player;
mod ui;


fn main() -> Result<(), ProjectErrors> {
    println!("Start Engine");
    let mut engine = Engine::init(TITLE, WIDTH, HEIGHT, PATH_SETTINGS)?;
    println!("engine: ok");

    println!("start menu");
    let mut menu = Menu::init("res/shaders/background_vertex.glsl", "res/shaders/background_fragment.glsl", "res/textures/fone.png")?;
    println!("menu: ok");

    println!("Вход в меню");
    while engine.window.is_open() {
        let elapsed = engine.start_time.elapsed().as_secs_f32();
        let mut world_name = "".to_string();

        if engine.game_state == GameState::Menu {
            menu.update_main_menu(&mut engine, elapsed);
        } else if engine.game_state == GameState::Settings {
            menu.update_setting(&mut engine, PATH_SETTINGS);
        } else if engine.game_state == GameState::WorldSelect {
            world_name = menu.update_world_select(&mut engine)?;
        } else if engine.game_state == GameState::Playing {
            let mut game = Game::init(&mut engine, "res/worlds", world_name, SPAWNPOINT, FOV)?;

            // println!("Позиция игрока при загрузке: x{} y{} z{}", world_info.player_position[0], world_info.player_position[1], world_info.player_position[2]);
            game.update(&mut engine, RAYCAST_DIST);
        }
    }

    println!("Stop Engine");
    Ok(())
}