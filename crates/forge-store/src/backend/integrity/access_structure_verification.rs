use crate::{
    evidence::{
        Milestone6AccessStructureVerification, Milestone6AccessStructureVerificationPath,
        Milestone7AccessStructureVerification, Milestone7AccessStructureVerificationPath,
    },
    media::DurableMediaReport,
};

use crate::backend::records::StoreState;

use super::identity::{durable_cursor_identity_artifact_id, subscriber_checkpoint_artifact_id};

pub(crate) fn verify_milestone_6_access_structures(
    state: &StoreState,
    media_report: DurableMediaReport,
) -> Milestone6AccessStructureVerification {
    Milestone6AccessStructureVerification {
        backend_family: media_report.backend_family(),
        aspect_layout_read: verify_milestone_6_scope_membership_keys(
            state,
            "aspect layout read remains proof-only until published Milestone 6 scope-to-slice membership records exist",
            "loaded Milestone 6 scope-to-slice membership records preserve canonical scope addressing for admitted aspect layout reads",
        ),
        structural_block_reuse: verify_milestone_6_structural_block_keys(
            state,
            "structural block reuse remains proof-only until published Milestone 6 structural-block records exist",
            "loaded Milestone 6 structural-block records preserve exact structural block identity and slice membership for reuse witnesses",
        ),
        chunk_model_freeze: verify_milestone_6_chunk_membership_keys(
            state,
            "chunk model freeze remains proof-only until published Milestone 6 chunk-membership records exist",
            "loaded Milestone 6 chunk-membership records preserve physical chunk addressing for frozen chunk witnesses",
        ),
        milestone_7_layout_reference: Milestone6AccessStructureVerificationPath::verified(
            "Milestone 7 layout references are compile-time isolated from slice, block, and placement internals",
        ),
        milestone_9_physical_chunk_reference: Milestone6AccessStructureVerificationPath::verified(
            "Milestone 9 physical chunk references are compile-time isolated from authority and mutation rights",
        ),
    }
}

pub(crate) fn verify_milestone_7_access_structures(
    state: &StoreState,
    media_report: DurableMediaReport,
) -> Milestone7AccessStructureVerification {
    let backend_family = media_report.backend_family();
    Milestone7AccessStructureVerification {
        backend_family,
        schema_boundary_fetch: verify_string_keyed_records(
            state.schema_support_records.iter().map(|(key, record)| {
                (
                    key.as_str(),
                    record.artifact_id.as_str(),
                    "schema support artifact id",
                )
            }),
            "loaded schema support map preserves exact artifact-id addressing",
        ),
        lineage_lookup: verify_string_keyed_records(
            state.lineage_support_records.iter().map(|(key, record)| {
                (
                    key.as_str(),
                    record.artifact_id.as_str(),
                    "lineage support artifact id",
                )
            }),
            "loaded lineage support map preserves exact artifact-id addressing",
        ),
        cursor_resume: verify_cursor_identity_keys(state),
        embedded_checkpoint_fetch: verify_string_keyed_records(
            state
                .embedded_checkpoint_records
                .iter()
                .map(|(key, record)| {
                    (
                        key.as_str(),
                        record.checkpoint_id.as_str(),
                        "embedded checkpoint id",
                    )
                }),
            "loaded embedded checkpoint map preserves exact checkpoint-id addressing",
        ),
        commit_coupled_support_publication: verify_commit_support_summary_keys(state),
        cursor_identity_admission: verify_subscriber_checkpoint_keys(state),
    }
}

fn verify_milestone_6_scope_membership_keys(
    state: &StoreState,
    missing_family_gap: &'static str,
    success_basis: &'static str,
) -> Milestone6AccessStructureVerificationPath {
    if state.milestone_6_scope_slice_membership_records.is_empty() {
        return Milestone6AccessStructureVerificationPath::debt(missing_family_gap);
    }
    for (stored_key, record) in &state.milestone_6_scope_slice_membership_records {
        if stored_key != record.artifact_id.as_str() {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match Milestone 6 scope membership artifact id `{}`",
                record.artifact_id
            ));
        }
        if !state
            .milestone_6_layout_materialization_records
            .contains_key(&record.layout_materialization_artifact_id)
        {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: Milestone 6 scope membership `{}` referenced missing layout materialization `{}`",
                record.artifact_id, record.layout_materialization_artifact_id
            ));
        }
    }
    Milestone6AccessStructureVerificationPath::verified(success_basis)
}

fn verify_milestone_6_structural_block_keys(
    state: &StoreState,
    missing_family_gap: &'static str,
    success_basis: &'static str,
) -> Milestone6AccessStructureVerificationPath {
    if state.milestone_6_structural_block_records.is_empty() {
        return Milestone6AccessStructureVerificationPath::debt(missing_family_gap);
    }
    for (stored_key, record) in &state.milestone_6_structural_block_records {
        if stored_key != record.artifact_id.as_str() {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match Milestone 6 structural block artifact id `{}`",
                record.artifact_id
            ));
        }
        if record
            .supporting_layout_materialization_artifact_ids
            .is_empty()
        {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: Milestone 6 structural block `{}` had no supporting layout materializations",
                record.artifact_id
            ));
        }
        for layout_materialization_artifact_id in
            &record.supporting_layout_materialization_artifact_ids
        {
            if !state
                .milestone_6_layout_materialization_records
                .contains_key(layout_materialization_artifact_id)
            {
                return Milestone6AccessStructureVerificationPath::debt(format!(
                    "open-time access structure verification failed: Milestone 6 structural block `{}` referenced missing layout materialization `{}`",
                    record.artifact_id, layout_materialization_artifact_id
                ));
            }
        }
    }
    Milestone6AccessStructureVerificationPath::verified(success_basis)
}

fn verify_milestone_6_chunk_membership_keys(
    state: &StoreState,
    missing_family_gap: &'static str,
    success_basis: &'static str,
) -> Milestone6AccessStructureVerificationPath {
    if state.milestone_6_chunk_membership_records.is_empty() {
        return Milestone6AccessStructureVerificationPath::debt(missing_family_gap);
    }
    for (stored_key, record) in &state.milestone_6_chunk_membership_records {
        if stored_key != record.artifact_id.as_str() {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match Milestone 6 chunk membership artifact id `{}`",
                record.artifact_id
            ));
        }
        if !state
            .milestone_6_layout_materialization_records
            .contains_key(&record.layout_materialization_artifact_id)
        {
            return Milestone6AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: Milestone 6 chunk membership `{}` referenced missing layout materialization `{}`",
                record.artifact_id, record.layout_materialization_artifact_id
            ));
        }
    }
    Milestone6AccessStructureVerificationPath::verified(success_basis)
}

fn verify_string_keyed_records<'a>(
    records: impl Iterator<Item = (&'a str, &'a str, &'static str)>,
    success_basis: &'static str,
) -> Milestone7AccessStructureVerificationPath {
    for (stored_key, expected_key, label) in records {
        if stored_key != expected_key {
            return Milestone7AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match expected {label} `{expected_key}`"
            ));
        }
    }
    Milestone7AccessStructureVerificationPath::verified(success_basis)
}

fn verify_commit_support_summary_keys(
    state: &StoreState,
) -> Milestone7AccessStructureVerificationPath {
    for (commit_id, summary) in &state.commit_support_summaries {
        if *commit_id != summary.commit_id.0 {
            return Milestone7AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: commit support summary map key `{commit_id}` did not match summary commit id `{}`",
                summary.commit_id.0
            ));
        }
    }
    Milestone7AccessStructureVerificationPath::verified(
        "loaded commit support summary map preserves exact commit-id addressing",
    )
}

fn verify_cursor_identity_keys(state: &StoreState) -> Milestone7AccessStructureVerificationPath {
    for (stored_key, record) in &state.durable_cursor_identity_records {
        let expected_key = durable_cursor_identity_artifact_id(&record.cursor_id);
        if stored_key != &expected_key {
            return Milestone7AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match expected durable cursor identity artifact id `{expected_key}`"
            ));
        }
    }
    Milestone7AccessStructureVerificationPath::verified(
        "loaded durable cursor identity map preserves exact cursor-id addressing",
    )
}

fn verify_subscriber_checkpoint_keys(
    state: &StoreState,
) -> Milestone7AccessStructureVerificationPath {
    for (stored_key, record) in &state.subscriber_checkpoint_records {
        let expected_key =
            subscriber_checkpoint_artifact_id(&record.cursor_id, record.checkpoint_sequence);
        if stored_key != &expected_key {
            return Milestone7AccessStructureVerificationPath::debt(format!(
                "open-time access structure verification failed: stored key `{stored_key}` did not match expected subscriber checkpoint artifact id `{expected_key}`"
            ));
        }
    }
    Milestone7AccessStructureVerificationPath::verified(
        "loaded subscriber checkpoint map preserves exact cursor checkpoint addressing",
    )
}
