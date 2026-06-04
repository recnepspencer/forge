use crate::error::{
    BridgeErrorContext, BridgePatchTargetCoordinate, BridgeRouteError, BridgeRouteErrorKind,
};
use crate::mapping::TruthDeltaSurfaceKind;

use super::{
    BridgeCommittedPatchBody, BridgeCommittedPatchDigest, BridgeCommittedPatchEnvelope,
    BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem, BridgeCommittedPatchSummary,
    BridgeEnvelopeCore, BridgePatchEnvelopeHeader, BridgeProducerAuthorityKind,
    BridgeProducerMetadata, CanonicalPatchEnvelopeBody, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;
use sha2::{Digest, Sha256};

pub(super) fn construct_committed_patch_envelope(
    envelope_identity: BridgeCommittedPatchEnvelopeIdentity,
    patch_items: Vec<BridgeCommittedPatchItem>,
) -> Result<BridgeCommittedPatchEnvelope, BridgeRouteError> {
    validate_producer_metadata(envelope_identity.producer_metadata())?;
    validate_identity(
        "commit identity",
        envelope_identity.commit_identity().as_str(),
    )?;
    validate_identity(
        "patch identity",
        envelope_identity.patch_identity().as_str(),
    )?;
    validate_identity(
        "snapshot identity",
        envelope_identity.snapshot_identity().as_str(),
    )?;
    validate_identity(
        "branch identity",
        envelope_identity.branch_identity().as_str(),
    )?;

    let patch_item_count = patch_items.len();
    let mut canonical_items = patch_items;
    canonical_items.sort_by(canonical_patch_item_order);
    canonical_items.dedup();

    if canonical_items.is_empty() {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedTruthPatchScope,
            "Committed patch envelope contained no canonical patch items to route.",
        ));
    }

    for (index, item) in canonical_items.iter().enumerate() {
        validate_patch_item(index, item)?;
    }

    let normalized_patch_item_count = canonical_items.len();
    let patch_summary =
        BridgeCommittedPatchSummary::new(patch_item_count, normalized_patch_item_count);
    let committed_patch_digest_basis = digest_basis(
        envelope_identity.commit_identity(),
        envelope_identity.patch_identity(),
        envelope_identity.snapshot_identity(),
        envelope_identity.branch_identity(),
        &canonical_items,
        normalized_patch_item_count,
    );
    let digest = BridgeCommittedPatchDigest::new(committed_patch_digest_from_basis(
        &committed_patch_digest_basis,
    ));
    validate_identity("committed patch digest", digest.as_str())?;

    Ok(BridgeCommittedPatchEnvelope::from_core(
        BridgeEnvelopeCore::new(
            BridgePatchEnvelopeHeader::new(
                envelope_identity.producer_metadata().clone(),
                envelope_identity.commit_identity().clone(),
                envelope_identity.patch_identity().clone(),
                envelope_identity.snapshot_identity().clone(),
                envelope_identity.branch_identity().clone(),
            ),
            CanonicalPatchEnvelopeBody {
                patch_summary,
                patch_body: BridgeCommittedPatchBody::new(canonical_items),
                digest,
            },
        ),
    ))
}

pub(super) fn canonical_patch_item_order(
    left: &BridgeCommittedPatchItem,
    right: &BridgeCommittedPatchItem,
) -> std::cmp::Ordering {
    left.entity_identity()
        .cmp(right.entity_identity())
        .then_with(|| left.aspect_key().cmp(right.aspect_key()))
        .then_with(|| left.surface_kind().cmp(&right.surface_kind()))
        .then_with(|| {
            left.target_canonical_basis()
                .cmp(&right.target_canonical_basis())
        })
}

fn validate_patch_item(
    index: usize,
    item: &BridgeCommittedPatchItem,
) -> Result<(), BridgeRouteError> {
    validate_identity(
        &format!("patch item #{index} entity identity"),
        item.entity_identity(),
    )
    .map_err(|error| error.with_context(patch_context(item)))?;
    validate_identity(
        &format!("patch item #{index} target canonical basis"),
        &item.target_canonical_basis(),
    )
    .map_err(|error| error.with_context(patch_context(item)))?;

    match (item.surface_kind(), item.field_locator()) {
        (TruthDeltaSurfaceKind::EntityField, Some(_)) => {
            validate_authoritative_locator(item)?;
            validate_field_masks(item)
        }
        (TruthDeltaSurfaceKind::EntityField, None) => Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedTruthDeltaSurface,
            format!(
                "Committed patch item #{index} entity-field target lacked a foundational field locator."
            ),
        )
        .with_context(patch_context(item))),
        (_, Some(_)) => Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedTruthDeltaSurface,
            format!(
                "Committed patch item #{index} non-field target unexpectedly carried a foundational field locator."
            ),
        )
        .with_context(patch_context(item))),
        (_, None) => {
            validate_authoritative_locator(item)?;
            validate_whole_aspect_masks(item)
        }
    }
}

fn validate_authoritative_locator(item: &BridgeCommittedPatchItem) -> Result<(), BridgeRouteError> {
    if item.aspect_locator().authority()
        == forge_foundational::facade::LocatorAuthority::Authoritative
    {
        return Ok(());
    }

    Err(BridgeRouteError::new(
        BridgeRouteErrorKind::UnsupportedTruthDeltaSurface,
        "Committed patch targets must carry authoritative foundational aspect locators.",
    )
    .with_context(patch_context(item)))
}

fn validate_field_masks(item: &BridgeCommittedPatchItem) -> Result<(), BridgeRouteError> {
    let field_path = item
        .field_locator()
        .expect("field targets validated as carrying field locators")
        .field_path();
    let expected_path = std::slice::from_ref(field_path);
    let mutation_matches = item.mutation_mask().paths() == expected_path;
    let projection_matches = item.projection_mask().paths() == expected_path;
    if mutation_matches && projection_matches {
        return Ok(());
    }

    Err(BridgeRouteError::new(
        BridgeRouteErrorKind::UnsupportedTruthDeltaSurface,
        "Committed patch field targets must carry mutation and projection masks for the same foundational field path.",
    )
    .with_context(patch_context(item)))
}

fn validate_whole_aspect_masks(item: &BridgeCommittedPatchItem) -> Result<(), BridgeRouteError> {
    if item.mutation_mask().is_whole_aspect() && item.projection_mask().is_whole_aspect() {
        return Ok(());
    }

    Err(BridgeRouteError::new(
        BridgeRouteErrorKind::UnsupportedTruthDeltaSurface,
        "Committed patch non-field targets must carry whole-aspect mutation and projection masks.",
    )
    .with_context(patch_context(item)))
}

fn patch_context(item: &BridgeCommittedPatchItem) -> BridgeErrorContext {
    BridgeErrorContext::patch(BridgePatchTargetCoordinate::new(
        item.entity_identity(),
        item.target().clone(),
    ))
}

fn validate_identity(label: &str, value: &str) -> Result<(), BridgeRouteError> {
    if value.trim().is_empty() {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedTruthPatchScope,
            format!("Committed patch envelope {label} must be non-empty."),
        ));
    }

    Ok(())
}

fn validate_producer_metadata(metadata: &BridgeProducerMetadata) -> Result<(), BridgeRouteError> {
    validate_identity(
        "producer export schema version",
        metadata.export_schema_version(),
    )?;
    if metadata.export_schema_version() != super::BRIDGE_PRODUCER_EXPORT_SCHEMA_V1 {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedProducerEnvelope,
            format!(
                "Committed patch producer schema `{}` is not supported; expected `{}`.",
                metadata.export_schema_version(),
                super::BRIDGE_PRODUCER_EXPORT_SCHEMA_V1
            ),
        ));
    }

    match metadata.authority_kind() {
        BridgeProducerAuthorityKind::RelationalPublication
        | BridgeProducerAuthorityKind::BridgeHarnessFixture => {}
        BridgeProducerAuthorityKind::Unknown => {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::UnsupportedProducerEnvelope,
                "Committed patch producer authority `unknown` is not supported.",
            ));
        }
    }

    if let Some(semantics_version) = metadata.producer_semantics_version() {
        validate_identity("producer semantics version", semantics_version)?;
    }
    if let Some(feedback_context) = metadata.writeback_feedback_context() {
        validate_identity(
            "writeback feedback context digest",
            feedback_context.digest(),
        )?;
        validate_identity(
            "writeback feedback provenance digest",
            feedback_context.provenance_digest(),
        )?;
        validate_identity(
            "writeback feedback causality digest",
            feedback_context.causality_digest(),
        )?;
        validate_identity(
            "writeback feedback effect intent digest",
            feedback_context.effect_intent_digest(),
        )?;
    }

    Ok(())
}

fn digest_basis(
    commit_identity: &TruthCommitIdentity,
    patch_identity: &TruthPatchIdentity,
    snapshot_identity: &TruthSnapshotIdentity,
    branch_identity: &TruthBranchIdentity,
    canonical_items: &[BridgeCommittedPatchItem],
    normalized_patch_item_count: usize,
) -> String {
    let mut basis = format!(
        "patch|commit={}|patch={}|snapshot={}|branch={}|normalized-item-count={}",
        commit_identity.as_str(),
        patch_identity.as_str(),
        snapshot_identity.as_str(),
        branch_identity.as_str(),
        normalized_patch_item_count,
    );

    for item in canonical_items {
        basis.push_str("|item=");
        basis.push_str(item.entity_identity());
        basis.push(':');
        basis.push_str(&item.target_canonical_basis());
    }

    basis
}

fn committed_patch_digest_from_basis(canonical_basis: &str) -> String {
    let digest = Sha256::digest(canonical_basis.as_bytes());
    format!("patch:sha256:{digest:x}")
}

#[cfg(test)]
#[path = "construction_tests.rs"]
mod construction_tests;
