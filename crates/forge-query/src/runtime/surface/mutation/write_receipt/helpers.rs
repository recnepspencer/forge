use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity};
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
    resolved_entity_identity: Option<&ForgeQueryEntityIdentity>,
) -> Option<ForgeQuerySymbolicTargetReferenceEvidence> {
    if mutation_family == ForgeQueryMutationFamily::Insert {
        return None;
    }
    bridge_reference
        .map(|bundle| {
            let target_collection = authored_reference.and_then(|reference| {
                reference
                    .target_collection()
                    .or_else(|| bundle.target_collection())
            });
            ForgeQuerySymbolicTargetReferenceEvidence::from_bridge_with_query_context(
                bundle,
                resolved_entity_identity,
                target_collection,
            )
        })
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
    resolved_target_entity_identity: Option<&ForgeQueryEntityIdentity>,
    target_collection: Option<&str>,
) -> Option<ForgeQueryNamingMutationEvidence> {
    bridge_naming
        .map(|bundle| {
            ForgeQueryNamingMutationEvidence::from_bridge_with_query_context(
                bundle,
                resolved_target_entity_identity,
                target_collection,
            )
        })
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
    resolved_target_entity_identity: Option<&ForgeQueryEntityIdentity>,
    target_collection: Option<&str>,
) -> Option<ForgeQueryContinuityMutationEvidence> {
    let basis_binding_identity =
        existing_truth_binding.map(|binding| binding.binding_evidence_identity());
    bridge_continuity
        .map(|bundle| {
            ForgeQueryContinuityMutationEvidence::from_bridge_with_query_context(
                bundle,
                basis_binding_identity,
                resolved_target_entity_identity,
                target_collection,
            )
        })
        .or_else(|| {
            authored_intent.map(|intent| {
                ForgeQueryContinuityMutationEvidence::from_intent(
                    intent,
                    basis_binding_identity,
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
    declared_aspect_value_digest: Option<&ForgeQueryEvidenceIdentity>,
    snapshot_identity: &ForgeQuerySnapshotIdentity,
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
            retained_assertion_verification_digest(snapshot_identity, declared_aspect_value_digest),
        ),
    )
}

fn retained_assertion_verification_digest(
    snapshot_identity: &ForgeQuerySnapshotIdentity,
    declared_aspect_value_digest: Option<&ForgeQueryEvidenceIdentity>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RetainedExistingTruthAssertionEvidence)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_identity"),
            &snapshot_identity.evidence_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("declared_aspect_value_digest"),
            declared_aspect_value_digest,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("mode"),
            ForgeQueryExistingTruthAssertionMode::RetainedAuthoritativeAssertion.as_str(),
        )
        .seal()
}

pub(super) fn target_evidence_from_receipt(
    mutation_family: ForgeQueryMutationFamily,
    declared_collection: Option<String>,
    declared_entity_identity: Option<ForgeQueryEntityIdentity>,
    target_collection: Option<String>,
    target_entity_identity: Option<ForgeQueryEntityIdentity>,
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
