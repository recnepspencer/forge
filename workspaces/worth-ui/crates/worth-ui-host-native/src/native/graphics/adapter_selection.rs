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
