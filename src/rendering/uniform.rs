// use eframe::glow::{self, NativeProgram, HasContext};

// pub unsafe fn set_uniform(gl: &glow::Context, program: NativeProgram, name: &str, value: f32) {
//     let uniform_location = gl.get_uniform_location(program, name);
//     // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
//     gl.uniform_1_f32(uniform_location.as_ref(), value)
// }