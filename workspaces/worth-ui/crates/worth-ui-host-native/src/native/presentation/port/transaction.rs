use crate::native::UiNativeGraphics;

use super::super::{
    clear_target, copy_evidence_pixels, draw_rectangle, draw_rectangle_after_clear,
    draw_retained_to_surface, pipeline, rectangle_shader, replace_pipeline, retained_transfer,
    UiNativePendingWgpuObligation, UiNativeReadbackPort, UiWgpuNativeReadbackPort,
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
    let submission = encode_and_present(
        graphics,
        output,
        retained_view,
        surface_view,
        surface_pipeline,
        surface_bind_group,
        &readback,
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
    plan: UiNativePresentationPortPlan,
) -> wgpu::SubmissionIndex {
    let mut encoder = graphics
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("worth-ui-initial-presentation"),
        });
    if plan.clear_retained_target && plan.operations.is_empty() {
        clear_target(&mut encoder, &retained_view);
    }
    for (index, operation) in plan.operations.iter().copied().enumerate() {
        draw_operation(
            graphics,
            &mut encoder,
            &retained_view,
            operation,
            plan.clear_retained_target && index == 0,
        );
    }
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

fn draw_operation(
    graphics: &UiNativeGraphics,
    encoder: &mut wgpu::CommandEncoder,
    retained_view: &wgpu::TextureView,
    operation: UiNativeRasterOperation,
    clear_before: bool,
) {
    let (rect, rgba, replace) = match operation {
        UiNativeRasterOperation::Clear(rect) => (rect, [0, 0, 0, 0], true),
        UiNativeRasterOperation::FilledRect { rect, source_rgba8 } => (rect, source_rgba8, false),
    };
    let shader = graphics
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("worth-ui-retained-draw-operation"),
            source: wgpu::ShaderSource::Wgsl(rectangle_shader(rect, rgba).into()),
        });
    let pipeline = if replace {
        replace_pipeline(
            &graphics.device,
            &shader,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        )
    } else {
        pipeline(
            &graphics.device,
            &shader,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        )
    };
    if clear_before {
        draw_rectangle_after_clear(encoder, retained_view, &pipeline);
    } else {
        draw_rectangle(encoder, retained_view, &pipeline);
    }
}
