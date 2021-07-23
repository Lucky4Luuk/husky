#[macro_use] extern crate log;

use std::time::Instant;
use std::fs::read_to_string;
use std::path::Path;
use std::env;

use getopts::Options;

use glutin::ContextBuilder;
use glutin::dpi::LogicalSize;
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};
use glutin::window::WindowBuilder;
use glutin::{GlProfile, GlRequest, Api};

use husky_lua::LuaProgram;

static DEFAULT_WINDOW_SIZE: (u32, u32) = (1280u32, 720u32);

static DEFAULT_PROG_SRC: &'static str = include_str!("../default_main.lua");

fn load_gl(gl_context: &glutin::Context<glutin::PossiblyCurrent>) {
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_input = if args.len() > 1 { args[1].clone() } else { "".to_string() };
    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help menu");
    let empty = Vec::new();
    let opts_passed = if args.len() > 2 { &args[2..] } else { &empty[..] };
    //TODO: Do something with this to create a proper CLI
    let matches = match opts.parse(opts_passed) {
        Ok(m) => m,
        Err(f) => panic!("{}", f)
    };

    let max_level = log::LevelFilter::max();
    pretty_env_logger::formatted_builder()
        .filter_level(max_level)
        .init();

    debug!("Hello, world!");

    let directory = {
        let p = Path::new(&path_input);
        let mut buf = p.to_path_buf();
        if !p.exists() {
            if path_input != "" && path_input != "." { error!("Directory passed does not exist!"); }
            buf = Path::new("").to_path_buf()
        }
        if p.is_dir() {
            buf
        } else {
            buf.pop();
            buf
        }
    };

    let event_loop = EventLoop::new();
    let logical_window_size: LogicalSize<u32> = DEFAULT_WINDOW_SIZE.into();

    let window_builder = WindowBuilder::new()
        .with_title("Husky v0.0.1")
        .with_inner_size(logical_window_size);

    let context = ContextBuilder::new().with_vsync(false).with_gl(GlRequest::Specific(Api::OpenGl, (4,5))).with_gl_profile(GlProfile::Core).build_windowed(window_builder, &event_loop).expect("Failed to create opengl context!");
    let context = unsafe { context.make_current().expect("Failed to make context current!") };

    load_gl(&context.context());

    let mut old_frametime = Instant::now();
    let mut close_requested = false;

    //Load program
    let path = directory.join(Path::new("main.lua"));
    debug!("Trying to load program from path `{}`", path.display());
    let source = read_to_string(path).unwrap_or(DEFAULT_PROG_SRC.to_string());
    let program = LuaProgram::from_source(directory.display().to_string(), &source).expect("Failed to get program!");
    let mut dimensions: (u32, u32) = context.window().inner_size().into();
    program.on_resize(dimensions);

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

                if dimensions != context.window().inner_size().into() {
                    dimensions = context.window().inner_size().into();
                    program.on_resize(dimensions);
                }

                program.update(delta_s);
                program.draw();

                context.window().set_title(&format!("FPS: {}", 1.0 / delta_s));

                context.swap_buffers().expect("Failed to swap buffers!");
            },
            _ => {},
        }
    });
}
