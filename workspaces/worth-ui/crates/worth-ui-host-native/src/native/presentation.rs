use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedFrameConsumptionView, UiMountedPaintCommand, UiMountedPresentationWorkView,
};

use super::{
    UiNativeGraphics, UiNativePresentationInput, UiNativePresentationObservation,
    UiNativeResourceClass, UiNativeResourceRegistry,
};

mod pipeline;
mod port;
mod raster;
mod readback_port;

use pipeline::{draw_rectangle, draw_retained_to_surface, pipeline, retained_transfer};
use raster::{raster_rect, rectangle_shader};
use readback_port::{UiNativeReadbackPort, UiWgpuNativeReadbackPort};

pub(crate) use port::{UiNativePresentationPort, UiWgpuNativePresentationPort};

pub(crate) const GPU_WAIT_DEADLINE: std::time::Duration = std::time::Duration::from_millis(5_000);

pub(crate) enum UiNativePresentationFailure {
    BeforeEffects(UiHostSurfacePresentationDenial),
    Indeterminate(UiNativePendingPresentation),
}

impl std::fmt::Debug for UiNativePresentationFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BeforeEffects(denial) => formatter
                .debug_tuple("BeforeEffects")
                .field(denial)
                .finish(),
            Self::Indeterminate(_) => formatter.write_str("Indeterminate(..)"),
        }
    }
}

pub(crate) struct UiNativePendingPresentation {
    _external: Box<dyn UiNativePendingExternalObligation>,
    _readback_owner: super::UiNativeResourceOwner,
    _submission_owner: super::UiNativeResourceOwner,
}

trait UiNativePendingExternalObligation {}

struct UiNativePendingWgpuObligation {
    _readback: wgpu::Buffer,
    _submission: wgpu::SubmissionIndex,
}

impl UiNativePendingExternalObligation for UiNativePendingWgpuObligation {}

impl UiNativePendingPresentation {
    fn wgpu(
        readback: wgpu::Buffer,
        submission: wgpu::SubmissionIndex,
        readback_owner: super::UiNativeResourceOwner,
        submission_owner: super::UiNativeResourceOwner,
    ) -> Self {
        Self {
            _external: Box::new(UiNativePendingWgpuObligation {
                _readback: readback,
                _submission: submission,
            }),
            _readback_owner: readback_owner,
            _submission_owner: submission_owner,
        }
    }

    #[cfg(test)]
    pub(super) fn scripted(
        resources: &mut UiNativeResourceRegistry,
        dropped: std::rc::Rc<std::cell::Cell<bool>>,
    ) -> Self {
        let mut owners = resources
            .reserve(&[
                UiNativeResourceClass::ReadbackBuffer,
                UiNativeResourceClass::PendingSubmission,
            ])
            .expect("scripted indeterminate presentation reserves exact owners");
        let submission_owner = owners.pop().expect("submission owner");
        let readback_owner = owners.pop().expect("readback owner");
        Self {
            _external: Box::new(UiNativePendingDropProbe(dropped)),
            _readback_owner: readback_owner,
            _submission_owner: submission_owner,
        }
    }
}

#[cfg(test)]
struct UiNativePendingDropProbe(std::rc::Rc<std::cell::Cell<bool>>);

#[cfg(test)]
impl UiNativePendingExternalObligation for UiNativePendingDropProbe {}

#[cfg(test)]
impl Drop for UiNativePendingDropProbe {
    fn drop(&mut self) {
        self.0.set(true);
    }
}

pub(crate) fn present_initial(
    graphics: &mut UiNativeGraphics,
    resources: &mut UiNativeResourceRegistry,
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<
    (
        UiNativePresentationObservation,
        UiHostPresentationCostReport,
    ),
    UiNativePresentationFailure,
> {
    let (frame, mechanic) =
        validate_initial(view).map_err(UiNativePresentationFailure::BeforeEffects)?;
    let source_rgba8 = mechanic.color().channels();
    let rect = raster_rect(mechanic, graphics)?;
    let mut presentation_resources = resources
        .reserve(&[
            UiNativeResourceClass::ReadbackBuffer,
            UiNativeResourceClass::PendingSubmission,
        ])
        .map_err(|_| malformed())?;
    let readback_resource = presentation_resources.remove(0);
    let submission_resource = presentation_resources.remove(0);
    let rectangle_shader = rectangle_shader(rect, source_rgba8);
    let shader = graphics
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("worth-ui-attributed-filled-rectangle"),
            source: wgpu::ShaderSource::Wgsl(rectangle_shader.into()),
        });
    let retained_pipeline = pipeline(
        &graphics.device,
        &shader,
        wgpu::TextureFormat::Rgba8UnormSrgb,
    );
    let (surface_pipeline, surface_bind_group) = retained_transfer(graphics);
    let output = match graphics.surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(output)
        | wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,
        wgpu::CurrentSurfaceTexture::Timeout
        | wgpu::CurrentSurfaceTexture::Occluded
        | wgpu::CurrentSurfaceTexture::Outdated
        | wgpu::CurrentSurfaceTexture::Lost
        | wgpu::CurrentSurfaceTexture::Validation => {
            resources
                .release(readback_resource)
                .expect("readback reservation must remain exact");
            resources
                .release(submission_resource)
                .expect("submission reservation must remain exact");
            return Err(UiNativePresentationFailure::BeforeEffects(
                UiHostSurfacePresentationDenial::AdapterDeclined,
            ));
        }
    };
    let retained_view = graphics
        .retained_target
        .create_view(&wgpu::TextureViewDescriptor::default());
    let surface_view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let readback = graphics.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("worth-ui-retained-evidence-readback"),
        size: 512,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
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
        &graphics.retained_target,
        &readback,
        graphics.extent(),
    );
    let submission = graphics.queue.submit([encoder.finish()]);
    output.present();
    let retained_pixels =
        UiWgpuNativeReadbackPort::read_two_pixels(&graphics.device, &readback, &submission);
    let [retained_baseline_rgba8, retained_center_rgba8] = match retained_pixels {
        Ok(pixels) => {
            resources
                .release(readback_resource)
                .expect("readback owner must remain exact");
            resources
                .release(submission_resource)
                .expect("submission owner must remain exact");
            pixels
        }
        Err(_) => {
            return Err(UiNativePresentationFailure::Indeterminate(
                UiNativePendingPresentation::wgpu(
                    readback,
                    submission,
                    readback_resource,
                    submission_resource,
                ),
            ));
        }
    };
    let extent = graphics.extent();
    let pixels = u64::from(extent[0]) * u64::from(extent[1]);
    let cost = UiHostPresentationCostReport::from_adapter(UiHostPresentationCostInput {
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
    });
    let bounds = mechanic.bounds();
    let observation = UiNativePresentationObservation::new(UiNativePresentationInput {
        client_physical_size: extent,
        scale_factor_milli: (graphics.scale_factor * 1_000.0).round() as u32,
        source_rgba8,
        retained_center_rgba8,
        retained_baseline_rgba8,
        presented_frame: frame,
        semantic_surface: view.surface().diagnostic_value(),
        binding_generation: view.binding().diagnostic_value(),
        mounted_instance: mechanic.mounted_instance().diagnostic_value(),
        node_receipt: mechanic.node_receipt().diagnostic_value(),
        presentation_attempt: view.attempt().diagnostic_value(),
        logical_bounds_milli: [
            milli(bounds.x()),
            milli(bounds.y()),
            milli(bounds.width()),
            milli(bounds.height()),
        ],
        order_ordinal: 0,
        port_crossings: 1,
        cost,
    });
    Ok((observation, cost))
}

fn milli(value: f32) -> i64 {
    (f64::from(value) * 1_000.0).round() as i64
}

fn copy_evidence_pixels(
    encoder: &mut wgpu::CommandEncoder,
    texture: &wgpu::Texture,
    buffer: &wgpu::Buffer,
    extent: [u32; 2],
) {
    copy_pixel(encoder, texture, buffer, [0, 0], 0);
    copy_pixel(
        encoder,
        texture,
        buffer,
        [extent[0] / 2, extent[1] / 2],
        256,
    );
}

fn copy_pixel(
    encoder: &mut wgpu::CommandEncoder,
    texture: &wgpu::Texture,
    buffer: &wgpu::Buffer,
    origin: [u32; 2],
    offset: u64,
) {
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: origin[0],
                y: origin[1],
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset,
                bytes_per_row: Some(256),
                rows_per_image: None,
            },
        },
        wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
    );
}

fn validate_initial(
    view: &UiMountedFrameConsumptionView<'_>,
) -> Result<
    (u64, worth_ui_host_contract::UiMountedFilledRectMechanic),
    UiHostSurfacePresentationDenial,
> {
    let UiMountedPresentationWorkView::Initial(initial) = view.presentation_work() else {
        return Err(UiHostSurfacePresentationDenial::AdapterDeclined);
    };
    if initial.commands().len() != 1
        || initial.order().len() != 1
        || initial.projection().filled_rects().rows().len() != 1
        || !initial.projection().semantic_text().rows().is_empty()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    let UiMountedPaintCommand::FilledRect {
        table_index,
        mechanic,
        ..
    } = &initial.commands()[0]
    else {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    };
    if *table_index != 0
        || initial.projection().filled_rects().rows()[0] != *mechanic
        || initial.order()[0].command() != initial.commands()[0].identity()
    {
        return Err(UiHostSurfacePresentationDenial::MalformedProjection);
    }
    Ok((initial.affinity().successor().diagnostic_value(), *mechanic))
}

fn malformed() -> UiNativePresentationFailure {
    UiNativePresentationFailure::BeforeEffects(UiHostSurfacePresentationDenial::MalformedProjection)
}
