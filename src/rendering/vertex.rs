
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], texcoords: [f32; 2]) -> Self {
        Self { position, tex_coords: texcoords }
    }

}