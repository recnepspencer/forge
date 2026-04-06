use crate::input::envelope::{
    BridgeCommittedPatchBody, BridgeCommittedPatchDigest, BridgeCommittedPatchItem,
    BridgeCommittedPatchSummary, NormalizedBridgePatchEnvelope, RawCommittedPatchEnvelope,
};

pub(crate) fn normalize_raw_envelope(
    raw: RawCommittedPatchEnvelope,
) -> NormalizedBridgePatchEnvelope {
    let raw_item_count = raw.patch_items().len();
    let mut canonical_items = raw.patch_items().to_vec();
    canonical_items.sort_by(canonical_patch_item_order);
    canonical_items.dedup();
    let normalized_patch_item_count = canonical_items.len();
    let digest = BridgeCommittedPatchDigest::new(digest_basis(
        &raw,
        &canonical_items,
        normalized_patch_item_count,
    ));

    NormalizedBridgePatchEnvelope::new(
        raw.producer_metadata().clone(),
        raw.commit_identity().clone(),
        raw.patch_identity().clone(),
        raw.snapshot_identity().clone(),
        raw.branch_identity().clone(),
        BridgeCommittedPatchSummary::new(
            raw_item_count,
            normalized_patch_item_count,
        ),
        BridgeCommittedPatchBody::new(canonical_items),
        digest,
    )
}

pub(crate) fn canonical_patch_item_order(
    left: &BridgeCommittedPatchItem,
    right: &BridgeCommittedPatchItem,
) -> std::cmp::Ordering {
    left.entity_identity()
        .cmp(right.entity_identity())
        .then_with(|| left.aspect_label().cmp(right.aspect_label()))
        .then_with(|| left.surface_label().cmp(right.surface_label()))
}

fn digest_basis(
    raw: &RawCommittedPatchEnvelope,
    canonical_items: &[BridgeCommittedPatchItem],
    normalized_patch_item_count: usize,
) -> String {
    let mut basis = format!(
        "patch|commit={}|patch={}|snapshot={}|branch={}|normalized-item-count={}",
        raw.commit_identity().as_str(),
        raw.patch_identity().as_str(),
        raw.snapshot_identity().as_str(),
        raw.branch_identity().as_str(),
        normalized_patch_item_count,
    );

    for item in canonical_items {
        basis.push_str("|item=");
        basis.push_str(item.entity_identity());
        basis.push(':');
        basis.push_str(item.aspect_label());
        basis.push(':');
        basis.push_str(item.surface_label());
    }

    basis
}

#[cfg(test)]
mod tests {
    use crate::input::envelope::{
        BridgeCommittedPatchItem, RawCommittedPatchEnvelope, TruthBranchIdentity,
        TruthCommitIdentity, TruthPatchIdentity,
    };
    use crate::snapshot::TruthSnapshotIdentity;

    use super::normalize_raw_envelope;

    #[test]
    fn normalization_sorts_and_deduplicates_patch_items() {
        let normalized = normalize_raw_envelope(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new("commit"),
            TruthPatchIdentity::new("patch"),
            TruthSnapshotIdentity::new("snapshot"),
            TruthBranchIdentity::new("branch"),
            vec![
                BridgeCommittedPatchItem::new("user", "profile", "name"),
                BridgeCommittedPatchItem::new("user", "profile", "avatar"),
                BridgeCommittedPatchItem::new("user", "profile", "name"),
            ],
        ));

        assert_eq!(normalized.patch_summary().patch_item_count(), 3);
        assert_eq!(normalized.patch_summary().normalized_patch_item_count(), 2);
        assert_eq!(
            normalized.patch_body().canonical_items()[0].surface_label(),
            "avatar"
        );
        assert_eq!(
            normalized.patch_body().canonical_items()[1].surface_label(),
            "name"
        );
    }
}
