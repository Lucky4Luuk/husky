use std::sync::{Arc, Mutex, MutexGuard};

use mlua::{UserData, UserDataMethods};

use crate::model::{
    Brick,
    Voxel,
    Model
};

#[derive(Clone)]
pub struct ModelReference {
    idx: usize,
}

impl UserData for ModelReference {}

#[derive(Clone)]
pub struct Scene {
    models: Vec<Arc<Box<dyn Model>>>,
}

impl Scene {
    pub fn new() -> Scene {
        Self {
            models: Vec::new(),
        }
    }

    pub fn add_model(&mut self, model: Box<dyn Model>) -> ModelReference {
        let idx = self.models.len();
        self.models.push(Arc::new(model));
        ModelReference {
            idx: idx,
        }
    }
}

#[derive(Clone)]
pub struct SceneGuard {
    scene_guard: Arc<Mutex<Scene>>,
}

impl SceneGuard {
    pub fn new() -> Self {
        Self {
            scene_guard: Arc::new(Mutex::new(Scene::new()))
        }
    }

    pub fn get_lock(&self) -> MutexGuard<Scene> {
        self.scene_guard.lock().expect("Failed to acquire lock on scene!")
    }
}

impl UserData for SceneGuard {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("newModel", |_, scene, (kind, path): (String, String)| {
            // let mod_ref = scene.get_lock().add_model(model);
            todo!();
            Ok(())
        });

        methods.add_method("setVoxel", |_, scene, (x,y,z, voxel): (u8, u8, u8, Voxel)| {
            todo!();
            Ok(())
        });
    }
}

#[derive(Clone)]
pub struct ModelWrapper {
    pub brick: Arc<Box<dyn Model>>,
    pub brick_index: usize,
}
