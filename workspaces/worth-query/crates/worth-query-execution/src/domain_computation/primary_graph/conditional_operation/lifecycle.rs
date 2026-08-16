use std::sync::Arc;
use worth_query_installation::facade::{
    WorthQueryHostConditionalPredicateProvider, WorthQueryInstalledTemporalConditionalOperation,
    WorthQueryNamedClock, WorthQueryNamedClockSource, WorthQueryTemporalIntentProjector,
};
use worth_runtime_bridge::facade::{BridgeManagedClockObservationParts, BridgeOwnedSignalRuntime};

use super::clock_observation::{
    ErasedClockObservationOutcome, ErasedClockObservationReceipt,
    WorthQueryConditionalClockObservationFailureKind,
};
use super::installation::ConditionalClockLease;
use super::signal_decision_reentry::WorthQueryConditionalTruthBasis;

mod authoritative_clock_progression;
mod bridge_clock_outcome;
mod clock_source_observation;
mod commit_routes;
mod due_wake_retention;
mod installed_operation;
mod registry;
mod retained_wake_retirement;
mod runtime_rebinding;
pub(in crate::domain_computation::primary_graph::conditional_operation) use clock_source_observation::isolate_clock_source;
pub(in crate::domain_computation::primary_graph::conditional_operation) use retained_wake_retirement::retire_stale_retained_wakes;
pub(in crate::domain_computation::primary_graph) use installed_operation::{
    WorthQueryConditionalRetainedResourceCounts, WorthQueryInstalledConditionalOperation,
    WorthQueryInstalledTemporalOperation, WorthQueryPreparedConditionalRuntimeBinding,
};
pub(in crate::domain_computation::primary_graph) use registry::WorthQueryConditionalOperationRegistry;

#[rustfmt::skip]
impl<
        Schema, ApplicationOperation, Input, D, O, F, Node, Provider, Clock, Source, Query, Parameters, QueryResult, Scope, Projector,
        PrincipalBinding, PrincipalMapping, Principal, PrincipalIdentity, ScopeAspect, ScopeField, ScopeValue, ScopeWrite, ScopeUnit, PrincipalSource,
        QueryAuthorization, Invoker, IntentEntity, IdentityAspect, IdentityField, IdentityValue, IdentityWrite, IdentityUnit, RevisionAspect, RevisionField,
        RevisionValue, RevisionWrite, RevisionEquality, RevisionUnit, LifecycleAspect, LifecycleField, LifecycleValue, LifecycleWrite, LifecycleEquality, LifecycleUnit, Authorization,
    > WorthQueryInstalledConditionalOperation<Schema>
    for WorthQueryInstalledTemporalOperation<
        WorthQueryInstalledTemporalConditionalOperation<
            Schema, ApplicationOperation, Input, D, O, F, Node, Provider,
            Clock, Source, Query, Parameters, QueryResult, Scope, Projector,
        >,
        super::reconstruction_authority::WorthQueryTemporalReconstructionAccess<
            Schema, PrincipalBinding, PrincipalMapping, Principal, PrincipalIdentity, Scope,
            ScopeAspect, ScopeField, ScopeValue, ScopeWrite, ScopeUnit, PrincipalSource, QueryAuthorization,
        >,
        super::operation_invocation::WorthQueryTemporalOperationExecution<
            Schema, ApplicationOperation, Input, Scope, Invoker, IntentEntity,
            IdentityAspect, IdentityField, IdentityValue, IdentityWrite, IdentityUnit,
            RevisionAspect, RevisionField, RevisionValue, RevisionWrite, RevisionEquality, RevisionUnit,
            LifecycleAspect, LifecycleField, LifecycleValue, LifecycleWrite, LifecycleEquality, LifecycleUnit, Authorization,
        >,
        Clock,
        Input,
    >
where
    Schema: worth_query_installation::facade::ApplicationSchema + 'static,
    ApplicationOperation: 'static,
    Input: Clone + Send + Sync + 'static,
    D: 'static,
    O: 'static,
    F: 'static,
    Node: 'static,
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
    Clock: WorthQueryNamedClock,
    Source: WorthQueryNamedClockSource<Clock>,
    Query: 'static,
    Parameters: 'static,
    QueryResult: crate::domain_computation::primary_graph::WorthQueryApplicationProjection<Schema, Query> + 'static,
    Scope: 'static,
    Projector: WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>,
    PrincipalBinding: 'static,
    PrincipalMapping: 'static,
    Principal: 'static,
    PrincipalIdentity: worth_query_installation::facade::TypedApplicationIdentityValue + 'static,
    ScopeAspect: 'static,
    ScopeField: 'static,
    ScopeValue: worth_query_installation::facade::TypedApplicationValue + Clone + Send + 'static,
    ScopeWrite: worth_query_installation::facade::WritePosture + 'static,
    ScopeUnit: worth_query_installation::facade::ApplicationFieldUnit + 'static,
    PrincipalSource: super::reconstruction_authority::WorthQueryTemporalPrincipalSource<Schema>,
    QueryAuthorization: super::WorthQueryTemporalQueryAuthorization<Schema, Query, Parameters, QueryResult, Principal, PrincipalIdentity, Scope>,
    Invoker: super::operation_invocation::WorthQueryTemporalOperationInvoker<Schema, ApplicationOperation, Input, Scope>,
    IntentEntity: 'static,
    IdentityAspect: 'static,
    IdentityField: worth_query_installation::facade::OperationReads<ApplicationOperation> + 'static,
    IdentityValue: worth_query_installation::facade::TypedApplicationReadableValue + Clone + Send + 'static,
    IdentityWrite: worth_query_installation::facade::WritePosture + 'static,
    IdentityUnit: worth_query_installation::facade::ApplicationFieldUnit + 'static,
    RevisionAspect: 'static,
    RevisionField: worth_query_installation::facade::OperationReads<ApplicationOperation> + worth_query_installation::facade::OperationWrites<ApplicationOperation> + 'static,
    RevisionValue: worth_query_installation::facade::WorthQueryTemporalIntentRevisionValue + worth_query_installation::facade::TypedApplicationReadableValue + Clone + Send + 'static,
    RevisionWrite: worth_query_installation::facade::WritableCapability + 'static,
    RevisionEquality: 'static,
    RevisionUnit: worth_query_installation::facade::ApplicationFieldUnit + 'static,
    LifecycleAspect: 'static,
    LifecycleField: worth_query_installation::facade::OperationReads<ApplicationOperation> + worth_query_installation::facade::OperationWrites<ApplicationOperation> + 'static,
    LifecycleValue: worth_query_installation::facade::TypedApplicationReadableValue + Clone + Send + 'static,
    LifecycleWrite: worth_query_installation::facade::WritableCapability + 'static,
    LifecycleEquality: 'static,
    LifecycleUnit: worth_query_installation::facade::ApplicationFieldUnit + 'static,
    Authorization: super::WorthQueryTemporalOperationAuthorization<Schema, ApplicationOperation, Input, Scope> + 'static,
{
    fn binding_identity(&self) -> &str {
        self.binding_identity.support_identity()
    }

    fn installation_canonical_work(
        &self,
    ) -> worth_query_installation::facade::WorthQueryCanonicalWorkEvidence {
        self.installation_canonical_work
    }

    fn matches_clock_lease(&self, lease: &Arc<ConditionalClockLease>) -> bool {
        Arc::ptr_eq(&self.clock_lease, lease)
    }

    fn reconstruct(
        &mut self,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
    ) -> Result<(), super::installation::WorthQueryConditionalRuntimeInstallationDenial> {
        self.authoritative_commit_cursor =
            Some(runtime.primary_provider.conditional_commit_sequence());
        let reconstruction = super::temporal_reconstruction::reconstruct_temporal_intents(
            runtime,
            &self.binding,
            &self.reconstruction,
            self.execution.identity_field,
        )?;
        self.reconstructed_intents = reconstruction.intents;
        self.reconstruction_work = reconstruction.work;
        Ok(())
    }

    fn intent_entity_kind(
        &self,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    ) -> Option<worth_relational::facade::identity::KindId> {
        runtime
            .primary_provider
            .conditional_entity_kind(self.execution.identity_field.entity())
    }

    fn authoritative_commit_routes(
        &self,
    ) -> (
        Vec<worth_relational::facade::transactions::RecordRef>,
        bool,
    ) {
        commit_routes::temporal_commit_routes(&self.reconstructed_intents, &self.lowering)
    }

    fn refresh_authoritative(
        &mut self,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        bridge: &mut BridgeOwnedSignalRuntime,
    ) -> Result<(), super::installation::WorthQueryConditionalRuntimeInstallationDenial> {
        let refreshed = super::temporal_reconstruction::reconstruct_temporal_intents(
            runtime,
            &self.binding,
            &self.reconstruction,
            self.execution.identity_field,
        )?;
        let mut intents = refreshed.intents;
        super::temporal_reconstruction::reconcile_refreshed_temporal_intents(
            bridge,
            &self.managed_clock,
            &self.reconstructed_intents,
            &mut intents,
        )?;
        self.reconstructed_intents = intents;
        self.reconstruction_work = refreshed.work;
        retire_stale_retained_wakes(&mut self.retained_wakes, &self.reconstructed_intents);
        Ok(())
    }

    #[rustfmt::skip]
    fn reconcile_reconstruction(&mut self, bridge: &mut BridgeOwnedSignalRuntime) -> Result<(), super::installation::WorthQueryConditionalRuntimeInstallationDenial> {
        super::temporal_reconstruction::reconcile_temporal_intents(bridge, &self.managed_clock, &mut self.reconstructed_intents)
    }

    fn prepare_derived_runtime_reinstallation(
        &self,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
        bridge: &mut BridgeOwnedSignalRuntime,
        graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        affinity: &super::publication::ConditionalRuntimeAffinity,
    ) -> Result<
        WorthQueryPreparedConditionalRuntimeBinding,
        super::installation::WorthQueryConditionalRuntimeInstallationDenial,
    > {
        let mut prepared = runtime_rebinding::prepare_temporal_runtime_binding(
            &self.binding,
            graph,
            bridge,
            affinity,
            &self.binding_identity,
        )?;
        prepared.authoritative_reconstruction = Box::new(
            super::temporal_reconstruction::reconstruct_temporal_intents(
                runtime,
                &self.binding,
                &self.reconstruction,
                self.execution.identity_field,
            )?,
        );
        Ok(prepared)
    }

    fn apply_derived_runtime_reinstallation(
        &mut self,
        prepared: WorthQueryPreparedConditionalRuntimeBinding,
    ) {
        self.lowering = prepared.lowering;
        self.managed_clock = prepared.managed_clock;
        self.runtime_binding_identity = prepared.runtime_binding_identity;
        self.runtime_canonical_identity = prepared.runtime_canonical_identity;
        self.installation_canonical_work = prepared.installation_canonical_work;
        self.runtime_capability_identity = prepared.runtime_capability_identity;
        let reconstruction = *prepared
            .authoritative_reconstruction
            .downcast::<super::temporal_reconstruction::WorthQueryTemporalReconstruction<Clock, Input>>()
            .expect("prepared temporal reconstruction retains its installed binding type");
        self.reconstructed_intents = reconstruction.intents;
        self.reconstruction_work = reconstruction.work;
        self.retained_wakes.clear();
    }

    #[rustfmt::skip]
    fn reconcile_prepared_runtime_reinstallation(&self, bridge: &mut BridgeOwnedSignalRuntime, prepared: &mut WorthQueryPreparedConditionalRuntimeBinding) -> Result<(), super::installation::WorthQueryConditionalRuntimeInstallationDenial> {
        let reconstruction = prepared.authoritative_reconstruction.downcast_mut::<super::temporal_reconstruction::WorthQueryTemporalReconstruction<Clock, Input>>().expect("prepared temporal reconstruction retains its installed binding type");
        super::temporal_reconstruction::reconcile_temporal_intents(bridge, &prepared.managed_clock, &mut reconstruction.intents)
    }

    fn observe_clock(
        &mut self,
        bridge: &mut BridgeOwnedSignalRuntime,
        runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
        truth: &WorthQueryConditionalTruthBasis,
    ) -> ErasedClockObservationOutcome {
        if let Some(detail) = runtime.primary_provider.conditional_maintenance_failure() {
            return ErasedClockObservationOutcome::Failed {
                kind: WorthQueryConditionalClockObservationFailureKind::RuntimeRejected,
                detail,
            };
        }
        let observation = match isolate_clock_source(|| self.binding.observe_clock_for_runtime()) {
            Ok(observation) => observation,
            Err(outcome) => return outcome,
        };
        let sequence = observation.sequence();
        let coordinate = observation.observed_time().nanoseconds();
        let (watched_records, include_whole_graph) = self.authoritative_commit_routes();
        let mut authoritative = match authoritative_clock_progression::reconsider_authoritative_clock_work(
            authoritative_clock_progression::AuthoritativeClockWork {
                runtime,
                bridge,
                lowering: &self.lowering,
                cursor: &mut self.authoritative_commit_cursor,
                maximum_commits: self.binding.bounds().maximum_due_wakes_per_observation(),
                watched_records,
                include_whole_graph,
                retained_wakes: &mut self.retained_wakes,
                runtime_binding_identity: &self.runtime_binding_identity,
                runtime_capability_identity: self.runtime_capability_identity,
                truth,
            },
        ) {
            Ok(progress) => progress,
            Err(outcome) => return outcome,
        };
        let outcome = bridge.observe_managed_clock(BridgeManagedClockObservationParts {
            binding: &self.managed_clock,
            source_identity: observation.source().as_str(),
            timeline_identity: observation.timeline().as_str(),
            sequence,
            observed_coordinate: coordinate,
        });
        let retain_and_reenter = |accepted| {
            let receipt = self.retain_due(
                accepted,
                bridge,
                truth,
                &authoritative.granular_invalidations,
            );
            let operation = self
                .binding
                .clocked_node()
                .provider()
                .node()
                .operation()
                .application_operation();
            let counts = super::application_operation_reentry::reenter_retained_wakes(
                runtime,
                bridge,
                &self.managed_clock,
                operation,
                &self.reconstruction,
                &self.execution,
                &mut self.reconstructed_intents,
                &mut self.retained_wakes,
                &self.runtime_canonical_identity,
            );
            authoritative.granular_invalidations =
                super::authoritative_reconsideration::promote_performed_signal_deliveries(
                    std::mem::take(&mut authoritative.granular_invalidations),
                    &mut self.retained_wakes,
                );
            self.complete_clock_receipt(receipt, counts, authoritative)
        };
        bridge_clock_outcome::map_bridge_clock_outcome(outcome, retain_and_reenter)
    }

    fn retained_resource_counts(&self) -> WorthQueryConditionalRetainedResourceCounts {
        super::lifecycle_inventory::retained_resource_counts(
            &self.retained_wakes,
            self.reconstructed_intents.len(),
        )
    }

    fn reconstruction_work(
        &self,
    ) -> super::temporal_reconstruction::WorthQueryTemporalReconstructionWork {
        self.reconstruction_work
    }

    fn lifecycle_resources(
        &self,
    ) -> super::lifecycle_inventory::WorthQueryConditionalOperationLiveness {
        super::lifecycle_inventory::WorthQueryConditionalOperationLiveness {
            binding: Arc::downgrade(&self.lifecycle_token),
            lease: Arc::downgrade(&self.clock_lease),
            wakes: self
                .retained_wakes
                .iter()
                .map(|wake| Arc::downgrade(&wake.lifecycle_token))
                .collect(),
            intents: self
                .reconstructed_intents
                .values()
                .map(|intent| Arc::downgrade(&intent.lifecycle_token))
                .collect(),
            attempts: self
                .retained_wakes
                .iter()
                .filter(|wake| wake.application_attempted)
                .map(|wake| Arc::downgrade(&wake.lifecycle_token))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests;
