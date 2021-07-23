use mlua::{UserData, UserDataMethods};

pub mod voxel;
pub mod gpu_repr;

pub fn add_methods<'lua, M: UserDataMethods<'lua, crate::RendererGuard>>(methods: &mut M) {
    voxel::add_methods(methods);
}
