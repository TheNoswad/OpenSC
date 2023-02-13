#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2]) -> Self {
        Self { pos, uv }
    }
}