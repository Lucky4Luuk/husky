use std::sync::Mutex;

use glam::*;

use husky_voxel::model::Voxel;

use gl_wrapper::gl_types::ShaderStorageBuffer;

//TODO: Limit the max size, to not consume too much VRAM
//      Perhaps a limit of 2GB would be good, that would equal 512x512x512 voxels
pub fn allocate_sdf_ssbo() -> ShaderStorageBuffer {
    let ssbo = ShaderStorageBuffer::new();

    //Minimum size is 128 megabytes, according to the OpenGL specs.
    let mut max_size = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_SHADER_STORAGE_BLOCK_SIZE, &mut max_size);
    }
    debug!("Max SSBO size: {} megabytes", max_size / 1024 / 1024);

    ssbo.bind();
    ssbo.empty_with_length(max_size as usize, gl::DYNAMIC_DRAW);
    ssbo.unbind();

    ssbo
}

/// Generic datapoint used to store data in the SDF-AS ssbo.
/// If we need to store more than 4 bytes, we simply use multiple datapoints
type GPU_Datapoint = u32;

struct Node {
    pub child: Option<u32>,
    pub distance: f32,
}

struct DistanceBrick {
    pub pos: (u16, u16, u16),
    pub data: [f32; 64*64*64],
}

impl Node {
    pub fn to_bits(&self) -> u32 {
        let bits = self.child.unwrap_or(self.distance.to_bits());
        (self.child.is_some() as u32) | bits << 1
    }
}

pub fn gen_sdf(voxel_max_axis: usize, sdf: ShaderStorageBuffer) {
    sdf.bind();

    let mut raw: Vec<GPU_Datapoint> = Vec::new();

    //Fill it with test data for now
    let mut nodes: Vec<Node> = Vec::new();
    let mut bricks: Vec<DistanceBrick> = Vec::new();
    nodes.push(Node {
        child: Some(1),
        distance: 0.0,
    });

    // for x in 0..voxel_max_axis {
    //     for y in 0..voxel_max_axis {
    //         for z in 0..voxel_max_axis {
    //             let fx = x as f32 - (voxel_max_axis as f32) / 2.0;
    //             let fy = y as f32 - (voxel_max_axis as f32) / 2.0;
    //             let fz = z as f32 - (voxel_max_axis as f32) / 2.0;
    //             let dist = vec3(fx,fy,fz).length() - 32.0;
    //             let data = dist.to_bits();
    //             let idx = x + y * voxel_max_axis + z * voxel_max_axis * voxel_max_axis;
    //             raw[idx] = data;
    //         }
    //     }
    // }

    sdf.data(&raw, gl::DYNAMIC_DRAW);

    sdf.unbind();
}
