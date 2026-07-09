//! Backdrop capture and blur infrastructure for the Liquid Glass effect.
//!
//! DOMAIN: Manages a pair of screen-sized GPU textures and a two-pass separable
//! Gaussian blur pipeline. Each frame, the previous frame's surface is copied to
//! `backdrop_texture`, blurred horizontally then vertically, and the final
//! `blurred_texture` is made available to the glass shader via `bind_group`.
//!
//! DEPENDENCIES: wgpu, egui-wgpu.

use wgpu::util::DeviceExt;

/// Screen-sized texture resources for backdrop blur.
pub struct BackdropManager {
    /// Full-screen capture of the previous rendered frame.
    pub backdrop_texture: wgpu::Texture,
    pub backdrop_view: wgpu::TextureView,

    /// Intermediate texture for the horizontal blur pass.
    blur_temp_texture: wgpu::Texture,
    blur_temp_view: wgpu::TextureView,

    /// Fully blurred output — sampled by the glass shader.
    pub blurred_texture: wgpu::Texture,
    pub blurred_view: wgpu::TextureView,

    /// Linear-clamp sampler shared by all blur operations.
    pub sampler: wgpu::Sampler,

    /// Render pipeline for a single blur pass (horiz or vert unified by uniform).
    blur_pipeline: wgpu::RenderPipeline,
    blur_bgl: wgpu::BindGroupLayout,

    /// Bind group layout for the glass shader's texture input (group 1).
    pub glass_bgl: wgpu::BindGroupLayout,
    /// Bind group used by the glass shader to sample the blurred result.
    pub glass_bind_group: wgpu::BindGroup,

    width: u32,
    height: u32,
}

impl BackdropManager {
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture_usage = wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC;

        let make_texture = |label: &str| -> wgpu::Texture {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: target_format,
                usage: texture_usage,
                view_formats: &[],
            })
        };

        let backdrop_texture = make_texture("backdrop_capture");
        let blur_temp_texture = make_texture("blur_temp");
        let blurred_texture = make_texture("blurred_output");

        let backdrop_view = backdrop_texture.create_view(&Default::default());
        let blur_temp_view = blur_temp_texture.create_view(&Default::default());
        let blurred_view = blurred_texture.create_view(&Default::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("backdrop_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // ── Blur pipeline ────────────────────────────────────────────────────

        let blur_shader_src = include_str!("../shaders/gaussian_blur.wgsl");
        let blur_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gaussian_blur_shader"),
            source: wgpu::ShaderSource::Wgsl(blur_shader_src.into()),
        });

        let blur_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("blur_bgl"),
            entries: &[
                // Blur uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            std::num::NonZeroU64::new(std::mem::size_of::<BlurUniforms>() as u64)
                                .unwrap(),
                        ),
                    },
                    count: None,
                },
                // Input texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let blur_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("blur_pipeline_layout"),
            bind_group_layouts: &[&blur_bgl],
            push_constant_ranges: &[],
        });

        let blur_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blur_pipeline"),
            layout: Some(&blur_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &blur_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &blur_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
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

        // ── Glass bind group layout (group 1) ────────────────────────────────

        let glass_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("glass_texture_bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let glass_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glass_texture_bg"),
            layout: &glass_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&blurred_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Self {
            backdrop_texture,
            backdrop_view,
            blur_temp_texture,
            blur_temp_view,
            blurred_texture,
            blurred_view,
            sampler,
            blur_pipeline,
            blur_bgl,
            glass_bgl,
            glass_bind_group,
            width,
            height,
        }
    }

    /// Copy `source_view` (the current surface texture) into `backdrop_texture`.
    ///
    /// Call this from `CallbackTrait::prepare()` **before** the blur passes.
    /// Uses `encoder.copy_texture_to_texture()` — zero-copy on GPU.
    pub fn capture_frame(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        source_texture: &wgpu::Texture,
    ) {
        encoder.copy_texture_to_texture(
            source_texture.as_image_copy(),
            self.backdrop_texture.as_image_copy(),
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
    }

    /// Run the two-pass Gaussian blur: backdrop → blur_temp (H) → blurred (V).
    ///
    /// Call this from `CallbackTrait::prepare()` after `capture_frame`.
    /// Returns the two command buffers to be submitted before the render pass.
    pub fn run_blur_passes(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Vec<wgpu::CommandBuffer> {
        let texel_size = [1.0 / self.width as f32, 1.0 / self.height as f32];

        let h_uniforms = BlurUniforms {
            direction: [1.0, 0.0],
            texel_size,
        };
        let v_uniforms = BlurUniforms {
            direction: [0.0, 1.0],
            texel_size,
        };

        let make_buf = |data: &BlurUniforms| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("blur_uniforms"),
                contents: bytemuck::bytes_of(data),
                usage: wgpu::BufferUsages::UNIFORM,
            })
        };

        let h_buf = make_buf(&h_uniforms);
        let v_buf = make_buf(&v_uniforms);

        let make_bg = |buf: &wgpu::Buffer, input: &wgpu::TextureView| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("blur_bg"),
                layout: &self.blur_bgl,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(input),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
            })
        };

        let h_bg = make_bg(&h_buf, &self.backdrop_view);
        let v_bg = make_bg(&v_buf, &self.blur_temp_view);

        let mut cbufs = Vec::new();

        // Horizontal pass: backdrop → blur_temp
        {
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("blur_h_encoder"),
            });
            {
                let mut rp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("blur_h_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.blur_temp_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                rp.set_pipeline(&self.blur_pipeline);
                rp.set_bind_group(0, &h_bg, &[]);
                rp.draw(0..3, 0..1);
            }
            cbufs.push(enc.finish());
        }

        // Vertical pass: blur_temp → blurred
        {
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("blur_v_encoder"),
            });
            {
                let mut rp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("blur_v_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.blurred_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                rp.set_pipeline(&self.blur_pipeline);
                rp.set_bind_group(0, &v_bg, &[]);
                rp.draw(0..3, 0..1);
            }
            cbufs.push(enc.finish());
        }

        cbufs
    }

    /// Returns the screen dimensions this manager was created for.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

/// Uniform buffer for a single Gaussian blur pass.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BlurUniforms {
    /// (1, 0) for horizontal, (0, 1) for vertical.
    pub direction: [f32; 2],
    /// 1/width, 1/height.
    pub texel_size: [f32; 2],
}

/// Register the BackdropManager with egui-wgpu's callback resources.
///
/// Called once during app startup.
pub fn register_backdrop_manager(render_state: &egui_wgpu::RenderState, width: u32, height: u32) {
    let manager = BackdropManager::new(
        &render_state.device,
        render_state.target_format,
        width,
        height,
    );
    render_state
        .renderer
        .write()
        .callback_resources
        .insert(manager);
}
