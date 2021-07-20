pub mod surface;
pub mod model;
pub mod scene;

use mlua::{UserData, UserDataMethods};

#[derive(Clone)]
pub struct VoxelInterface;

impl VoxelInterface {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl UserData for VoxelInterface {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("newScene", |_, _obj, ()| {
            Ok(scene::SceneGuard::new())
        });
    }
}
