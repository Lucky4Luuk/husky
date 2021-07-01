#[macro_use] extern crate log;

use std::time::Instant;

use glutin::ContextBuilder;
use glutin::dpi::LogicalSize;
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};
use glutin::window::WindowBuilder;

use husky_lua::LuaProgram;

static DEFAULT_WINDOW_SIZE: (u32, u32) = (1920u32, 1080u32);

static DEFAULT_PROG_SRC: &'static str = include_str!("../default_main.lua");

fn main() {
    let max_level = log::LevelFilter::max();
    pretty_env_logger::formatted_builder()
        .filter_level(max_level)
        .init();

    debug!("Hello, world!");

    let event_loop = EventLoop::new();
    let logical_window_size: LogicalSize<u32> = DEFAULT_WINDOW_SIZE.into();

    let window_builder = WindowBuilder::new()
        .with_title("Solarsystem raymarching proof of concept")
        .with_inner_size(logical_window_size);

    let context = ContextBuilder::new().build_windowed(window_builder, &event_loop).expect("Failed to create opengl context!");
    let context = unsafe { context.make_current().expect("Failed to make context current!") };

    husky_graphics::load_gl(&context.context());

    let mut old_frametime = Instant::now();
    let mut close_requested = false;

    //Load program
    let lua = LuaProgram::new_lua_env();
    let source = std::fs::read_to_string("main.lua").unwrap_or(DEFAULT_PROG_SRC.to_string());
    let program = LuaProgram::from_source(&lua, &source).expect("Failed to get program!");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    close_requested = true;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match virtual_code {
                    VirtualKeyCode::Escape => {
                        close_requested = true;
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                if close_requested {
                    *control_flow = ControlFlow::Exit;
                } else {
                    context.window().request_redraw();
                }
            },
            Event::RedrawRequested(..) => {
                let delta = Instant::now() - old_frametime;
                old_frametime = Instant::now();
                let delta_s = delta.as_secs() as f32 + (delta.subsec_micros() as f32 / 1_000_000f32);

                // context.window().set_title(&format!("FPS: {}", 1.0 / delta_s));

                context.swap_buffers().expect("Failed to swap buffers!");
            },
            _ => {},
        }
    });
}
