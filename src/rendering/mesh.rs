use super::Vertex;

/// A 3d mesh
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn empty() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
        }
    }

    pub fn get_vertices_slice(&self) -> &[u8] {
        unsafe {
            let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
                self.vertices.as_ptr() as *const u8,
                self.vertices.len() * core::mem::size_of::<Vertex>(),
            );
            return triangle_vertices_u8;
        }
    }

    pub fn get_indices_slice(&self) -> &[u8] {
        unsafe {
            let triangle_indices_u8: &[u8] = core::slice::from_raw_parts(
                self.indices.as_ptr() as *const u8,
                self.indices.len() * core::mem::size_of::<u32>(),
            );
            return triangle_indices_u8;
        }
    }
}