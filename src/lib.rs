extern crate glfw;
extern crate deno_bindgen;
extern crate libc;

use deno_bindgen::deno_bindgen;

static mut GLFW: Option<glfw::Glfw> = None;
static mut WINDOW: Option<glfw::Window> = None;
static mut EVENTS: Option<std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>> = None;

unsafe fn get_glfw() -> &'static mut  glfw::Glfw {
    match GLFW {
        Some(ref mut x) => &mut *x,
        None => panic!()
    }
}

unsafe fn get_window() -> &'static mut  glfw::Window {
    match WINDOW {
        Some(ref mut x) => &mut *x,
        None => panic!()
    }
}

unsafe fn get_events() -> &'static mut  std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)> {
    match EVENTS {
        Some(ref mut x) => &mut *x,
        None => panic!()
    }
}

#[deno_bindgen]
pub fn init() {
    unsafe {
        GLFW = Some(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());
    }
}

#[deno_bindgen]
pub fn createWindow(width:u32, height:u32, name: &str) {
    unsafe {
        let glfw = get_glfw();
        let (mut window, events) = glfw
            .create_window(width, height, name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        
        WINDOW = Some(window);
        EVENTS = Some(events);
    }
}

#[deno_bindgen]
pub fn pollEvents() {
    unsafe {
        let glfw = get_glfw();
        glfw.poll_events();
        let window = get_window();
        for (_, event) in glfw::flush_messages(&get_events()) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => window.set_should_close(true),
                _ => {}
            }
        }
    }
}

#[deno_bindgen]
pub fn shouldClose() -> i16 {
    unsafe {
        let window = get_window();
        if window.should_close() {
            return 1;
        } else {
            return 0;
        }
    }
}