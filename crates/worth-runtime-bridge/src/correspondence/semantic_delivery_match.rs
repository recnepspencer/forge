use worth_foundational::facade::{AspectBinding, AuthoritativeAspectChangeKind as Change};

use crate::input::envelope::{
    BridgeAspectChangePrecision, BridgeCommittedPatchEnvelope, BridgeCommittedRecordChangeKind,
};

use super::{
    BridgeCorrespondenceDeliveryDenial, BridgeCorrespondenceDenialKind,
    BridgeDeliveredCorrespondenceChange, BridgeSemanticDependencyCandidate,
    CorrespondenceDeliveryCounters, InstalledCorrespondenceTarget,
};

pub(crate) struct BridgeMatchedCorrespondenceChanges {
    pub(crate) counters: CorrespondenceDeliveryCounters,
    pub(crate) changes: Vec<BridgeDeliveredCorrespondenceChange>,
}

pub(crate) fn match_envelope(
    dependency: &BridgeSemanticDependencyCandidate,
    targets: &[InstalledCorrespondenceTarget],
    envelope: &BridgeCommittedPatchEnvelope,
    mut counters: CorrespondenceDeliveryCounters,
) -> Result<BridgeMatchedCorrespondenceChanges, BridgeCorrespondenceDeliveryDenial> {
    let mut changes = Vec::new();
    let source_partition = envelope
        .producer_metadata()
        .authoritative_source()
        .and_then(|source| source.partition_role());
    for item in envelope.patch_body().canonical_items() {
        counters.correspondence_lookups += 1;
        let Some(change) = item.semantic_change() else {
            continue;
        };
        counters.semantic_match_checks += 1;
        if !semantic_change_matches_basis(
            SemanticMatchBasis {
                dependency,
                record: item.relational_record_identity_parts(),
                source_partition,
            },
            change,
            &mut counters,
        ) {
            continue;
        }
        if change.precision() == BridgeAspectChangePrecision::DeclaredWidening {
            let source_widening_admitted = targets.iter().all(|target| {
                counters.source_widening_target_checks += 1;
                target.admitted_source_widening == change.widening_cause()
            });
            if !source_widening_admitted {
                counters.failed_deliveries += 1;
                return Err(BridgeCorrespondenceDeliveryDenial::new(
                    BridgeCorrespondenceDenialKind::MappingSemanticMismatch,
                    counters,
                ));
            }
        }
        counters.truth_targets_admitted += 1;
        changes.push(BridgeDeliveredCorrespondenceChange::semantic_aspect(
            item.entity_identity().into(),
            item.relational_record_identity_parts(),
            change.clone(),
        ));
    }

    if counters.truth_targets_admitted == 0 {
        changes.extend(match_structural_changes(
            dependency,
            envelope,
            source_partition,
            &mut counters,
        ));
    }
    Ok(BridgeMatchedCorrespondenceChanges { counters, changes })
}

struct SemanticMatchBasis<'a> {
    dependency: &'a BridgeSemanticDependencyCandidate,
    record: Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
    source_partition: Option<&'a worth_foundational::facade::TruthPartitionRole>,
}

fn semantic_change_matches_basis(
    basis: SemanticMatchBasis<'_>,
    change: &crate::input::envelope::BridgeSemanticAspectChange,
    counters: &mut CorrespondenceDeliveryCounters,
) -> bool {
    basis.dependency.contract.key() == change.aspect_key()
        && basis.dependency.contract.identity() == change.aspect_identity()
        && basis.dependency.contract.revision() == change.contract_revision()
        && &basis.dependency.binding == change.binding()
        && change_meaning_matches(&basis.dependency.relevant_changes, change.kind(), counters)
        && locality_matches(
            &basis.dependency.locality,
            basis.dependency.source_record_identity,
            basis.record,
            basis.source_partition,
        )
        && (basis.dependency.projection_mask.is_whole_aspect()
            || matches!(
                change.kind(),
                Change::WholeAspectSet | Change::WholeAspectClear
            )
            || change.field_path().is_some_and(|path| {
                basis
                    .dependency
                    .projection_mask
                    .paths()
                    .iter()
                    .any(|candidate| {
                        counters.projection_paths_inspected += 1;
                        candidate == path
                    })
            }))
}

fn match_structural_changes(
    dependency: &BridgeSemanticDependencyCandidate,
    envelope: &BridgeCommittedPatchEnvelope,
    source_partition: Option<&worth_foundational::facade::TruthPartitionRole>,
    counters: &mut CorrespondenceDeliveryCounters,
) -> Vec<BridgeDeliveredCorrespondenceChange> {
    let mut changes = Vec::new();
    for change in envelope.patch_body().canonical_record_changes() {
        counters.correspondence_lookups += 1;
        counters.semantic_match_checks += 1;
        if structural_change_matches(dependency, change, source_partition, counters) {
            counters.truth_targets_admitted += 1;
            changes.push(BridgeDeliveredCorrespondenceChange::structural_record(
                change.clone(),
            ));
        }
    }
    changes
}

fn change_meaning_matches(
    admitted: &[Change],
    authoritative: Change,
    counters: &mut CorrespondenceDeliveryCounters,
) -> bool {
    contains_change(admitted, authoritative, counters)
        || matches!(
            authoritative,
            Change::WholeAspectSet | Change::WholeAspectClear
                if contains_change(admitted, Change::FieldSet, counters)
                    || contains_change(admitted, Change::FieldClear, counters)
        )
}

fn contains_change(
    admitted: &[Change],
    expected: Change,
    counters: &mut CorrespondenceDeliveryCounters,
) -> bool {
    admitted.iter().any(|candidate| {
        counters.relevant_change_checks += 1;
        *candidate == expected
    })
}

fn structural_change_matches(
    dependency: &BridgeSemanticDependencyCandidate,
    change: &crate::input::envelope::BridgeCommittedRecordChange,
    source_partition: Option<&worth_foundational::facade::TruthPartitionRole>,
    counters: &mut CorrespondenceDeliveryCounters,
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
    meaning.is_some_and(|meaning| contains_change(&dependency.relevant_changes, meaning, counters))
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
