use crate::native::UiNativeGraphics;
use wgpu::util::DeviceExt;

use super::super::{
    copy_evidence_pixels, draw_presentation_operations, draw_retained_to_surface,
    presentation_pipelines, rectangle_vertices, retained_transfer, GlyphVertex, RasterVertex,
    UiNativePendingWgpuObligation, UiNativePresentationPipelines, UiNativeWgpuReadbackPoll,
};
use super::{
    UiNativePresentationPortFailure, UiNativePresentationPortObservation,
    UiNativePresentationPortPlan, UiNativeRasterOperation,
};

pub(super) fn present(
    graphics: &mut UiNativeGraphics,
    atlas: Option<&crate::native::text_atlas::UiNativeTextAtlasGpuPages>,
    plan: UiNativePresentationPortPlan,
    defer_initial_observation: bool,
) -> Result<UiNativePresentationPortObservation, UiNativePresentationPortFailure> {
    let cost = plan.cost;
    let (surface_pipeline, surface_bind_group) = retained_transfer(graphics);
    let output = acquire_surface(graphics)?;
    let retained_view = graphics
        .retained_target()
        .create_view(&wgpu::TextureViewDescriptor::default());
    let surface_view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let readback = evidence_buffer(&graphics.device);
    let raster_vertices = raster_vertices(&plan.operations);
    let raster_bytes = encode_raster_vertices(&raster_vertices);
    let raster_vertex_buffer = (!raster_bytes.is_empty()).then(|| {
        graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("worth-ui-retained-raster-vertices"),
                contents: &raster_bytes,
                usage: wgpu::BufferUsages::VERTEX,
            })
    });
    let glyph_vertices = glyph_vertices(&plan.operations, graphics.extent());
    let glyph_bytes = encode_glyph_vertices(&glyph_vertices);
    let glyph_vertex_buffer = (!glyph_bytes.is_empty()).then(|| {
        graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("worth-ui-retained-glyph-vertices"),
                contents: &glyph_bytes,
                usage: wgpu::BufferUsages::VERTEX,
            })
    });
    let pipelines = presentation_pipelines(&graphics.device);
    let submission = encode_and_present(
        graphics,
        output,
        retained_view,
        surface_view,
        surface_pipeline,
        surface_bind_group,
        &readback,
        raster_vertex_buffer.as_ref(),
        glyph_vertex_buffer.as_ref(),
        atlas,
        &pipelines,
        plan,
    );
    let mut pending = UiNativePendingWgpuObligation::new(readback, submission, cost);
    if defer_initial_observation {
        pending.retain_async_handoff();
        return Err(UiNativePresentationPortFailure::ReadbackUnsettled(
            Box::new(pending),
        ));
    }
    match pending.poll_readback(&graphics.device) {
        UiNativeWgpuReadbackPoll::Presented(pixels) => Ok(
            UiNativePresentationPortObservation::from_async_readback(pixels, cost),
        ),
        UiNativeWgpuReadbackPoll::Pending | UiNativeWgpuReadbackPoll::Indeterminate => {
            pending.retain_async_handoff();
            Err(UiNativePresentationPortFailure::ReadbackUnsettled(
                Box::new(pending),
            ))
        }
    }
}

fn acquire_surface(
    graphics: &mut UiNativeGraphics,
) -> Result<wgpu::SurfaceTexture, UiNativePresentationPortFailure> {
    match graphics.surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(output)
        | wgpu::CurrentSurfaceTexture::Suboptimal(output) => Ok(output),
        _ => Err(UiNativePresentationPortFailure::SurfaceUnavailable),
    }
}

fn evidence_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("worth-ui-retained-evidence-readback"),
        size: 512,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    })
}

#[allow(clippy::too_many_arguments)]
fn encode_and_present(
    graphics: &UiNativeGraphics,
    output: wgpu::SurfaceTexture,
    retained_view: wgpu::TextureView,
    surface_view: wgpu::TextureView,
    surface_pipeline: wgpu::RenderPipeline,
    surface_bind_group: wgpu::BindGroup,
    readback: &wgpu::Buffer,
    raster_vertex_buffer: Option<&wgpu::Buffer>,
    glyph_vertex_buffer: Option<&wgpu::Buffer>,
    atlas: Option<&crate::native::text_atlas::UiNativeTextAtlasGpuPages>,
    pipelines: &UiNativePresentationPipelines,
    plan: UiNativePresentationPortPlan,
) -> wgpu::SubmissionIndex {
    let mut encoder = graphics
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("worth-ui-initial-presentation"),
        });
    draw_presentation_operations(
        &graphics.device,
        &mut encoder,
        &retained_view,
        raster_vertex_buffer,
        glyph_vertex_buffer,
        &plan.operations,
        atlas,
        pipelines,
        plan.clear_retained_target,
    );
    draw_retained_to_surface(
        &mut encoder,
        &surface_view,
        &surface_pipeline,
        &surface_bind_group,
    );
    copy_evidence_pixels(
        &mut encoder,
        graphics.retained_target(),
        readback,
        graphics.extent(),
    );
    let submission = graphics.queue.submit([encoder.finish()]);
    output.present();
    submission
}

fn raster_vertices(operations: &[UiNativeRasterOperation]) -> Vec<RasterVertex> {
    operations
        .iter()
        .filter_map(|operation| match *operation {
            UiNativeRasterOperation::Clear(rect) => Some(rectangle_vertices(rect, [0, 0, 0, 0])),
            UiNativeRasterOperation::FilledRect { rect, source_rgba8 } => {
                Some(rectangle_vertices(rect, source_rgba8))
            }
            UiNativeRasterOperation::Glyph(_) => None,
        })
        .flatten()
        .collect()
}

fn glyph_vertices(operations: &[UiNativeRasterOperation], extent: [u32; 2]) -> Vec<GlyphVertex> {
    operations
        .iter()
        .filter_map(|operation| match operation {
            UiNativeRasterOperation::Glyph(command) => {
                Some(super::super::text::glyph_vertices(*command, extent))
            }
            UiNativeRasterOperation::Clear(_) | UiNativeRasterOperation::FilledRect { .. } => None,
        })
        .flatten()
        .collect()
}

fn encode_raster_vertices(vertices: &[RasterVertex]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vertices.len() * 24);
    for vertex in vertices {
        for value in vertex.position.into_iter().chain(vertex.color) {
            bytes.extend_from_slice(&value.to_ne_bytes());
        }
    }
    bytes
}

fn encode_glyph_vertices(vertices: &[GlyphVertex]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vertices.len() * 32);
    for vertex in vertices {
        for value in vertex
            .position
            .into_iter()
            .chain(vertex.texture_uv)
            .chain(vertex.color)
        {
            bytes.extend_from_slice(&value.to_ne_bytes());
        }
    }
    bytes
}
