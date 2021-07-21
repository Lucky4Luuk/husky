use mlua::{UserData, UserDataMethods};

use husky_voxel::scene::SceneGuard;

pub fn add_methods<'lua, M: UserDataMethods<'lua, crate::RendererGuard>>(methods: &mut M) {
    methods.add_method("drawVoxelScene", |_, obj, scene: SceneGuard| {
        let renderer = obj.get_lock();
        renderer.voxel_renderer.draw_scene(scene);
        Ok(())
    });
}

/// A distance bucket stores the distance to a voxel brick.
/// It does so as a 32 bit float, because the spec of glsl
/// says that it's perfectly legal for the compiler to output
/// 32 bit floats for any precision specified. Kinda stupid, but
/// GPUs are a fan of 32 bit floats anyway.
struct DistanceBucket(f32);

#[derive(Clone)]
pub struct VoxelRenderer {

}

impl VoxelRenderer {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw_scene(&self, scene: SceneGuard) {
        //Current process:
        //1 - Process all voxel bricks in the scene into a distance field
        //2 - Raymarch the distance field
        //TODO: Cache the already-uploaded bricks. Otherwise, performance will probably be awful.
    }
}
