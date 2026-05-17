extern crate gl;
extern crate glfw;

use glfw::Context;
use crate::loger::ProjectErrors;
use std::fmt::Error;
use gl::{DEPTH_BUFFER_BIT, DEPTH_TEST};
use glfw::ffi::glfwTerminate;
use glfw::{fail_on_errors, Glfw, GlfwReceiver, PWindow, WindowEvent};


pub struct Window {
    pub glfw: Glfw,
    pub window: PWindow,
    pub receiver: GlfwReceiver<(f64, WindowEvent)>
}


impl Window {
    pub fn init(title: &str, width: u32, height: u32) -> Result<Self, ProjectErrors> {
        // инициализация glfw
        let mut glfw = glfw::init(glfw::fail_on_errors!()).map_err(|e| ProjectErrors::GlfwInitError(format!("{e}")))?;
        
        // ??
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Resizable(true));

        // создание окна
        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).ok_or_else(|| ProjectErrors::WindowCreateError("GLFW не смог создать окно".into()))?;

        window.make_current();

        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const _);

        unsafe {
            let version = gl::GetString(gl::VERSION);
            println!("OpenGL version: {:?}", std::ffi::CStr::from_ptr(version as *const i8));
        }

        window.set_key_polling(true);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Ok(Self {
        glfw,
        window,
        receiver: events,
        })
    }
    

    /// проверка открыто ли окно сейчас
    pub fn is_open(&self) -> bool {
        !self.window.should_close()
    }


    /// сканирование всех событий от ОС
    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }


    /// обновление буфера цвета
    pub fn gl_clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }


    /// перестановка буфера
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }


    /// установка нового цвета фона
    pub fn gl_clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }


    /// активация прослушивания клавиатуры
    pub fn set_key_polling(&mut self, choice: bool) {
        self.window.set_key_polling(choice);
    }


    /// активация прослушивания мышки
    pub fn set_mouse_button_polling(&mut self, choice: bool) {
        self.window.set_mouse_button_polling(choice);
    }


    /// включить отслеживание позиции курсора
    pub fn set_cursor_pos_polling(&mut self, choice: bool) {
        self.window.set_cursor_pos_polling(choice);
    }


    /// отслеживание курсор в окне или нет
    pub fn set_cursor_enter_polling(&mut self, choice: bool) {
        self.window.set_cursor_enter_polling(choice);
    }


    /// отслеживание изменения размера окна
    pub fn set_size_polling(&mut self, choice: bool) {
        self.window.set_size_polling(choice);
    }
}