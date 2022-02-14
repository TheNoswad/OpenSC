use glam::{Vec3, vec2, Vec2};
use rend3::types::{Mesh, MeshBuilder};

use crate::chunks32h::Chunk;

const TEXTURE_ATLAS_SIZE: u8 = 16;

pub fn generate_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut vertices: Vec<Vec3> = vec![];
    let mut indices: Vec<u32> = vec![];

    let mut count = 0;
    let mut indices_count = 0;

    for z in 0..15 {
        for x in 0..15 {
            for y in 0..255 {
                let block = chunk.blocks[count].blockdata;
                if block != 0 {
                    let face_occulsion = compute_face_oculsion(chunk, x, y, z);
                    let block_mesh = generate_block_mesh(x, y, z, block, indices_count, face_occulsion);
                }
                count += 1;
            }
        }
    }
    let mesh = MeshBuilder::new(vertices, rend3::types::Handedness::Left)
        .with_indices(indices)
        .build()
        .unwrap();
    return mesh
}

/// Computes faces with adjacent blocks to ignore meshing
fn compute_face_oculsion(
    chunk: &Chunk,
    x: u8, y: u8, z: u8,
    //top_chunk: Option<&world::Chunk>,
    //bottom_chunk: Option<&world::Chunk>,
    //left_chunk: Option<&world::Chunk>,
    //right_chunk: Option<&world::Chunk>,
    //front_chunk: Option<&world::Chunk>,
    //back_chunk: Option<&world::Chunk>,
) -> [bool; 6] {
    let mut top = false;
    let mut bottom = false;
    let mut left = false;
    let mut right = false;
    let mut front = false;
    let mut back = false;

    match x {
        // If the x axis is at the far left (0)
        0 => {
            // match left_chunk {
            //     // Check if the left chunk exists
            //     Some(unwrapped) => {
            //         // If it does exist, get the desired block in the chunk
            //         match unwrapped.get_block(7.0, y.into(), z.into()).id {
            //             0 => left = false,
            //             _ => left = true
            //         }
            //     },
            //     // If the chunk does not exist
            //     None => left = false,
            //}
             // Right
             match chunk.get_block((x + 1).into(), y.into(), z.into()) {
                0 => right = false,
                _ => right = true
            }
        },

        // If the x axis is at the far right (7)
        7 => {
            // match right_chunk {
            //     // Check if the right chunk exists
            //     Some(unwrapped) => {
            //         // If it does exist, get the desired block in the chunk
            //         match unwrapped.get_block(0.0, y.into(), z.into()).id {
            //             0 => right = false,
            //             _ => right = true
            //         }
            //     },
            //     // If the chunk does not exist
            //     None => right = false,
            // }
            // Left
            match chunk.get_block((x - 1).into(), y.into(), z.into()) {
                0 => left = false,
                _ => left = true
            }
        },

        _ => {
            // Left
            match chunk.get_block((x - 1).into(), y.into(), z.into()) {
                0 => left = false,
                _ => left = true
            }

            // Right
            match chunk.get_block((x + 1).into(), y.into(), z.into()) {
                0 => right = false,
                _ => right = true
            }
        }
    }

    match y {
        0 => {
            // match bottom_chunk {
            //     Some(unwrapped) => {
            //         match unwrapped.get_block(x.into(), 7.0, z.into()).id {
            //             0 => bottom = false,
            //             _ => bottom = true
            //         }
            //     },
            //     None => bottom = false,
            // }
            // Top
            match chunk.get_block(x.into(), (y + 1).into(), z.into()) {
                0 => top = false,
                _ => top = true
            }
        },

        7 => {
            // match top_chunk {
            //     Some(unwrapped) => {
            //         match unwrapped.get_block(x.into(), 0.0, z.into()).id {
            //             0 => top = false,
            //             _ => top = true
            //         }
            //     },
            //     None => top = false,
            // }
            // Bottom
            match chunk.get_block(x.into(), (y - 1).into(), z.into()) {
                0 => bottom = false,
                _ => bottom = true
            }
        },

        _ => {
            // Bottom
            match chunk.get_block(x.into(), (y - 1).into(), z.into()) {
                0 => bottom = false,
                _ => bottom = true
            }

            // Top
            match chunk.get_block(x.into(), (y + 1).into(), z.into()) {
                0 => top = false,
                _ => top = true
            }
        }
    }

    match z {
        0 => {
            // match front_chunk {
            //     Some(unwrapped) => {
            //         match unwrapped.get_block(x.into(), y.into(), 7.0).id {
            //             0 => front = false,
            //             _ => front = true
            //         }
            //     },
            //     None => front = false,
            // }
            // Back
            match chunk.get_block(x.into(), y.into(), (z + 1).into()) {
                0 => back = false,
                _ => back = true
            }
        },
        7 => {
            //back = false;
            // match back_chunk {
            //     Some(unwrapped) => {
            //         match unwrapped.get_block(x.into(), y.into(), 0.0).id {
            //             0 => back = false,
            //             _ => back = true
            //         }
            //     },
            //     None => back = false,
            // }
            // Front
            match chunk.get_block(x.into(), y.into(), (z - 1).into()) {
                0 => front = false,
                _ => front = true
            }
        },
        _ => {
            // Front
            match chunk.get_block(x.into(), y.into(), (z - 1).into()) {
                0 => front = false,
                _ => front = true
            }

            // Back
            match chunk.get_block(x.into(), y.into(), (z + 1).into()) {
                0 => back = false,
                _ => back = true
            }
        }
    }
    return [front, back, left, right, top, bottom]
}

/// Generate a mesh for a single block
pub fn generate_block_mesh (x: u8, y: u8, z: u8, block: u32, indices_count: u32, face_occulsion: [bool; 6]) -> (Vec<Vec3>, Vec<u32>, Vec<Vec2>, u32) {
    // Stores the coordinate values of the block as a f32
    let x_as_float = x as f32;
    let y_as_float = y as f32;
    let z_as_float = z as f32;

    // Calculates the offset of the indices
    // This is needed because we are adding indicies to the chunk mesh in the meshing function
    let indices_cnt_offset: u32;

    if indices_count == 0 {
        indices_cnt_offset = 0;
    }
    else {
        indices_cnt_offset = indices_count; // og is 8!!!!
    }

    // let blockid = block as usize;
    // let textureslot = &blockatlas.blocks[blockid].uv;

    // let texcoordsx = vec2(textureslot[0] as f32 / TEXTURE_ATLAS_SIZE as f32, textureslot[0] as f32 + 1.0 / TEXTURE_ATLAS_SIZE as f32);
    // let texcoordsy = vec2(textureslot[1] as f32 / TEXTURE_ATLAS_SIZE as f32, textureslot[1] as f32 + 1.0 / TEXTURE_ATLAS_SIZE as f32);
    let texcoordsx = vec2(0.0, 1.0);
    let texcoordsy = vec2(0.0, 1.0);


    // Allocates texcoords for the vertices
    // Please note! The x value of the texcoord is the start range of the area and the y is the end of that range.
    let texcoords = vec![
        // Bottom left front
        // 0
        vec2(texcoordsx.x, texcoordsy.y),
        
        // Bottom right front
        // 1
        vec2(texcoordsx.y, texcoordsy.y),
        
        // Bottom right back
        // 2
        vec2(texcoordsx.x, texcoordsy.y),
        
        // Bottom left back
        // 3
        vec2(texcoordsx.y, texcoordsy.y),


        // Top left front
        // 4
        vec2(texcoordsx.x, texcoordsy.x),
        
        // Top right front
        // 5
        vec2(texcoordsx.y, texcoordsy.x),
        
        // Top right back
        // 6
        vec2(texcoordsx.x, texcoordsy.x),
        
        // Top left back
        // 7
        vec2(texcoordsx.y, texcoordsy.x),
    ];

    // Allocates vertices for all 8 corners of the cube
    let vertices = vec![
        // Bottom left front
        // 0
        vertex([0.0 + x_as_float, 0.0 + y_as_float, 0.0 + z_as_float]),
        // Vertex {
        //     position: [0.0 + x_as_float, 0.0 + y_as_float, 0.0 + z_as_float],
        //     texture: [0.0, 0.0]
        // },
        // Bottom right front
        // 1
        vertex([1.0 + x_as_float, 0.0 + y_as_float, 0.0 + z_as_float]),
        // Vertex {
        //     position: [1.0 + x_as_float, 0.0 + y_as_float, 0.0 + z_as_float],
        //     texture: [1.0, 0.0]
        // },
        // Bottom right back
        // 2
        vertex([1.0 + x_as_float, 0.0 + y_as_float, 1.0 + z_as_float]),
        // Vertex {
        //     position: [1.0 + x_as_float, 0.0 + y_as_float, 1.0 + z_as_float],
        //     texture: [1.0, 1.0]
        // },
        // Bottom left back
        // 3
        vertex([0.0 + x_as_float, 0.0 + y_as_float, 1.0 + z_as_float]),
        // Vertex {
        //     position: [0.0 + x_as_float, 0.0 + y_as_float, 1.0 + z_as_float],
        //     texture: [0.0, 1.0]
        // },


        // Top left front
        // 4
        vertex([0.0 + x_as_float, 1.0 + y_as_float, 0.0 + z_as_float]),
        // Vertex {
        //     position: [0.0 + x_as_float, 1.0 + y_as_float, 0.0 + z_as_float],
        //     texture: [0.0, 1.0]
        // },
        // Top right front
        // 5
        vertex([1.0 + x_as_float, 1.0 + y_as_float, 0.0 + z_as_float]),
        // Vertex {
        //     position: [1.0 + x_as_float, 1.0 + y_as_float, 0.0 + z_as_float],
        //     texture: [1.0, 1.0]
        // },
        // Top right back
        // 6
        vertex([1.0 + x_as_float, 1.0 + y_as_float, 1.0 + z_as_float]),
        // Vertex {
        //     position: [1.0 + x_as_float, 1.0 + y_as_float, 1.0 + z_as_float],
        //     texture: [1.0, 0.0]
        // },
        // Top left back
        // 7
        vertex([0.0 + x_as_float, 1.0 + y_as_float, 1.0 + z_as_float]),
        // Vertex {
        //     position: [0.0 + x_as_float, 1.0 + y_as_float, 1.0 + z_as_float],
        //     texture: [0.0, 0.0]
        // },
    ];
    let mut indices: Vec<u32> = vec![];

    for i in 1..7 {
        match i {
            // Front Face
            1 => {
                if face_occulsion[0] != true {
                    indices.append(&mut vec![
                        0 + indices_cnt_offset, 1  + indices_cnt_offset, 5  + indices_cnt_offset,
                        5 + indices_cnt_offset, 4 + indices_cnt_offset, 0 + indices_cnt_offset
                        ]);
                    //indices_count += 1
                }
            },
            // Back Face
            2 => {
                if face_occulsion[1] != true {
                    indices.append(&mut vec![
                        2 + indices_cnt_offset, 3 + indices_cnt_offset, 7 + indices_cnt_offset,
                        7 + indices_cnt_offset, 6 + indices_cnt_offset, 2 + indices_cnt_offset// the last one was three... idk maybe it works
                    ]);
                    //indices_count += 1
                }
                
            },
            // Left Face
            3 => {
                if face_occulsion[2] != true {
                    indices.append(&mut vec![
                        3 + indices_cnt_offset, 0 + indices_cnt_offset, 4 + indices_cnt_offset,
                        4 + indices_cnt_offset, 7 + indices_cnt_offset, 3 + indices_cnt_offset
                    ]);
                    //indices_count += 1
                }
                
            },
            // Right Face
            4 => {
                if face_occulsion[3] != true {
                    indices.append(&mut vec![
                        1 + indices_cnt_offset, 2 + indices_cnt_offset, 6 + indices_cnt_offset, 
                        6 + indices_cnt_offset, 5 + indices_cnt_offset, 1 + indices_cnt_offset
                    ]);
                    //indices_count += 1
                }
                
            },
            // Top Face
            5 => {
                if face_occulsion[4] != true {
                    indices.append(&mut vec![
                        4 + indices_cnt_offset, 5 + indices_cnt_offset, 6 + indices_cnt_offset,
                        6 + indices_cnt_offset, 7 + indices_cnt_offset, 4 + indices_cnt_offset
                    ]);
                    //indices_count += 1
                }
                
            },
            // Bottom Face
            6 =>{
                if face_occulsion[5] != true {
                    indices.append(&mut vec![
                        0 + indices_cnt_offset, 1 + indices_cnt_offset, 2 + indices_cnt_offset,
                        2 + indices_cnt_offset, 3 + indices_cnt_offset, 0 + indices_cnt_offset
                    ]);
                    //indices_count += 1
                }
                
            }
            // Bad shit. Makes the linter shut up.
            _ => {
                println!("Run and hide");
                panic!("Oh shit.")
            },
        }
    }
    return (vertices, indices, texcoords, indices_count + 8)// og was 8!!!!
}

/// Generates a vertex
fn vertex(pos: [f32; 3]) -> glam::Vec3 {
    glam::Vec3::from(pos)
}