use std::sync::Arc;
use std::time::Instant;
use glam::Vec3;

use crate::physics::update_time;
use crate::graphics::{Mesh, Shader, Texture, VoxelRenderer, create_crosshair_mesh, create_wireframe_mesh, load_shader, load_texture_from_png};
use crate::loger::ProjectErrors;
use crate::core::{Events, Window, Camera};
use crate::settings::{Settings, PERMISION_TEXTURE, KEY_ESC, KEY_TAB};
use crate::mods::BlocksManager;
use std::path::{Path, PathBuf};
use crate::world::{World, WorldController};
use std::fs;
use crate::player::Player;
use crate::world::raycast;
use crate::controls::update_actions;
use crate::world::draw_world;


#[derive(PartialEq)]
pub enum GameState {
    Menu,
    WorldSelect,
    Settings,
    Playing,
}


pub struct Engine {
    pub window: Window,
    pub events: Events,
    pub settings: Settings,
    pub start_time: Instant,
    pub game_state: GameState
}


impl Engine {
    pub fn init(title: &str, width: u32, height: u32, path_to_file_settings: &str) -> Result<Self, ProjectErrors> {
        println!("инициализация окна");
        let mut window = Window::init(title, width, height)?;
        window.set_swap_interval();
        println!("инициализация окна: ок");

        println!("Инициализация обработчика событий");
        let mut events = Events::init();
        events.setting(&mut window);
        println!("Инициализация обработчика событий завершена");

        println!("настройки");
        let mut settings = Settings::load(path_to_file_settings);
        println!("настройки загружены");

        let start_time = std::time::Instant::now();
        let game_state = GameState::Menu;

        Ok(Self {
            window,
            events,
            settings,
            start_time,
            game_state
        })
    }


    pub fn calculate_ndc(&self) -> (f32, f32) {
        let (mx, my) = (self.events.x as f32, self.events.y as f32);
        let (ww, wh) = (self.window.width as f32, self.window.height as f32);
        let ndc_x = 2.0 * mx / ww - 1.0;
        let ndc_y = 1.0 - 2.0 * my / wh;

        return (ndc_x, ndc_y);
    }
}


pub struct Game {
    pub active_world: String,
    pub last_frame: Instant,
    pub shader: Shader,
    pub crosshair_shader: Shader,
    pub line_shader: Shader,
    pub blocks_manager: Arc<BlocksManager>,
    pub renderer: VoxelRenderer,
    pub world: World,
    pub camera: Camera,
    pub player: Player,
    pub world_controller: WorldController,
    pub texture: Texture,
    pub crosshair_mesh: Mesh,
    pub cube_mesh: Mesh,
    pub world_path: PathBuf
}


impl Game {
    pub fn init(engine: &mut Engine, path_worlds: &str, world_name: String, spawnpoint: Vec3, fov: f32) -> Result<Self, ProjectErrors> {
        // ПРОМЕЖУТОЧНЫЙ ЭТАП (ЗАГРУЗКА МИРА)
        if !engine.events.cursor_locked {
            engine.events.switch_cursor_mode(&mut engine.window);
        }

        println!("загрузка шейдеров");
        let shader = load_shader("res/shaders/vertex_shader.glsl", "res/shaders/fragment_shader.glsl")?;
        let crosshair_shader = load_shader("res/shaders/crosshair_vertex.glsl", "res/shaders/crosshair_fragment.glsl")?;
        let line_shader = load_shader("res/shaders/line_vertex.glsl", "res/shaders/line_fragment.glsl")?;
        println!("загрузка шейдеров завершена");

        println!("загрузка текстуры");
        let mut blocks_manager = BlocksManager::init("res/textures/atlas.png", "res/textures/blocks", PERMISION_TEXTURE)?;
        blocks_manager.build_atlas().expect("FATAL ERROR: текстурный атлас не собран");
        let blocks_manager = Arc::new(blocks_manager);
        let texture = load_texture_from_png("res/textures/atlas.png")?;
        println!("загрузка текстуры: ок");

        println!("инициализация рендер движка");
        let mut renderer = VoxelRenderer::init();
        println!("инициализация рендер движка: ок");

        println!("Создание мира");
        let worlds_dir = Path::new(path_worlds);
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
        engine.window.setting_open_gl();
        let mut last_frame = Instant::now();

        println!("инициализация камеры");
        //let spawnpoint = Vec3::new(world_info.player_position[0], world_info.player_position[1], world_info.player_position[2]);
        let mut camera = Camera::init(spawnpoint, fov);
        println!("инициализация камеры: ок");

        let mut player = Player::init(spawnpoint);

        println!("Start main loop");

        Ok( Self { 
            active_world: world_name, 
            last_frame, 
            shader, 
            crosshair_shader, 
            line_shader, 
            blocks_manager, 
            renderer,
            world,
            camera,
            player,
            world_controller,
            texture,
            crosshair_mesh,
            cube_mesh,
            world_path
        })
    }


    pub fn update(&mut self, engine: &mut Engine, range: f32) {
        while engine.game_state == GameState::Playing {
            engine.events.pull_events(&mut engine.window);

            if engine.events.j_pressed(KEY_ESC) {
                //world_controller.save_world(&mut world_info, &world, &world_path, &camera);
                engine.game_state = GameState::WorldSelect;
                break;
            } else if engine.events.j_pressed(KEY_TAB) {
                engine.events.switch_cursor_mode(&mut engine.window);
            }

            let (delta_time, now) = update_time(self.last_frame);
            self.last_frame = now;

            let hit = raycast(&self.world, self.camera.position, self.camera.front, range);
            update_actions(&mut engine.events, &mut self.camera, &mut self.world, delta_time, &mut self.renderer, &hit, &mut self.player, &self.blocks_manager);

            self.world_controller.generate_world(&self.camera, &mut self.world, &self.blocks_manager, &self.world_path);

            draw_world(&mut engine.window, &self.shader, &self.camera, &self.texture, &self.world.chunks_meshes, &self.crosshair_shader, &self.crosshair_mesh, &self.line_shader, &self.cube_mesh, &hit);
        }
    }
}