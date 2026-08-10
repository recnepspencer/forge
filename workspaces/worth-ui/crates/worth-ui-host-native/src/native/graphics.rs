mod adapter_selection;
mod ownership;
mod port;

pub(crate) use ownership::UiNativeOwnedGraphics;
#[cfg(test)]
pub(crate) use port::QUALIFIED_DX12_PRESENTATION_SYSTEM;
pub(crate) use port::{UiNativeGraphicsPort, UiWgpuNativeGraphicsPort};

pub(crate) struct UiNativeGraphics {
    pub(crate) _instance: wgpu::Instance,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) _adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    retained_target: Option<wgpu::Texture>,
    pub(crate) surface_configuration: wgpu::SurfaceConfiguration,
    pub(crate) scale_factor: f64,
    pub(crate) adapter_info: wgpu::AdapterInfo,
}

impl UiNativeGraphics {
    pub(crate) fn retained_target(&self) -> &wgpu::Texture {
        self.retained_target
            .as_ref()
            .expect("live graphics retains its presentation target")
    }

    pub(crate) const fn extent(&self) -> [u32; 2] {
        [
            self.surface_configuration.width,
            self.surface_configuration.height,
        ]
    }
}

pub(super) fn basis_changed(
    current_scale: f64,
    current_extent: [u32; 2],
    next_scale: f64,
    next_extent: [u32; 2],
) -> bool {
    (current_scale - next_scale).abs() > f64::EPSILON || current_extent != next_extent
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

    #[test]
    fn qualified_adapter_tie_break_uses_the_complete_frozen_observation_key() {
        let mut limits = wgpu::Limits::downlevel_defaults();
        limits.max_texture_dimension_2d = 16_384;
        let candidates = vec![
            (ranked_candidate(&limits, (2, 1, "a", "a")), "vendor"),
            (ranked_candidate(&limits, (1, 5, "a", "a")), "device"),
            (ranked_candidate(&limits, (1, 4, "z", "a")), "name"),
            (ranked_candidate(&limits, (1, 4, "a", "z")), "driver"),
            (ranked_candidate(&limits, (1, 4, "a", "a")), "exact"),
        ];
        let (observation, selected) = select_eligible_adapter(candidates).unwrap();
        assert_eq!(selected, "exact");
        assert_eq!(observation.vendor, 1);
        assert_eq!(observation.device, 4);
        assert_eq!(observation.name, "a");
        assert_eq!(observation.driver_info, "a");
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

    fn ranked_candidate(limits: &wgpu::Limits, key: (u32, u32, &str, &str)) -> AdapterCandidate {
        let (vendor, device, name, driver_info) = key;
        AdapterCandidate {
            surface_supported: true,
            device_type: wgpu::DeviceType::DiscreteGpu,
            limits: limits.clone(),
            vendor,
            device,
            name: name.to_owned(),
            driver_info: driver_info.to_owned(),
        }
    }
}
