use glam::*;

use crate::gl_wrapper::mesh::{Vertex, Mesh};
use crate::gl_wrapper::shader::ShaderProgram;

const RECTANGLE_VERTICES: Vec<Vertex> = vec![

];

lazy_static! {
    static ref RECTANGLE: Mesh = Mesh::from_vertices(&RECTANGLE_VERTICES);
}

#[non_exhaustive]
pub enum Drawmode2D {
    Lines,
    Filled,
}

impl super::Renderer2D {
    pub fn rect(&self, active_shader: &ShaderProgram, mode: Drawmode2D, x: f32, y: f32, w: f32, h: f32) {
        let scale = vec3(w,h,1f32);
        let translation = vec3(x,y,1f32);
        let model = Mat4::from_scale_rotation_translation(scale, Quat::IDENTITY, translation);
        active_shader.uniform("model", model);
        match mode {
            Drawmode2D::Filled => RECTANGLE.draw(),
            Drawmode2D::Lines => RECTANGLE.draw_wireframe(),
        }
    }
}
