use core::panic;

use crate::rendering::{Vertex};
use crate::furniture::FurnitureDesign;

pub fn mesh_furniture(furniture: &FurnitureDesign) -> crate::rendering::Mesh {
    let resolution = furniture.resolution;

    // This is the mesh that is being built and returned
    let mut tempmesh = crate::rendering::Mesh::empty();

    // A counter for indices when building the mesh
    let mut indices_count: u32 = 0;

    // A counter for the iterations in the loop
    let mut count = 0;
    //dbg!(&furniture.values_expanded[8191]);
    // dbg!(furniture.values_expanded.len());
    // dbg!(&furniture.resolution);

    // Iterates over the entire furniture and generates the mesh
    for z in 0..resolution -1 {
        for x in 0..resolution -1 {
            for y in 0..resolution -1 {
                //println!("{} {} {} {} {}", x, y, z, count, &furniture.values_expanded[count]);
                //dbg!(furniture.values_expanded[count]);
                if furniture.values_expanded[count] != 0 {
                    
                    //Generate the block mesh to be added to the chunk mesh
                    let mut block_mesh = generate_block_mesh(x, y, z, furniture.values_expanded[count], indices_count, [false; 6]);

                    // Append mesh vertices to the tempmesh
                    tempmesh.vertices.append(&mut block_mesh.0);
                    tempmesh.indices.append(&mut block_mesh.1);

                    // Add the number of indices in the block mesh to the indices count of the entire chunk
                    indices_count = block_mesh.2;
                }
                count += 1;
            }
        }
    }
    //panic!();
    tempmesh
}

/// Generate a mesh for a single block.
/// Returns vertices, indices, and indices count
pub fn generate_block_mesh(x: u32, y: u32, z: u32, block: u32, indices_count: u32, face_occulsion: [bool; 6]) -> (Vec<Vertex>, Vec<u32>, u32) {
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

    //let blockid = block.id as usize;
    //let textureslot = &blockatlas.blocks[blockid].uv;

    //let texel_size = (1.0 / TEXTURE_ATLAS_SIZE as f32) / 2.0;

    // textureslot[0] is the x and [1] is y

    // let texcoordsx = vec2(0.0, 0.0);
    // let texcoordsy = vec2(0.0, 0.0);

    // if textureslot[0] == 0 {
    //     let texcoordsx = vec2((textureslot[0] as f32 / TEXTURE_ATLAS_SIZE as f32), (textureslot[0] as f32 + 1.0 / TEXTURE_ATLAS_SIZE as f32) - texel_size);
    // }
    // else {
    //     let texcoordsx = vec2((textureslot[0] as f32 / TEXTURE_ATLAS_SIZE as f32) - texel_size, (textureslot[0] as f32 + 1.0 / TEXTURE_ATLAS_SIZE as f32) - texel_size);
    // }

    // if textureslot[1] == 0 {
    //     let texcoordsy = vec2((textureslot[1] as f32 / TEXTURE_ATLAS_SIZE as f32), (textureslot[1] as f32 + 1.0 / TEXTURE_ATLAS_SIZE as f32) - texel_size);
    // }
    // else {
    //     let texcoordsy = vec2((textureslot[1] as f32 / TEXTURE_ATLAS_SIZE as f32) - texel_size, (textureslot[1] as f32 + 1.0 / TEXTURE_ATLAS_SIZE as f32) - texel_size);
    // }


    // ===========================================================================================================
    // IMPL TEXCOORDS LATER
    //let texcoordsx = vec2(textureslot[0] as f32 / TEXTURE_ATLAS_SIZE as f32, textureslot[0] as f32 + 1.0 / TEXTURE_ATLAS_SIZE as f32);
    //let texcoordsy = vec2(textureslot[1] as f32 / TEXTURE_ATLAS_SIZE as f32, textureslot[1] as f32 + 1.0 / TEXTURE_ATLAS_SIZE as f32);




    // ===========================================================================================================
    // Allocates texcoords for the vertices
    // Please note! The x value of the texcoord is the start range of the area and the y is the end of that range.
    // ===========================================================================================================




    let texcoords = vec![];
    //     // Bottom left front
    //     // 0
    //     //         7-------6
    //     //        /|      /|
    //     //       / |     / |
    //     //      4--|----5  |
    //     //      |  3----|--2
    //     //      | /     | /
    //     // ■■■▶ 0-------1
    //     texcoord([texcoordsx.x, texcoordsy.y]),
        
    //     // Bottom right front
    //     // 1
    //     //         7-------6
    //     //        /|      /|
    //     //       / |     / |
    //     //      4--|----5  |
    //     //      |  3----|--2
    //     //      | /     | /
    //     //      0-------1 ◀■■■
    //     texcoord([texcoordsx.y, texcoordsy.y]),
        
    //     // Bottom right back
    //     // 2
    //     //         7-------6
    //     //        /|      /|
    //     //       / |     / |
    //     //      4--|----5  |
    //     //      |  3----|--2 ◀■■■
    //     //      | /     | /
    //     //      0-------1
    //     texcoord([texcoordsx.x, texcoordsy.y]),
        
    //     // Bottom left back
    //     // 3
    //     //         7-------6
    //     //        /|      /|
    //     //       / |     / |
    //     //      4--|----5  |
    //     //   ■■■|▶ 3----|--2
    //     //      | /     | /
    //     //      0-------1
    //     texcoord([texcoordsx.y, texcoordsy.y]),


    //     // Top left front
    //     // 4
    //     //         7-------6
    //     //        /|      /|
    //     //       / |     / |
    //     // ■■■▶ 4--|----5  |
    //     //      |  3----|--2
    //     //      | /     | /
    //     //      0-------1
    //     texcoord([texcoordsx.x, texcoordsy.x]),
        
    //     // Top right front
    //     // 5
    //     //         7-------6
    //     //        /|      /|
    //     //       / |     / |
    //     //      4--|----5 ◀|■■■
    //     //      |  3----|--2
    //     //      | /     | /
    //     //      0-------1
    //     texcoord([texcoordsx.y, texcoordsy.x]),
        
    //     // Top right back
    //     // 6
    //     //         7-------6 ◀■■■
    //     //        /|      /|
    //     //       / |     / |
    //     //      4--|----5  |
    //     //      |  3----|--2
    //     //      | /     | /
    //     //      0-------1
    //     texcoord([texcoordsx.x, texcoordsy.x]),
        
    //     // Top left back
    //     // 7
    //     //    ■■■▶ 7-------6
    //     //        /|      /|
    //     //       / |     / |
    //     //      4--|----5  |
    //     //      |  3----|--2
    //     //      | /     | /
    //     //      0-------1
    //     texcoord([texcoordsx.y, texcoordsy.x]),
    // ];
    



    // =================================================
    // Allocates vertices for all 8 corners of the cube
    // =================================================




    let vertex_positions = vec![
        // Bottom left front
        // 0
        //         7-------6
        //        /|      /|
        //       / |     / |
        //      4--|----5  |
        //      |  3----|--2
        //      | /     | /
        // ■■■▶ 0-------1
        position([0.0 + x_as_float, 0.0 + y_as_float, 0.0 + z_as_float]),

        // Bottom right front
        // 1
        //         7-------6
        //        /|      /|
        //       / |     / |
        //      4--|----5  |
        //      |  3----|--2
        //      | /     | /
        //      0-------1 ◀■■■
        position([1.0 + x_as_float, 0.0 + y_as_float, 0.0 + z_as_float]),
  
        // Bottom right back
        // 2
        //         7-------6
        //        /|      /|
        //       / |     / |
        //      4--|----5  |
        //      |  3----|--2 ◀■■■
        //      | /     | /
        //      0-------1
        position([1.0 + x_as_float, 0.0 + y_as_float, 1.0 + z_as_float]),

        // Bottom left back
        // 3
        //         7-------6
        //        /|      /|
        //       / |     / |
        //      4--|----5  |
        //   ■■■|▶ 3----|--2
        //      | /     | /
        //      0-------1
        position([0.0 + x_as_float, 0.0 + y_as_float, 1.0 + z_as_float]),



        // Top left front
        // 4
        //         7-------6
        //        /|      /|
        //       / |     / |
        // ■■■▶ 4--|----5  |
        //      |  3----|--2
        //      | /     | /
        //      0-------1
        position([0.0 + x_as_float, 1.0 + y_as_float, 0.0 + z_as_float]),

        // Top right front
        // 5
        //         7-------6
        //        /|      /|
        //       / |     / |
        //      4--|----5 ◀|■■■
        //      |  3----|--2
        //      | /     | /
        //      0-------1
        position([1.0 + x_as_float, 1.0 + y_as_float, 0.0 + z_as_float]),

        // Top right back
        // 6
        //         7-------6 ◀■■■
        //        /|      /|
        //       / |     / |
        //      4--|----5  |
        //      |  3----|--2
        //      | /     | /
        //      0-------1
        position([1.0 + x_as_float, 1.0 + y_as_float, 1.0 + z_as_float]),

        // Top left back
        // 7
        //    ■■■▶ 7-------6
        //        /|      /|
        //       / |     / |
        //      4--|----5  |
        //      |  3----|--2
        //      | /     | /
        //      0-------1
        position([0.0 + x_as_float, 1.0 + y_as_float, 1.0 + z_as_float]),
    ];
    



    // ==============================================
    //             Generate Indices
    // ==============================================
    //     7-------6
    //     /|      /|
    //    / |     / |
    //   4--|----5  |
    //   |  3----|--2
    //   | /     | /
    //   0-------1




    let mut indices: Vec<u32> = vec![];
    for i in 1..7 {
        match i {
            // Front Face
            //      7-------6
            //     /|      /|
            //    / |     / |
            //   4▄▄▄▄▄▄▄5  |
            //   |███████|--2
            //   |███████| /
            //   0▀▀▀▀▀▀▀1
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
            //      7▄▄▄▄▄▄▄6
            //     /|██████▚|
            //    / |█████▚█|
            //   4--|▒▒▒▒5██|
            //   |  3▀▀▀▀|▀▀2
            //   | /     | /
            //   0-------1
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
            //      7-------6
            //     /|      /|
            //    /█|     / |
            //   4▒▒|----5  |
            //   |██3----|--2
            //   |█/     | /
            //   0-------1
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
            //      7-------6
            //     /|      /|
            //    / |     /█|
            //   4--|----5██|
            //   |  3----|▒▒2
            //   | /     |█/
            //   0-------1
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
            //      7▄▄▄▄▄▄▄6
            //     /▒██████/|
            //    /█▒█████/ |
            //   4▀▀▒▀▀▀▀5  |
            //   |  3----|--2
            //   | /     | /
            //   0-------1
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
            //      7-------6
            //     /|      /|
            //    / |     / |
            //   4--|----5  |
            //   |  3▄▄▄▄▒▄▄2
            //   | /█████▒█/
            //   0▀▀▀▀▀▀▀1
            6 =>{
                if face_occulsion[5] != true {
                    indices.append(&mut vec![
                        0 + indices_cnt_offset, 1 + indices_cnt_offset, 2 + indices_cnt_offset,
                        2 + indices_cnt_offset, 3 + indices_cnt_offset, 0 + indices_cnt_offset
                    ]);
                    //indices_count += 1
                }
                
            }
            // Makes the linter shut up.
            // TODO FIX THIS
            _ => {
                println!("Run and hide");
                panic!("Oh shit.")
            },
        }
    }

    // Create an iterator that combines positions and texcoords
    let vertices_iter = vertex_positions.iter().zip(texcoords.iter());

    // A vec to store the final vertices
    let mut vertices = vec![]; 

    for (position, texcoords) in vertices_iter {
        let vertex = Vertex::new(*position, *texcoords);
        vertices.push(vertex)
    }

    return (vertices, indices, indices_count + 8)// og was 8!!!!
}

/// Generates a texcoord
fn texcoord(pos: [f32; 2]) -> [f32; 2] {
    pos
}

/// Generates a vertex
fn position(pos: [f32; 3]) -> [f32; 3] {
    pos
}