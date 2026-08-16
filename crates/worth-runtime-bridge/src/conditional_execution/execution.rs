use worth_signal::facade::adapters::SignalInvalidationExecutionReceipt;
use worth_signal::facade::{SignalConditionalDecisionEvidence, SignalConditionalExecutionRequest};

use super::resolver_adapters::{ComparatorAdapter, ConditionAdapter};
use super::retained_decision::BridgeRetainedConditionalDecisionCore;
use super::{
    BridgeConditionalDecisionEvidence, BridgeConditionalDenial, BridgeConditionalDenialKind,
    BridgeInstalledConditionalLowering, BridgeOwnedSignalRuntime,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BridgeConditionalExecutionCounters {
    pub signal_graph_checks: usize,
    pub snapshot_admission_attempts: usize,
    pub compute_provider_checks: usize,
    pub signal_execution_contacts: usize,
    pub observation_baseline_writes: usize,
    pub decisions_retained: usize,
    pub unrelated_lowering_scans: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BridgeConditionalReentryCounters {
    pub runtime_key_checks: usize,
    pub lowering_identity_checks: usize,
    pub installed_lowering_lookups: usize,
    pub signal_graph_checks: usize,
    pub signal_contract_checks: usize,
    pub snapshot_identity_checks: usize,
    pub query_continuation_rebindings: usize,
    pub unrelated_lowering_scans: usize,
}

pub struct BridgeConditionalExecutionRequest<'a> {
    pub lowering: &'a std::sync::Arc<BridgeInstalledConditionalLowering>,
    pub query_binding_identity: &'a str,
    pub query_capability_identity: u64,
    pub snapshot_identity: &'a str,
    pub truth_branch_identity: Option<&'a str>,
    pub bridge_snapshot_identity: Option<&'a crate::snapshot::TruthSnapshotIdentity>,
    pub execution_identity: &'a str,
    pub attempt: u64,
}

pub struct BridgeConditionalQueryContinuationAdmission<'a> {
    pub lowering: &'a std::sync::Arc<BridgeInstalledConditionalLowering>,
    pub query_binding_identity: &'a str,
    pub query_capability_identity: u64,
    pub signal_snapshot_projection: &'a str,
    pub bridge_snapshot_identity: Option<&'a crate::snapshot::TruthSnapshotIdentity>,
    pub signal_execution_projection: &'a str,
    pub attempt: u64,
}

impl BridgeOwnedSignalRuntime {
    pub fn execute(
        &mut self,
        request: BridgeConditionalExecutionRequest<'_>,
        compute_context: &mut dyn std::any::Any,
    ) -> Result<BridgeConditionalDecisionEvidence, BridgeConditionalDenial> {
        self.execute_with_managed_source_record(request, None, compute_context)
    }

    pub(super) fn execute_with_managed_source_record(
        &mut self,
        request: BridgeConditionalExecutionRequest<'_>,
        managed_source_record: Option<
            crate::relational_identity::RelationalBridgeRecordIdentityParts,
        >,
        compute_context: &mut dyn std::any::Any,
    ) -> Result<BridgeConditionalDecisionEvidence, BridgeConditionalDenial> {
        let mut counters = BridgeConditionalExecutionCounters {
            signal_graph_checks: 1,
            ..BridgeConditionalExecutionCounters::default()
        };
        self.require_current_signal_graph(request.lowering)
            .map_err(|denial| denial.with_bridge_execution_counters(counters))?;
        counters.snapshot_admission_attempts =
            usize::from(request.bridge_snapshot_identity.is_some());
        let admitted_snapshot = self
            .open_conditional_snapshot(request.bridge_snapshot_identity)
            .map_err(|denial| denial.with_bridge_execution_counters(counters))?;
        let bridge_snapshot_identity = admitted_snapshot
            .as_ref()
            .map(|snapshot| snapshot.snapshot_identity().clone());
        let execution = self.execute_installed_signal_conditional(
            &request,
            admitted_snapshot.as_ref(),
            managed_source_record,
            compute_context,
            &mut counters,
        );
        let (signal, observations, performed_signal_invalidation) =
            execution.map_err(|denial| denial.with_bridge_execution_counters(counters))?;
        self.retain_successful_observation_baseline(
            request.lowering,
            managed_source_record,
            &observations,
        );
        counters.observation_baseline_writes = observations.len();
        counters.decisions_retained = 1;
        Ok(retain_bridge_decision(
            &request,
            self.bridge.signal_runtime_key,
            RetainedBridgeDecisionOutcome {
                bridge_snapshot_identity,
                signal,
                observations,
                counters,
                performed_signal_invalidation,
            },
        ))
    }

    fn execute_installed_signal_conditional(
        &mut self,
        request: &BridgeConditionalExecutionRequest<'_>,
        admitted_snapshot: Option<
            &crate::snapshot::AdmittedSnapshotContext<
                Box<dyn crate::snapshot::TruthSnapshotReader>,
            >,
        >,
        managed_source_record: Option<
            crate::relational_identity::RelationalBridgeRecordIdentityParts,
        >,
        compute_context: &mut dyn std::any::Any,
        counters: &mut BridgeConditionalExecutionCounters,
    ) -> Result<
        (
            SignalConditionalDecisionEvidence,
            std::sync::Arc<[super::BridgeConditionalSemanticObservation]>,
            Option<SignalInvalidationExecutionReceipt>,
        ),
        BridgeConditionalDenial,
    > {
        let signal_request = signal_execution_request(request);
        counters.compute_provider_checks = 1;
        let compute = request.lowering.providers.compute.as_ref().ok_or_else(|| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::MissingComputeProvider,
                "installed conditional lowering lost its exact compute provider",
            )
        })?;
        let mut condition = ConditionAdapter::new(
            request.lowering,
            admitted_snapshot,
            &self.conditional_observations,
            managed_source_record,
            request.truth_branch_identity,
            request.snapshot_identity,
        );
        let mut comparator = ComparatorAdapter::new(request.lowering);
        counters.signal_execution_contacts = 1;
        let observation = self.graph.begin_invalidation_execution_observation();
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
        let signal = admit_signal_execution(signal, &mut condition)?;
        let performed_signal_invalidation = self
            .graph
            .finish_optional_invalidation_execution_observation(observation)
            .map_err(|error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::SignalExecution,
                    error.to_string(),
                )
            })?;
        let observations = condition.take_observations();
        Ok((signal, observations, performed_signal_invalidation))
    }

    fn require_current_signal_graph(
        &self,
        lowering: &BridgeInstalledConditionalLowering,
    ) -> Result<(), BridgeConditionalDenial> {
        if lowering.signal_contract.graph_instance_id()
            == self.graph.installed_graph_capability().graph_instance_id()
        {
            return Ok(());
        }
        Err(BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::StaleLowering,
            "conditional lowering belongs to another Signal graph",
        ))
    }

    fn open_conditional_snapshot(
        &self,
        identity: Option<&crate::snapshot::TruthSnapshotIdentity>,
    ) -> Result<
        Option<
            crate::snapshot::AdmittedSnapshotContext<Box<dyn crate::snapshot::TruthSnapshotReader>>,
        >,
        BridgeConditionalDenial,
    > {
        identity
            .map(|identity| crate::delivery::open_planned_snapshot(&self.bridge, identity))
            .transpose()
            .map_err(|error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::SnapshotAdmission,
                    format!("conditional snapshot admission failed: {error:?}"),
                )
            })
    }

    fn retain_successful_observation_baseline(
        &mut self,
        lowering: &BridgeInstalledConditionalLowering,
        managed_source_record: Option<
            crate::relational_identity::RelationalBridgeRecordIdentityParts,
        >,
        observations: &[super::BridgeConditionalSemanticObservation],
    ) {
        // Successful semantic reads advance the baseline even when compute is
        // suppressed, so a later domain delta can become observable.
        for observation in observations {
            let key = (
                lowering.signal_node(),
                observation.dependency_ordinal(),
                lowering
                    .semantic_observation_plan
                    .as_ref()
                    .and_then(|plan| {
                        plan.baseline_record(
                            observation.dependency_ordinal(),
                            managed_source_record,
                        )
                    }),
            );
            if let Some(current) = observation.current() {
                self.conditional_observations.insert(key, current.clone());
            } else {
                self.conditional_observations.remove(&key);
            }
        }
    }
}

fn signal_execution_request<'request>(
    request: &'request BridgeConditionalExecutionRequest<'_>,
) -> SignalConditionalExecutionRequest<'request> {
    let mut signal_request = SignalConditionalExecutionRequest::new(
        &request.lowering.signal_contract,
        request.snapshot_identity,
        request.execution_identity,
        request.attempt,
    );
    if request
        .lowering
        .providers
        .trigger
        .as_ref()
        .is_some_and(|provider| provider.requested())
    {
        signal_request = signal_request.force_on_demand();
    }
    signal_request
}

fn admit_signal_execution(
    signal: Result<
        SignalConditionalDecisionEvidence,
        worth_signal::facade::SignalConditionalExecutionFailure,
    >,
    condition: &mut ConditionAdapter<'_>,
) -> Result<SignalConditionalDecisionEvidence, BridgeConditionalDenial> {
    match signal {
        Ok(signal) => Ok(signal),
        Err(failure) => {
            let counters = failure.counters();
            let observation_reads = condition.observation_count();
            if let Some(denial) = condition.take_observation_denial() {
                return Err(denial.with_execution_counters(counters, observation_reads));
            }
            Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SignalExecution,
                format!("{:?}", failure.into_error()),
            )
            .with_execution_counters(counters, observation_reads))
        }
    }
}

struct RetainedBridgeDecisionOutcome {
    bridge_snapshot_identity: Option<crate::snapshot::TruthSnapshotIdentity>,
    signal: SignalConditionalDecisionEvidence,
    observations: std::sync::Arc<[super::BridgeConditionalSemanticObservation]>,
    counters: BridgeConditionalExecutionCounters,
    performed_signal_invalidation: Option<SignalInvalidationExecutionReceipt>,
}

fn retain_bridge_decision(
    request: &BridgeConditionalExecutionRequest<'_>,
    bridge_runtime_key: u64,
    outcome: RetainedBridgeDecisionOutcome,
) -> BridgeConditionalDecisionEvidence {
    let RetainedBridgeDecisionOutcome {
        bridge_snapshot_identity,
        signal,
        observations,
        counters,
        performed_signal_invalidation,
    } = outcome;
    BridgeConditionalDecisionEvidence {
        core: std::sync::Arc::new(BridgeRetainedConditionalDecisionCore {
            bridge_runtime_key,
            lowering: std::sync::Arc::clone(request.lowering),
            bridge_snapshot_identity,
            signal_snapshot_projection: request.snapshot_identity.into(),
            signal_execution_projection: request.execution_identity.into(),
            attempt: request.attempt,
            signal,
            semantic_observations: observations,
            bridge_execution_counters: counters,
            triggering_change_set: None,
        }),
        query_binding_identity: request.query_binding_identity.into(),
        query_capability_identity: request.query_capability_identity,
        reentry_counters: BridgeConditionalReentryCounters::default(),
        performed_signal_invalidation,
    }
}
