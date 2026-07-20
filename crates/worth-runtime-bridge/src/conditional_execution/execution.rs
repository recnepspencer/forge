use worth_signal::facade::{SignalConditionalDecisionEvidence, SignalConditionalExecutionRequest};

use super::resolver_adapters::{ComparatorAdapter, ConditionAdapter};
use super::{
    BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeInstalledConditionalLowering,
    BridgeOwnedSignalRuntime,
};

pub struct BridgeConditionalExecutionRequest<'a> {
    pub lowering: &'a std::sync::Arc<BridgeInstalledConditionalLowering>,
    pub query_binding_identity: &'a str,
    pub query_capability_identity: u64,
    pub snapshot_identity: &'a str,
    pub bridge_snapshot_identity: Option<&'a crate::snapshot::TruthSnapshotIdentity>,
    pub execution_identity: &'a str,
    pub attempt: u64,
}

pub struct BridgeConditionalDecisionEvidence {
    lowering: std::sync::Arc<BridgeInstalledConditionalLowering>,
    query_binding_identity: std::sync::Arc<str>,
    query_capability_identity: u64,
    bridge_snapshot_identity: Option<crate::snapshot::TruthSnapshotIdentity>,
    signal: SignalConditionalDecisionEvidence,
    semantic_observations: std::sync::Arc<[super::BridgeConditionalSemanticObservation]>,
}

impl BridgeConditionalDecisionEvidence {
    pub fn lowering_identity(&self) -> &str {
        self.lowering.identity().as_str()
    }
    pub fn retains_exact_lowering(
        &self,
        lowering: &std::sync::Arc<BridgeInstalledConditionalLowering>,
    ) -> bool {
        std::sync::Arc::ptr_eq(&self.lowering, lowering)
    }
    pub fn query_binding_identity(&self) -> &str {
        &self.query_binding_identity
    }
    pub const fn query_capability_identity(&self) -> u64 {
        self.query_capability_identity
    }
    pub fn bridge_snapshot_identity(&self) -> Option<&crate::snapshot::TruthSnapshotIdentity> {
        self.bridge_snapshot_identity.as_ref()
    }
    pub fn signal(&self) -> &SignalConditionalDecisionEvidence {
        &self.signal
    }
    pub fn semantic_observation_reads(&self) -> usize {
        self.semantic_observations.len()
    }
    pub fn semantic_observations(&self) -> &[super::BridgeConditionalSemanticObservation] {
        &self.semantic_observations
    }
}

impl BridgeOwnedSignalRuntime {
    pub fn execute(
        &mut self,
        request: BridgeConditionalExecutionRequest<'_>,
        compute_context: &mut dyn std::any::Any,
    ) -> Result<BridgeConditionalDecisionEvidence, BridgeConditionalDenial> {
        if request.lowering.signal_contract.graph_instance_id()
            != self.graph.installed_graph_capability().graph_instance_id()
        {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::StaleLowering,
                "conditional lowering belongs to another Signal graph",
            ));
        }
        let admitted_snapshot = request
            .bridge_snapshot_identity
            .map(|identity| crate::delivery::open_planned_snapshot(&self.bridge, identity))
            .transpose()
            .map_err(|error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::SnapshotAdmission,
                    format!("conditional snapshot admission failed: {error:?}"),
                )
            })?;
        let bridge_snapshot_identity = admitted_snapshot
            .as_ref()
            .map(|snapshot| snapshot.snapshot_identity().clone());
        let force_on_demand = request
            .lowering
            .providers
            .trigger
            .as_ref()
            .is_some_and(|provider| provider.requested());
        let mut signal_request = SignalConditionalExecutionRequest::new(
            &request.lowering.signal_contract,
            request.snapshot_identity,
            request.execution_identity,
            request.attempt,
        );
        if force_on_demand {
            signal_request = signal_request.force_on_demand();
        }
        let compute = request.lowering.providers.compute.as_ref().ok_or_else(|| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::MissingComputeProvider,
                "installed conditional lowering lost its exact compute provider",
            )
        })?;
        let mut condition = ConditionAdapter::new(
            request.lowering,
            admitted_snapshot.as_ref(),
            &self.conditional_observations,
        );
        let mut comparator = ComparatorAdapter::new(request.lowering);
        let signal = self.graph.execute_installed_conditional(
            signal_request,
            &mut condition,
            &mut comparator,
            || {
                compute
                    .compute(compute_context)
                    .map_err(worth_signal::facade::SignalError::invalid_input)
            },
        );
        let signal = match signal {
            Ok(signal) => signal,
            Err(failure) => {
                let counters = failure.counters();
                let observation_reads = condition.observation_count();
                if let Some(denial) = condition.take_observation_denial() {
                    return Err(denial.with_execution_counters(counters, observation_reads));
                }
                return Err(BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::SignalExecution,
                    format!("{:?}", failure.into_error()),
                )
                .with_execution_counters(counters, observation_reads));
            }
        };
        let observations = condition.take_observations();
        // A successful semantic read advances the condition's observation
        // baseline even when the condition suppresses compute. Otherwise a
        // threshold that initially suppresses would compare against `None`
        // forever and could never observe a later domain delta.
        for observation in observations.iter() {
            self.conditional_observations.insert(
                (
                    request.lowering.signal_node(),
                    observation.dependency_ordinal(),
                ),
                observation.current().clone(),
            );
        }
        Ok(BridgeConditionalDecisionEvidence {
            lowering: std::sync::Arc::clone(request.lowering),
            query_binding_identity: request.query_binding_identity.into(),
            query_capability_identity: request.query_capability_identity,
            bridge_snapshot_identity,
            signal,
            semantic_observations: observations,
        })
    }
}
