mod diagnostics;
mod disposition;
mod recovery_join;
#[allow(
    dead_code,
    reason = "Wave A establishes family cutover seams before record-serving consumers move"
)]
pub(in crate::physical_runtime) mod resident_admission;
mod root_protocol_admission_denial;
mod scrub;

pub use diagnostics::{
    PhysicalRootProtocolRoute, ResidentAdmissionCounters, RootProtocolRouteCounters,
};
pub(in crate::physical_runtime) use diagnostics::{
    ResidentAdmissionCounterCells, RootProtocolRouteCounterCells,
};
pub(in crate::physical_runtime) use disposition::{
    project_resident_current_root_selector_authority,
    project_resident_previous_root_selector_authority, project_resident_root_manifest_authority,
    StoreOwnerDispositionAdapterDenial,
};
pub use disposition::{
    DamagedPhysicalAuthorityObservation, DamagedPhysicalDerivedDisposition,
    IndeterminateDerivedRebuildability, IntactPhysicalAuthorityObservation,
    IntactPhysicalDerivedObservation, OwnerDispositionProjectionDenial,
    PhysicalArtifactDisposition, PhysicalArtifactRoleDisposition,
    RebuildablePhysicalDerivedObservation, UnknownDerivedRebuildability,
};
pub(in crate::physical_runtime) use recovery_join::{
    RecoveryIntegrityHandoffBinding, RecoveryIntegrityRuntimeGeneration,
};
pub use root_protocol_admission_denial::RootProtocolAdmissionDenial;
pub(in crate::physical_runtime) use scrub::{
    ManagedPhysicalIntegrityScrubHandle, ManagedPhysicalIntegrityScrubProgress,
    ManagedPhysicalIntegrityScrubRequest,
};

#[cfg(test)]
mod owner_valid_compile_contracts {
    use worth_store_physical_integrity::PhysicalIntegrityObservationOutcome;

    use super::*;
    use crate::physical_runtime::LifecycleGeneration;

    fn bind_recovery_handoff(
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
        root_generation: u64,
        generation: LifecycleGeneration,
        residency: &crate::physical_runtime::record_serving::RecordFramePorts,
        observations: Vec<PhysicalIntegrityObservationOutcome>,
    ) {
        residency.invalidate_integrity_validation_for_runtime_transition();
        let runtime_generation = RecoveryIntegrityRuntimeGeneration::bind(generation);
        let _ = RecoveryIntegrityHandoffBinding::bind(
            store,
            root_generation,
            runtime_generation,
            observations,
        );
    }

    fn drive_scrub<'runtime, 'media>(
        request: ManagedPhysicalIntegrityScrubRequest<'runtime, 'media>,
        generation: LifecycleGeneration,
    ) {
        let mut handle = ManagedPhysicalIntegrityScrubHandle::start(request);
        let _: ManagedPhysicalIntegrityScrubProgress = handle.next(generation, |_| {});
        handle.cancel();
        handle.close();
    }

    #[test]
    fn phase_two_owner_bind_shapes_type_check_without_forging_validation() {
        let _ = bind_recovery_handoff;
        let _ = drive_scrub;
    }
}
