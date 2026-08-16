use std::{collections::BTreeMap, sync::Arc};

use worth_runtime_bridge::facade::{
    BridgeInstalledConditionalLowering, BridgeManagedClockBinding, BridgeOwnedSignalRuntime,
};

use super::{
    ConditionalClockLease, ErasedClockObservationOutcome, WorthQueryConditionalTruthBasis,
};
use crate::domain_computation::primary_graph::conditional_operation::{
    installation::WorthQueryConditionalRuntimeInstallationDenial,
    signal_decision_reentry::WorthQueryRetainedConditionalWake,
    temporal_reconstruction::WorthQueryReconstructedTemporalIntent,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

pub(in crate::domain_computation::primary_graph) trait WorthQueryInstalledConditionalOperation<
    Schema,
>: Send
{
    fn binding_identity(&self) -> &str;

    fn installation_canonical_work(
        &self,
    ) -> worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

    fn matches_clock_lease(&self, lease: &Arc<ConditionalClockLease>) -> bool;

    fn reconstruct(
        &mut self,
        runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial>;

    fn intent_entity_kind(
        &self,
        runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    ) -> Option<worth_relational::facade::identity::KindId>;

    fn authoritative_commit_routes(
        &self,
    ) -> (Vec<worth_relational::facade::transactions::RecordRef>, bool) {
        (Vec::new(), false)
    }

    fn refresh_authoritative(
        &mut self,
        runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        bridge: &mut BridgeOwnedSignalRuntime,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial>;

    fn reconcile_reconstruction(
        &mut self,
        bridge: &mut BridgeOwnedSignalRuntime,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial>;

    fn prepare_derived_runtime_reinstallation(
        &self,
        runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        bridge: &mut BridgeOwnedSignalRuntime,
        graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        affinity: &super::super::publication::ConditionalRuntimeAffinity,
    ) -> Result<
        WorthQueryPreparedConditionalRuntimeBinding,
        WorthQueryConditionalRuntimeInstallationDenial,
    >;

    fn reconcile_prepared_runtime_reinstallation(
        &self,
        bridge: &mut BridgeOwnedSignalRuntime,
        prepared: &mut WorthQueryPreparedConditionalRuntimeBinding,
    ) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial>;

    fn apply_derived_runtime_reinstallation(
        &mut self,
        prepared: WorthQueryPreparedConditionalRuntimeBinding,
    );

    fn observe_clock(
        &mut self,
        bridge: &mut BridgeOwnedSignalRuntime,
        runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        truth: &WorthQueryConditionalTruthBasis,
    ) -> ErasedClockObservationOutcome;

    fn retained_resource_counts(&self) -> WorthQueryConditionalRetainedResourceCounts;

    fn reconstruction_work(
        &self,
    ) -> super::super::temporal_reconstruction::WorthQueryTemporalReconstructionWork;

    fn lifecycle_resources(
        &self,
    ) -> super::super::lifecycle_inventory::WorthQueryConditionalOperationLiveness;
}

#[derive(Clone, Copy, Default)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryConditionalRetainedResourceCounts
{
    pub(in crate::domain_computation::primary_graph::conditional_operation) wakes: usize,
    pub(in crate::domain_computation::primary_graph::conditional_operation) intents: usize,
    pub(in crate::domain_computation::primary_graph::conditional_operation) attempts: usize,
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryPreparedConditionalRuntimeBinding
{
    pub(super) lowering: Arc<BridgeInstalledConditionalLowering>,
    pub(super) managed_clock: BridgeManagedClockBinding,
    pub(super) runtime_binding_identity: Arc<str>,
    pub(super) runtime_canonical_identity:
        Arc<super::super::canonical_identity::WorthQueryTemporalRuntimeBindingIdentity>,
    pub(super) installation_canonical_work:
        worth_query_installation::facade::WorthQueryCanonicalWorkEvidence,
    pub(super) runtime_capability_identity: u64,
    pub(super) authoritative_reconstruction: Box<dyn std::any::Any + Send>,
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryInstalledTemporalOperation<
    Binding,
    Reconstruction,
    Execution,
    Clock,
    Input,
> {
    pub(in crate::domain_computation::primary_graph::conditional_operation) lifecycle_token:
        Arc<()>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) binding_identity:
        Arc<crate::domain_computation::primary_graph::conditional_operation::canonical_identity::WorthQueryTemporalBindingIdentity>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) installation_canonical_work:
        worth_query_installation::facade::WorthQueryCanonicalWorkEvidence,
    pub(in crate::domain_computation::primary_graph::conditional_operation) clock_lease:
        Arc<ConditionalClockLease>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) binding: Binding,
    pub(in crate::domain_computation::primary_graph::conditional_operation) reconstruction:
        Reconstruction,
    pub(in crate::domain_computation::primary_graph::conditional_operation) execution: Execution,
    pub(in crate::domain_computation::primary_graph::conditional_operation) lowering:
        Arc<BridgeInstalledConditionalLowering>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) managed_clock:
        BridgeManagedClockBinding,
    pub(in crate::domain_computation::primary_graph::conditional_operation) runtime_binding_identity:
        Arc<str>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) runtime_canonical_identity:
        Arc<crate::domain_computation::primary_graph::conditional_operation::canonical_identity::WorthQueryTemporalRuntimeBindingIdentity>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) runtime_capability_identity:
        u64,
    pub(in crate::domain_computation::primary_graph::conditional_operation) retained_wakes:
        Vec<WorthQueryRetainedConditionalWake>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) reconstructed_intents:
        BTreeMap<String, WorthQueryReconstructedTemporalIntent<Clock, Input>>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) reconstruction_work:
        crate::domain_computation::primary_graph::conditional_operation::temporal_reconstruction::WorthQueryTemporalReconstructionWork,
    pub(in crate::domain_computation::primary_graph::conditional_operation) authoritative_commit_cursor:
        Option<u64>,
    pub(in crate::domain_computation::primary_graph::conditional_operation) committed_operation_count:
        usize,
    pub(in crate::domain_computation::primary_graph::conditional_operation) already_committed_operation_count:
        usize,
    pub(in crate::domain_computation::primary_graph::conditional_operation) failed_operation_count:
        usize,
    pub(in crate::domain_computation::primary_graph::conditional_operation) indeterminate_operation_count:
        usize,
}
