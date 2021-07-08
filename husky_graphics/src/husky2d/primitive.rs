use glam::*;

use crate::gl_wrapper::mesh::{Vertex, Mesh};
use crate::gl_wrapper::shader::ShaderProgram;


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
}

#[non_exhaustive]
pub enum Drawmode2D {
    Lines,
    Filled,
}

impl super::Renderer2D {
    pub fn rect(&mut self, mode: Drawmode2D, x: f32, y: f32, w: f32, h: f32) {
        let scale = vec3(w,h,1f32);
        let translation = vec3(x,y,1f32);
        let model = Mat4::from_scale_rotation_translation(scale, Quat::IDENTITY, translation);

        let shader = self.get_active_shader();
        shader.uniform("model", model);
        match mode {
            Drawmode2D::Filled => RECTANGLE.draw(),
            Drawmode2D::Lines => RECTANGLE.draw_wireframe(),
        }
    }
}
