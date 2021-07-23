use mlua::{UserData, UserDataMethods};

use gl_wrapper::gl_types::ShaderStorageBuffer;
use gl_wrapper::shader::Shader as GlShader;
use gl_wrapper::gl_types::Texture;
use gl_wrapper::mesh::{Vertex, Mesh};

use husky_voxel::scene::SceneGuard;

use crate::Shader;
use super::gpu_repr;

pub fn add_methods<'lua, M: UserDataMethods<'lua, crate::RendererGuard>>(methods: &mut M) {
    methods.add_method("drawVoxelScene", |_, obj, scene: SceneGuard| {
        let renderer = obj.get_lock();
        renderer.voxel_renderer.draw_scene(scene);
        Ok(())
    });
}

#[derive(Clone)]
pub struct VoxelRenderer {
    sdf_ssbo: ShaderStorageBuffer,

    shader: Shader,
    render_texture: Texture,
    // render_mesh: Mesh,
}

impl VoxelRenderer {
    pub fn new() -> Self {
        let sdf_ssbo = gpu_repr::allocate_sdf_ssbo();

        let raymarch_src = include_str!("../../../shaders/raymarch.glsl");
        let raymarch_shader = GlShader::from_source(raymarch_src, gl::COMPUTE_SHADER).expect("Failed to compile shader!");
        let shader = Shader::from_shaders(vec![&raymarch_shader]);

        let render_texture = Texture::from_ptr((1280, 720), std::ptr::null(), gl::RGBA32F as i32, gl::RGBA, gl::FLOAT);

        Self {
            sdf_ssbo: sdf_ssbo,

            shader: shader,
            render_texture: render_texture,
        }
    }

    pub fn draw_scene(&self, scene: SceneGuard) {
        //Current process:
        //1 - Process all added voxels in the scene into the main distance field
        //2 - Process all removed voxels in the scene into the subtracted distance field
        //3 - Merge the 2 distance fields
        //4 - Raymarch the final distance field

        //TODO: Implement step 1, 2 and 3

        self.raymarch_scene();
    }

    fn raymarch_scene(&self) {
        let (resx, resy) = {
            let win_size = *crate::WINDOW_SIZE.lock().unwrap();
            win_size
        };

        self.sdf_ssbo.bind_buffer_base(1);
        self.shader.raw_program.bind();
        self.render_texture.bind();
        self.sdf_ssbo.bind();

        unsafe {
            gl::DispatchCompute(resx / 32, resy / 32, 1);
        }

        self.render_texture.unbind();
        self.sdf_ssbo.unbind();
        self.shader.raw_program.unbind();
    }
}
