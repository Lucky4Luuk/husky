use mlua::{UserData, UserDataMethods};

use husky_voxel::scene::SceneGuard;

pub fn add_methods<'lua, M: UserDataMethods<'lua, crate::RendererGuard>>(methods: &mut M) {
    methods.add_method("drawVoxelScene", |_, obj, scene: SceneGuard| {
        let renderer = obj.get_lock();
        renderer.voxel_renderer.draw_scene(scene);
        Ok(())
    });
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
        //Current process:
        //1 - Process all added voxels in the scene into the main distance field
        //2 - Process all removed voxels in the scene into the subtracted distance field
        //3 - Merge the 2 distance fields
        //4 - Raymarch the final distance field
    }
}
