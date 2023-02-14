// use eframe::glow::{self, NativeBuffer, NativeVertexArray, HasContext};

// pub unsafe fn create_vertex_buffer(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {
//     // This is a flat array of f32s that are to be interpreted as vec2s.
//     let triangle_vertices = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];
//     let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
//         triangle_vertices.as_ptr() as *const u8,
//         triangle_vertices.len() * core::mem::size_of::<f32>(),
//     );

//     // We construct a buffer and upload the data
//     let vbo = gl.create_buffer().unwrap();
//     gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
//     gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);

//     // We now construct a vertex array to describe the format of the input buffer
//     let vao = gl.create_vertex_array().unwrap();
//     gl.bind_vertex_array(Some(vao));
//     gl.enable_vertex_attrib_array(0);
//     gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);


//     (vbo, vao)
// }

// pub unsafe fn create_index_buffer(gl: &glow::Context) -> NativeBuffer {
//     // This is a flat array of u32s that are to be interpreted as indices.
//     let triangle_indices = [0u32, 1u32, 2u32];
//     let triangle_indices_u8: &[u8] = core::slice::from_raw_parts(
//         triangle_indices.as_ptr() as *const u8,
//         triangle_indices.len() * core::mem::size_of::<u32>(),
//     );

//     // We construct a buffer and upload the data
//     let ibo = gl.create_buffer().unwrap();
//     gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
//     gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, triangle_indices_u8, glow::STATIC_DRAW);

//     ibo
// }

// pub unsafe fn update_vertex_buffer(gl: &glow::Context, vbo: NativeBuffer, vertices: Vec<super::Vertex>) {
//     let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
//         vertices.as_ptr() as *const u8,
//         vertices.len() * core::mem::size_of::<f32>(),
//     );

//     // We construct a buffer and upload the data
//     gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
//     gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);
// }

// pub unsafe fn update_index_buffer(gl: &glow::Context, ibo: NativeBuffer, indices: Vec<u32>) {
//     let triangle_indices_u8: &[u8] = core::slice::from_raw_parts(
//         indices.as_ptr() as *const u8,
//         indices.len() * core::mem::size_of::<u32>(),
//     );

//     // We construct a buffer and upload the data
//     gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
//     gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, triangle_indices_u8, glow::STATIC_DRAW);
// }