use std::sync::Arc;

use worth_query_installation::facade::{
    compare_portable_conditional_node_declarations, WorthQueryPortableConditionalComparisonOutcome,
};

use super::super::provider_admission::BridgeConditionalProviderAdmission;
use super::super::provider_semantics::{
    BridgeConditionalProviderSemanticContracts, BridgeErasedProviderSemanticContract,
};
use super::super::BridgeConditionalProviderSet;
use super::{
    BridgeConditionalComparisonWork, BridgeConditionalContinuityMismatch,
    BridgeConditionalExecutionAffinityMismatch, BridgeConditionalProviderRole,
};

pub(super) fn compare_provider_admissions(
    current: &BridgeConditionalProviderAdmission,
    candidate: &BridgeConditionalProviderAdmission,
    work: &mut BridgeConditionalComparisonWork,
) -> Result<(), BridgeConditionalContinuityMismatch> {
    let same_contract = match compare_portable_conditional_node_declarations(
        current.declaration(),
        candidate.declaration(),
    ) {
        WorthQueryPortableConditionalComparisonOutcome::Equivalent(evidence) => {
            work.record_portable(evidence.comparison_count());
            true
        }
        WorthQueryPortableConditionalComparisonOutcome::Mismatched(mismatch) => {
            work.record_portable(mismatch.comparison_count());
            false
        }
        WorthQueryPortableConditionalComparisonOutcome::Unsupported(unsupported) => {
            work.record_portable(unsupported.comparison_count());
            false
        }
    };
    if !same_contract || current.required_roles() != candidate.required_roles() {
        return Err(BridgeConditionalContinuityMismatch::ProviderAdmission);
    }
    compare_semantic_contracts(
        current.semantic_contracts(),
        candidate.semantic_contracts(),
        work,
    )?;
    Ok(())
}

fn compare_semantic_contracts(
    current: &BridgeConditionalProviderSemanticContracts,
    candidate: &BridgeConditionalProviderSemanticContracts,
    work: &mut BridgeConditionalComparisonWork,
) -> Result<(), BridgeConditionalContinuityMismatch> {
    semantic(
        &current.condition,
        &candidate.condition,
        BridgeConditionalProviderRole::Condition,
        work,
    )?;
    semantic(
        &current.dependency_comparator,
        &candidate.dependency_comparator,
        BridgeConditionalProviderRole::DependencyComparator,
        work,
    )?;
    semantic(
        &current.output_comparator,
        &candidate.output_comparator,
        BridgeConditionalProviderRole::OutputComparator,
        work,
    )?;
    semantic(
        &current.reuse_comparator,
        &candidate.reuse_comparator,
        BridgeConditionalProviderRole::ArtifactReuseComparator,
        work,
    )?;
    semantic(
        &current.trigger,
        &candidate.trigger,
        BridgeConditionalProviderRole::Trigger,
        work,
    )?;
    semantic(
        &current.wake,
        &candidate.wake,
        BridgeConditionalProviderRole::Wake,
        work,
    )?;
    semantic(
        &current.compute,
        &candidate.compute,
        BridgeConditionalProviderRole::Compute,
        work,
    )
}

fn semantic(
    current: &Option<BridgeErasedProviderSemanticContract>,
    candidate: &Option<BridgeErasedProviderSemanticContract>,
    role: BridgeConditionalProviderRole,
    work: &mut BridgeConditionalComparisonWork,
) -> Result<(), BridgeConditionalContinuityMismatch> {
    work.inspect_provider_role();
    match (current, candidate) {
        (Some(current), Some(candidate)) if current.is_equivalent_to(candidate) => Ok(()),
        (None, None) => Ok(()),
        _ => Err(BridgeConditionalContinuityMismatch::ProviderSemanticContract { role }),
    }
}

pub(super) fn compare_provider_affinity(
    current: &BridgeConditionalProviderSet,
    candidate: &BridgeConditionalProviderSet,
    work: &mut BridgeConditionalComparisonWork,
) -> Result<(), BridgeConditionalExecutionAffinityMismatch> {
    exact(
        &current.condition,
        &candidate.condition,
        BridgeConditionalProviderRole::Condition,
        work,
    )?;
    exact(
        &current.dependency_comparator,
        &candidate.dependency_comparator,
        BridgeConditionalProviderRole::DependencyComparator,
        work,
    )?;
    exact(
        &current.output_comparator,
        &candidate.output_comparator,
        BridgeConditionalProviderRole::OutputComparator,
        work,
    )?;
    exact(
        &current.reuse_comparator,
        &candidate.reuse_comparator,
        BridgeConditionalProviderRole::ArtifactReuseComparator,
        work,
    )?;
    exact(
        &current.trigger,
        &candidate.trigger,
        BridgeConditionalProviderRole::Trigger,
        work,
    )?;
    exact(
        &current.wake,
        &candidate.wake,
        BridgeConditionalProviderRole::Wake,
        work,
    )?;
    exact(
        &current.compute,
        &candidate.compute,
        BridgeConditionalProviderRole::Compute,
        work,
    )
}

fn exact<T: ?Sized>(
    current: &Option<Arc<T>>,
    candidate: &Option<Arc<T>>,
    role: BridgeConditionalProviderRole,
    work: &mut BridgeConditionalComparisonWork,
) -> Result<(), BridgeConditionalExecutionAffinityMismatch> {
    work.inspect_provider_role();
    match (current, candidate) {
        (Some(current), Some(candidate)) if Arc::ptr_eq(current, candidate) => Ok(()),
        (None, None) => Ok(()),
        _ => Err(BridgeConditionalExecutionAffinityMismatch::ProviderIdentity { role }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conditional_execution::BridgeConditionalTriggerProvider;

    struct Trigger;

    impl BridgeConditionalTriggerProvider for Trigger {
        fn requested(&self) -> bool {
            true
        }
    }

    impl crate::conditional_execution::BridgeConditionalProviderSemantics for Trigger {
        type SemanticContract = ();

        fn semantic_contract(&self) -> Self::SemanticContract {}
    }

    #[test]
    fn independent_provider_instances_deny_exact_affinity() {
        let current = BridgeConditionalProviderSet::new().trigger(Trigger);
        let candidate = BridgeConditionalProviderSet::new().trigger(Trigger);
        let mut work = BridgeConditionalComparisonWork::default();

        assert!(matches!(
            compare_provider_affinity(&current, &candidate, &mut work),
            Err(
                BridgeConditionalExecutionAffinityMismatch::ProviderIdentity {
                    role: BridgeConditionalProviderRole::Trigger
                }
            )
        ));
        assert_eq!(work.provider_roles_inspected(), 5);
    }

    #[test]
    fn cloning_a_provider_set_preserves_exact_provider_affinity() {
        let current = BridgeConditionalProviderSet::new().trigger(Trigger);
        let candidate = current.clone();
        let mut work = BridgeConditionalComparisonWork::default();

        compare_provider_affinity(&current, &candidate, &mut work).unwrap();
        assert_eq!(work.provider_roles_inspected(), 7);
    }
}
