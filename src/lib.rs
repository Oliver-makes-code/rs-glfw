extern crate deno_bindgen;
extern crate glfw;
extern crate gl;
extern crate libc;

use deno_bindgen::deno_bindgen;

use glfw::{Context};
use gl::types::*;

static mut GLFW: Option<glfw::Glfw> = None;
static mut WINDOW: Option<glfw::Window> = None;
static mut EVENTS: Option<std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>> = None;
static mut RENDER_CTX: Option<glfw::RenderContext> = None;
static mut KEYS: [bool; 348] = [false; 348];

unsafe fn get_glfw() -> &'static mut glfw::Glfw {
    match GLFW {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

unsafe fn get_window() -> &'static mut glfw::Window {
    match WINDOW {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

unsafe fn get_render_ctx() -> &'static mut glfw::RenderContext {
    match RENDER_CTX {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

unsafe fn get_events() -> &'static mut std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)> {
    match EVENTS {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

#[deno_bindgen]
pub fn init() {
    unsafe {
        GLFW = Some(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());
    }
}

#[deno_bindgen]
pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

#[deno_bindgen]
pub struct Color {
    r: i32,
    g: i32,
    b: i32,
    a: i32
}

#[deno_bindgen]
pub fn clearColor(color: Color) {
    unsafe {
        let r = color.r as f32 / 255.;
        let g = color.g as f32 / 255.;
        let b = color.b as f32 / 255.;
        let a = color.a as f32 / 255.;

        gl::ClearColor(r,g,b,a);
    }
}

#[deno_bindgen]
pub fn createWindow(width: u32, height: u32, name: &str) {
    unsafe {
        let glfw = get_glfw();
        let (mut window, events) = glfw
            .create_window(width, height, name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        WINDOW = Some(window);
        EVENTS = Some(events);
        let ctx = get_window().render_context();
        RENDER_CTX = Some(ctx);
        get_render_ctx().make_current();
    }
}

fn key_pressed(scancode: i32) -> bool {
    unsafe {
        return KEYS[(scancode as usize)];
    }
}

#[deno_bindgen]
pub fn keyPressed(scancode: i32) -> i16 {
    if key_pressed(scancode) {
        return 1;
    }
    return 0;
}

#[deno_bindgen]
pub fn pollEvents() {
    unsafe {
        let glfw = get_glfw();
        glfw.poll_events();
        let window = get_window();
        for (_, event) in glfw::flush_messages(&get_events()) {
            match event {
                glfw::WindowEvent::Key(key, scancode, action, mods) => match action {
                    glfw::Action::Press => {
                        KEYS[(scancode as usize)] = true;
                    }
                    glfw::Action::Release => {
                        KEYS[(scancode as usize)] = false;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

#[deno_bindgen]
pub fn swapBuffers() {
    unsafe {
        let ctx = get_render_ctx();
        ctx.swap_buffers();

        
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
