use crate::basis_lifecycle::BasisOperationLane;

use super::super::WorthQueryBoundDomainOperation;
use super::denial::{
    WorthQueryCompatibilityCounters, WorthQueryCompatibilityDenial,
    WorthQueryCompatibilityDenialKind,
};

pub(super) struct WorthQueryConditionalContinuityEvidence {
    _items: Vec<worth_runtime_bridge::facade::BridgeConditionalLoweringContinuity>,
}

impl WorthQueryConditionalContinuityEvidence {
    pub(super) fn count(&self) -> usize {
        self._items.len()
    }

    pub(super) fn candidate_is_live(&self) -> bool {
        self._items
            .iter()
            .all(|evidence| evidence.candidate_is_live())
    }
}

pub(super) struct WorthQueryConditionalAffinityEvidence {
    _items: Vec<worth_runtime_bridge::facade::BridgeConditionalExecutionAffinity>,
}

impl WorthQueryConditionalAffinityEvidence {
    pub(super) fn count(&self) -> usize {
        self._items.len()
    }

    pub(super) fn both_are_live(&self) -> bool {
        self._items.iter().all(|evidence| evidence.both_are_live())
    }
}

pub(super) fn compare_conditional_continuity<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<WorthQueryConditionalContinuityEvidence, WorthQueryCompatibilityDenial> {
    require_conditional_width(subject, candidate, counters)?;
    let mut items = Vec::with_capacity(subject.conditional_nodes().len());
    for (subject, candidate) in subject
        .conditional_nodes()
        .iter()
        .zip(candidate.conditional_nodes())
    {
        counters.conditional_lowerings_compared += 1;
        compare_query_conditional_meaning(subject, candidate, counters)?;
        match subject
            .lowering
            .compare_semantic_continuity(&candidate.lowering)
        {
            Ok(evidence) => {
                record_owner_work(evidence.work(), counters);
                items.push(evidence);
            }
            Err(denial) => {
                record_owner_work(denial.work(), counters);
                return Err(WorthQueryCompatibilityDenial::conditional_continuity(
                    denial.mismatch().clone(),
                    *counters,
                ));
            }
        }
    }
    Ok(WorthQueryConditionalContinuityEvidence { _items: items })
}

pub(super) fn compare_conditional_affinity<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<WorthQueryConditionalAffinityEvidence, WorthQueryCompatibilityDenial> {
    require_conditional_width(subject, candidate, counters)?;
    let mut items = Vec::with_capacity(subject.conditional_nodes().len());
    for (subject, candidate) in subject
        .conditional_nodes()
        .iter()
        .zip(candidate.conditional_nodes())
    {
        counters.conditional_lowerings_compared += 1;
        compare_query_conditional_meaning(subject, candidate, counters)?;
        match subject
            .lowering
            .compare_execution_affinity(&candidate.lowering)
        {
            Ok(evidence) => {
                record_owner_work(evidence.work(), counters);
                items.push(evidence);
            }
            Err(denial) => {
                record_owner_work(denial.work(), counters);
                return Err(WorthQueryCompatibilityDenial::conditional_affinity(
                    denial.mismatch().clone(),
                    *counters,
                ));
            }
        }
    }
    Ok(WorthQueryConditionalAffinityEvidence { _items: items })
}

fn compare_query_conditional_meaning(
    subject: &crate::domain_installation::WorthQueryInstalledConditionalNode,
    candidate: &crate::domain_installation::WorthQueryInstalledConditionalNode,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    use worth_query_installation::facade::WorthQueryPortableConditionalComparisonOutcome;
    match worth_query_installation::facade::compare_portable_conditional_node_declarations(
        &subject.declaration,
        &candidate.declaration,
    ) {
        WorthQueryPortableConditionalComparisonOutcome::Equivalent(evidence) => {
            counters.conditional_foundational_comparisons += evidence.comparison_count() as usize;
            Ok(())
        }
        WorthQueryPortableConditionalComparisonOutcome::Mismatched(mismatch) => {
            counters.conditional_foundational_comparisons += mismatch.comparison_count() as usize;
            Err(WorthQueryCompatibilityDenial::installed_conditional_mismatch(mismatch, *counters))
        }
        WorthQueryPortableConditionalComparisonOutcome::Unsupported(mismatch) => {
            counters.conditional_foundational_comparisons += mismatch.comparison_count() as usize;
            Err(
                WorthQueryCompatibilityDenial::installed_conditional_unsupported(
                    mismatch, *counters,
                ),
            )
        }
    }
}

fn require_conditional_width<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    if subject.conditional_nodes().len() != candidate.conditional_nodes().len() {
        Err(WorthQueryCompatibilityDenial::plain(
            WorthQueryCompatibilityDenialKind::ConditionalLoweringSet,
            "installed conditional lowering inventories differ",
            *counters,
        ))
    } else {
        Ok(())
    }
}

fn record_owner_work(
    work: worth_runtime_bridge::facade::BridgeConditionalComparisonWork,
    counters: &mut WorthQueryCompatibilityCounters,
) {
    counters.conditional_bridge_contract_comparisons += work.bridge_contract_comparisons() as usize;
    counters.conditional_liveness_checks += work.liveness_checks() as usize;
    counters.conditional_correspondences_inspected += work.correspondences_inspected() as usize;
    counters.conditional_targets_inspected += work.targets_inspected() as usize;
    counters.conditional_provider_roles_inspected += work.provider_roles_inspected() as usize;
    counters.conditional_signal_semantic_dimensions_inspected +=
        work.signal_semantic_dimensions_inspected() as usize;
    counters.conditional_signal_affinity_dimensions_inspected +=
        work.signal_affinity_dimensions_inspected() as usize;
    counters.conditional_bridge_affinity_dimensions_inspected +=
        work.bridge_affinity_dimensions_inspected() as usize;
}
