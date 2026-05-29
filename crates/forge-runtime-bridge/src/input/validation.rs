use crate::error::{
    BridgeErrorContext, BridgePatchCoordinate, BridgeRouteError, BridgeRouteErrorKind,
};
use crate::input::envelope::{
    BridgeCommittedPatchEnvelope, BridgeProducerAuthorityKind, BridgeProducerMetadata,
    NormalizedBridgePatchEnvelope, BRIDGE_PRODUCER_EXPORT_SCHEMA_V1,
};

pub(crate) fn validate_normalized_envelope(
    normalized: NormalizedBridgePatchEnvelope,
) -> Result<BridgeCommittedPatchEnvelope, BridgeRouteError> {
    validate_producer_metadata(normalized.producer_metadata())?;
    validate_identity("commit identity", normalized.commit_identity().as_str())?;
    validate_identity("patch identity", normalized.patch_identity().as_str())?;
    validate_identity("snapshot identity", normalized.snapshot_identity().as_str())?;
    validate_identity("branch identity", normalized.branch_identity().as_str())?;
    validate_identity("committed patch digest", normalized.digest().as_str())?;

    let normalized_patch_item_count = normalized.patch_body().canonical_items().len();
    if normalized_patch_item_count == 0 {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedTruthPatchScope,
            "Committed patch envelope contained no canonical patch items to route.",
        ));
    }

    if normalized.patch_summary().normalized_patch_item_count() != normalized_patch_item_count {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedTruthPatchScope,
            format!(
                "Committed patch envelope normalized count `{}` did not match canonical body size `{normalized_patch_item_count}`.",
                normalized.patch_summary().normalized_patch_item_count()
            ),
        ));
    }

    if normalized.patch_summary().patch_item_count()
        < normalized.patch_summary().normalized_patch_item_count()
    {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedTruthPatchScope,
            format!(
                "Committed patch envelope raw patch item count `{}` was smaller than normalized count `{}`.",
                normalized.patch_summary().patch_item_count(),
                normalized.patch_summary().normalized_patch_item_count()
            ),
        ));
    }

    for (index, item) in normalized.patch_body().canonical_items().iter().enumerate() {
        validate_identity(
            &format!("patch item #{index} entity identity"),
            item.entity_identity(),
        )
        .map_err(|error| {
            error.with_context(BridgeErrorContext::patch(BridgePatchCoordinate::new(
                item.entity_identity(),
                item.aspect_label(),
                item.surface_label(),
            )))
        })?;
        validate_identity(
            &format!("patch item #{index} surface label"),
            item.surface_label(),
        )
        .map_err(|error| {
            error.with_context(BridgeErrorContext::patch(BridgePatchCoordinate::new(
                item.entity_identity(),
                item.aspect_label(),
                item.surface_label(),
            )))
        })?;
    }

    Ok(BridgeCommittedPatchEnvelope::from_normalized(normalized))
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
    if metadata.export_schema_version() != BRIDGE_PRODUCER_EXPORT_SCHEMA_V1 {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedProducerEnvelope,
            format!(
                "Committed patch producer schema `{}` is not supported; expected `{}`.",
                metadata.export_schema_version(),
                BRIDGE_PRODUCER_EXPORT_SCHEMA_V1
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
    if let Some(feedback_provenance_digest) = metadata.writeback_feedback_provenance_digest() {
        validate_identity(
            "writeback feedback provenance digest",
            feedback_provenance_digest,
        )?;
    }
    if let Some(feedback_causality_digest) = metadata.writeback_feedback_causality_digest() {
        validate_identity(
            "writeback feedback causality digest",
            feedback_causality_digest,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::AspectKey;

    use crate::error::BridgeRouteErrorKind;
    use crate::input::envelope::{
        BridgeCommittedPatchBody, BridgeCommittedPatchDigest, BridgeCommittedPatchItem,
        BridgeCommittedPatchSummary, BridgeProducerAuthorityKind, BridgeProducerMetadata,
        NormalizedBridgePatchEnvelope, TruthBranchIdentity, TruthCommitIdentity,
        TruthPatchIdentity,
    };
    use crate::snapshot::TruthSnapshotIdentity;

    use super::validate_normalized_envelope;

    #[test]
    fn validation_rejects_empty_identity_bearing_fields() {
        let parts = NormalizedBridgePatchEnvelope::new(
            BridgeProducerMetadata::bridge_harness_fixture(),
            TruthCommitIdentity::new("  "),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
            BridgeCommittedPatchSummary::new(1, 1),
            BridgeCommittedPatchBody::new(vec![BridgeCommittedPatchItem::new(
                "entity-1",
                aspect_key("profile"),
                "name",
            )]),
            BridgeCommittedPatchDigest::new("digest-a"),
        );

        let error = validate_normalized_envelope(parts)
            .expect_err("empty canonical identities must be rejected");

        assert_eq!(
            error.kind(),
            BridgeRouteErrorKind::UnsupportedTruthPatchScope
        );
        assert!(error.to_string().contains("commit identity"));
    }

    #[test]
    fn validation_rejects_empty_patch_item_surface_labels() {
        let parts = NormalizedBridgePatchEnvelope::new(
            BridgeProducerMetadata::bridge_harness_fixture(),
            TruthCommitIdentity::new("commit-a"),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
            BridgeCommittedPatchSummary::new(1, 1),
            BridgeCommittedPatchBody::new(vec![BridgeCommittedPatchItem::new(
                "entity-1",
                aspect_key("profile"),
                "",
            )]),
            BridgeCommittedPatchDigest::new("digest-a"),
        );

        let error = validate_normalized_envelope(parts)
            .expect_err("empty canonical patch item surface labels must be rejected");

        assert_eq!(
            error.kind(),
            BridgeRouteErrorKind::UnsupportedTruthPatchScope
        );
        assert!(error.to_string().contains("patch item #0 surface label"));
    }

    #[test]
    fn validation_rejects_unsupported_producer_schema() {
        let parts = NormalizedBridgePatchEnvelope::new(
            BridgeProducerMetadata::new(
                BridgeProducerAuthorityKind::BridgeHarnessFixture,
                "forge-runtime-bridge.producer-envelope.v999",
            ),
            TruthCommitIdentity::new("commit-a"),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
            BridgeCommittedPatchSummary::new(1, 1),
            BridgeCommittedPatchBody::new(vec![BridgeCommittedPatchItem::new(
                "entity-1",
                aspect_key("profile"),
                "name",
            )]),
            BridgeCommittedPatchDigest::new("digest-a"),
        );

        let error = validate_normalized_envelope(parts)
            .expect_err("unsupported producer schemas must fail at ingress");

        assert_eq!(
            error.kind(),
            BridgeRouteErrorKind::UnsupportedProducerEnvelope
        );
    }

    #[test]
    fn validation_rejects_empty_writeback_feedback_provenance_fields() {
        let parts = NormalizedBridgePatchEnvelope::new(
            BridgeProducerMetadata::bridge_harness_fixture()
                .with_writeback_feedback_provenance(" ", "bridge-writeback-causality:sha256:a"),
            TruthCommitIdentity::new("commit-a"),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
            BridgeCommittedPatchSummary::new(1, 1),
            BridgeCommittedPatchBody::new(vec![BridgeCommittedPatchItem::new(
                "entity-1",
                aspect_key("profile"),
                "name",
            )]),
            BridgeCommittedPatchDigest::new("digest-a"),
        );

        let error = validate_normalized_envelope(parts)
            .expect_err("empty writeback feedback provenance fields must fail ingress validation");

        assert_eq!(
            error.kind(),
            BridgeRouteErrorKind::UnsupportedTruthPatchScope
        );
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("valid bridge patch aspect key")
    }
}
