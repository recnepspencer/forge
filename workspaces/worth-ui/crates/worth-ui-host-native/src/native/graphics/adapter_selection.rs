#[derive(Clone, Debug)]
pub(crate) struct AdapterCandidate {
    pub(crate) surface_supported: bool,
    pub(crate) device_type: wgpu::DeviceType,
    pub(crate) limits: wgpu::Limits,
    pub(crate) vendor: u32,
    pub(crate) device: u32,
    pub(crate) name: String,
    pub(crate) driver_info: String,
}

pub(crate) fn select_eligible_adapter<T>(
    mut candidates: Vec<(AdapterCandidate, T)>,
) -> Option<(AdapterCandidate, T)> {
    let index = candidates
        .iter()
        .enumerate()
        .filter(|(_, (candidate, _))| candidate.surface_supported && eligible(candidate))
        .min_by_key(|(_, (candidate, _))| selection_key(candidate))
        .map(|(index, _)| index)?;
    Some(candidates.swap_remove(index))
}

fn eligible(candidate: &AdapterCandidate) -> bool {
    matches!(
        candidate.device_type,
        wgpu::DeviceType::DiscreteGpu
            | wgpu::DeviceType::IntegratedGpu
            | wgpu::DeviceType::VirtualGpu
    ) && wgpu::Limits::downlevel_defaults().check_limits(&candidate.limits)
        && candidate.limits.max_texture_dimension_2d >= 16_384
}

fn selection_key(candidate: &AdapterCandidate) -> (u8, u32, u32, &str, &str) {
    let rank = match candidate.device_type {
        wgpu::DeviceType::DiscreteGpu => 0,
        wgpu::DeviceType::IntegratedGpu => 1,
        wgpu::DeviceType::VirtualGpu => 2,
        wgpu::DeviceType::Other | wgpu::DeviceType::Cpu => 3,
    };
    (
        rank,
        candidate.vendor,
        candidate.device,
        &candidate.name,
        &candidate.driver_info,
    )
}

#[cfg(test)]
mod tests {
    use super::{select_eligible_adapter, AdapterCandidate};

    #[test]
    fn selection_returns_the_exact_qualified_candidate_and_rejects_substitutes() {
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
        let (_, adapter) = select_eligible_adapter(candidates).unwrap();
        assert_eq!(adapter, 4);
    }

    #[test]
    fn tie_break_uses_the_complete_frozen_observation_key() {
        let mut limits = wgpu::Limits::downlevel_defaults();
        limits.max_texture_dimension_2d = 16_384;
        let candidates = vec![
            (ranked_candidate(&limits, (2, 1, "a", "a")), "vendor"),
            (ranked_candidate(&limits, (1, 5, "a", "a")), "device"),
            (ranked_candidate(&limits, (1, 4, "z", "a")), "name"),
            (ranked_candidate(&limits, (1, 4, "a", "z")), "driver"),
            (ranked_candidate(&limits, (1, 4, "a", "a")), "exact"),
        ];
        let (_, selected) = select_eligible_adapter(candidates).unwrap();
        assert_eq!(selected, "exact");
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
