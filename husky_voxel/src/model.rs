use mlua::{UserData, UserDataMethods};

/// Voxels are 4 byte large, to facilitate storing colour and
/// material properties. I could have opted for a material/colour
/// palette, but that would have severely limited the amount of
/// colours/materials I could use, and would make the interface
/// a lot less intuitive.
#[derive(Copy, Clone)]
pub struct Voxel(u32);

impl Voxel {
    pub fn new(r: u8, g: u8, b: u8, roughness: u8, metalness: u8) -> Self {
        let mut colour: u16 = 0;
        colour |= (r as u16) & 0b0000_0000_0001_1111;
        colour |= (g as u16) & 0b0000_0111_1110_0000;
        colour |= (b as u16) & 0b1111_1000_0000_0000;
        let mut mat: u16 = 0;
        mat |= roughness as u16;
        mat |= (metalness as u16) << 8;
        let mut data: u32 = 0;
        data |= colour as u32;
        data |= (mat as u32) << 16;
        Self(data)
    }
}

impl UserData for Voxel {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("setColor", |_, voxel, (r,g,b): (f32, f32, f32)| {
            todo!();
            Ok(())
        });
    }
}

/// Bricks are a fundamental part of our memory management.
/// By storing only 64x64x64 blocks of voxels raw, we can still
/// exclude storing large blocks of empty voxels. Using these
/// bricks is less efficient than using a sparse octree, but
/// it is much quicker to modify. These bricks will be stored
/// in a multi-level array, so both raytracing and modifying
/// them is easy. The multi-level array relies on 16 bit
/// coordinates which is reflected in the bricks as well.
pub struct Brick {
    pub pos: (u16, u16, u16),
    pub data: [Voxel; 64*64*64], //64x64x64 voxels = 1 megabyte (voxel is 4 bytes)
}

impl Brick {
    pub fn empty(x: u16, y: u16, z: u16) -> Self {
        Self {
            pos: (x,y,z),
            data: [Voxel::new(255, 255, 255, 255, 0); 64*64*64]
        }
    }
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
