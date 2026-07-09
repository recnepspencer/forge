use std::collections::BTreeMap;

use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
};

pub(super) fn rebuild_debt_reason(family_label: &str) -> &'static str {
    match family_label {
        "milestone_6_layout_materialization" => {
            "policy executed reclaim of a rebuildable Milestone 6 layout materialization family"
        }
        "milestone_6_scope_slice_membership" => {
            "policy executed reclaim of Milestone 6 scope membership backed by a surviving retained basis"
        }
        "milestone_6_chunk_membership" => {
            "policy executed reclaim of Milestone 6 chunk membership backed by a surviving retained basis"
        }
        "milestone_6_structural_block" => {
            "policy executed reclaim of Milestone 6 structural blocks backed by a surviving retained basis"
        }
        _ => "policy executed reclaim of a rebuildable derived family",
    }
}

pub(super) fn milestone_6_family_artifact_count(state: &StoreState, family_label: &str) -> u64 {
    match family_label {
        "milestone_6_layout_materialization" => {
            state.milestone_6_layout_materialization_records.len() as u64
        }
        "milestone_6_scope_slice_membership" => {
            state.milestone_6_scope_slice_membership_records.len() as u64
        }
        "milestone_6_chunk_membership" => state.milestone_6_chunk_membership_records.len() as u64,
        "milestone_6_structural_block" => state.milestone_6_structural_block_records.len() as u64,
        _ => 0,
    }
}

pub(super) fn derived_artifact_exists(
    state: &StoreState,
    family_label: &str,
    artifact_id: &str,
) -> bool {
    match family_label {
        "milestone_6_layout_materialization" => state
            .milestone_6_layout_materialization_records
            .contains_key(artifact_id),
        "milestone_6_scope_slice_membership" => state
            .milestone_6_scope_slice_membership_records
            .contains_key(artifact_id),
        "milestone_6_chunk_membership" => state
            .milestone_6_chunk_membership_records
            .contains_key(artifact_id),
        "milestone_6_structural_block" => state
            .milestone_6_structural_block_records
            .contains_key(artifact_id),
        _ => false,
    }
}

pub(super) fn apply_derived_reclaim(
    state: &mut StoreState,
    reclaim_unit: &crate::DerivedFamilyReclaimUnit,
) -> Result<u64, StoreError> {
    match reclaim_unit.family_label() {
        "milestone_6_layout_materialization" => {
            if state
                .milestone_6_layout_materialization_records
                .remove(reclaim_unit.artifact_id())
                .is_none()
            {
                return Err(StoreError::new(
                    StoreErrorKind::ReclaimEligibilityViolation,
                    format!(
                        "milestone 6 layout materialization `{}` was not present for reclaim",
                        reclaim_unit.artifact_id()
                    ),
                ));
            }
            let scope_deleted = remove_matching_keys(
                &mut state.milestone_6_scope_slice_membership_records,
                |record| record.layout_materialization_artifact_id == reclaim_unit.artifact_id(),
            );
            let chunk_deleted =
                remove_matching_keys(&mut state.milestone_6_chunk_membership_records, |record| {
                    record.layout_materialization_artifact_id == reclaim_unit.artifact_id()
                });
            let structural_deleted =
                remove_matching_keys(&mut state.milestone_6_structural_block_records, |record| {
                    record
                        .supporting_layout_materialization_artifact_ids
                        .iter()
                        .any(|artifact_id| artifact_id == reclaim_unit.artifact_id())
                });
            Ok(1 + scope_deleted + chunk_deleted + structural_deleted)
        }
        "milestone_6_scope_slice_membership" => remove_required_artifact(
            &mut state.milestone_6_scope_slice_membership_records,
            reclaim_unit.artifact_id(),
            "milestone 6 scope slice membership",
        )
        .map(|_| 1),
        "milestone_6_chunk_membership" => remove_required_artifact(
            &mut state.milestone_6_chunk_membership_records,
            reclaim_unit.artifact_id(),
            "milestone 6 chunk membership",
        )
        .map(|_| 1),
        "milestone_6_structural_block" => remove_required_artifact(
            &mut state.milestone_6_structural_block_records,
            reclaim_unit.artifact_id(),
            "milestone 6 structural block",
        )
        .map(|_| 1),
        family_label => Err(StoreError::new(
            StoreErrorKind::ReclaimEligibilityViolation,
            format!("derived reclaim does not support family `{family_label}`"),
        )),
    }
}

fn remove_required_artifact<T>(
    records: &mut BTreeMap<String, T>,
    artifact_id: &str,
    family_label: &str,
) -> Result<T, StoreError> {
    records.remove(artifact_id).ok_or_else(|| {
        StoreError::new(
            StoreErrorKind::ReclaimEligibilityViolation,
            format!("{family_label} `{artifact_id}` was not present for reclaim"),
        )
    })
}

fn remove_matching_keys<T>(
    records: &mut BTreeMap<String, T>,
    predicate: impl Fn(&T) -> bool,
) -> u64 {
    let keys = records
        .iter()
        .filter_map(|(artifact_id, record)| predicate(record).then(|| artifact_id.clone()))
        .collect::<Vec<_>>();
    let count = keys.len() as u64;
    for key in keys {
        records.remove(&key);
    }
    count
}
