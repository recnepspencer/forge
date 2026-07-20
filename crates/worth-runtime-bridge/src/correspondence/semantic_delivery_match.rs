use worth_foundational::facade::{AspectBinding, AuthoritativeAspectChangeKind as Change};

use crate::input::envelope::{
    BridgeAspectChangePrecision, BridgeCommittedPatchEnvelope, BridgeCommittedRecordChangeKind,
};

use super::{
    BridgeCorrespondenceDeliveryDenial, BridgeCorrespondenceDenialKind,
    BridgeSemanticDependencyCandidate, CorrespondenceDeliveryCounters,
    InstalledCorrespondenceTarget,
};

pub(crate) fn match_envelope(
    dependency: &BridgeSemanticDependencyCandidate,
    targets: &[InstalledCorrespondenceTarget],
    envelope: &BridgeCommittedPatchEnvelope,
    mut counters: CorrespondenceDeliveryCounters,
) -> Result<CorrespondenceDeliveryCounters, BridgeCorrespondenceDeliveryDenial> {
    let source_partition = envelope
        .producer_metadata()
        .authoritative_source()
        .and_then(|source| source.partition_role());
    for item in envelope.patch_body().canonical_items() {
        counters.correspondence_lookups += 1;
        let Some(change) = item.semantic_change() else {
            continue;
        };
        if !semantic_change_matches_basis(
            dependency,
            item.relational_record_identity_parts(),
            change,
            source_partition,
        ) {
            continue;
        }
        let source_widening_admitted = targets
            .iter()
            .all(|target| target.admitted_source_widening == change.widening_cause());
        if change.precision() == BridgeAspectChangePrecision::DeclaredWidening
            && !source_widening_admitted
        {
            counters.failed_deliveries += 1;
            return Err(BridgeCorrespondenceDeliveryDenial::new(
                BridgeCorrespondenceDenialKind::MappingSemanticMismatch,
                counters,
            ));
        }
        if change.precision() == BridgeAspectChangePrecision::Exact || source_widening_admitted {
            counters.truth_targets_admitted += 1;
        }
    }

    if counters.truth_targets_admitted == 0 {
        for change in envelope.patch_body().canonical_record_changes() {
            counters.correspondence_lookups += 1;
            if structural_change_matches(dependency, change, source_partition) {
                counters.truth_targets_admitted += 1;
            }
        }
    }
    Ok(counters)
}

fn semantic_change_matches_basis(
    dependency: &BridgeSemanticDependencyCandidate,
    record: Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
    change: &crate::input::envelope::BridgeSemanticAspectChange,
    source_partition: Option<&worth_foundational::facade::TruthPartitionRole>,
) -> bool {
    dependency.contract.key() == change.aspect_key()
        && dependency.contract.identity() == change.aspect_identity()
        && dependency.contract.revision() == change.contract_revision()
        && &dependency.binding == change.binding()
        && change_meaning_matches(&dependency.relevant_changes, change.kind())
        && locality_matches(
            &dependency.locality,
            dependency.source_record_identity,
            record,
            source_partition,
        )
        && (dependency.projection_mask.is_whole_aspect()
            || matches!(
                change.kind(),
                Change::WholeAspectSet | Change::WholeAspectClear
            )
            || change
                .field_path()
                .is_some_and(|path| dependency.projection_mask.paths().contains(path)))
}

fn change_meaning_matches(admitted: &[Change], authoritative: Change) -> bool {
    admitted.contains(&authoritative)
        || matches!(
            authoritative,
            Change::WholeAspectSet | Change::WholeAspectClear
                if admitted.contains(&Change::FieldSet)
                    || admitted.contains(&Change::FieldClear)
        )
}

fn structural_change_matches(
    dependency: &BridgeSemanticDependencyCandidate,
    change: &crate::input::envelope::BridgeCommittedRecordChange,
    source_partition: Option<&worth_foundational::facade::TruthPartitionRole>,
) -> bool {
    let meaning = match (&dependency.binding, change.kind()) {
        (
            AspectBinding::StructuralRegion
            | AspectBinding::StructuralPartition
            | AspectBinding::StructuralFacet,
            BridgeCommittedRecordChangeKind::Created,
        ) => Some(Change::StructuralCreate),
        (
            AspectBinding::StructuralRegion
            | AspectBinding::StructuralPartition
            | AspectBinding::StructuralFacet,
            BridgeCommittedRecordChangeKind::Updated,
        ) => Some(Change::StructuralUpdate),
        (
            AspectBinding::StructuralRegion
            | AspectBinding::StructuralPartition
            | AspectBinding::StructuralFacet,
            BridgeCommittedRecordChangeKind::Deleted,
        ) => Some(Change::StructuralDelete),
        (
            AspectBinding::StructuralRegion
            | AspectBinding::StructuralPartition
            | AspectBinding::StructuralFacet,
            BridgeCommittedRecordChangeKind::RetainedForAudit,
        ) => Some(Change::StructuralRetainForAudit),
        (AspectBinding::LifecycleTransition, BridgeCommittedRecordChangeKind::Created) => {
            Some(Change::LifecycleCreate)
        }
        (AspectBinding::LifecycleTransition, BridgeCommittedRecordChangeKind::Deleted) => {
            Some(Change::LifecycleDelete)
        }
        (AspectBinding::LifecycleTransition, BridgeCommittedRecordChangeKind::RetainedForAudit) => {
            Some(Change::LifecycleRetainForAudit)
        }
        _ => None,
    };
    meaning.is_some_and(|meaning| dependency.relevant_changes.contains(&meaning))
        && locality_matches(
            &dependency.locality,
            dependency.source_record_identity,
            Some(change.record_identity()),
            source_partition,
        )
}

fn locality_matches(
    locality: &super::BridgeSemanticLocality,
    source_record_identity: Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
    record: Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
    source_partition: Option<&worth_foundational::facade::TruthPartitionRole>,
) -> bool {
    match locality {
        super::BridgeSemanticLocality::WholeLogicalGraph => true,
        super::BridgeSemanticLocality::SourceRecord => matches!(
            (source_record_identity, record),
            (Some(expected), Some(actual)) if expected == actual
        ),
        super::BridgeSemanticLocality::SourcePartition(role) => source_partition == Some(role),
    }
}
