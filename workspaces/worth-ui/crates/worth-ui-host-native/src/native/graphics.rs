use std::sync::Arc;

use winit::window::Window;

mod adapter_selection;
mod port;

pub(crate) use port::{UiNativeGraphicsPort, UiWgpuNativeGraphicsPort};

pub(crate) const QUALIFIED_DX12_PRESENTATION_SYSTEM: wgpu::Dx12SwapchainKind =
    wgpu::Dx12SwapchainKind::DxgiFromVisual;

pub(crate) struct UiNativeGraphics {
    pub(crate) _instance: wgpu::Instance,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) _adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) retained_target: wgpu::Texture,
    pub(crate) surface_configuration: wgpu::SurfaceConfiguration,
    pub(crate) scale_factor: f64,
    pub(crate) adapter_info: wgpu::AdapterInfo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeGraphicsPreparationDenial;

impl UiNativeGraphics {
    pub(crate) fn prepare(window: Arc<Window>) -> Result<Self, UiNativeGraphicsPreparationDenial> {
        let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        descriptor.backends = wgpu::Backends::DX12;
        descriptor.backend_options.dx12.presentation_system = QUALIFIED_DX12_PRESENTATION_SYSTEM;
        let instance = wgpu::Instance::new(descriptor);
        let surface = instance
            .create_surface(Arc::clone(&window))
            .map_err(|_| UiNativeGraphicsPreparationDenial)?;
        let adapter = select_adapter(&instance, &surface)?;
        let adapter_info = adapter.get_info();
        let capabilities = surface.get_capabilities(&adapter);
        validate_surface_capabilities(&capabilities)?;
        let required_limits = qualified_required_limits(&adapter);
        let device_descriptor = wgpu::DeviceDescriptor {
            label: Some("worth-ui-windows-dx12-v1-device"),
            required_features: wgpu::Features::empty(),
            required_limits,
            ..Default::default()
        };
        let (device, queue) = pollster::block_on(adapter.request_device(&device_descriptor))
            .map_err(|_| UiNativeGraphicsPreparationDenial)?;
        let size = window.inner_size();
        let extent = [size.width.max(1), size.height.max(1)];
        let surface_configuration = surface_configuration(extent);
        surface.configure(&device, &surface_configuration);
        let retained_target = retained_target(&device, extent);
        Ok(Self {
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
            retained_target,
            surface_configuration,
            scale_factor: window.scale_factor(),
            adapter_info,
        })
    }

    pub(crate) fn resize(&mut self, extent: [u32; 2]) -> bool {
        let extent = [extent[0].max(1), extent[1].max(1)];
        if !basis_changed(self.scale_factor, self.extent(), self.scale_factor, extent) {
            return false;
        }
        self.surface_configuration.width = extent[0];
        self.surface_configuration.height = extent[1];
        self.surface
            .configure(&self.device, &self.surface_configuration);
        self.retained_target = retained_target(&self.device, extent);
        true
    }

    pub(crate) fn rebind_scale(&mut self, scale_factor: f64, extent: [u32; 2]) -> bool {
        let changed = basis_changed(self.scale_factor, self.extent(), scale_factor, extent);
        self.scale_factor = scale_factor;
        self.resize(extent);
        changed
    }

    pub(crate) const fn extent(&self) -> [u32; 2] {
        [
            self.surface_configuration.width,
            self.surface_configuration.height,
        ]
    }
}

fn basis_changed(
    current_scale: f64,
    current_extent: [u32; 2],
    next_scale: f64,
    next_extent: [u32; 2],
) -> bool {
    (current_scale - next_scale).abs() > f64::EPSILON || current_extent != next_extent
}

pub(crate) fn qualified_required_limits(adapter: &wgpu::Adapter) -> wgpu::Limits {
    wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits())
}

fn select_adapter(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface<'_>,
) -> Result<wgpu::Adapter, UiNativeGraphicsPreparationDenial> {
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
        .ok_or(UiNativeGraphicsPreparationDenial)
}

fn validate_surface_capabilities(
    capabilities: &wgpu::SurfaceCapabilities,
) -> Result<(), UiNativeGraphicsPreparationDenial> {
    if !capabilities
        .formats
        .contains(&wgpu::TextureFormat::Bgra8UnormSrgb)
        || !capabilities
            .present_modes
            .contains(&wgpu::PresentMode::Fifo)
        || !capabilities
            .alpha_modes
            .contains(&wgpu::CompositeAlphaMode::PreMultiplied)
    {
        return Err(UiNativeGraphicsPreparationDenial);
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::adapter_selection::{select_eligible_adapter, AdapterCandidate};
    use super::basis_changed;

    #[test]
    fn window_basis_classifier_rearms_only_for_new_scale_or_nonzero_extent() {
        assert!(!basis_changed(1.5, [240, 144], 1.5, [240, 144]));
        assert!(basis_changed(1.5, [240, 144], 1.5, [320, 192]));
        assert!(basis_changed(1.5, [240, 144], 2.0, [320, 192]));
    }

    #[test]
    fn adapter_selection_returns_the_exact_qualified_candidate_and_rejects_substitutes() {
        let mut qualified = wgpu::Limits::downlevel_defaults();
        qualified.max_texture_dimension_2d = 16_384;
        let mut too_small = qualified.clone();
        too_small.max_texture_dimension_2d = 8_192;
        let candidates = vec![
            (
                candidate(true, wgpu::DeviceType::Cpu, qualified.clone(), 0),
                0,
            ),
            (
                candidate(true, wgpu::DeviceType::DiscreteGpu, too_small, 1),
                1,
            ),
            (
                candidate(false, wgpu::DeviceType::DiscreteGpu, qualified.clone(), 2),
                2,
            ),
            (
                candidate(true, wgpu::DeviceType::IntegratedGpu, qualified.clone(), 3),
                3,
            ),
            (
                candidate(true, wgpu::DeviceType::DiscreteGpu, qualified, 4),
                4,
            ),
        ];
        let (observation, adapter) = select_eligible_adapter(candidates).unwrap();
        assert_eq!(adapter, 4);
        assert_eq!(observation.device_type, wgpu::DeviceType::DiscreteGpu);
    }

    fn candidate(
        surface_supported: bool,
        device_type: wgpu::DeviceType,
        limits: wgpu::Limits,
        device: u32,
    ) -> AdapterCandidate {
        AdapterCandidate {
            surface_supported,
            device_type,
            limits,
            vendor: 1,
            device,
            name: format!("candidate-{device}"),
            driver_info: String::new(),
        }
    }
}
