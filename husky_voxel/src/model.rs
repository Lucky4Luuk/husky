/// Voxels are 2 byte large, and store a reference to a
/// material, as well as a reference to the colour palette.
/// It uses 8 bits for each, which means we can use a maximum
/// of 256 materials and 256 colours. This is not a lot, so
/// I'll definitely have to find a way to change this.
pub struct Voxel(u16);

impl Voxel {
    pub fn from_id(mat_id: u8, pal_id: u8) -> Self {
        let mut data: u16 = mat_id as u16;
        data |= (pal_id as u16) << 8;
        Self(data)
    }
}

/// Bricks are a fundamental part of our memory management.
/// By storing only 8x8x8 blocks of voxels raw, we can still
/// exclude storing large blocks of empty voxels. Using these
/// bricks is less efficient than using a sparse octree, but
/// it is much quicker to modify. These bricks will be stored
/// in a multi-level array, so both raytracing and modifying
/// them is easy. The multi-level array relies on 16 bit
/// coordinates which is reflected in the bricks as well.
pub struct Brick {
    pub pos: (u16, u16, u16),
    pub data: [Voxel; 8*8*8],
}

/// The model trait is to be implemented for any struct meant
/// to store voxels in some way or another. It provides the
/// scene code an easy way to interact with the data through
/// a simple interface, without it having to worry about whatever
/// internal format was used.
pub trait Model {
    fn get_bricks(&self) -> Vec<Brick>;
}

/// The modifyable trait is for any struct that allows the user
/// to modify the voxels inside.
pub trait Modifyable {
    fn get_voxel(&self, pos: (u64, u64, u64)) -> Voxel;
    fn set_voxel(&self, pos: (u64, u64, u64), voxel: Voxel);
}
