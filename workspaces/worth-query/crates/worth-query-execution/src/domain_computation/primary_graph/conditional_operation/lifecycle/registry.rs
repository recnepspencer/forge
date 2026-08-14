use std::{collections::BTreeMap, sync::Arc};

use worth_runtime_bridge::facade::BridgeOwnedSignalRuntime;

use super::WorthQueryInstalledConditionalOperation;
use crate::domain_computation::primary_graph::conditional_operation::{
    clock_observation::ErasedClockObservationOutcome,
    installation::{ConditionalClockLease, WorthQueryConditionalRuntimeInstallationDenial},
    signal_decision_reentry::WorthQueryConditionalTruthBasis,
};

pub(in crate::domain_computation::primary_graph) struct WorthQueryConditionalOperationRegistry<
    Schema,
> {
    installed: BTreeMap<String, Box<dyn WorthQueryInstalledConditionalOperation<Schema>>>,
    marker: std::marker::PhantomData<fn() -> Schema>,
}

impl<Schema> Default for WorthQueryConditionalOperationRegistry<Schema> {
    fn default() -> Self {
        Self {
            installed: BTreeMap::new(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<Schema> WorthQueryConditionalOperationRegistry<Schema> {
    pub(in crate::domain_computation::primary_graph) fn lifecycle_probe(
        &self,
        bridge: worth_runtime_bridge::facade::BridgeConditionalRuntimeLifecycleProbe,
    ) -> super::super::WorthQueryConditionalRuntimeLifecycleProbe {
        super::super::WorthQueryConditionalRuntimeLifecycleProbe::from_resources(
            self.installed
                .values()
                .map(|operation| operation.lifecycle_resources()),
            bridge,
        )
    }

    pub(in crate::domain_computation::primary_graph) fn len(&self) -> usize {
        self.installed.len()
    }

    pub(in crate::domain_computation::primary_graph) fn retained_resource_counts(
        &self,
    ) -> super::WorthQueryConditionalRetainedResourceCounts {
        self.installed.values().fold(
            super::WorthQueryConditionalRetainedResourceCounts::default(),
            |mut total, operation| {
                let counts = operation.retained_resource_counts();
                total.wakes = total.wakes.saturating_add(counts.wakes);
                total.intents = total.intents.saturating_add(counts.intents);
                total.attempts = total.attempts.saturating_add(counts.attempts);
                total
            },
        )
    }

    pub(in crate::domain_computation::primary_graph) fn installation_canonical_work(
        &self,
    ) -> worth_query_installation::facade::WorthQueryCanonicalWorkEvidence {
        self.installed.values().fold(
            worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
            |total, operation| total.combine(operation.installation_canonical_work()),
        )
    }

    pub(in crate::domain_computation::primary_graph) fn reconstruction_work(
        &self,
    ) -> crate::domain_computation::primary_graph::conditional_operation::temporal_reconstruction::WorthQueryTemporalReconstructionWork{
        self.installed
            .values()
            .fold(Default::default(), |mut total, operation| {
                let work = operation.reconstruction_work();
                total.examined_candidates = total
                    .examined_candidates
                    .saturating_add(work.examined_candidates);
                total.projected_records = total
                    .projected_records
                    .saturating_add(work.projected_records);
                total.projected_fields =
                    total.projected_fields.saturating_add(work.projected_fields);
                total.total_work_units =
                    total.total_work_units.saturating_add(work.total_work_units);
                total
            })
    }

    pub(in crate::domain_computation::primary_graph) fn prepare_derived_runtime_reinstallation(
        &self,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        bridge: &mut BridgeOwnedSignalRuntime,
        graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        affinity: &super::super::publication::ConditionalRuntimeAffinity,
    ) -> Result<
        BTreeMap<String, super::WorthQueryPreparedConditionalRuntimeBinding>,
        WorthQueryConditionalRuntimeInstallationDenial,
    > {
        self.installed
            .iter()
            .map(|(identity, operation)| {
                operation
                    .prepare_derived_runtime_reinstallation(runtime, bridge, graph, affinity)
                    .map(|prepared| (identity.clone(), prepared))
            })
            .collect()
    }

    pub(in crate::domain_computation::primary_graph) fn apply_derived_runtime_reinstallation(
        &mut self,
        mut prepared: BTreeMap<String, super::WorthQueryPreparedConditionalRuntimeBinding>,
    ) {
        for (identity, operation) in &mut self.installed {
            operation.apply_derived_runtime_reinstallation(
                prepared
                    .remove(identity)
                    .expect("prepared conditional inventory matches installed registry"),
            );
        }
        assert!(prepared.is_empty());
    }

    pub(in crate::domain_computation::primary_graph) fn reconcile_prepared_runtime_reinstallation(
        &mut self,
        bridge: &mut BridgeOwnedSignalRuntime,
        prepared: &mut BTreeMap<String, super::WorthQueryPreparedConditionalRuntimeBinding>,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        for (identity, operation) in &self.installed {
            operation.reconcile_prepared_runtime_reinstallation(
                bridge,
                prepared
                    .get_mut(identity)
                    .expect("prepared conditional inventory matches installed registry"),
            )?;
        }
        Ok(())
    }

    pub(in crate::domain_computation::primary_graph::conditional_operation) fn install(
        &mut self,
        operation: Box<dyn WorthQueryInstalledConditionalOperation<Schema>>,
    ) -> Result<(), ()> {
        let identity = operation.binding_identity().to_string();
        if self.installed.contains_key(&identity) {
            return Err(());
        }
        self.installed.insert(identity, operation);
        Ok(())
    }

    pub(in crate::domain_computation::primary_graph::conditional_operation) fn contains_clock(
        &self,
        identity: &str,
        lease: &Arc<ConditionalClockLease>,
    ) -> bool {
        self.installed
            .get(identity)
            .is_some_and(|operation| operation.matches_clock_lease(lease))
    }

    pub(in crate::domain_computation::primary_graph::conditional_operation) fn observe_clock(
        &mut self,
        identity: &str,
        lease: &Arc<ConditionalClockLease>,
        bridge: &mut BridgeOwnedSignalRuntime,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
        truth: &WorthQueryConditionalTruthBasis,
    ) -> Option<ErasedClockObservationOutcome> {
        let operation = self.installed.get_mut(identity)?;
        operation
            .matches_clock_lease(lease)
            .then(|| operation.observe_clock(bridge, runtime, truth))
    }

    pub(in crate::domain_computation::primary_graph) fn reconstruct_all(
        &mut self,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        for operation in self.installed.values_mut() {
            operation.reconstruct(runtime)?;
        }
        self.synchronize_commit_routes(runtime);
        Ok(())
    }

    pub(in crate::domain_computation::primary_graph) fn refresh_changed_intent_kinds(
        &mut self,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        bridge: &mut BridgeOwnedSignalRuntime,
        changed: &std::collections::BTreeSet<worth_relational::facade::identity::KindId>,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        for operation in self.installed.values_mut().filter(|operation| {
            operation
                .intent_entity_kind(runtime)
                .is_some_and(|kind| changed.contains(&kind))
        }) {
            operation.refresh_authoritative(runtime, bridge)?;
        }
        self.synchronize_commit_routes(runtime);
        Ok(())
    }

    pub(in crate::domain_computation::primary_graph) fn synchronize_commit_routes(
        &self,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    ) {
        let mut records = std::collections::BTreeSet::new();
        let mut whole_graph = false;
        for operation in self.installed.values() {
            let (operation_records, operation_whole_graph) =
                operation.authoritative_commit_routes();
            records.extend(operation_records);
            whole_graph |= operation_whole_graph;
        }
        runtime
            .primary_provider
            .replace_conditional_commit_routes(records, whole_graph);
    }

    pub(in crate::domain_computation::primary_graph) fn reconcile_all(
        &mut self,
        bridge: &mut BridgeOwnedSignalRuntime,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
        for operation in self.installed.values_mut() {
            operation.reconcile_reconstruction(bridge)?;
        }
        Ok(())
    }
}
