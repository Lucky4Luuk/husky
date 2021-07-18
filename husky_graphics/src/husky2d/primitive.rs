use glam::*;

use crate::WINDOW_SIZE;

use crate::gl_wrapper::gl_types::f32_f32_f32_f32;
use crate::gl_wrapper::mesh::{Vertex, Mesh};
use crate::gl_wrapper::shader::ShaderProgram;

use mlua::UserDataMethods;

pub fn add_methods<'lua, M: UserDataMethods<'lua, crate::RendererGuard>>(methods: &mut M) {
    methods.add_method("rect", |_, obj, (mode_raw, x,y, w,h): (String, f32,f32, f32,f32)| {
        let win_size = *WINDOW_SIZE.lock().unwrap();
        let mode = Drawmode2D::from_str(&mode_raw);
        let draw_x = (x / win_size.0 as f32) * 2.0 - 1.0;
        let draw_y = (y / win_size.1 as f32) * 2.0 - 1.0;
        let draw_w = (w / win_size.0 as f32) * 2.0;
        let draw_h = (h / win_size.1 as f32) * 2.0;
        let mut renderer = obj.get_lock();
        let color = renderer.active_color;
        let shader = renderer.get_active_shader().clone();
        renderer.renderer2d.rect(&shader.raw_program, color, mode, draw_x,draw_y, draw_w,draw_h);
        Ok(())
    });

    methods.add_method("circle", |_, obj, (mode_raw, x,y, r): (String, f32,f32, f32)| {
        let win_size = *WINDOW_SIZE.lock().unwrap();
        let mode = Drawmode2D::from_str(&mode_raw);
        let draw_x = (x / win_size.0 as f32) * 2.0 - 1.0;
        let draw_y = (y / win_size.1 as f32) * 2.0 - 1.0;
        let draw_w = (r / win_size.0 as f32) * 2.0;
        let draw_h = (r / win_size.1 as f32) * 2.0;
        let mut renderer = obj.get_lock();
        let color = renderer.active_color;
        let shader = renderer.get_active_shader().clone();
        renderer.renderer2d.circle(&shader.raw_program, color, mode, draw_x,draw_y, draw_w, draw_h);
        Ok(())
    });

    methods.add_method("tri", |_, obj, (mode_raw, x,y, w,h): (String, f32,f32, f32,f32)| {
        let win_size = *WINDOW_SIZE.lock().unwrap();
        let mode = Drawmode2D::from_str(&mode_raw);
        let draw_x = (x / win_size.0 as f32) * 2.0 - 1.0;
        let draw_y = (y / win_size.1 as f32) * 2.0 - 1.0;
        let draw_w = (w / win_size.0 as f32) * 2.0;
        let draw_h = (h / win_size.1 as f32) * 2.0;
        let mut renderer = obj.get_lock();
        let color = renderer.active_color;
        let shader = renderer.get_active_shader().clone();
        renderer.renderer2d.tri(&shader.raw_program, color, mode, draw_x,draw_y, draw_w,draw_h);
        Ok(())
    });
}

lazy_static! {
    static ref RECTANGLE_VERTICES: Vec<Vertex> = vec![
        Vertex {
            pos: (0.0, 0.0, 0.0).into(),
            uv: (0.0, 0.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },
        Vertex {
            pos: (1.0, 0.0, 0.0).into(),
            uv: (1.0, 0.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },
        Vertex {
            pos: (1.0, 1.0, 0.0).into(),
            uv: (1.0, 1.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },

        Vertex {
            pos: (0.0, 1.0, 0.0).into(),
            uv: (0.0, 1.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },
        Vertex {
            pos: (0.0, 0.0, 0.0).into(),
            uv: (0.0, 0.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },
        Vertex {
            pos: (1.0, 1.0, 0.0).into(),
            uv: (1.0, 1.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },
    ];

    static ref RECTANGLE: Mesh = Mesh::from_vertices(&RECTANGLE_VERTICES);

    static ref TRIANGLE_VERTICES: Vec<Vertex> = vec![
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            uv: (0.0, 0.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },
        Vertex {
            pos: (0.5, -0.5, 0.0).into(),
            uv: (1.0, 0.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },
        Vertex {
            pos: (0.0, 0.5, 0.0).into(),
            uv: (1.0, 1.0).into(),
            rgba: (1.0, 1.0, 1.0, 1.0).into(),
        },
    ];

    static ref TRIANGLE: Mesh = Mesh::from_vertices(&TRIANGLE_VERTICES);

    static ref CIRCLE_VERTICES: Vec<Vertex> = {
        let mut verts = Vec::new();

        for i in 0..32 {
            let center = Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                uv: (0.0, 0.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            };
            let r = ((i as f32) / 32f32) * std::f32::consts::PI * 2f32;
            let r_next = (((i + 1) as f32) / 32f32) * std::f32::consts::PI * 2f32;
            let last = Vertex {
                pos: (r.cos(), r.sin(), 0.0).into(),
                uv: (0.0, 0.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            };
            let next = Vertex {
                pos: (r_next.cos(), r_next.sin(), 0.0).into(),
                uv: (0.0, 0.0).into(),
                rgba: (1.0, 1.0, 1.0, 1.0).into(),
            };
            verts.push(center);
            verts.push(last);
            verts.push(next);
        }

        verts
    };

    static ref CIRCLE: Mesh = Mesh::from_vertices(&CIRCLE_VERTICES);
}

#[non_exhaustive]
pub enum Drawmode2D {
    Lines,
    Filled,
    Unknown
}

impl Drawmode2D {
    pub fn from_str(s: &str) -> Self {
        match s {
            "fill" => Self::Filled,
            "line" => Self::Lines,
            _ => Self::Unknown,
        }
    }
}

impl super::Renderer2D {
    pub fn rect(&mut self, shader: &ShaderProgram, color: (f32, f32, f32, f32), mode: Drawmode2D, x: f32, y: f32, w: f32, h: f32) {
        let scale = vec3(w,h,1f32);
        let translation = vec3(x,y,1f32);
        let model = Mat4::from_scale_rotation_translation(scale, Quat::IDENTITY, translation);

        // let shader = self.get_active_shader();
        shader.uniform("mvp", model);
        shader.uniform("drawColor", f32_f32_f32_f32::from(color));
        match mode {
            Drawmode2D::Filled => RECTANGLE.draw(),
            Drawmode2D::Lines => RECTANGLE.draw_wireframe(),
            _ => panic!("Unsupported drawmode!"),
        }
    }

    pub fn circle(&mut self, shader: &ShaderProgram, color: (f32, f32, f32, f32), mode: Drawmode2D, x: f32, y: f32, w: f32, h: f32) {
        let scale = vec3(w,h,1f32);
        let translation = vec3(x,y,1f32);
        let model = Mat4::from_scale_rotation_translation(scale, Quat::IDENTITY, translation);

        // let shader = self.get_active_shader();
        shader.uniform("mvp", model);
        shader.uniform("drawColor", f32_f32_f32_f32::from(color));
        match mode {
            Drawmode2D::Filled => CIRCLE.draw(),
            Drawmode2D::Lines => CIRCLE.draw_wireframe(),
            _ => panic!("Unsupported drawmode!"),
        }
    }

    pub fn tri(&mut self, shader: &ShaderProgram, color: (f32, f32, f32, f32), mode: Drawmode2D, x: f32, y: f32, w: f32, h: f32) {
        let scale = vec3(w,h,1f32);
        let translation = vec3(x,y,1f32);
        let model = Mat4::from_scale_rotation_translation(scale, Quat::IDENTITY, translation);

        // let shader = self.get_active_shader();
        shader.uniform("mvp", model);
        shader.uniform("drawColor", f32_f32_f32_f32::from(color));
        match mode {
            Drawmode2D::Filled => TRIANGLE.draw(),
            Drawmode2D::Lines => TRIANGLE.draw_wireframe(),
            _ => panic!("Unsupported drawmode!"),
        }
    }
}
