use core::Window;
use core::Events;
use glfw::Key;
use glfw::MouseButton;
use crate::constant::*;
use crate::loger::ProjectErrors;
use glfw::CursorMode;
use graphics::load_shader;

use gl::types::*;


mod constant;
mod loger;
mod core;
mod graphics;


fn create_triangle() -> GLuint {
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0,
    ];

    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as GLsizei, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    vao
}


fn main() -> Result<(), ProjectErrors> {
    println!("инициализация окна");
    let mut window = match Window::init("Voxel Craft", 1920, 1080) {
        Ok(window) => window,

        // слом glfw
        Err(ProjectErrors::GlfwInitError(error)) => {
            eprintln!("FATAL ERROR: Ошибка инициализации glfw {error}");
            std::process::exit(1)
        },

        // ошибка инициализации окна 
        Err(ProjectErrors::WindowCreateError(error)) => {
            eprintln!("WINDOW ERROR: Ошибка инициализации окна. {error} Запуск повторной инициализации");
            match Window::init("title", 1920, 1080) {
                // Успех
                Ok(window) => {
                    println!("INFO: Повторная инициализация успешна");
                    window
                },
                
                // аааааа ошибка
                Err(e) => {
                    eprint!("FATAL ERROR: {e}");
                    std::process::exit(1)
                }
            }
        },

        // другая ошибка 
        _ => panic!("FATAL ERROR: неизвестная ошибка")
    };


    println!("Инициализация обработчика событий");
    let mut events = Events::init();
    events.setting(&mut window);
    println!("Инициализация обработчика событий завершена");

    println!("создание базовой шейдерной программы");
    let shader = load_shader("res/shaders/vertex_shader.glsl", "res/shaders/fragment_shader.glsl")?;
    println!("создание базовой шейдерной программы завершено");

    println!("отрисовка треугольника");
    let triangle_vao = create_triangle();
    println!("Треугольник нарисован");

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(1.0,1.0, 1.0, 0.4);
    }


    println!("Start main loop");
    while window.is_open() {
        // прослушивание всех устройств и обработка событий 
        events.pull_events(&mut window);

        if events.j_clicked(LCM) {
            println!("ЛКМ нажата");
            window.gl_clear_color(0.3, 0.4, 0.5, 0.6);
        } else if events.j_pressed(TAB) && events.cursor_in_window {
            events.switch_cursor_mode(&mut window);
        }

        // игровая логика

        // очистка буфера 
        window.gl_clear();

        // рендер нового кадра
        shader.use_shader();

        unsafe {
            gl::BindVertexArray(triangle_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        // отрисовка
        window.swap_buffers();
    }

    println!("Hello, world!");
    Ok(())
}
