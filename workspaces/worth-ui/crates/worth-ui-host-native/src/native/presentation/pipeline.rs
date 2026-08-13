use crate::native::UiNativeGraphics;

const RETAINED_TO_SURFACE_SHADER: &str = r#"
@group(0) @binding(0) var retained: texture_2d<f32>;

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0)
    );
    return vec4<f32>(positions[index], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return textureLoad(retained, vec2<i32>(position.xy), 0);
}
"#;

const RASTER_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(position, 0.0, 1.0);
    output.color = color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
"#;

const RASTER_ATTRIBUTES: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];

fn transfer_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    pipeline_with_blend(device, shader, format, None, &[])
}

pub(super) fn raster_pipelines(
    device: &wgpu::Device,
) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("worth-ui-retained-raster"),
        source: wgpu::ShaderSource::Wgsl(RASTER_SHADER.into()),
    });
    let buffers = [wgpu::VertexBufferLayout {
        array_stride: 24,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &RASTER_ATTRIBUTES,
    }];
    let blended = pipeline_with_blend(
        device,
        &shader,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
        &buffers,
    );
    let replacing = pipeline_with_blend(
        device,
        &shader,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        Some(wgpu::BlendState::REPLACE),
        &buffers,
    );
    (blended, replacing)
}

fn pipeline_with_blend(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
    buffers: &[wgpu::VertexBufferLayout<'_>],
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("worth-ui-filled-rectangle-pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers,
            compilation_options: Default::default(),
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        multiview_mask: None,
        cache: None,
    })
}

pub(super) fn draw_raster_operations(
    encoder: &mut wgpu::CommandEncoder,
    target: &wgpu::TextureView,
    vertex_buffer: Option<&wgpu::Buffer>,
    replace_operations: &[bool],
    pipelines: (&wgpu::RenderPipeline, &wgpu::RenderPipeline),
    clear_target: bool,
) {
    let attachments = [Some(wgpu::RenderPassColorAttachment {
        view: target,
        depth_slice: None,
        resolve_target: None,
        ops: wgpu::Operations {
            load: if clear_target {
                wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
            } else {
                wgpu::LoadOp::Load
            },
            store: wgpu::StoreOp::Store,
        },
    })];
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("worth-ui-retained-raster-pass"),
        color_attachments: &attachments,
        ..Default::default()
    });
    let Some(vertex_buffer) = vertex_buffer else {
        return;
    };
    pass.set_vertex_buffer(0, vertex_buffer.slice(..));
    for (index, replace) in replace_operations.iter().copied().enumerate() {
        pass.set_pipeline(if replace { pipelines.1 } else { pipelines.0 });
        let start = u32::try_from(index * 6).expect("profile-bounded raster vertices fit u32");
        pass.draw(start..start + 6, 0..1);
    }
}

pub(super) fn retained_transfer(
    graphics: &UiNativeGraphics,
) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    let shader = graphics
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("worth-ui-retained-to-surface"),
            source: wgpu::ShaderSource::Wgsl(RETAINED_TO_SURFACE_SHADER.into()),
        });
    let pipeline = transfer_pipeline(
        &graphics.device,
        &shader,
        wgpu::TextureFormat::Bgra8UnormSrgb,
    );
    let view = graphics
        .retained_target()
        .create_view(&wgpu::TextureViewDescriptor::default());
    let bind_group = graphics
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("worth-ui-retained-to-surface-bind-group"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
        });
    (pipeline, bind_group)
}

pub(super) fn draw_retained_to_surface(
    encoder: &mut wgpu::CommandEncoder,
    target: &wgpu::TextureView,
    pipeline: &wgpu::RenderPipeline,
    bind_group: &wgpu::BindGroup,
) {
    let attachments = [Some(wgpu::RenderPassColorAttachment {
        view: target,
        depth_slice: None,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
            store: wgpu::StoreOp::Store,
        },
    })];
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("worth-ui-retained-to-surface-pass"),
        color_attachments: &attachments,
        ..Default::default()
    });
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, bind_group, &[]);
    pass.draw(0..3, 0..1);
}
