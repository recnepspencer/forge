use std::sync::Arc;

use winit::window::Window;

use super::{adapter_selection, UiNativeGraphics};

/// Contractual device/surface preparation boundary. The wgpu implementation
/// returns observations and owned mechanics, never a framework lifecycle
/// verdict.
pub(crate) trait UiNativeGraphicsPort {
    fn prepare(window: Arc<Window>)
        -> Result<UiNativePreparedGraphics, UiNativeGraphicsPortDenial>;

    fn replacement_target(
        graphics: &mut UiNativeGraphics,
        scale_factor: f64,
        extent: [u32; 2],
    ) -> wgpu::Texture;
}

pub(crate) struct UiWgpuNativeGraphicsPort;

pub(crate) struct UiNativePreparedGraphics {
    graphics: UiNativeGraphics,
    crossing_count: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeGraphicsPortDenial {
    Surface,
    Adapter,
    Device,
}

pub(crate) const QUALIFIED_DX12_PRESENTATION_SYSTEM: wgpu::Dx12SwapchainKind =
    wgpu::Dx12SwapchainKind::DxgiFromVisual;

impl UiNativePreparedGraphics {
    pub(crate) fn into_parts(self) -> (UiNativeGraphics, u8) {
        (self.graphics, self.crossing_count)
    }
}

impl UiNativeGraphicsPort for UiWgpuNativeGraphicsPort {
    fn prepare(
        window: Arc<Window>,
    ) -> Result<UiNativePreparedGraphics, UiNativeGraphicsPortDenial> {
        let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        descriptor.backends = wgpu::Backends::DX12;
        descriptor.backend_options.dx12.presentation_system = QUALIFIED_DX12_PRESENTATION_SYSTEM;
        let instance = wgpu::Instance::new(descriptor);
        let surface = instance
            .create_surface(Arc::clone(&window))
            .map_err(|_| UiNativeGraphicsPortDenial::Surface)?;
        let adapter = select_adapter(&instance, &surface)?;
        let adapter_info = adapter.get_info();
        validate_surface_capabilities(&surface.get_capabilities(&adapter))?;
        let descriptor = wgpu::DeviceDescriptor {
            label: Some("worth-ui-windows-dx12-v1-device"),
            required_features: wgpu::Features::empty(),
            required_limits: qualified_required_limits(&adapter),
            ..Default::default()
        };
        let (device, queue) = pollster::block_on(adapter.request_device(&descriptor))
            .map_err(|_| UiNativeGraphicsPortDenial::Device)?;
        let size = window.inner_size();
        let extent = [size.width.max(1), size.height.max(1)];
        let surface_configuration = surface_configuration(extent);
        surface.configure(&device, &surface_configuration);
        let retained_target = retained_target(&device, extent);
        Ok(UiNativePreparedGraphics {
            graphics: UiNativeGraphics {
                _instance: instance,
                surface,
                _adapter: adapter,
                device,
                queue,
                retained_target: Some(retained_target),
                surface_configuration,
                scale_factor: window.scale_factor(),
                adapter_info,
            },
            crossing_count: 1,
        })
    }

    fn replacement_target(
        graphics: &mut UiNativeGraphics,
        scale_factor: f64,
        extent: [u32; 2],
    ) -> wgpu::Texture {
        graphics.surface_configuration.width = extent[0];
        graphics.surface_configuration.height = extent[1];
        graphics.scale_factor = scale_factor;
        graphics
            .surface
            .configure(&graphics.device, &graphics.surface_configuration);
        retained_target(&graphics.device, extent)
    }
}

fn select_adapter(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface<'_>,
) -> Result<wgpu::Adapter, UiNativeGraphicsPortDenial> {
    let candidates = pollster::block_on(instance.enumerate_adapters(wgpu::Backends::DX12));
    let observed = candidates
        .into_iter()
        .map(|adapter| {
            let info = adapter.get_info();
            (
                adapter_selection::AdapterCandidate {
                    surface_supported: adapter.is_surface_supported(surface),
                    device_type: info.device_type,
                    limits: adapter.limits(),
                    vendor: info.vendor,
                    device: info.device,
                    name: info.name,
                    driver_info: info.driver_info,
                },
                adapter,
            )
        })
        .collect::<Vec<_>>();
    adapter_selection::select_eligible_adapter(observed)
        .map(|(_, adapter)| adapter)
        .ok_or(UiNativeGraphicsPortDenial::Adapter)
}

fn validate_surface_capabilities(
    capabilities: &wgpu::SurfaceCapabilities,
) -> Result<(), UiNativeGraphicsPortDenial> {
    let exact = capabilities
        .formats
        .contains(&wgpu::TextureFormat::Bgra8UnormSrgb)
        && capabilities
            .present_modes
            .contains(&wgpu::PresentMode::Fifo)
        && capabilities
            .alpha_modes
            .contains(&wgpu::CompositeAlphaMode::PreMultiplied);
    exact
        .then_some(())
        .ok_or(UiNativeGraphicsPortDenial::Surface)
}

fn qualified_required_limits(adapter: &wgpu::Adapter) -> wgpu::Limits {
    wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits())
}

fn surface_configuration(extent: [u32; 2]) -> wgpu::SurfaceConfiguration {
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: extent[0],
        height: extent[1],
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 2,
        alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
        view_formats: Vec::new(),
    }
}

fn retained_target(device: &wgpu::Device, extent: [u32; 2]) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("worth-ui-retained-presentation-target"),
        size: wgpu::Extent3d {
            width: extent[0].max(1),
            height: extent[1].max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}
