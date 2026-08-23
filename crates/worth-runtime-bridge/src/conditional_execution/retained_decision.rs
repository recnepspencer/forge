use std::sync::Arc;

use worth_signal::facade::SignalConditionalDecisionEvidence;

use super::{
    BridgeConditionalDecisionEvidence, BridgeConditionalDenial, BridgeConditionalDenialKind,
    BridgeConditionalSemanticObservation, BridgeInstalledConditionalLowering,
    BridgeOwnedSignalRuntime,
};

pub(super) struct BridgeRetainedConditionalDecisionCore {
    pub(super) bridge_runtime_key: u64,
    pub(super) lowering: Arc<BridgeInstalledConditionalLowering>,
    pub(super) bridge_snapshot_identity: Option<crate::snapshot::TruthSnapshotIdentity>,
    pub(super) signal_snapshot_projection: Arc<str>,
    pub(super) signal_execution_projection: Arc<str>,
    pub(super) attempt: u64,
    pub(super) signal: SignalConditionalDecisionEvidence,
    pub(super) semantic_observations: Arc<[BridgeConditionalSemanticObservation]>,
    pub(super) bridge_execution_counters: super::BridgeConditionalExecutionCounters,
    pub(super) triggering_change_set:
        Option<crate::correspondence::BridgeDeliveredCorrespondenceChangeSet>,
}

/// Bridge-owned handle to one exact Signal evaluation. It is intentionally
/// non-Clone and exposes no descriptive fields from which evidence can be
/// reconstructed.
pub struct BridgeRetainedConditionalDecisionSeed {
    core: Arc<BridgeRetainedConditionalDecisionCore>,
}

pub struct BridgeConditionalDecisionReentryRequest<'a> {
    pub seed: &'a BridgeRetainedConditionalDecisionSeed,
    pub lowering: &'a Arc<BridgeInstalledConditionalLowering>,
    pub query_binding_identity: &'a str,
    pub query_capability_identity: u64,
    pub snapshot_identity: &'a str,
    pub bridge_snapshot_identity: Option<&'a crate::snapshot::TruthSnapshotIdentity>,
}

impl BridgeOwnedSignalRuntime {
    pub fn reenter_retained_conditional_decision(
        &self,
        request: BridgeConditionalDecisionReentryRequest<'_>,
    ) -> Result<BridgeConditionalDecisionEvidence, BridgeConditionalDenial> {
        let mut counters = super::BridgeConditionalReentryCounters::default();
        let core = &request.seed.core;
        self.validate_retained_core(core, request.lowering, &mut counters)?;
        counters.snapshot_identity_checks = 2;
        if core.signal_snapshot_projection.as_ref() != request.snapshot_identity
            || core.bridge_snapshot_identity.as_ref() != request.bridge_snapshot_identity
        {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SnapshotAdmission,
                "retained conditional decision belongs to another exact snapshot",
            )
            .with_reentry_counters(counters));
        }
        counters.query_continuation_rebindings = 1;
        Ok(BridgeConditionalDecisionEvidence {
            core: Arc::clone(core),
            query_binding_identity: request.query_binding_identity.into(),
            query_capability_identity: request.query_capability_identity,
            reentry_counters: counters,
            performed_signal_invalidation: None,
        })
    }

    fn validate_retained_core(
        &self,
        core: &BridgeRetainedConditionalDecisionCore,
        lowering: &Arc<BridgeInstalledConditionalLowering>,
        counters: &mut super::BridgeConditionalReentryCounters,
    ) -> Result<(), BridgeConditionalDenial> {
        counters.runtime_key_checks = 1;
        if core.bridge_runtime_key != self.bridge.signal_runtime_key {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::StaleLowering,
                "retained conditional decision belongs to another Bridge runtime",
            )
            .with_reentry_counters(*counters));
        }
        counters.lowering_identity_checks = 1;
        counters.installed_lowering_lookups = 1;
        if !Arc::ptr_eq(&core.lowering, lowering)
            || !self
                .conditional_lowerings
                .get(&lowering.signal_node())
                .is_some_and(|installed| Arc::ptr_eq(installed, lowering))
        {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::StaleLowering,
                "retained conditional decision lost its exact installed lowering",
            )
            .with_reentry_counters(*counters));
        }
        counters.signal_graph_checks = 1;
        let graph = self
            .signal_runtime
            .graph()
            .installed_graph_capability()
            .graph_instance_id();
        if lowering.signal_graph_instance_id() != graph {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::ForeignSignalGraph,
                "retained conditional decision belongs to another Signal graph",
            )
            .with_reentry_counters(*counters));
        }
        counters.signal_contract_checks = 1;
        lowering
            .validate_signal_decision_contract(&core.signal)
            .map_err(|denial| denial.with_reentry_counters(*counters))
    }
}

impl BridgeConditionalDecisionEvidence {
    /// Retains this already-issued Bridge decision for exact target reentry.
    /// This is infallible because the seed carries the same private core that
    /// was created atomically with the successful Signal evaluation.
    pub fn retain_for_reentry(&self) -> BridgeRetainedConditionalDecisionSeed {
        BridgeRetainedConditionalDecisionSeed {
            core: Arc::clone(&self.core),
        }
    }
}
