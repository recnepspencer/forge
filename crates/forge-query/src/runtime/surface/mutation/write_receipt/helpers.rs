use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationIntent,
    ForgeQueryExistingTruthAssertionEvidence, ForgeQueryExistingTruthAssertionMode,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationFamily, ForgeQueryMutationTargetClass,
    ForgeQueryMutationTargetDescriptor, ForgeQueryMutationTargetEvidence,
    ForgeQueryNamingMutationEvidence, ForgeQueryNamingMutationIntent,
    ForgeQuerySymbolicTargetReference, ForgeQuerySymbolicTargetReferenceEvidence,
    ForgeQueryVerifiedExistingTruthAssertion,
};

pub(super) fn symbolic_target_reference_evidence(
    mutation_family: ForgeQueryMutationFamily,
    bridge_reference: Option<&forge_runtime_bridge::facade::BridgeSymbolicTargetReferenceBundle>,
    authored_reference: Option<&ForgeQuerySymbolicTargetReference>,
    resolved_entity_identity: Option<&str>,
) -> Option<ForgeQuerySymbolicTargetReferenceEvidence> {
    if mutation_family == ForgeQueryMutationFamily::Insert {
        return None;
    }
    bridge_reference
        .map(ForgeQuerySymbolicTargetReferenceEvidence::from_bridge)
        .or_else(|| {
            authored_reference.zip(resolved_entity_identity).map(
                |(reference, resolved_entity_identity)| {
                    ForgeQuerySymbolicTargetReferenceEvidence::from_reference(
                        reference,
                        resolved_entity_identity,
                    )
                },
            )
        })
}

pub(super) fn naming_mutation_evidence(
    bridge_naming: Option<&forge_runtime_bridge::facade::BridgeNamingMutationBundle>,
    authored_intent: Option<&ForgeQueryNamingMutationIntent>,
    resolved_target_entity_identity: Option<&str>,
    target_collection: Option<&str>,
) -> Option<ForgeQueryNamingMutationEvidence> {
    bridge_naming
        .map(ForgeQueryNamingMutationEvidence::from_bridge)
        .or_else(|| {
            authored_intent.map(|intent| {
                ForgeQueryNamingMutationEvidence::from_intent(
                    intent,
                    resolved_target_entity_identity,
                    target_collection,
                )
            })
        })
}

pub(super) fn continuity_mutation_evidence(
    bridge_continuity: Option<&forge_runtime_bridge::facade::BridgeContinuityMutationBundle>,
    authored_intent: Option<&ForgeQueryContinuityMutationIntent>,
    existing_truth_binding: Option<&crate::runtime::ForgeQueryExistingTruthTargetBinding>,
    resolved_target_entity_identity: Option<&str>,
    target_collection: Option<&str>,
) -> Option<ForgeQueryContinuityMutationEvidence> {
    let basis_binding_digest = existing_truth_binding.map(|binding| binding.binding_digest());
    bridge_continuity
        .map(ForgeQueryContinuityMutationEvidence::from_bridge)
        .or_else(|| {
            authored_intent.map(|intent| {
                ForgeQueryContinuityMutationEvidence::from_intent(
                    intent,
                    basis_binding_digest.as_deref(),
                    resolved_target_entity_identity,
                    target_collection,
                )
            })
        })
}

pub(super) fn assertion_evidence(
    mutation_family: ForgeQueryMutationFamily,
    existing_truth_binding: Option<&ForgeQueryExistingTruthTargetBinding>,
    existing_truth_assertion: Option<&ForgeQueryVerifiedExistingTruthAssertion>,
    declared_aspect_operations: &[crate::runtime::ForgeQueryAspectMutationOperation],
    declared_aspect_value_digest: Option<&str>,
    snapshot_token: &str,
) -> Option<ForgeQueryExistingTruthAssertionEvidence> {
    let _binding = existing_truth_binding?;
    if let Some(verification) = existing_truth_assertion {
        return Some(ForgeQueryExistingTruthAssertionEvidence::backend_verified(
            verification,
        ));
    }
    if mutation_family != ForgeQueryMutationFamily::Assertion {
        return None;
    }
    Some(
        ForgeQueryExistingTruthAssertionEvidence::retained_assertion(
            declared_aspect_operations.len(),
            crate::identity::hash_parts(&[
                "forge_query_retained_existing_truth_assertion_v1".to_string(),
                format!("snapshot:{snapshot_token}"),
                format!(
                    "aspect-digest:{}",
                    declared_aspect_value_digest.unwrap_or("none")
                ),
                format!(
                    "mode:{}",
                    ForgeQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion
                ),
            ]),
        ),
    )
}

pub(super) fn target_evidence_from_receipt(
    mutation_family: ForgeQueryMutationFamily,
    declared_collection: Option<String>,
    declared_entity_identity: Option<String>,
    target_collection: Option<String>,
    target_entity_identity: Option<String>,
) -> ForgeQueryMutationTargetEvidence {
    let declared_target_class = match mutation_family {
        ForgeQueryMutationFamily::Insert => ForgeQueryMutationTargetClass::Collection,
        ForgeQueryMutationFamily::Update
        | ForgeQueryMutationFamily::Assertion
        | ForgeQueryMutationFamily::Delete => ForgeQueryMutationTargetClass::Entity,
    };
    let resolved_target_class = if target_entity_identity.is_some() {
        ForgeQueryMutationTargetClass::Entity
    } else {
        ForgeQueryMutationTargetClass::Collection
    };
    ForgeQueryMutationTargetEvidence::new(
        ForgeQueryMutationTargetDescriptor::new(
            declared_target_class,
            declared_collection,
            declared_entity_identity,
        ),
        ForgeQueryMutationTargetDescriptor::new(
            resolved_target_class,
            target_collection,
            target_entity_identity,
        ),
    )
}
