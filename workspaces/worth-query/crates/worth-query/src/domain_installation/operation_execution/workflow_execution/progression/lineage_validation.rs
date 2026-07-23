use super::WorthQueryWorkflowEffectEvidence;
use crate::domain_installation::{
    WorthQueryConditionalOutcomeClass, WorthQueryConditionalProvenance,
    WorthQueryOperationLineageContract,
};
use crate::identity_evolution::InstalledIdentityEvolutionOutcome;

pub(super) fn valid_stage_lineage(
    lineage: &[InstalledIdentityEvolutionOutcome],
    contract: WorthQueryOperationLineageContract,
    conditional: &[WorthQueryConditionalProvenance],
    operation_identity: &str,
    run_identity: &str,
    stage_identity: &str,
    effects: &[WorthQueryWorkflowEffectEvidence],
) -> bool {
    let mut identities = std::collections::BTreeSet::new();
    let effect_receipt_identities = effects
        .iter()
        .map(WorthQueryWorkflowEffectEvidence::receipt_identity)
        .collect::<std::collections::BTreeSet<_>>();
    let bindings_are_exact = lineage.iter().all(|outcome| {
        outcome.binds(
            operation_identity,
            run_identity,
            stage_identity,
            &effect_receipt_identities,
        ) && identities.insert(outcome.semantic_identity())
    });
    let matches_contract = match contract {
        WorthQueryOperationLineageContract::NotRequired => lineage.is_empty(),
        WorthQueryOperationLineageContract::Preserve => lineage.iter().all(|outcome| {
            outcome.kind()
                == crate::identity_evolution::InstalledIdentityEvolutionKind::PreservedIdentity
        }),
        WorthQueryOperationLineageContract::Evolve => lineage.iter().all(|outcome| {
            outcome.kind()
                != crate::identity_evolution::InstalledIdentityEvolutionKind::PreservedIdentity
        }),
    };
    let conditional_established_fresh_lineage = lineage.is_empty()
        || conditional.is_empty()
        || conditional
            .iter()
            .any(|item| item.class() == WorthQueryConditionalOutcomeClass::ComputedChanged);
    bindings_are_exact && matches_contract && conditional_established_fresh_lineage
}
