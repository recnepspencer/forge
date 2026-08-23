mod layout_indexes;
mod offline_verifier;
mod operations;
mod physical_backend;
mod physical_certification;
mod physical_format;
mod physical_isolation;
mod process_bundle;
mod recovery_physics_planning;
mod recovery_physics_redo;
mod recovery_physics_source;
mod recovery_runtime_entry;
mod recovery_runtime_orchestration;
mod recovery_runtime_outcomes;
mod recovery_runtime_progression;
mod security;
mod store_construction;
mod store_coordination;
mod store_freshness;
mod wal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DestinationTopologyContract {
    pub(super) path: &'static str,
    pub(super) owner: &'static str,
    pub(super) responsibility: &'static str,
    pub(super) dependency_posture: &'static str,
    pub(super) phase: &'static str,
    pub(super) status: &'static str,
}

impl DestinationTopologyContract {
    pub(super) const fn new(
        path: &'static str,
        owner: &'static str,
        responsibility: &'static str,
        dependency_posture: &'static str,
        phase: &'static str,
        status: &'static str,
    ) -> Self {
        Self {
            path,
            owner,
            responsibility,
            dependency_posture,
            phase,
            status,
        }
    }
}

const CONTRACT_FAMILIES: &[&[DestinationTopologyContract]] = &[
    recovery_runtime_entry::DESTINATIONS,
    recovery_runtime_progression::DESTINATIONS,
    recovery_runtime_orchestration::DESTINATIONS,
    recovery_runtime_outcomes::DESTINATIONS,
    recovery_physics_source::DESTINATIONS,
    recovery_physics_redo::DESTINATIONS,
    recovery_physics_planning::DESTINATIONS,
    store_freshness::DESTINATIONS,
    store_coordination::DESTINATIONS,
    store_construction::DESTINATIONS,
    physical_backend::DESTINATIONS,
    physical_format::DESTINATIONS,
    wal::DESTINATIONS,
    offline_verifier::DESTINATIONS,
    physical_certification::DESTINATIONS,
    layout_indexes::DESTINATIONS,
    security::DESTINATIONS,
    operations::DESTINATIONS,
    physical_isolation::DESTINATIONS,
    process_bundle::DESTINATIONS,
];

pub(super) fn destinations() -> impl Iterator<Item = &'static DestinationTopologyContract> {
    CONTRACT_FAMILIES.iter().flat_map(|family| family.iter())
}

pub(super) fn expected_destination(path: &str) -> Option<&'static DestinationTopologyContract> {
    destinations().find(|destination| destination.path == path)
}
