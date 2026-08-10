use worth_ui_host_contract::{UiHostPresentationCostInput, UiHostPresentationCostReport};

use crate::native::UiNativeGraphics;

use super::super::{
    copy_evidence_pixels, draw_rectangle, draw_retained_to_surface, pipeline, rectangle_shader,
    retained_transfer, UiNativePendingWgpuObligation, UiNativeReadbackPort,
    UiWgpuNativeReadbackPort,
};
use super::{
    UiNativePresentationPortFailure, UiNativePresentationPortObservation,
    UiNativePresentationPortPlan,
};

pub(super) fn present(
    graphics: &mut UiNativeGraphics,
    plan: UiNativePresentationPortPlan,
) -> Result<UiNativePresentationPortObservation, UiNativePresentationPortFailure> {
    let shader_source = rectangle_shader(plan.rect, plan.source_rgba8);
    let shader = graphics
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("worth-ui-attributed-filled-rectangle"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });
    let retained_pipeline = pipeline(
        &graphics.device,
        &shader,
        wgpu::TextureFormat::Rgba8UnormSrgb,
    );
    let (surface_pipeline, surface_bind_group) = retained_transfer(graphics);
    let output = acquire_surface(graphics)?;
    let retained_view = graphics
        .retained_target()
        .create_view(&wgpu::TextureViewDescriptor::default());
    let surface_view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let readback = evidence_buffer(&graphics.device);
    let submission = encode_and_present(
        graphics,
        output,
        retained_view,
        surface_view,
        retained_pipeline,
        surface_pipeline,
        surface_bind_group,
        &readback,
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
        cost: presentation_cost(graphics.extent(), plan.rect),
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
    retained_pipeline: wgpu::RenderPipeline,
    surface_pipeline: wgpu::RenderPipeline,
    surface_bind_group: wgpu::BindGroup,
    readback: &wgpu::Buffer,
) -> wgpu::SubmissionIndex {
    let mut encoder = graphics
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("worth-ui-initial-presentation"),
        });
    draw_rectangle(&mut encoder, &retained_view, &retained_pipeline);
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

fn presentation_cost(
    extent: [u32; 2],
    rect: super::super::RasterRect,
) -> UiHostPresentationCostReport {
    let pixels = u64::from(extent[0]) * u64::from(extent[1]);
    UiHostPresentationCostReport::from_adapter(UiHostPresentationCostInput {
        presented_surfaces: 1,
        translated_rows: 1,
        native_resource_cache_misses: 1,
        intersecting_commands: 1,
        replayed_commands: 1,
        cleared_pixels: pixels,
        rendered_pixels: u64::from(rect.physical_width) * u64::from(rect.physical_height),
        presented_pixels: pixels,
        gpu_writes: 3,
        render_passes: 2,
        surface_acquisitions: 1,
        queue_submissions: 1,
        presents: 1,
        ..Default::default()
    })
}
