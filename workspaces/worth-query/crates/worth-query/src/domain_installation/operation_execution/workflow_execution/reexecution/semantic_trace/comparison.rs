use super::{
    WorthQueryLineageTraceMeaning, WorthQueryReplayComparison, WorthQueryReplayDivergence,
    WorthQueryReplayNoiseContract, WorthQueryWorkflowStageTraceSemantics,
    WorthQueryWorkflowTraceSemantics,
};

pub fn compare_exact_workflow_traces(
    original: &WorthQueryWorkflowTraceSemantics,
    candidate: &WorthQueryWorkflowTraceSemantics,
    noise: WorthQueryReplayNoiseContract,
) -> WorthQueryReplayComparison {
    compare_exact_workflow_traces_counted(original, candidate, noise).0
}

pub(crate) fn compare_exact_workflow_traces_counted(
    original: &WorthQueryWorkflowTraceSemantics,
    candidate: &WorthQueryWorkflowTraceSemantics,
    noise: WorthQueryReplayNoiseContract,
) -> (WorthQueryReplayComparison, usize) {
    use WorthQueryReplayDivergence as D;
    if original.operation_identity != candidate.operation_identity {
        return (WorthQueryReplayComparison::Diverged(D::Operation), 0);
    }
    if original.conditional_path != candidate.conditional_path {
        return (
            WorthQueryReplayComparison::Diverged(D::OperationConditionalPath),
            0,
        );
    }
    if original.stages.len() != candidate.stages.len()
        || original
            .stages
            .iter()
            .zip(&candidate.stages)
            .any(|(left, right)| left.stage_identity != right.stage_identity)
    {
        return (WorthQueryReplayComparison::Diverged(D::StageSet), 0);
    }
    for (index, (left, right)) in original.stages.iter().zip(&candidate.stages).enumerate() {
        if let Some(divergence) = stage_divergence(left, right, noise) {
            return (WorthQueryReplayComparison::Diverged(divergence), index + 1);
        }
    }
    if original.publication != candidate.publication {
        return (
            WorthQueryReplayComparison::Diverged(D::Publication),
            original.stages.len(),
        );
    }
    (
        WorthQueryReplayComparison::Equivalent,
        original.stages.len(),
    )
}

fn stage_divergence(
    original: &WorthQueryWorkflowStageTraceSemantics,
    candidate: &WorthQueryWorkflowStageTraceSemantics,
    noise: WorthQueryReplayNoiseContract,
) -> Option<WorthQueryReplayDivergence> {
    use WorthQueryReplayDivergence as D;
    let stage = original.stage_identity.clone();
    if original.predecessor_stage_identities != candidate.predecessor_stage_identities {
        Some(D::PredecessorTopology { stage })
    } else if original.input != candidate.input {
        Some(D::Input { stage })
    } else if original.output != candidate.output {
        Some(D::Output { stage })
    } else if original.result_state != candidate.result_state {
        Some(D::ResultState { stage })
    } else if !noise.diagnostic_warnings && original.warnings != candidate.warnings {
        Some(D::Diagnostic { stage })
    } else if original.effects != candidate.effects {
        Some(D::Effect { stage })
    } else if original.invariants != candidate.invariants {
        Some(D::Invariant { stage })
    } else if original.conditional_path != candidate.conditional_path {
        Some(D::ConditionalPath { stage })
    } else if !lineage_semantics_eq(&original.lineage, &candidate.lineage) {
        Some(D::Lineage { stage })
    } else {
        None
    }
}

fn lineage_semantics_eq(
    original: &[WorthQueryLineageTraceMeaning],
    candidate: &[WorthQueryLineageTraceMeaning],
) -> bool {
    original.len() == candidate.len()
        && original.iter().zip(candidate).all(|(left, right)| {
            left.effect_indices == right.effect_indices
                && left.outcome.semantic_replay_eq(&right.outcome)
        })
}
