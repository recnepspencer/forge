use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryExistingTruthAssertionEvidence,
    ForgeQueryExistingTruthBindingEvidence, ForgeQueryMutationEvidenceDigest,
    ForgeQueryMutationTargetEvidence, ForgeQueryNamingMutationEvidence,
    ForgeQuerySymbolicAspectResolutionEvidence, ForgeQuerySymbolicTargetReferenceEvidence,
};

pub(super) fn batch_target_digest(
    components: &[ForgeQueryMutationTargetEvidence],
) -> ForgeQueryMutationEvidenceDigest {
    let identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(ForgeQueryEvidenceTag::new("role"), "batch-target")
            .field_evidence_identity_sequence(
                ForgeQueryEvidenceTag::new("component"),
                components
                    .iter()
                    .map(target_component_identity)
                    .collect::<Vec<_>>()
                    .iter(),
            )
            .seal();
    ForgeQueryMutationEvidenceDigest::aggregate("batch-target", identity)
}

pub(super) fn batch_existing_truth_binding_digest(
    bindings: &[Option<ForgeQueryExistingTruthBindingEvidence>],
) -> Option<ForgeQueryMutationEvidenceDigest> {
    let bindings = bindings
        .iter()
        .flatten()
        .map(|binding| binding.binding_digest().evidence_identity().clone())
        .collect::<Vec<_>>();
    if bindings.is_empty() {
        return None;
    }
    Some(aggregate_sequence_identity(
        "batch-existing-truth-binding",
        bindings,
    ))
}

pub(super) fn batch_existing_truth_assertion_digest(
    assertions: &[Option<ForgeQueryExistingTruthAssertionEvidence>],
) -> Option<ForgeQueryMutationEvidenceDigest> {
    let assertions = assertions
        .iter()
        .flatten()
        .map(existing_truth_assertion_identity)
        .collect::<Vec<_>>();
    if assertions.is_empty() {
        return None;
    }
    Some(aggregate_sequence_identity(
        "batch-existing-truth-assertion",
        assertions,
    ))
}

pub(super) fn batch_symbolic_target_reference_digest(
    references: &[Option<ForgeQuerySymbolicTargetReferenceEvidence>],
) -> Option<ForgeQueryMutationEvidenceDigest> {
    let references = references
        .iter()
        .flatten()
        .map(|reference| symbolic_target_reference_identity("symbolic-target-reference", reference))
        .collect::<Vec<_>>();
    if references.is_empty() {
        return None;
    }
    Some(aggregate_sequence_identity(
        "batch-symbolic-target-reference",
        references,
    ))
}

pub(super) fn batch_symbolic_resolution_digest(
    target_references: &[Option<ForgeQuerySymbolicTargetReferenceEvidence>],
    aspect_resolutions: &[Vec<ForgeQuerySymbolicAspectResolutionEvidence>],
) -> Option<ForgeQueryMutationEvidenceDigest> {
    let rows = target_references
        .iter()
        .flatten()
        .map(|reference| {
            symbolic_target_reference_identity("symbolic-resolution-target", reference)
        })
        .chain(
            aspect_resolutions
                .iter()
                .flatten()
                .map(symbolic_aspect_resolution_identity),
        )
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return None;
    }
    Some(aggregate_sequence_identity(
        "batch-symbolic-resolution",
        rows,
    ))
}

pub(super) fn batch_continuity_mutation_digest(
    continuities: &[Option<ForgeQueryContinuityMutationEvidence>],
) -> Option<ForgeQueryMutationEvidenceDigest> {
    let continuities = continuities
        .iter()
        .flatten()
        .map(continuity_mutation_identity)
        .collect::<Vec<_>>();
    if continuities.is_empty() {
        return None;
    }
    Some(aggregate_sequence_identity(
        "batch-continuity-mutation",
        continuities,
    ))
}

pub(super) fn batch_naming_mutation_digest(
    namings: &[Option<ForgeQueryNamingMutationEvidence>],
) -> Option<ForgeQueryMutationEvidenceDigest> {
    let namings = namings
        .iter()
        .flatten()
        .map(naming_mutation_identity)
        .collect::<Vec<_>>();
    if namings.is_empty() {
        return None;
    }
    Some(aggregate_sequence_identity(
        "batch-naming-mutation",
        namings,
    ))
}

fn existing_truth_assertion_identity(
    assertion: &ForgeQueryExistingTruthAssertionEvidence,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "existing-truth-assertion",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("mode"),
            assertion.mode().as_str(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("asserted_aspect_count"),
            assertion.asserted_aspect_count(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("verification"),
            assertion.verification_evidence_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("assumption_snapshot"),
            assertion.assumption_snapshot_evidence_digest(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("verified_precondition"),
            assertion.verified_precondition_evidence_digest(),
        )
        .seal()
}

fn symbolic_target_reference_identity(
    role: &'static str,
    reference: &ForgeQuerySymbolicTargetReferenceEvidence,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(ForgeQueryEvidenceTag::new("role"), role)
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                reference.family().as_str(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("symbol"),
                &reference.symbol().evidence_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("resolved_entity"),
                &reference.resolved_entity_identity().evidence_identity(),
            );
    if let Some(collection) = reference.target_collection() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_collection"),
            collection.evidence_identity(),
        );
    }
    identity.seal()
}

fn symbolic_aspect_resolution_identity(
    resolution: &ForgeQuerySymbolicAspectResolutionEvidence,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(
                ForgeQueryEvidenceTag::new("role"),
                "symbolic-resolution-aspect",
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                resolution.family().as_str(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("admitted_aspect_touch"),
                resolution.aspect_touch().admitted_touch_digest_part(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("symbol"),
                &resolution.symbol().evidence_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("resolved_entity"),
                &resolution.resolved_entity_identity().evidence_identity(),
            );
    if let Some(collection) = resolution.target_collection() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_collection"),
            collection.evidence_identity(),
        );
    }
    identity.seal()
}

fn continuity_mutation_identity(
    continuity: &ForgeQueryContinuityMutationEvidence,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(ForgeQueryEvidenceTag::new("role"), "continuity-mutation")
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                continuity.family().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("outcome"),
                continuity.outcome_class().as_str(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("prior_authority"),
                &continuity
                    .prior_authoritative_identity()
                    .evidence_identity(),
            )
            .field_evidence_identity_sequence(
                ForgeQueryEvidenceTag::new("successor_authority"),
                continuity
                    .successor_authoritative_identities()
                    .iter()
                    .map(|identity| identity.evidence_identity()),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("lineage"),
                &continuity.lineage_digest().evidence_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("continuity_resolution"),
                &continuity
                    .continuity_resolution_digest()
                    .evidence_identity(),
            );
    if let Some(digest) = continuity.basis_binding_digest() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_binding"),
            digest.evidence_identity(),
        );
    }
    if let Some(resolved) = continuity.resolved_target_entity_identity() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("resolved_entity"),
            &resolved.evidence_identity(),
        );
    }
    if let Some(collection) = continuity.target_collection() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_collection"),
            collection.evidence_identity(),
        );
    }
    identity.seal()
}

fn naming_mutation_identity(
    naming: &ForgeQueryNamingMutationEvidence,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(ForgeQueryEvidenceTag::new("role"), "naming-mutation")
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                naming.family().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("outcome"),
                naming.outcome().as_str(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("attachment"),
                &naming.attachment_identity().evidence_identity(),
            );
    if let Some(prior) = naming.prior_authoritative_identity() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("prior_authority"),
            prior.evidence_identity(),
        );
    }
    if let Some(target) = naming.target_authoritative_identity() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_authority"),
            target.evidence_identity(),
        );
    }
    if let Some(resolved) = naming.resolved_target_entity_identity() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("resolved_entity"),
            &resolved.evidence_identity(),
        );
    }
    if let Some(collection) = naming.target_collection() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_collection"),
            collection.evidence_identity(),
        );
    }
    identity.seal()
}

fn target_component_identity(
    component: &ForgeQueryMutationTargetEvidence,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(ForgeQueryEvidenceTag::new("role"), "target-component")
            .field_shape(
                ForgeQueryEvidenceTag::new("declared_class"),
                component.declared().target_class().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("resolved_class"),
                component.resolved().target_class().as_str(),
            );
    if let Some(collection) = component.declared().collection() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("declared_collection"),
            collection.evidence_identity(),
        );
    }
    if let Some(entity) = component.declared().entity_identity() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("declared_entity"),
            &entity.evidence_identity(),
        );
    }
    if let Some(collection) = component.resolved().collection() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("resolved_collection"),
            collection.evidence_identity(),
        );
    }
    if let Some(entity) = component.resolved().entity_identity() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("resolved_entity"),
            &entity.evidence_identity(),
        );
    }
    identity.seal()
}

fn aggregate_sequence_identity(
    role: &'static str,
    entries: Vec<ForgeQueryEvidenceIdentity>,
) -> ForgeQueryMutationEvidenceDigest {
    let identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(ForgeQueryEvidenceTag::new("role"), role)
            .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("entry"), entries.iter())
            .seal();
    ForgeQueryMutationEvidenceDigest::aggregate(role, identity)
}
