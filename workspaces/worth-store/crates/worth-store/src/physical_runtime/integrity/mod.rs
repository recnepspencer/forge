mod diagnostics;
mod recovery_join;
mod resident_admission;
mod root_protocol_admission_denial;
mod scrub;

pub(in crate::physical_runtime) use diagnostics::RootProtocolRouteCounterCells;
pub use diagnostics::{PhysicalRootProtocolRoute, RootProtocolRouteCounters};
pub(in crate::physical_runtime) use recovery_join::{
    RecoveryIntegrityHandoffBinding, RecoveryIntegrityRuntimeGeneration,
};
pub(in crate::physical_runtime) use resident_admission::{
    admit_loaded_root_manifest, ResidentIntegrityRecordBinding,
};
pub use root_protocol_admission_denial::RootProtocolAdmissionDenial;
pub(in crate::physical_runtime) use scrub::{
    ManagedPhysicalIntegrityScrubHandle, ManagedPhysicalIntegrityScrubProgress,
    ManagedPhysicalIntegrityScrubRequest,
};

#[cfg(test)]
mod owner_valid_compile_contracts {
    use worth_store_buffer_pool::PhysicalFrameLease;
    use worth_store_physical_integrity::{
        IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
        IntegrityValidatedRootManifest, PhysicalIntegrityObservationOutcome,
    };

    use super::*;
    use crate::physical_runtime::LifecycleGeneration;

    fn bind_current<'lease>(
        lease: &'lease PhysicalFrameLease,
        generation: LifecycleGeneration,
        validated: IntegrityValidatedCurrentRootSelector<'lease>,
    ) {
        let _ = ResidentIntegrityRecordBinding::bind_current_root_selector(
            lease, generation, validated,
        );
    }

    fn bind_previous<'lease>(
        lease: &'lease PhysicalFrameLease,
        generation: LifecycleGeneration,
        validated: IntegrityValidatedPreviousRootSelector<'lease>,
    ) {
        let _ = ResidentIntegrityRecordBinding::bind_previous_root_selector(
            lease, generation, validated,
        );
    }

    fn bind_manifest<'lease>(
        lease: &'lease PhysicalFrameLease,
        generation: LifecycleGeneration,
        validated: IntegrityValidatedRootManifest<'lease>,
    ) {
        let _ = ResidentIntegrityRecordBinding::bind_root_manifest(lease, generation, validated);
    }

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
        let _ = bind_current;
        let _ = bind_previous;
        let _ = bind_manifest;
        let _ = bind_recovery_handoff;
        let _ = drive_scrub;
    }
}
