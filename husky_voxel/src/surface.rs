#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Material {
    pub roughness: f32,
    pub metalness: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
