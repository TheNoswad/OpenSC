// use std::sync::Arc;

// use eframe::{
//     egui_wgpu::wgpu::util::DeviceExt,
//     egui_wgpu::{self, wgpu}, wgpu::{Device, ShaderModule},
// };

// pub fn create_shader(device: &Arc<Device>) -> ShaderModule {
//     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: Some("shader"),
//         source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
//         //flags: wgpu::ShaderFlags::all(),
//     });

//     shader
// }