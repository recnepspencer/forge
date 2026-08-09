use worth_ui_host_contract::UiHostSurfacePresentationDenial;

use super::GPU_WAIT_DEADLINE;

/// Contractual retained-target readback boundary. It returns bytes or an
/// external failure observation and cannot settle framework presentation.
pub(super) trait UiNativeReadbackPort {
    fn read_two_pixels(
        device: &wgpu::Device,
        buffer: &wgpu::Buffer,
        submission: &wgpu::SubmissionIndex,
    ) -> Result<[[u8; 4]; 2], UiHostSurfacePresentationDenial>;
}

pub(super) struct UiWgpuNativeReadbackPort;

impl UiNativeReadbackPort for UiWgpuNativeReadbackPort {
    fn read_two_pixels(
        device: &wgpu::Device,
        buffer: &wgpu::Buffer,
        submission: &wgpu::SubmissionIndex,
    ) -> Result<[[u8; 4]; 2], UiHostSurfacePresentationDenial> {
        let bytes = map_bytes(device, buffer, submission)?;
        let first = bytes
            .get(..4)
            .and_then(|pixel| pixel.try_into().ok())
            .ok_or(UiHostSurfacePresentationDenial::AdapterDeclined)?;
        let second = bytes
            .get(256..260)
            .and_then(|pixel| pixel.try_into().ok())
            .ok_or(UiHostSurfacePresentationDenial::AdapterDeclined)?;
        drop(bytes);
        buffer.unmap();
        Ok([first, second])
    }
}

fn map_bytes(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
    submission: &wgpu::SubmissionIndex,
) -> Result<wgpu::BufferView, UiHostSurfacePresentationDenial> {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    buffer.map_async(wgpu::MapMode::Read, .., move |result| {
        let _ = sender.send(result);
    });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: Some(submission.clone()),
            timeout: Some(GPU_WAIT_DEADLINE),
        })
        .map_err(|_| UiHostSurfacePresentationDenial::AdapterDeclined)?;
    receiver
        .recv_timeout(std::time::Duration::from_millis(50))
        .map_err(|_| UiHostSurfacePresentationDenial::AdapterDeclined)?
        .map_err(|_| UiHostSurfacePresentationDenial::AdapterDeclined)?;
    Ok(buffer.get_mapped_range(..))
}

#[cfg(test)]
mod tests {
    use super::{UiNativeReadbackPort, UiWgpuNativeReadbackPort};

    #[test]
    fn wgpu_readback_port_returns_the_mapped_nonuniform_source_bytes() {
        let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        descriptor.backends = wgpu::Backends::DX12;
        let instance = wgpu::Instance::new(descriptor);
        let adapter = pollster::block_on(instance.enumerate_adapters(wgpu::Backends::DX12))
            .into_iter()
            .next()
            .expect("one DX12 adapter for the qualified readback control");
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            required_limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
            ..Default::default()
        }))
        .expect("qualified readback device");
        let source = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("worth-ui-readback-control-source"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: true,
        });
        let mut mapped = source.get_mapped_range_mut(..);
        mapped.slice(..4).copy_from_slice(&[13, 29, 71, 255]);
        mapped.slice(256..260).copy_from_slice(&[199, 5, 151, 17]);
        drop(mapped);
        source.unmap();
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("worth-ui-readback-control-target"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("worth-ui-readback-control-copy"),
        });
        encoder.copy_buffer_to_buffer(&source, 0, &readback, 0, 512);
        let submission = queue.submit([encoder.finish()]);
        assert_eq!(
            UiWgpuNativeReadbackPort::read_two_pixels(&device, &readback, &submission).unwrap(),
            [[13, 29, 71, 255], [199, 5, 151, 17]]
        );
    }
}
