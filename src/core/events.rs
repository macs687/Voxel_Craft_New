use glfw::Action;

use super::Window;


const KEYS_COUNT: usize = 1024;
const MOUSE_BUTTON_COUNT: usize = 8;
const TOTAL_KEYS: usize = KEYS_COUNT + MOUSE_BUTTON_COUNT;


pub struct Events {
    keys: [bool; TOTAL_KEYS],
    frames: [u32; TOTAL_KEYS],
    current: u32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub x: f64,
    pub y: f64,
    pub cursor_locked: bool,
    cursor_started: bool,
    pub cursor_in_window: bool
}


impl Events {
    pub fn init() -> Self {
        let keys = [false; TOTAL_KEYS];
        let frames = [0; TOTAL_KEYS];
        Self { keys,
            frames,
            current: 0,
            delta_x: 0.0,
            delta_y: 0.0, 
            x: 0.0,
            y: 0.0, 
            cursor_locked: false, 
            cursor_started: false,
            cursor_in_window: false
        }
    }


    pub fn setting(&self, window: &mut Window) {
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_cursor_enter_polling(true);
        window.set_size_polling(true)
    }


    fn update_key(&mut self, key: usize, action: Action) {
        match action {
            Action::Press => {
                self.keys[key] = true;
                self.frames[key] = self.current;
            },
            Action::Release => {
                self.keys[key] = false;
                self.frames[key] = self.current;
            },
            _ => ()
        }
    }


    pub fn pressed(&self, keycode: i32) -> bool {
        let key = keycode as usize;
        if key >= TOTAL_KEYS {
            return false;
        }
        self.keys[key]
    }


    pub fn j_pressed(&self, keycode: i32) -> bool {
        let key = keycode as usize;
        if key >= TOTAL_KEYS {
            return false;
        }
        self.keys[key] && self.frames[key] == self.current
    }


    pub fn clicked(&self, button: i32) -> bool {
        if button < 0 || button > 7 {
            return false;
        }else {
            return self.keys[(1024 + button) as usize];
        }
    }


    pub fn j_clicked(&self, button: i32) -> bool {
        if button < 0 || button > 7 {
            return  false;
        }else {
            return self.keys[(1024 + button) as usize] && self.frames[(1024 + button) as usize] == self.current;
        }
    }


    pub fn switch_cursor_mode(&mut self, window: &mut Window) {
        self.cursor_locked = !self.cursor_locked;
        if self.cursor_locked {
            window.window.set_cursor_mode(glfw::CursorMode::Disabled);
        }else {
            window.window.set_cursor_mode(glfw::CursorMode::Normal);
            
        }
    }  


    pub fn pull_events(&mut self, window: &mut Window) {
        window.poll_events();
        self.current += 1;
        self.delta_x = 0.0;
        self.delta_y = 0.0;

        for (_, event) in glfw::flush_messages(&window.receiver) {
            match event {
                glfw::WindowEvent::Size(w, h) => {
                    window.width = w as u32;
                    window.height = h as u32;
                    unsafe { gl::Viewport(0, 0, w, h); }
                },

                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    if self.cursor_started{
                        self.delta_x += (xpos - self.x) as f32;
                        self.delta_y += (ypos - self.y) as f32;
                    }else {
                        self.cursor_started = true;
                    }
                    self.x = xpos;
                    self.y = ypos;
                },

                glfw::WindowEvent::MouseButton(button, action, _) => {
                    let button_id = match button {
                        glfw::MouseButton::Button1 => 1024,
                        glfw::MouseButton::Button2 => 1025,
                        glfw::MouseButton::Button3 => 1026,
                        glfw::MouseButton::Button4 => 1027,
                        glfw::MouseButton::Button5 => 1028,
                        glfw::MouseButton::Button6 => 1029,
                        glfw::MouseButton::Button7 => 1030,
                        glfw::MouseButton::Button8 => 1031,
                    };
                    self.update_key(button_id, action);
                },

                glfw::WindowEvent::Key(key,_ ,action ,_) => {
                    self.update_key(key as usize, action);
                },

                glfw::WindowEvent::CursorEnter(entered) => {
                    self.cursor_in_window = entered;
                    if entered {
                        self.cursor_started = false;
                    }
                }

                _ => {}
            }
        }

    }
}