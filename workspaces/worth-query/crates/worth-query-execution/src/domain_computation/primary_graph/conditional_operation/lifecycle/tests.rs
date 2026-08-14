use super::*;
use crate::domain_computation::primary_graph::conditional_operation::installation::WorthQueryConditionalRuntimeInstallationDenial;
use worth_query_installation::facade::{
    WorthQueryNamedClockFailure, WorthQueryNamedClockFailureKind,
};

struct TestSchema;

struct InstalledClock {
    identity: String,
    lease: Arc<ConditionalClockLease>,
}

impl WorthQueryInstalledConditionalOperation<TestSchema> for InstalledClock {
    fn binding_identity(&self) -> &str {
        &self.identity
    }

    fn installation_canonical_work(
        &self,
    ) -> worth_query_installation::facade::WorthQueryCanonicalWorkEvidence {
        worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero()
    }

    fn matches_clock_lease(&self, lease: &Arc<ConditionalClockLease>) -> bool {
        Arc::ptr_eq(&self.lease, lease)
    }

    fn reconstruct(
        &mut self,
        _runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            TestSchema,
        >,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        Ok(())
    }

    fn intent_entity_kind(
        &self,
        _runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<TestSchema>,
    ) -> Option<worth_relational::facade::identity::KindId> {
        None
    }

    fn refresh_authoritative(
        &mut self,
        _runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<TestSchema>,
        _bridge: &mut BridgeOwnedSignalRuntime,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        Ok(())
    }

    fn reconcile_reconstruction(
        &mut self,
        _bridge: &mut BridgeOwnedSignalRuntime,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        Ok(())
    }

    fn prepare_derived_runtime_reinstallation(
        &self,
        _runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            TestSchema,
        >,
        _bridge: &mut BridgeOwnedSignalRuntime,
        _graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        _affinity: &super::super::publication::ConditionalRuntimeAffinity,
    ) -> Result<
        WorthQueryPreparedConditionalRuntimeBinding,
        WorthQueryConditionalRuntimeInstallationDenial,
    > {
        unreachable!("registry lease fixture does not install a runtime binding")
    }

    fn apply_derived_runtime_reinstallation(
        &mut self,
        _prepared: WorthQueryPreparedConditionalRuntimeBinding,
    ) {
        unreachable!("registry lease fixture does not install a runtime binding")
    }

    fn reconcile_prepared_runtime_reinstallation(
        &self,
        _bridge: &mut BridgeOwnedSignalRuntime,
        _prepared: &mut WorthQueryPreparedConditionalRuntimeBinding,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        Ok(())
    }

    fn observe_clock(
        &mut self,
        _bridge: &mut BridgeOwnedSignalRuntime,
        _runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
                TestSchema,
            >,
        _truth: &WorthQueryConditionalTruthBasis,
    ) -> ErasedClockObservationOutcome {
        ErasedClockObservationOutcome::Stale
    }

    fn retained_resource_counts(&self) -> WorthQueryConditionalRetainedResourceCounts {
        Default::default()
    }

    fn reconstruction_work(
        &self,
    ) -> crate::domain_computation::primary_graph::conditional_operation::temporal_reconstruction::WorthQueryTemporalReconstructionWork{
        Default::default()
    }

    fn lifecycle_resources(
        &self,
    ) -> crate::domain_computation::primary_graph::conditional_operation::lifecycle_inventory::WorthQueryConditionalOperationLiveness{
        crate::domain_computation::primary_graph::conditional_operation::lifecycle_inventory::WorthQueryConditionalOperationLiveness {
            binding: Default::default(),
            lease: Default::default(),
            wakes: Vec::new(),
            intents: Vec::new(),
            attempts: Vec::new(),
        }
    }
}

#[test]
fn registry_requires_the_exact_private_installation_lease() {
    let installed_lease = Arc::new(ConditionalClockLease);
    let foreign_lease = Arc::new(ConditionalClockLease);
    let mut registry = WorthQueryConditionalOperationRegistry::<TestSchema>::default();
    registry
        .install(Box::new(InstalledClock {
            identity: "clock-binding".to_string(),
            lease: Arc::clone(&installed_lease),
        }))
        .unwrap();

    assert!(registry.contains_clock("clock-binding", &installed_lease));
    assert!(!registry.contains_clock("clock-binding", &foreign_lease));
    assert!(!registry.contains_clock("another-binding", &installed_lease));
}

#[test]
fn clock_source_failure_and_panic_are_isolated_as_typed_postures() {
    let unavailable = isolate_clock_source::<()>(|| {
        Err(WorthQueryNamedClockFailure::new(
            WorthQueryNamedClockFailureKind::SourceUnavailable,
            "timer service unavailable",
        ))
    })
    .unwrap_err();
    assert!(matches!(
        unavailable,
        ErasedClockObservationOutcome::Failed {
            kind: WorthQueryConditionalClockObservationFailureKind::SourceUnavailable,
            ..
        }
    ));

    let panicked = isolate_clock_source::<()>(|| panic!("clock panic")).unwrap_err();
    assert!(matches!(
        panicked,
        ErasedClockObservationOutcome::Failed {
            kind: WorthQueryConditionalClockObservationFailureKind::SourcePanicked,
            ..
        }
    ));
}
