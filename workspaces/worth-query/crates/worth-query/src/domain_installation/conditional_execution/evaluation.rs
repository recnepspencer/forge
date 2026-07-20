use crate::basis_lifecycle::BasisOperationLane;

use super::reentry::admit_conditional_decision;
use super::{
    WorthQueryConditionalComputeContext, WorthQueryConditionalProvenance,
    WorthQueryInstalledConditionalNode,
};

pub(crate) enum WorthQueryConditionalEvaluationStop {
    Deferred(Vec<WorthQueryConditionalProvenance>),
    Failed {
        kind: worth_runtime_bridge::facade::BridgeConditionalDenialKind,
        detail: String,
    },
    Reentry(super::WorthQueryConditionalAdmissionDenial),
}

pub(crate) fn evaluate_bound_conditionals<D, O, F, L: BasisOperationLane>(
    bound: &super::super::WorthQueryBoundDomainOperation<D, O, F, L>,
    workspace: &mut crate::runtime::WorthQueryWorkspace,
    snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
    execution_identity: &str,
    stage_identity: Option<&str>,
    workflow_run_identity: Option<&str>,
    attempt: u64,
    counters: &mut super::super::WorthQueryOperationExecutionCounters,
) -> Result<Vec<WorthQueryConditionalProvenance>, WorthQueryConditionalEvaluationStop> {
    let snapshot_identity = snapshot.evidence_identity();
    let mut admitted = Vec::new();
    for node in bound
        .conditional_nodes()
        .iter()
        .filter(|node| location_matches(node, stage_identity))
    {
        let authority = super::reentry::admit_conditional_authority(bound, node)
            .map_err(WorthQueryConditionalEvaluationStop::Reentry)?;
        let mut context = WorthQueryConditionalComputeContext::new(
            node.lowering.location().clone(),
            bound.definition().canonical_identity().to_string(),
            bound.binding_identity().to_string(),
            bound.basis().capability_digest().to_string(),
            workflow_run_identity.map(str::to_string),
            snapshot_identity.as_str().to_string(),
            attempt,
        );
        let bridge = match workspace.execute_installed_conditional(
            &node.lowering,
            bound.binding_identity(),
            bound.capability_identity(),
            snapshot_identity.as_str(),
            snapshot.bridge_identity(),
            execution_identity,
            attempt,
            &mut context,
        ) {
            Ok(bridge) => bridge,
            Err((kind, detail, signal_counters, semantic_observation_reads)) => {
                retain_conditional_counters(counters, signal_counters);
                counters.conditional_semantic_reads += semantic_observation_reads;
                return Err(WorthQueryConditionalEvaluationStop::Failed { kind, detail });
            }
        };
        let signal_counters = bridge.signal().counters();
        counters.conditional_semantic_reads += bridge.semantic_observation_reads();
        retain_conditional_counters(counters, signal_counters);
        let provenance = admit_conditional_decision(
            bound,
            authority,
            bridge,
            snapshot_identity.as_str(),
            snapshot.bridge_identity(),
            execution_identity,
            attempt,
        )
        .map_err(WorthQueryConditionalEvaluationStop::Reentry)?;
        let changed =
            provenance.class() == super::WorthQueryConditionalOutcomeClass::ComputedChanged;
        admitted.push(provenance);
        if !changed {
            return Err(WorthQueryConditionalEvaluationStop::Deferred(admitted));
        }
    }
    Ok(admitted)
}

fn retain_conditional_counters(
    counters: &mut super::super::WorthQueryOperationExecutionCounters,
    signal: worth_signal::facade::SignalConditionalDecisionCounters,
) {
    counters.conditional_dependency_checks += signal.dependency_version_checks;
    counters.conditional_condition_checks += signal.condition_checks;
    counters.conditional_comparator_checks += signal.comparator_checks;
    counters.conditional_compute_contacts += signal.compute_contacts;
    counters.conditional_semantic_changes += signal.semantic_changes;
    counters.conditional_reuse_checks += signal.reuse_checks;
}

fn location_matches(
    node: &WorthQueryInstalledConditionalNode,
    stage_identity: Option<&str>,
) -> bool {
    match (node.lowering.location(), stage_identity) {
        (
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::Operation {
                ..
            },
            None,
        ) => true,
        (
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::WorkflowStage {
                stage_identity: installed,
                ..
            },
            Some(expected),
        ) => installed == expected,
        _ => false,
    }
}
