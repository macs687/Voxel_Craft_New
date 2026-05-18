use core::{Window, Events, Camera};
use std::time::Instant;
use glam::{Vec3, Mat4};
use glfw::Key::*;
use glfw::MouseButton;
use crate::constant::*;
use crate::loger::ProjectErrors;
use glfw::CursorMode;
use graphics::load_shader;
use graphics::create_mesh_cube;
use graphics::load_texture_from_png;


use gl::types::*;

mod voxels;
mod constant;
mod loger;
mod core;
mod graphics;


fn main() -> Result<(), ProjectErrors> {
    println!("инициализация окна");
    let mut window = Window::init("Voxel Craft", 1920, 1080)?;
    window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    println!("Инициализация обработчика событий");
    let mut events = Events::init();
    events.setting(&mut window);
    println!("Инициализация обработчика событий завершена");

    println!("инициализация камеры");
    let mut camera = Camera::init(Vec3::new(0.0, 0.0, 3.0), 70.0_f32.to_radians());
    println!("инициализация камеры: ок");

    events.switch_cursor_mode(&mut window);

    println!("создание базовой шейдерной программы");
    let shader = load_shader("res/shaders/vertex_shader.glsl", "res/shaders/fragment_shader.glsl")?;
    println!("создание базовой шейдерной программы завершено");

    println!("загрузка текстуры");
    let mut texture = load_texture_from_png("res/textures/planks.jpg")?;
    println!("загрузка текстуры: ок");



    println!("отрисовка куба");
    let (_vao, cube_index_count) = create_mesh_cube();
    println!("куб нарисован");

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(1.0,1.0, 1.0, 0.4);
        //gl::Enable(gl::CULL_FACE);
    }

    const MOUSE_SENSITIVITY: f32 = 0.001;

    let mut last_frame = Instant::now();

    println!("Start main loop");
    while window.is_open() {
        let now = Instant::now();
        let delta_time = (now - last_frame).as_secs_f32();
        last_frame = now;
        let delta_time = delta_time.min(0.05);

        println!("дельта {delta_time}");

        // прослушивание всех устройств и обработка событий 
        events.pull_events(&mut window);

        if events.j_clicked(LCM) {
            println!("ЛКМ нажата");
            window.gl_clear_color(0.3, 0.4, 0.5, 0.6);
        } else if events.j_pressed(TAB) && events.cursor_in_window {
            events.switch_cursor_mode(&mut window);
        }else if events.j_pressed(Escape as i32) {
            window.close();
            
        }

        // игровая логика
        let mut direction = Vec3::ZERO;

        println!("front: {:?}, right: {:?}", camera.front, camera.right);


        let pitch_delta = events.delta_y * MOUSE_SENSITIVITY;
        let yaw_delta = events.delta_x * MOUSE_SENSITIVITY;

        if events.cursor_locked {
            camera.rotate(-pitch_delta, yaw_delta, 0.0);
        }
        if events.pressed(W as i32) {
            direction += camera.front;
            println!("W нажата");
            println!("W pressed, direction = {:?}", direction);
        }
        if events.pressed(constant::S) {
            direction -= camera.front; // назад
            println!("S нажата");
            println!("S pressed, direction = {:?}", direction);
        }
        if events.pressed(constant::A) {
            println!("A нажата");
            direction -= camera.right; // влево
            println!("A pressed, direction = {:?}", direction);
        }
        if events.pressed(constant::D) {
            direction += camera.right; // вправо
            println!("D нажата");
            println!("D pressed, direction = {:?}", direction);
        }
        if events.pressed(constant::SPACE) {
            direction += Vec3::Y; // вверх (мировая ось)
        }
        if events.pressed(constant::LEFT_SHIFT) {
            direction -= Vec3::Y; // вниз
        }

        //println!("final direction: {:?}", direction);

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            camera.position += direction * MOVE_SPEED * delta_time;
        }


        // очистка буфера 
        window.gl_clear();

        // рендер нового кадра
        shader.use_shader();

        let view = camera.get_view();
        let projection = camera.get_projection(window.width as f32, window.height as f32);
        let model = Mat4::IDENTITY; // или переместите треугольник, если нужно

        shader.uniform_matrix("uModel", model);
        shader.uniform_matrix("uView", view);
        shader.uniform_matrix("uProjection", projection);
        
        texture.bind(0);
        shader.uniform_texture("uTexture", 0);



        unsafe {
            gl::BindVertexArray(_vao);
            gl::DrawElements(gl::TRIANGLES, cube_index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }

        // отрисовка
        window.swap_buffers();
    }

    println!("Hello, world!");
    Ok(())
}