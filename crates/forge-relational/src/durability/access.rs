use crate::capabilities::{
    DurabilityRead, RuntimeConfigSource, RuntimeIdentitySource, SchemaVersionSource,
};
use crate::durability::data::{
    DurabilityMode, RecoveryAuthorityParity, RecoveryCompatibilityCheck, RecoveryCursor,
    RecoveryIntegrityReport, RecoveryCompatibilityMismatch, RecoveryPlan,
    RecoveryVerificationOutcome,
};
use crate::history::data::BranchHead;
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::ReplayVerificationLayer;
use crate::schema::logic::{
    validate_schema_continuity_bundle, SchemaContinuityBundleIssue, ValidatedSchemaContinuityBundle,
};

use crate::durability::log::local_store::{
    load_store_from_disk, read_json, DurableCheckpointFile, DurableSegmentFile,
};

pub struct DurabilityAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> DurabilityAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn recovery_plan(&self) -> RecoveryPlan {
        match self.runtime.runtime_config().durability.policy.mode {
            DurabilityMode::InMemoryCanonical => in_memory_recovery_plan(self.runtime),
            DurabilityMode::PersistedSegmentedLocalFs => self.persisted_recovery_plan(),
        }
    }

    pub fn durable_log(&self) -> &[crate::replay::data::CanonicalCommitEnvelope] {
        DurabilityRead::durable_log(self.runtime)
    }

    pub fn durable_branch_heads(&self) -> Vec<BranchHead> {
        self.runtime.history_access().branches()
    }

    fn persisted_recovery_plan(&self) -> RecoveryPlan {
        let Ok(store) = load_store_from_disk(self.runtime) else {
            return RecoveryPlan::new(
                self.runtime.runtime_config().clone(),
                self.runtime.durable_store().cloned(),
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
                RecoveryCompatibilityCheck::verified_at(ReplayVerificationLayer::DigestParity),
                crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
                crate::schema::data::DescriptorSemanticsVersion::default(),
            );
        };
        let mut skipped_corrupt_checkpoints = Vec::new();
        let mut selected_checkpoint = None;
        let mut selected_checkpoint_manifest = None;
        for manifest in store.checkpoints.iter().rev() {
            match read_json::<DurableCheckpointFile>(&manifest.path) {
                Ok(file) => {
                    selected_checkpoint = Some(file.checkpoint);
                    selected_checkpoint_manifest = Some(manifest.clone());
                    break;
                }
                Err(_) => skipped_corrupt_checkpoints.push(manifest.checkpoint_id),
            }
        }
        let checkpoint_commit = selected_checkpoint
            .as_ref()
            .and_then(|checkpoint| checkpoint.coverage.up_to_commit.as_ref())
            .map(|commit| commit.commit_id);
        let mut tail_log = Vec::new();
        let mut verified_segment_ids = Vec::new();
        let mut corrupt_segment_id = None;
        for manifest in &store.segments {
            if checkpoint_commit
                .is_some_and(|covered| manifest.last_commit_id.is_some_and(|last| last <= covered))
            {
                continue;
            }
            match read_json::<DurableSegmentFile>(&manifest.path) {
                Ok(file) => {
                    verified_segment_ids.push(manifest.segment_id);
                    tail_log.extend(file.entries.into_iter().filter(|entry| {
                        checkpoint_commit.is_none_or(|covered| entry.commit.commit_id > covered)
                    }));
                }
                Err(_) => {
                    corrupt_segment_id = Some(manifest.segment_id);
                    break;
                }
            }
        }
        let descriptor_semantics_version = descriptor_semantics_version_for_envelopes(
            selected_checkpoint
                .as_ref()
                .map(|checkpoint| checkpoint.envelopes.as_slice())
                .unwrap_or(&[]),
            &tail_log,
        );
        let mut continuity_compatibility = continuity_compatibility_for_envelopes(
            self.runtime,
            selected_checkpoint
                .as_ref()
                .map(|checkpoint| checkpoint.envelopes.as_slice())
                .unwrap_or(&[]),
            &tail_log,
        );
        let schema_match = selected_checkpoint_manifest
            .as_ref()
            .map(|manifest| manifest.schema_version == self.runtime.primary_schema_version_id())
            .unwrap_or(true);
        let profile_match = selected_checkpoint_manifest
            .as_ref()
            .map(|manifest| manifest.profile == self.runtime.runtime_profile())
            .unwrap_or(true);
        let runtime_name_match = selected_checkpoint_manifest
            .as_ref()
            .map(|manifest| manifest.runtime_name == self.runtime.runtime_name())
            .unwrap_or(true);
        continuity_compatibility.schema_parity = if schema_match {
            RecoveryAuthorityParity::verified_at(ReplayVerificationLayer::DigestParity)
        } else {
            RecoveryAuthorityParity::drift()
        };
        continuity_compatibility.profile_parity = if profile_match {
            RecoveryAuthorityParity::verified_at(ReplayVerificationLayer::DigestParity)
        } else {
            RecoveryAuthorityParity::drift()
        };
        continuity_compatibility.runtime_name_parity = if runtime_name_match {
            RecoveryAuthorityParity::verified_at(ReplayVerificationLayer::DigestParity)
        } else {
            RecoveryAuthorityParity::drift()
        };
        if continuity_compatibility.first_mismatch.is_none() {
            continuity_compatibility.first_mismatch = recovery_basis_mismatch(
                selected_checkpoint_manifest.as_ref(),
                &self.runtime.runtime_config().schema.registry,
                self.runtime.runtime_profile(),
                self.runtime.runtime_name(),
                self.runtime.primary_schema_version_id(),
            );
        }
        RecoveryPlan::new(
            self.runtime.runtime_config().clone(),
            Some(store.clone()),
            selected_checkpoint_manifest.clone(),
            selected_checkpoint.clone(),
            tail_log,
            RecoveryCursor {
                checkpoint_id: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.checkpoint_id),
                segment_ids: verified_segment_ids.clone(),
            },
            RecoveryIntegrityReport {
                selected_checkpoint_id: selected_checkpoint_manifest
                    .as_ref()
                    .map(|manifest| manifest.checkpoint_id),
                skipped_corrupt_checkpoints,
                verified_segment_ids,
                corrupt_segment_id,
            },
            continuity_compatibility,
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
            descriptor_semantics_version,
        )
    }
}

impl RelationalRuntime {
    pub fn durability_access(&self) -> DurabilityAccess<'_> {
        DurabilityAccess::new(self)
    }
}

fn in_memory_recovery_plan(runtime: &RelationalRuntime) -> RecoveryPlan {
    let checkpoint = runtime.durable_checkpoints().last().cloned();
    let tail_log = match checkpoint
        .as_ref()
        .and_then(|c| c.coverage.up_to_commit.as_ref())
    {
        Some(up_to_commit) => runtime
            .durable_log()
            .iter()
            .filter(|entry| entry.commit.commit_id > up_to_commit.commit_id)
            .cloned()
            .collect(),
        None => runtime.durable_log().to_vec(),
    };
    let descriptor_semantics_version = descriptor_semantics_version_for_envelopes(
        checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.envelopes.as_slice())
            .unwrap_or(&[]),
        &tail_log,
    );
    let continuity_compatibility = continuity_compatibility_for_envelopes(
        runtime,
        checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.envelopes.as_slice())
            .unwrap_or(&[]),
        &tail_log,
    );
    RecoveryPlan::new(
        runtime.runtime_config().clone(),
        runtime.durable_store().cloned(),
        None,
        checkpoint,
        tail_log,
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
        continuity_compatibility,
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        descriptor_semantics_version,
    )
}

fn descriptor_semantics_version_for_envelopes(
    checkpoint_envelopes: &[crate::replay::data::CanonicalCommitEnvelope],
    tail_log: &[crate::replay::data::CanonicalCommitEnvelope],
) -> crate::schema::data::DescriptorSemanticsVersion {
    tail_log
        .last()
        .or_else(|| checkpoint_envelopes.last())
        .map(|envelope| envelope.descriptor_semantics_version)
        .unwrap_or_else(crate::schema::data::DescriptorSemanticsVersion::default)
}

fn continuity_compatibility_for_envelopes(
    runtime: &RelationalRuntime,
    checkpoint_envelopes: &[crate::replay::data::CanonicalCommitEnvelope],
    tail_log: &[crate::replay::data::CanonicalCommitEnvelope],
) -> RecoveryCompatibilityCheck {
    let expected_descriptor_semantics_version =
        crate::schema::data::DescriptorSemanticsVersion::default();
    let mut compatibility =
        RecoveryCompatibilityCheck::verified_at(ReplayVerificationLayer::DigestParity);

    for envelope in checkpoint_envelopes.iter().chain(tail_log.iter()) {
        if envelope.descriptor_semantics_version != expected_descriptor_semantics_version {
            runtime
                .performance_access()
                .count_descriptor_version_mismatch();
            runtime
                .performance_access()
                .count_replay_verification_layer(ReplayVerificationLayer::DigestParity);
            compatibility.descriptor_version_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::DescriptorSemanticsVersion {
                    expected: expected_descriptor_semantics_version,
                    found: envelope.descriptor_semantics_version,
                },
            );
            compatibility.verification_outcome = RecoveryVerificationOutcome::Rejected {
                layer: ReplayVerificationLayer::DigestParity,
                detail: "descriptor semantics version mismatch".to_string(),
            };
        }

        match validated_recovery_continuity_envelope(envelope) {
            Ok(validated_bundle) => {
                runtime
                    .performance_access()
                    .count_replay_verification_layer(ReplayVerificationLayer::DigestParity);
                let _ = (
                    validated_bundle.envelope(),
                    validated_bundle.transition(),
                    validated_bundle.continuation(),
                    validated_bundle.reconciliation(),
                );
            }
            Err(issue) => apply_continuity_issue(runtime, &mut compatibility, envelope, issue),
        }
    }

    compatibility
}

fn apply_continuity_issue(
    runtime: &RelationalRuntime,
    compatibility: &mut RecoveryCompatibilityCheck,
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
    issue: SchemaContinuityBundleIssue,
) {
    let detail = issue.detail();
    runtime
        .performance_access()
        .count_replay_verification_layer(ReplayVerificationLayer::DigestParity);
    compatibility.verification_outcome = RecoveryVerificationOutcome::Rejected {
        layer: ReplayVerificationLayer::DigestParity,
        detail: detail.clone(),
    };
    match issue {
        SchemaContinuityBundleIssue::IncompleteBundle => {
            compatibility.schema_transition_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::SchemaTransitionArtifact {
                    commit_id: envelope.commit.commit_id.0,
                    detail,
                },
            );
        }
        SchemaContinuityBundleIssue::ContinuationDescriptorDrift {
            boundary_fingerprint,
        } => {
            compatibility.continuation_descriptor_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::ContinuationDescriptor {
                    commit_id: envelope.commit.commit_id.0,
                    boundary_fingerprint,
                    detail,
                },
            );
        }
        SchemaContinuityBundleIssue::ReconciliationDescriptorDrift => {
            compatibility.reconciliation_descriptor_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::ReconciliationDescriptor {
                    commit_id: envelope.commit.commit_id.0,
                    detail,
                },
            );
        }
        SchemaContinuityBundleIssue::ContinuationBoundaryFingerprintMismatch {
            boundary_fingerprint,
        } => {
            compatibility.continuation_descriptor_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::ContinuationDescriptor {
                    commit_id: envelope.commit.commit_id.0,
                    boundary_fingerprint: Some(boundary_fingerprint),
                    detail,
                },
            );
        }
        SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch { expected, found } => {
            runtime.performance_access().count_descriptor_version_mismatch();
            compatibility.descriptor_version_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::DescriptorSemanticsVersion { expected, found },
            );
        }
        SchemaContinuityBundleIssue::TargetSchemaVersionMismatch => {
            compatibility.schema_transition_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::SchemaTransitionArtifact {
                    commit_id: envelope.commit.commit_id.0,
                    detail,
                },
            );
        }
        SchemaContinuityBundleIssue::LineageSchemaVersionMismatch => {
            compatibility.schema_lineage_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::SchemaLineage {
                    commit_id: envelope.commit.commit_id.0,
                    detail,
                },
            );
        }
        SchemaContinuityBundleIssue::HistoricalReinterpretationViolation => {
            compatibility.continuation_descriptor_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::ContinuationDescriptor {
                    commit_id: envelope.commit.commit_id.0,
                    boundary_fingerprint: envelope
                        .schema_continuation_descriptor
                        .as_ref()
                        .map(|descriptor| descriptor.boundary_fingerprint),
                    detail,
                },
            );
        }
    }
}

fn validated_recovery_continuity_envelope(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> Result<ValidatedSchemaContinuityBundle<'_>, SchemaContinuityBundleIssue> {
    validate_schema_continuity_bundle(envelope)
}

fn recovery_basis_mismatch(
    checkpoint_manifest: Option<&crate::durability::data::DurableCheckpointManifest>,
    runtime_registry: &crate::schema::data::RelationalSchemaRegistry,
    runtime_profile: crate::config::data::RelationalRuntimeProfile,
    runtime_name: &str,
    primary_schema_version_id: crate::schema::data::SchemaVersionId,
) -> Option<RecoveryCompatibilityMismatch> {
    let manifest = checkpoint_manifest?;
    if manifest.schema_version != primary_schema_version_id {
        return Some(RecoveryCompatibilityMismatch::SchemaRegistryShape {
            expected_primary_schema_version: manifest.schema_version,
            found_primary_schema_version: primary_schema_version_id,
            expected_entity_kind_count: runtime_registry.entity_kinds.len(),
            found_entity_kind_count: runtime_registry.entity_kinds.len(),
            expected_relation_kind_count: runtime_registry.relation_kinds.len(),
            found_relation_kind_count: runtime_registry.relation_kinds.len(),
        });
    }
    if manifest.profile != runtime_profile {
        return Some(RecoveryCompatibilityMismatch::RuntimeProfile {
            expected: format!("{:?}", manifest.profile),
            found: format!("{runtime_profile:?}"),
        });
    }
    if manifest.runtime_name != runtime_name {
        return Some(RecoveryCompatibilityMismatch::RuntimeName {
            expected: manifest.runtime_name.clone(),
            found: runtime_name.to_string(),
        });
    }
    None
}
