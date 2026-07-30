use worth_store::physical_runtime::{
    PhysicalSignalAspectRole, PhysicalWorkSignalFamilySet, ServingPhysicalRuntime,
};

#[derive(Debug)]
pub(in crate::bounded_residency) struct PhysicalWorkSignalBindingEvidence {
    pub(in crate::bounded_residency) digest: [u8; 32],
    pub(in crate::bounded_residency) aspect_key: String,
    pub(in crate::bounded_residency) role: PhysicalSignalAspectRole,
    pub(in crate::bounded_residency) families: PhysicalWorkSignalFamilySet,
    pub(in crate::bounded_residency) partition: Option<String>,
}

pub(super) fn observe(
    serving: &ServingPhysicalRuntime,
) -> Box<[PhysicalWorkSignalBindingEvidence]> {
    serving
        .physical_signal_aspect_binding_observations()
        .into_vec()
        .into_iter()
        .map(|binding| PhysicalWorkSignalBindingEvidence {
            digest: *binding.digest().as_bytes(),
            aspect_key: binding.identity().aspect_key().as_str().to_owned(),
            role: binding.role(),
            families: binding.families(),
            partition: binding
                .partition()
                .map(|partition| partition.partition.0.clone()),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
