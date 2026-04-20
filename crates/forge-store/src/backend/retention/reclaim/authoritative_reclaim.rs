use std::collections::BTreeSet;

use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        integrity::{
            commit_artifact_id, commit_support_summary_artifact_id, digest_artifact_key,
            lineage_support_artifact_id, schema_support_artifact_id,
        },
        records::{AuthoritativeArtifactFamily, StoreState},
    },
    failure::{StoreError, StoreErrorKind},
    retention::{
        AuthoritativeReclaimReport, RetainedReadCostSurface, RetainedReadPath,
        RetentionClosureSummary,
    },
};

use super::maintenance_verification::maintenance_verification;

pub(crate) fn execute_authoritative_reclaim<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    range: crate::PolicyExpiredAuthorityRange,
) -> Result<AuthoritativeReclaimReport, StoreError> {
    let reclaim_unit = crate::AuthoritativeRangeReclaimUnit::new(
        range.branch_id().clone(),
        range.expired_commit_ids().to_vec(),
    );
    let mut next = backend.state().clone();
    let deleted_artifact_count = apply_authoritative_reclaim(&mut next, &reclaim_unit)?;
    let verification = maintenance_verification(&next, "execute_authoritative_reclaim", None)
        .inspect_err(|_| backend.counters().record_retention_restore_parity_failure())?;
    backend.commit_replacement_state(next)?;
    backend
        .counters()
        .record_reclaimed_authoritative_artifacts(deleted_artifact_count);

    Ok(AuthoritativeReclaimReport::new(
        reclaim_unit,
        deleted_artifact_count,
        RetainedReadCostSurface::new(
            RetainedReadPath::CanonicalRetainedAuthority,
            RetentionClosureSummary::new(0, 0, 0, 0),
            0,
            0,
            deleted_artifact_count,
            0,
            0,
        ),
        verification,
    ))
}

fn apply_authoritative_reclaim(
    state: &mut StoreState,
    reclaim_unit: &crate::AuthoritativeRangeReclaimUnit,
) -> Result<u64, StoreError> {
    let expired_commit_set = reclaim_unit
        .expired_commit_ids()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if expired_commit_set.is_empty() {
        return Ok(0);
    }

    for commit_id in reclaim_unit.expired_commit_ids() {
        let record = state.commit_record(*commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::PolicyExpiredRangeIllegal,
                format!(
                    "authoritative reclaim referenced missing expired commit {}",
                    commit_id.0
                ),
            )
        })?;
        if record.envelope.branch_context != *reclaim_unit.branch_id() {
            return Err(StoreError::new(
                StoreErrorKind::PolicyExpiredRangeIllegal,
                format!(
                    "authoritative reclaim for branch `{}` referenced commit {} owned by branch `{}`",
                    reclaim_unit.branch_id().0,
                    commit_id.0,
                    record.envelope.branch_context.0
                ),
            ));
        }
    }

    if state
        .branch_head_records
        .values()
        .filter_map(|record| record.head_commit_id)
        .any(|commit_id| expired_commit_set.contains(&commit_id))
    {
        return Err(StoreError::new(
            StoreErrorKind::PolicyExpiredRangeIllegal,
            "authoritative reclaim cannot delete a currently retained branch head commit",
        ));
    }
    if state
        .snapshot_basis_records
        .values()
        .any(|record| expired_commit_set.contains(&record.snapshot_frontier_commit_id))
    {
        return Err(StoreError::new(
            StoreErrorKind::ReclaimLiveBasisConflict,
            "authoritative reclaim cannot delete a snapshot frontier still serving as live basis",
        ));
    }
    if state
        .durable_cursor_identity_records
        .values()
        .any(|record| expired_commit_set.contains(&record.latest_basis_commit_id))
        || state
            .subscriber_checkpoint_records
            .values()
            .any(|record| expired_commit_set.contains(&record.basis_commit_id))
        || state
            .stable_basis_records
            .values()
            .any(|record| expired_commit_set.contains(&record.request.frontier_commit_id()))
    {
        return Err(StoreError::new(
            StoreErrorKind::ReclaimLiveBasisConflict,
            "authoritative reclaim cannot delete a commit still referenced by a live basis surface",
        ));
    }
    if state.commit_envelopes.values().any(|record| {
        !expired_commit_set.contains(&record.envelope.commit.commit_id)
            && record
                .envelope
                .commit
                .parents
                .iter()
                .any(|parent_id| expired_commit_set.contains(parent_id))
    }) {
        return Err(StoreError::new(
            StoreErrorKind::PolicyExpiredRangeIllegal,
            "authoritative reclaim cannot delete commits that still parent surviving authoritative commits",
        ));
    }
    if state.branch_delta_layer_records.values().any(|layer| {
        let contains_expired = layer
            .commit_ids
            .iter()
            .any(|commit_id| expired_commit_set.contains(commit_id));
        contains_expired
            && layer
                .commit_ids
                .iter()
                .any(|commit_id| !expired_commit_set.contains(commit_id))
    }) {
        return Err(StoreError::new(
            StoreErrorKind::PolicyExpiredRangeIllegal,
            "authoritative reclaim cannot partially delete a persisted branch-delta layer",
        ));
    }

    let mut deleted_count = 0_u64;
    for commit_id in reclaim_unit.expired_commit_ids() {
        state.commit_envelopes.remove(&commit_id.0);
        state.commit_support_summaries.remove(&commit_id.0);
        deleted_count += 2;

        let parent_keys = state
            .commit_parent_records
            .iter()
            .filter_map(|(artifact_id, record)| {
                ((record.commit_id == *commit_id)
                    || expired_commit_set.contains(&record.parent_commit_id))
                .then_some(artifact_id.clone())
            })
            .collect::<Vec<_>>();
        for key in parent_keys {
            state.commit_parent_records.remove(&key);
            deleted_count += 1;
            state
                .authoritative_artifact_digests
                .remove(&digest_artifact_key(
                    &AuthoritativeArtifactFamily::CommitParentRecord,
                    &key,
                    state.canonicalization_version,
                ));
        }

        if state
            .schema_support_records
            .remove(&schema_support_artifact_id(*commit_id))
            .is_some()
        {
            deleted_count += 1;
        }
        if state
            .lineage_support_records
            .remove(&lineage_support_artifact_id(*commit_id))
            .is_some()
        {
            deleted_count += 1;
        }

        state
            .authoritative_artifact_digests
            .remove(&digest_artifact_key(
                &AuthoritativeArtifactFamily::CommitEnvelope,
                &commit_artifact_id(*commit_id),
                state.canonicalization_version,
            ));
        state
            .authoritative_artifact_digests
            .remove(&digest_artifact_key(
                &AuthoritativeArtifactFamily::CommitSupportSummary,
                &commit_support_summary_artifact_id(*commit_id),
                state.canonicalization_version,
            ));
        state
            .authoritative_artifact_digests
            .remove(&digest_artifact_key(
                &AuthoritativeArtifactFamily::SchemaSupportRecord,
                &schema_support_artifact_id(*commit_id),
                state.canonicalization_version,
            ));
        state
            .authoritative_artifact_digests
            .remove(&digest_artifact_key(
                &AuthoritativeArtifactFamily::LineageSupportRecord,
                &lineage_support_artifact_id(*commit_id),
                state.canonicalization_version,
            ));
    }

    let removable_layers = state
        .branch_delta_layer_records
        .iter()
        .filter_map(|(layer_id, layer)| {
            layer
                .commit_ids
                .iter()
                .all(|commit_id| expired_commit_set.contains(commit_id))
                .then_some(*layer_id)
        })
        .collect::<Vec<_>>();
    for layer_id in removable_layers {
        state.branch_delta_layer_records.remove(&layer_id);
        deleted_count += 1;
    }

    Ok(deleted_count)
}
