use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryExistingTruthAssertionEvidence,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryMutationTargetEvidence,
    ForgeQueryNamingMutationEvidence, ForgeQuerySymbolicTargetReferenceEvidence,
};

pub(super) fn batch_target_digest(components: &[ForgeQueryMutationTargetEvidence]) -> String {
    crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-target-evidence-v1".to_string())
            .chain(components.iter().map(|component| {
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    component.declared().target_class(),
                    component.declared().collection().unwrap_or(""),
                    component.declared().entity_identity().unwrap_or(""),
                    component.resolved().target_class(),
                    component.resolved().collection().unwrap_or(""),
                    component.resolved().entity_identity().unwrap_or("")
                )
            }))
            .collect::<Vec<_>>(),
    )
}

pub(super) fn batch_existing_truth_binding_digest(
    bindings: &[Option<ForgeQueryExistingTruthBindingEvidence>],
) -> Option<String> {
    let bindings = bindings
        .iter()
        .flatten()
        .map(|binding| binding.binding_digest().to_string())
        .collect::<Vec<_>>();
    if bindings.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-existing-truth-binding-v1".to_string())
            .chain(bindings)
            .collect::<Vec<_>>(),
    ))
}

pub(super) fn batch_existing_truth_assertion_digest(
    assertions: &[Option<ForgeQueryExistingTruthAssertionEvidence>],
) -> Option<String> {
    let assertions = assertions
        .iter()
        .flatten()
        .map(|assertion: &ForgeQueryExistingTruthAssertionEvidence| {
            format!(
                "{}:{}:{}",
                assertion.mode(),
                assertion.asserted_aspect_count(),
                assertion.verification_digest()
            )
        })
        .collect::<Vec<_>>();
    if assertions.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-existing-truth-assertion-v1".to_string())
            .chain(assertions)
            .collect::<Vec<_>>(),
    ))
}

pub(super) fn batch_symbolic_target_reference_digest(
    references: &[Option<ForgeQuerySymbolicTargetReferenceEvidence>],
) -> Option<String> {
    let references = references
        .iter()
        .flatten()
        .map(|reference| {
            format!(
                "{}:{}:{}:{}",
                reference.family(),
                reference.symbol(),
                reference.resolved_entity_identity(),
                reference.target_collection().unwrap_or("none")
            )
        })
        .collect::<Vec<_>>();
    if references.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-symbolic-target-reference-v1".to_string())
            .chain(references)
            .collect::<Vec<_>>(),
    ))
}

pub(super) fn batch_continuity_mutation_digest(
    continuities: &[Option<ForgeQueryContinuityMutationEvidence>],
) -> Option<String> {
    let continuities = continuities
        .iter()
        .flatten()
        .map(|continuity| {
            format!(
                "{:?}:{:?}:{}:{}:{}:{}:{}:{}:{}",
                continuity.family(),
                continuity.outcome_class(),
                continuity.prior_authoritative_identity(),
                if continuity.successor_authoritative_identities().is_empty() {
                    "none".to_string()
                } else {
                    continuity.successor_authoritative_identities().join("|")
                },
                continuity.basis_binding_digest().unwrap_or("none"),
                continuity
                    .resolved_target_entity_identity()
                    .unwrap_or("none"),
                continuity.target_collection().unwrap_or("none"),
                continuity.lineage_digest(),
                continuity.continuity_resolution_digest()
            )
        })
        .collect::<Vec<_>>();
    if continuities.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-continuity-mutation-v1".to_string())
            .chain(continuities)
            .collect::<Vec<_>>(),
    ))
}

pub(super) fn batch_naming_mutation_digest(
    namings: &[Option<ForgeQueryNamingMutationEvidence>],
) -> Option<String> {
    let namings = namings
        .iter()
        .flatten()
        .map(|naming| {
            format!(
                "{:?}:{:?}:{}:{}:{}:{}:{}",
                naming.family(),
                naming.outcome(),
                naming.attachment_identity(),
                naming.prior_authoritative_identity().unwrap_or("none"),
                naming.target_authoritative_identity().unwrap_or("none"),
                naming.resolved_target_entity_identity().unwrap_or("none"),
                naming.target_collection().unwrap_or("none")
            )
        })
        .collect::<Vec<_>>();
    if namings.is_empty() {
        return None;
    }
    Some(crate::identity::hash_parts(
        &std::iter::once("forge-query-batch-naming-mutation-v1".to_string())
            .chain(namings)
            .collect::<Vec<_>>(),
    ))
}
