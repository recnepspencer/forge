use crate::capabilities::{
    DurabilityRead, RuntimeConfigSource, RuntimeIdentitySource, SchemaVersionSource,
};
use crate::durability::access::{
    authority_continuity_for_envelopes, descriptor_semantics_version_for_envelopes,
    recovery_basis_mismatch,
};
use crate::durability::data::{
    RecoveryAuthorityContinuityCheck, RecoveryAuthorityParity, RecoveryCursor,
    RecoveryIntegrityReport, RecoveryPlan, RecoveryVerificationMode,
};
use crate::durability::log::local_store::{load_store_from_disk, read_segment_entries};
use crate::durability::log::native_file_codec::read_checkpoint_file;
use crate::replay::data::ReplayVerificationLayer;
use crate::runtime::RelationalRuntime;

pub(super) fn persisted_recovery_plan(
    runtime: &RelationalRuntime,
    verification_mode: RecoveryVerificationMode,
) -> RecoveryPlan {
    let Ok(store) = load_store_from_disk(runtime) else {
        return empty_persisted_recovery_plan(runtime, verification_mode);
    };

    let selected_checkpoint = select_latest_readable_checkpoint(&store);
    let checkpoint_commit = selected_checkpoint
        .checkpoint
        .as_ref()
        .and_then(|checkpoint| checkpoint.coverage.up_to_commit.as_ref())
        .map(|commit| commit.commit_id);
    let verified_tail = read_verified_tail_log(&store, checkpoint_commit);
    let descriptor_semantics_version = descriptor_semantics_version_for_envelopes(
        selected_checkpoint
            .checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.envelopes.as_slice())
            .unwrap_or(&[]),
        &verified_tail.tail_log,
    );
    let recovery_authority_continuity = persisted_authority_continuity(
        runtime,
        selected_checkpoint.checkpoint.as_ref(),
        selected_checkpoint.manifest.as_ref(),
        &verified_tail.tail_log,
    );
    let restore_authoritative_envelope_commit_ids = verified_tail
        .tail_log
        .iter()
        .map(|entry| entry.commit.commit_id)
        .collect();

    RecoveryPlan::new(
        runtime.runtime_config().clone(),
        Some(store.clone()),
        selected_checkpoint.manifest.clone(),
        selected_checkpoint.checkpoint.clone(),
        verified_tail.tail_log,
        RecoveryCursor {
            checkpoint_id: selected_checkpoint
                .manifest
                .as_ref()
                .map(|manifest| manifest.checkpoint_id),
            segment_ids: verified_tail.verified_segment_ids.clone(),
        },
        RecoveryIntegrityReport {
            selected_checkpoint_id: selected_checkpoint
                .manifest
                .as_ref()
                .map(|manifest| manifest.checkpoint_id),
            skipped_corrupt_checkpoints: selected_checkpoint.skipped_corrupt_checkpoints,
            verified_segment_ids: verified_tail.verified_segment_ids,
            corrupt_segment_id: verified_tail.corrupt_segment_id,
        },
        recovery_authority_continuity,
        verification_mode,
        descriptor_semantics_version,
        restore_authoritative_envelope_commit_ids,
    )
    .with_commit_strategy_executors(runtime.commit_strategy_executor_registry().clone())
}

#[derive(Debug)]
struct SelectedPersistedCheckpoint {
    checkpoint: Option<crate::durability::data::DurableCheckpoint>,
    manifest: Option<crate::durability::data::DurableCheckpointManifest>,
    skipped_corrupt_checkpoints: Vec<crate::durability::data::DurableCheckpointId>,
}

#[derive(Debug)]
struct VerifiedTailLog {
    tail_log: Vec<crate::history::data::CanonicalCommitEnvelope>,
    verified_segment_ids: Vec<crate::durability::data::DurableSegmentId>,
    corrupt_segment_id: Option<crate::durability::data::DurableSegmentId>,
}

fn empty_persisted_recovery_plan(
    runtime: &RelationalRuntime,
    verification_mode: RecoveryVerificationMode,
) -> RecoveryPlan {
    RecoveryPlan::new(
        runtime.runtime_config().clone(),
        runtime.durable_store().cloned(),
        None,
        None,
        Vec::new(),
        RecoveryCursor {
            checkpoint_id: None,
            segment_ids: Vec::new(),
        },
        RecoveryIntegrityReport {
            selected_checkpoint_id: None,
            skipped_corrupt_checkpoints: Vec::new(),
            verified_segment_ids: Vec::new(),
            corrupt_segment_id: None,
        },
        RecoveryAuthorityContinuityCheck::verified_at(ReplayVerificationLayer::DigestParity),
        verification_mode,
        runtime
            .runtime_config()
            .schema
            .descriptor_semantics_policy
            .current_write_version(),
        Vec::new(),
    )
    .with_commit_strategy_executors(runtime.commit_strategy_executor_registry().clone())
}

fn select_latest_readable_checkpoint(
    store: &crate::durability::data::DurableStore,
) -> SelectedPersistedCheckpoint {
    let mut skipped_corrupt_checkpoints = Vec::new();
    for manifest in store.checkpoints.iter().rev() {
        match read_checkpoint_file(&manifest.path) {
            Ok(file) => {
                return SelectedPersistedCheckpoint {
                    checkpoint: Some(file.checkpoint),
                    manifest: Some(manifest.clone()),
                    skipped_corrupt_checkpoints,
                };
            }
            Err(_) => skipped_corrupt_checkpoints.push(manifest.checkpoint_id),
        }
    }
    SelectedPersistedCheckpoint {
        checkpoint: None,
        manifest: None,
        skipped_corrupt_checkpoints,
    }
}

fn read_verified_tail_log(
    store: &crate::durability::data::DurableStore,
    checkpoint_commit: Option<crate::history::data::CommitId>,
) -> VerifiedTailLog {
    let mut tail_log = Vec::new();
    let mut verified_segment_ids = Vec::new();
    let mut corrupt_segment_id = None;
    for manifest in &store.segments {
        if checkpoint_commit
            .is_some_and(|covered| manifest.last_commit_id.is_some_and(|last| last <= covered))
        {
            continue;
        }
        match read_segment_entries(&manifest.path) {
            Ok(entries) => {
                verified_segment_ids.push(manifest.segment_id);
                tail_log.extend(entries.into_iter().filter(|entry| {
                    checkpoint_commit.is_none_or(|covered| entry.commit.commit_id > covered)
                }));
            }
            Err(_) => {
                corrupt_segment_id = Some(manifest.segment_id);
                break;
            }
        }
    }
    VerifiedTailLog {
        tail_log,
        verified_segment_ids,
        corrupt_segment_id,
    }
}

fn persisted_authority_continuity(
    runtime: &RelationalRuntime,
    selected_checkpoint: Option<&crate::durability::data::DurableCheckpoint>,
    selected_checkpoint_manifest: Option<&crate::durability::data::DurableCheckpointManifest>,
    tail_log: &[crate::history::data::CanonicalCommitEnvelope],
) -> RecoveryAuthorityContinuityCheck {
    let mut recovery_authority_continuity = authority_continuity_for_envelopes(
        runtime,
        selected_checkpoint
            .map(|checkpoint| checkpoint.envelopes.as_slice())
            .unwrap_or(&[]),
        tail_log,
    );
    apply_checkpoint_manifest_parity(
        runtime,
        &mut recovery_authority_continuity,
        selected_checkpoint_manifest,
    );
    recovery_authority_continuity
}

fn apply_checkpoint_manifest_parity(
    runtime: &RelationalRuntime,
    recovery_authority_continuity: &mut RecoveryAuthorityContinuityCheck,
    selected_checkpoint_manifest: Option<&crate::durability::data::DurableCheckpointManifest>,
) {
    let schema_match = selected_checkpoint_manifest
        .map(|manifest| manifest.schema_version == runtime.primary_schema_version_id())
        .unwrap_or(true);
    let profile_match = selected_checkpoint_manifest
        .map(|manifest| manifest.profile == runtime.runtime_profile())
        .unwrap_or(true);
    let runtime_name_match = selected_checkpoint_manifest
        .map(|manifest| manifest.runtime_name == runtime.runtime_name())
        .unwrap_or(true);
    recovery_authority_continuity.schema_parity = if schema_match {
        RecoveryAuthorityParity::verified_at(ReplayVerificationLayer::DigestParity)
    } else {
        RecoveryAuthorityParity::drift()
    };
    recovery_authority_continuity.profile_parity = if profile_match {
        RecoveryAuthorityParity::verified_at(ReplayVerificationLayer::DigestParity)
    } else {
        RecoveryAuthorityParity::drift()
    };
    recovery_authority_continuity.runtime_name_parity = if runtime_name_match {
        RecoveryAuthorityParity::verified_at(ReplayVerificationLayer::DigestParity)
    } else {
        RecoveryAuthorityParity::drift()
    };
    if recovery_authority_continuity.first_mismatch.is_none() {
        recovery_authority_continuity.first_mismatch = recovery_basis_mismatch(
            selected_checkpoint_manifest,
            &runtime.runtime_config().schema.registry,
            runtime.runtime_profile(),
            runtime.runtime_name(),
            runtime.primary_schema_version_id(),
        );
    }
}
