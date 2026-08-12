use crate::engine::{Engine, GameState};
use crate::graphics::{Shader, create_ui_quad, load_shader, load_texture_from_png};
use crate::{loger::ProjectErrors, menu::fone::Background, ui::Button};
use crate::settings::{KEY_ESC, KEY_F11, KEY_LCM};
use gl::types::GLuint;
use std::path::Path;
use std::io;
use crate::mods::MenuConfig;

mod fone;


pub struct Menu {
    background: Background,
    ui_shader: Shader,
    ui_vao: GLuint,
    button_worlds: Button,
    button_settings: Button,
    button_exit: Button,
    button_back: Button,
    button_sens_up: Button,
    button_sens_down: Button,
    new_world_button: Button
}


impl Menu {
    pub fn init(menu_vertex_shader_file: &str, menu_fragment_shader_file: &str, menu_texture: &str) -> Result<Self, ProjectErrors> {
        let background = Background::init(menu_vertex_shader_file, menu_fragment_shader_file, menu_texture)?;

        let button_worlds = Button::new("Worlds", 0.0, 0.4, 0.3, 0.1)?;
        let button_settings = Button::new("Settings", 0.0, 0.0, 0.3, 0.1)?;
        let button_exit = Button::new("Exit", 0.0, -0.4, 0.3, 0.1)?;

        let ui_shader = load_shader("res/shaders/ui_vertex.glsl", "res/shaders/ui_fragment.glsl")?;
        let ui_vao = create_ui_quad();

        let button_back = Button::new("Back", 0.0, -0.4, 0.3, 0.1)?;
        let button_sens_up = Button::new("Sens+", -0.2, 0.2, 0.2, 0.1)?;
        let button_sens_down = Button::new("Sens-", 0.2, 0.2, 0.2, 0.1)?;

        let new_world_button = Button::new("Create New World", 0.0, -0.7, 0.5, 0.1)?;

        let menu_config = MenuConfig::load("res/menu_config.toml");
        let menu_background_texture = load_texture_from_png(&menu_config.background_texture)?;

        Ok(Self { 
            background,
            button_worlds, 
            button_settings, 
            button_exit,
            ui_shader,
            ui_vao,
            button_back,
            button_sens_up,
            button_sens_down,
            new_world_button
        })
    }


    pub fn update_main_menu(&mut self, engine: &mut Engine, elapsed: f32) {
        while engine.game_state == GameState::Menu {
            engine.events.pull_events(&mut engine.window);

            if engine.events.cursor_locked {
                engine.events.switch_cursor_mode(&mut engine.window);
            }

            if engine.events.j_pressed(KEY_ESC) {
                engine.window.close();
                break;
            } else if engine.events.j_pressed(KEY_F11) {
                //window.switch_window_mode();
            }

            self.background.draw(elapsed);

            self.button_worlds.draw_button(&self.ui_shader, self.ui_vao);
            self.button_settings.draw_button(&self.ui_shader, self.ui_vao);
            self.button_exit.draw_button(&self.ui_shader, self.ui_vao);

            if engine.events.j_clicked(KEY_LCM) {
                println!("лкм нажата");
                let (ndc_x, ndc_y) = engine.calculate_ndc();

                if self.button_worlds.contains(ndc_x, ndc_y) {
                    engine.game_state = GameState::WorldSelect;
                    break
                } else if self.button_settings.contains(ndc_x, ndc_y) {
                    engine.game_state = GameState::Settings;
                    break
                } else if self.button_exit.contains(ndc_x, ndc_y) {
                    println!("Выход из игры");
                    engine.window.close();
                    break
                }
            }

            engine.window.swap_buffers();
        }
    }


    pub fn update_setting(&self, engine: &mut Engine, path: &str) {
        while engine.game_state == GameState::Settings {
            engine.events.pull_events(&mut engine.window);

            if engine.events.cursor_locked {
                engine.events.switch_cursor_mode(&mut engine.window);
            }

            if engine.events.j_pressed(KEY_ESC) {
                engine.game_state = GameState::Menu;
                break;
            }

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.button_back.draw_button(&self.ui_shader, self.ui_vao);
            self.button_sens_up.draw_button(&self.ui_shader, self.ui_vao);
            self.button_sens_down.draw_button(&self.ui_shader, self.ui_vao);

            if engine.events.j_clicked(KEY_LCM) {
                let (ndc_x, ndc_y) = engine.calculate_ndc();

                if self.button_back.contains(ndc_x, ndc_y) {
                    engine.game_state = GameState::Menu;
                    break;
                } else if self.button_sens_up.contains(ndc_x, ndc_y) {
                    engine.settings.mouse_sensitivity = (engine.settings.mouse_sensitivity + 0.05).min(2.0);
                    println!("чувствительность мыши {}", engine.settings.mouse_sensitivity);
                    engine.settings.save(path);
                } else if self.button_sens_down.contains(ndc_x, ndc_y) {
                    engine.settings.mouse_sensitivity = (engine.settings.mouse_sensitivity - 0.05).max(0.01);
                    println!("чувствительность мыши {}", engine.settings.mouse_sensitivity);
                    engine.settings.save(path);
                }

                engine.window.swap_buffers();
            }
        }
    }


    pub fn update_world_select(&self, engine: &mut Engine) -> Result<String, ProjectErrors> {
        let mut active_world = "".to_string();

        while engine.game_state == GameState::WorldSelect {
            engine.events.pull_events(&mut engine.window);

            if engine.events.cursor_locked {
                engine.events.switch_cursor_mode(&mut engine.window);
            }

            if engine.events.j_pressed(KEY_ESC) {
                engine.game_state = GameState::Menu;
                break;
            }

            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.new_world_button.draw_button(&self.ui_shader, self.ui_vao);

            let mut worlds = list_worlds("res/worlds");
            let mut world_buttons = Vec::new();

            if !worlds.is_empty() {
                for (i, world_name) in worlds.iter().enumerate() {
                    let btn = Button::new(world_name, 0.0, 0.4 - i as f32 * 0.15, 0.5, 0.1)?;
                    btn.draw_button(&self.ui_shader, self.ui_vao);
                    world_buttons.push(btn);
                }

                if engine.events.j_clicked(KEY_LCM) {
                    let (ndc_x, ndc_y) = engine.calculate_ndc();

                    if self.new_world_button.contains(ndc_x, ndc_y) {
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
                            engine.game_state = GameState::Playing;
                            break;
                        }
                    }

                    for (i, btn) in world_buttons.iter().enumerate() {
                        if btn.contains(ndc_x, ndc_y) {
                            active_world = worlds[i].clone();
                            println!("Выбран мир '{}'", &active_world.clone());
                            engine.game_state = GameState::Playing;
                            break;
                        }
                    }        
                }
            } else {
                println!("Нет миров для отображения");

                if engine.events.j_clicked(KEY_LCM) {
                    let (ndc_x, ndc_y) = engine.calculate_ndc();

                    if self.new_world_button.contains(ndc_x, ndc_y) {
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
                            engine.game_state = GameState::Playing;
                            break;
                        }
                    }      
                }
            }

            engine.window.swap_buffers();
        }

        Ok(active_world)
    }
}






fn list_worlds(path_worlds: &str) -> Vec<String> {
    let worlds_dir = Path::new(path_worlds);
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