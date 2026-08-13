use crate::native::UiNativeGraphics;
use wgpu::util::DeviceExt;

use super::super::{
    copy_evidence_pixels, draw_raster_operations, draw_retained_to_surface, raster_pipelines,
    rectangle_vertices, retained_transfer, RasterVertex, UiNativePendingWgpuObligation,
    UiNativeReadbackPort, UiWgpuNativeReadbackPort,
};
use super::{
    UiNativePresentationPortFailure, UiNativePresentationPortObservation,
    UiNativePresentationPortPlan, UiNativeRasterOperation,
};

pub(super) fn present(
    graphics: &mut UiNativeGraphics,
    plan: UiNativePresentationPortPlan,
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
    let vertices = raster_vertices(&plan.operations);
    let vertex_bytes = encode_vertices(&vertices);
    let vertex_buffer = (!vertex_bytes.is_empty()).then(|| {
        graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("worth-ui-retained-raster-vertices"),
                contents: &vertex_bytes,
                usage: wgpu::BufferUsages::VERTEX,
            })
    });
    let replace_operations = plan
        .operations
        .iter()
        .map(|operation| matches!(operation, UiNativeRasterOperation::Clear(_)))
        .collect::<Vec<_>>();
    let raster_pipelines = raster_pipelines(&graphics.device);
    let submission = encode_and_present(
        graphics,
        output,
        retained_view,
        surface_view,
        surface_pipeline,
        surface_bind_group,
        &readback,
        vertex_buffer.as_ref(),
        &replace_operations,
        (&raster_pipelines.0, &raster_pipelines.1),
        plan,
    );
    let (pixels, readback_crossings) =
        match UiWgpuNativeReadbackPort::read_two_pixels(&graphics.device, &readback, &submission) {
            Ok(observation) => observation.into_parts(),
            Err(_) => {
                return Err(UiNativePresentationPortFailure::ReadbackUnsettled(
                    Box::new(UiNativePendingWgpuObligation::new(readback, submission)),
                ));
            }
        };
    Ok(UiNativePresentationPortObservation {
        pixels,
        cost,
        crossing_count: readback_crossings.saturating_add(1),
    })
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
    vertex_buffer: Option<&wgpu::Buffer>,
    replace_operations: &[bool],
    raster_pipelines: (&wgpu::RenderPipeline, &wgpu::RenderPipeline),
    plan: UiNativePresentationPortPlan,
) -> wgpu::SubmissionIndex {
    let mut encoder = graphics
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("worth-ui-initial-presentation"),
        });
    draw_raster_operations(
        &mut encoder,
        &retained_view,
        vertex_buffer,
        replace_operations,
        raster_pipelines,
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
        .flat_map(|operation| match *operation {
            UiNativeRasterOperation::Clear(rect) => rectangle_vertices(rect, [0, 0, 0, 0]),
            UiNativeRasterOperation::FilledRect { rect, source_rgba8 } => {
                rectangle_vertices(rect, source_rgba8)
            }
        })
        .collect()
}

fn encode_vertices(vertices: &[RasterVertex]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vertices.len() * 24);
    for vertex in vertices {
        for value in vertex.position.into_iter().chain(vertex.color) {
            bytes.extend_from_slice(&value.to_ne_bytes());
        }
    }
    bytes
}
