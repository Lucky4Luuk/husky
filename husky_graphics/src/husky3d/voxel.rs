use mlua::{UserData, UserDataMethods};

use husky_voxel::scene::SceneGuard;

pub fn add_methods<'lua, M: UserDataMethods<'lua, crate::RendererGuard>>(methods: &mut M) {
    methods.add_method("drawVoxelScene", |_, obj, scene: SceneGuard| {
        let mut renderer = obj.get_lock();
        renderer.voxel_renderer.draw_scene(scene);
        Ok(())
    });
}

struct DistanceBucket {
    
}

#[derive(Clone)]
pub struct VoxelRenderer {

}

impl VoxelRenderer {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw_scene(&self, scene: SceneGuard) {

    }
}
