use std::sync::{Arc, Mutex};

use image::{DynamicImage, RgbaImage};

mod text;
mod primitive;

pub use primitive::Drawmode2D;

use crate::gl_wrapper::gl_types::f32_f32;
use crate::gl_wrapper::gl_types::Texture;
use crate::gl_wrapper::mesh::{Vertex, Mesh};
use crate::gl_wrapper::shader::{Shader, ShaderProgram};

#[derive(Clone)]
pub struct Renderer2D {
    active_fontobj: String,

    ///Lets us know if the shader is actually already bound
    is_default_shader_bound: bool,
    ///Lets us know if the shader is actually already bound
    is_shader_bound: bool,

    default_shader: ShaderProgram,
    active_shader: Option<ShaderProgram>,

    //TODO: Abstract to text.rs
    font_program: ShaderProgram,
    font_mesh: Mesh,
    font_image: Arc<Mutex<RgbaImage>>,
    font_texture: Texture,
}

lazy_static! {
    static ref MAX_IMAGE_DIMENSION: u32 = {
        let mut value = 0;
        unsafe { gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut value) };
        value as u32
    };
}

impl Renderer2D {
    pub fn new(active_fontobj: String) -> Self {
        let font_vs = Shader::from_source(include_str!("../../../shaders/vs_text.glsl"), gl::VERTEX_SHADER).expect("Failed to compile font vertex shader!");
        let font_fs = Shader::from_source(include_str!("../../../shaders/fs_text.glsl"), gl::FRAGMENT_SHADER).expect("Failed to compile font fragment shader!");
        let font_program = ShaderProgram::from_shaders(vec![&font_vs, &font_fs]);

        let font_mesh_verts: Vec<Vertex> = vec![
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
        let font_mesh = Mesh::from_vertices(&font_mesh_verts);

        let image = DynamicImage::new_rgba8(1280, 720).to_rgba8();
        let texture = Texture::from_ptr((1280, 720), std::ptr::null(), gl::RGBA as i32, gl::RGBA, gl::UNSIGNED_BYTE);

        let default_shader_vs = Shader::from_source(include_str!("../../../shaders/default_vs.glsl"), gl::VERTEX_SHADER).expect("Failed to compile default vs shader!");
        let default_shader_fs = Shader::from_source(include_str!("../../../shaders/default_fs.glsl"), gl::FRAGMENT_SHADER).expect("Failed to compile default fs shader!");
        let default_shader = ShaderProgram::from_shaders(vec![&default_shader_vs, &default_shader_fs]);

        Self {
            active_fontobj: active_fontobj,

            is_default_shader_bound: false,
            is_shader_bound: false,
            default_shader: default_shader,
            active_shader: None,

            font_program: font_program,
            font_mesh: font_mesh,
            font_image: Arc::new(Mutex::new(image)),
            font_texture: texture,
        }
    }

    pub fn finish_frame(&mut self) {
        if self.is_default_shader_bound {
            self.default_shader.unbind();
            self.is_default_shader_bound = false;
        }
        if self.is_shader_bound {
            match &self.active_shader {
                Some(shader) => {
                    shader.unbind();
                    self.is_shader_bound = false;
                },
                None => panic!("Shader bound but not in memory!")
            }
        }

        let win_size: (u32, u32) = {
            let raw_win_size = crate::WINDOW_SIZE.lock().unwrap();
            (raw_win_size.0, raw_win_size.1)
        };

        let ortho_matrix = glam::Mat4::orthographic_rh_gl(0.0, win_size.0 as f32, win_size.1 as f32, 0.0, -1.0, 1.0);

        {
            let image_lock = self.font_image.lock().unwrap();
            self.font_texture.bind();
            self.font_texture.data(image_lock.as_raw());
            self.font_texture.unbind();
        }

        //Render textured quad with the above image
        self.font_program.bind();
        self.font_texture.bind();
        self.font_program.uniform("offset", f32_f32::from((0f32, 0f32)));
        self.font_program.uniform("ortho", ortho_matrix);
        self.font_program.uniform("scale", f32_f32::from( (win_size.0 as f32, win_size.1 as f32) ));
        self.font_mesh.draw();
        self.font_texture.unbind();
        self.font_program.unbind();
    }

    fn get_active_shader(&mut self) -> &ShaderProgram {
        if self.is_default_shader_bound { return &self.default_shader; }
        if self.is_shader_bound { return self.active_shader.as_ref().unwrap(); }

        //No shader is already bound, so we have to figure out which one to bind
        match &self.active_shader {
            Some(shader) => {
                self.is_shader_bound = true;
                shader.bind();
                shader
            }
            None => {
                self.default_shader.bind();
                self.is_default_shader_bound = true;
                &self.default_shader
            }
        }
    }
}
