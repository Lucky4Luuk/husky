use std::sync::Arc;

use mlua::{Result as LuaResult, Error as LuaError, UserData, UserDataMethods};

use dot_vox::Model as VoxModelRaw;

use crate::model::{Brick, Model};

#[derive(Clone)]
pub struct VoxModel {
    bricks: Vec<Arc<Brick>>,
}

impl VoxModel {
    pub fn from_filename(path: &str) -> LuaResult<Self> {
        let vox = dot_vox::load(path).or_else(|_| Err(LuaError::RuntimeError("Failed to find file!".into())) )?;
        //TODO: When dot_vox updates to support scene graph loading, we should place models in the right spot
        //      Right now all models will be at 0,0,0
        let mut brick = Brick::empty(0,0,0);

        for model in &vox.models {
            for voxel in &model.voxels {
                let wx = voxel.x;
                let wy = voxel.y;
                let wz = voxel.z;
                brick.data[wx][wy][wz] = Voxel::new(r,g,b, roughness, metalness);
            }
        }

        Ok(Self {
            bricks: vec![Arc::new(brick)]
        })
    }
}

impl Model for VoxModel {
    fn get_bricks(&self) -> Vec<Brick> {
        Vec::new()
    }
}
