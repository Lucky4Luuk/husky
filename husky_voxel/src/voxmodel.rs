use std::sync::Arc;

use mlua::{Result as LuaResult, Error as LuaError, UserData, UserDataMethods};

use dot_vox::Model as VoxModelRaw;

use crate::model::{Voxel, Brick, Model};

#[derive(Clone)]
pub struct VoxModel {
    bricks: Vec<Brick>,
}

impl VoxModel {
    pub fn from_filename(path: &str) -> LuaResult<Self> {
        let vox = dot_vox::load(path).or_else(|_| Err(LuaError::RuntimeError("Failed to find file!".into())) )?;
        //TODO: When dot_vox updates to support scene graph loading, we should place models in the right spot
        //      Right now all models will be at 0,0,0

        //Create a cluster of bricks big enough for a 256x256x256 model
        let mut bricks = Vec::new();
        for ix in 0..4 {
            for iy in 0..4 {
                for iz in 0..4 {
                    bricks.push(Brick::empty(ix, iy, iz));
                }
            }
        }

        let obj = Self {
            bricks: bricks
        };

        for model in &vox.models {
            for voxel in &model.voxels {
                let wx = voxel.x as u64;
                let wy = voxel.y as u64;
                let wz = voxel.z as u64;
                let r = 255;
                let g = 255;
                let b = 255;
                let roughness = 255;
                let metalness = 0;
                obj.set_voxel(wx,wy,wz, Voxel::new(r,g,b, roughness, metalness));
            }
        }

        Ok(obj)
    }

    pub fn set_voxel(&self, x: u64, y: u64, z: u64, voxel: Voxel) {
        //TODO: Support creating bricks when needed
        todo!();
    }
}

impl Model for VoxModel {
    fn get_bricks(&self) -> &Vec<Brick> {
        &self.bricks
    }
}
