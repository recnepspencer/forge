//! GPU shader pipeline for Liquid Glass UI rendering.
//!
//! DOMAIN: Manages the wgpu render pipeline for the glass shader that reads
//! a blurred backdrop texture (group 1) and uniforms (group 0). Each button
//! instance owns its own per-draw bind group so multiple elements don't
//! clobber each other.
//! DEPENDENCIES: wgpu, egui-wgpu, crate::backdrop::BackdropManager.

use std::sync::Mutex;
use wgpu::util::DeviceExt;

use crate::backdrop::BackdropManager;

/// Uniform data sent to the glass shader.
///
/// Layout must match the WGSL `Uniforms` struct exactly.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MetalUniforms {
    /// xy = rect_min (pixels), zw = rect_size (pixels)
    pub rect: [f32; 4],
    /// rgba base color (linear)
    pub base_color: [f32; 4],
    /// x = gloss, y = highlight_shift, z = rim_alpha, w = rounding
    pub params: [f32; 4],
    /// x = time, y = pressed (0 or 1), z = mouse_x (pixels), w = mouse_y (pixels)
    pub params2: [f32; 4],
    /// xy = screen_size (physical pixels), zw = unused
    pub screen: [f32; 4],
}

/// The shared pipeline object stored in `CallbackResources`.
///
/// Contains the render pipeline and the group-0 BGL (uniforms).
/// Group-1 BGL and bind group come from `BackdropManager`.
pub struct MetalPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl MetalPipeline {
    /// Create the glass shader pipeline using both bind group layouts.
    ///
    /// `backdrop_bgl` is the group-1 layout from `BackdropManager`.
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        backdrop_bgl: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader_source = include_str!("../shaders/metal_rect.wgsl");
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("glass_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Group 0: uniforms
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("glass_uniforms_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(
                        std::num::NonZeroU64::new(std::mem::size_of::<MetalUniforms>() as u64)
                            .expect("MetalUniforms is not zero-sized"),
                    ),
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("glass_pipeline_layout"),
            // group 0 = uniforms, group 1 = blurred backdrop texture
            bind_group_layouts: &[&bind_group_layout, backdrop_bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("glass_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

/// Per-instance GPU resources created in `prepare()` and consumed in `paint()`.
struct PerDrawResources {
    /// Bind group for group 0 (uniforms).
    uniforms_bind_group: wgpu::BindGroup,
    /// Keep alive until paint() finishes.
    _uniform_buffer: wgpu::Buffer,
}

/// Callback that draws one glass rectangle using the Liquid Glass shader.
///
/// Each button/card creates its own `MetalShaderCallback`. Stores per-draw
/// resources (uniforms BG) inside the instance via `Mutex<Option<...>>` so
/// multiple callbacks don't clobber each other.
pub struct MetalShaderCallback {
    pub uniforms: MetalUniforms,
    per_draw: Mutex<Option<PerDrawResources>>,
}

impl MetalShaderCallback {
    pub fn new(uniforms: MetalUniforms) -> Self {
        Self {
            uniforms,
            per_draw: Mutex::new(None),
        }
    }
}

impl egui_wgpu::CallbackTrait for MetalShaderCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let pipeline: &MetalPipeline = callback_resources
            .get()
            .expect("MetalPipeline missing from CallbackResources");

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("glass_uniforms_buf"),
            contents: bytemuck::bytes_of(&self.uniforms),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let uniforms_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glass_uniforms_bg"),
            layout: &pipeline.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        *self.per_draw.lock().expect("per_draw lock poisoned") = Some(PerDrawResources {
            uniforms_bind_group,
            _uniform_buffer: uniform_buffer,
        });

        Vec::new()
    }

    fn paint<'a>(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        let pipeline: &MetalPipeline = callback_resources.get().expect("MetalPipeline missing");
        let backdrop: &BackdropManager = callback_resources
            .get()
            .expect("BackdropManager missing from CallbackResources");

        let guard = self.per_draw.lock().expect("per_draw lock poisoned");
        let draw = guard
            .as_ref()
            .expect("prepare() was not called before paint()");

        render_pass.set_pipeline(&pipeline.pipeline);
        render_pass.set_bind_group(0, &draw.uniforms_bind_group, &[]);
        render_pass.set_bind_group(1, &backdrop.glass_bind_group, &[]);

        let clip = info.clip_rect_in_pixels();
        render_pass.set_scissor_rect(
            clip.left_px as u32,
            clip.top_px as u32,
            clip.width_px as u32,
            clip.height_px as u32,
        );

        render_pass.draw(0..3, 0..1);
    }
}

/// A zero-size callback placed *before* any glass elements each frame.
///
/// Its `prepare()` runs the two-pass Gaussian blur and returns the
/// `CommandBuffer`s so they're submitted to the GPU before the render pass
/// begins — ensuring `blurred_texture` is fully ready for all glass
/// `paint()` calls in the same frame.
pub struct BlurCaptureCallback;

impl egui_wgpu::CallbackTrait for BlurCaptureCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let backdrop: &BackdropManager = match callback_resources.get() {
            Some(b) => b,
            None => return Vec::new(),
        };
        backdrop.run_blur_passes(device, queue)
    }

    fn paint<'a>(
        &self,
        _info: egui::PaintCallbackInfo,
        _render_pass: &mut wgpu::RenderPass<'static>,
        _callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        // No draw call — this callback exists only to trigger the blur passes.
    }
}

/// Register the MetalPipeline and BackdropManager with egui-wgpu's resources.
///
/// Call once during `ForgeApp::new()` after obtaining the wgpu render state.
/// `width` and `height` should be the initial window size in physical pixels.
pub fn register_metal_pipeline(render_state: &egui_wgpu::RenderState) {
    register_with_size(render_state, 1400, 900);
}

/// Register with explicit initial screen size (physical pixels).
pub fn register_with_size(render_state: &egui_wgpu::RenderState, width: u32, height: u32) {
    let backdrop = BackdropManager::new(
        &render_state.device,
        render_state.target_format,
        width,
        height,
    );

    // Build the pipeline using the backdrop's texture BGL (group 1)
    let pipeline = MetalPipeline::new(
        &render_state.device,
        render_state.target_format,
        &backdrop.glass_bgl,
    );

    let mut renderer = render_state.renderer.write();
    renderer.callback_resources.insert(backdrop);
    renderer.callback_resources.insert(pipeline);
}
