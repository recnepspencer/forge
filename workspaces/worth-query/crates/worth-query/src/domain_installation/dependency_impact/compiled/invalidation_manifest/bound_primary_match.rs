pub(super) fn bound_delivery_matches(
    consumer: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    changes: &[worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange],
) -> bool {
    consumer.contract() == delivered.contract()
        && super::masks_overlap(consumer.projection_mask(), delivered.projection_mask())
        && consumer.binding() == delivered.binding()
        && delivery_changes_match(consumer, delivered, changes)
}

fn delivery_changes_match(
    consumer: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    changes: &[worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange],
) -> bool {
    let mut saw_typed_change = false;
    for delivered_change in changes {
        if delivered_change
            .effective_change_kind_for(delivered)
            .is_none()
        {
            continue;
        }
        saw_typed_change = true;
        if bound_change_matches(consumer, delivered, delivered_change) {
            return true;
        }
    }
    !saw_typed_change
        && consumer
            .relevant_changes()
            .iter()
            .any(|kind| delivered.relevant_changes().contains(kind))
        && bound_fallback_locality_matches(consumer, delivered, changes)
}

pub(super) fn bound_change_matches(
    consumer: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    delivered_change: &worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange,
) -> bool {
    let record = delivered_change.relational_record_identity();
    if let Some(change) = delivered_change.semantic_change() {
        return bound_semantic_change_matches(consumer, delivered, change, record);
    }
    delivered_change
        .effective_change_kind_for(delivered)
        .is_some_and(|kind| bound_structural_change_matches(consumer, delivered, kind, record))
}

fn bound_structural_change_matches(
    consumer: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    kind: worth_foundational::facade::AuthoritativeAspectChangeKind,
    record: Option<worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts>,
) -> bool {
    consumer.relevant_changes().contains(&kind)
        && delivered.relevant_changes().contains(&kind)
        && bound_change_locality_matches(consumer, delivered, record)
}

fn bound_semantic_change_matches(
    consumer: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    change: &worth_runtime_bridge::facade::BridgeSemanticAspectChange,
    record: Option<worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts>,
) -> bool {
    bridge_candidate_accepts_change(consumer, change)
        && bridge_candidate_accepts_change(delivered, change)
        && bound_change_locality_matches(consumer, delivered, record)
}

fn bridge_candidate_accepts_change(
    candidate: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    change: &worth_runtime_bridge::facade::BridgeSemanticAspectChange,
) -> bool {
    change.aspect_key() == candidate.contract().key()
        && change.aspect_identity() == candidate.contract().identity()
        && change.contract_revision() == candidate.contract().revision()
        && change.binding() == candidate.binding()
        && candidate
            .relevant_changes()
            .iter()
            .copied()
            .any(|kind| change.intersects_relevant_change(kind))
        && change
            .effective_field_path()
            .is_none_or(|path| super::mask_matches_path(candidate.projection_mask(), path))
}

fn bound_change_locality_matches(
    consumer: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    record: Option<worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts>,
) -> bool {
    use worth_runtime_bridge::facade::BridgeSemanticLocality as Locality;
    match consumer.locality() {
        Locality::SourceRecord => consumer.source_record_identity() == record,
        Locality::ManagedSourceRecord => record.is_some(),
        Locality::SourcePartition(left) => matches!(
            delivered.locality(),
            Locality::SourcePartition(right) if left == right
        ),
        Locality::WholeLogicalGraph => true,
    }
}

fn bound_fallback_locality_matches(
    consumer: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    changes: &[worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange],
) -> bool {
    changes.iter().any(|change| {
        bound_change_locality_matches(consumer, delivered, change.relational_record_identity())
    }) || (changes.is_empty()
        && bound_change_locality_matches(consumer, delivered, delivered.source_record_identity()))
}

#[cfg(test)]
#[path = "bound_primary_match/tests.rs"]
mod tests;
