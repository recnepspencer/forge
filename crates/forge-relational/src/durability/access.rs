use crate::capabilities::{
    DurabilityRead, RuntimeConfigSource, RuntimeIdentitySource, SchemaVersionSource,
};
use crate::durability::data::{
    DurabilityMode, RecoveryAuthorityParity, RecoveryCompatibilityCheck,
    RecoveryCompatibilityMismatch, RecoveryCursor, RecoveryIntegrityReport, RecoveryPlan,
    RecoveryVerificationOutcome,
};
use crate::durability::log::local_store::{
    load_store_from_disk, read_json, DurableCheckpointFile, DurableSegmentFile,
};
use crate::history::data::BranchHead;
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::ReplayVerificationLayer;
use crate::schema::logic::{
    validate_schema_continuity_bundle, SchemaContinuityBundleIssue, ValidatedSchemaContinuityBundle,
};

pub struct DurabilityAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> DurabilityAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn recovery_plan(
        &self,
        verification_mode: crate::durability::data::RecoveryVerificationMode,
    ) -> RecoveryPlan {
        match self.runtime.runtime_config().durability.policy.mode {
            DurabilityMode::InMemoryCanonical => {
                in_memory_recovery_plan(self.runtime, verification_mode)
            }
            DurabilityMode::PersistedSegmentedLocalFs => {
                self.persisted_recovery_plan(verification_mode)
            }
        }
    }

    pub fn durable_log(&self) -> &[crate::replay::data::CanonicalCommitEnvelope] {
        DurabilityRead::durable_log(self.runtime)
    }

    pub fn durable_branch_heads(&self) -> Vec<BranchHead> {
        self.runtime.history_access().branches()
    }

    fn persisted_recovery_plan(
        &self,
        verification_mode: crate::durability::data::RecoveryVerificationMode,
    ) -> RecoveryPlan {
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
                verification_mode,
                self.runtime
                    .runtime_config()
                    .schema
                    .descriptor_semantics_policy
                    .current_write_version(),
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
            verification_mode,
            descriptor_semantics_version,
        )
    }
}

impl RelationalRuntime {
    pub fn durability_access(&self) -> DurabilityAccess<'_> {
        DurabilityAccess::new(self)
    }
}

fn in_memory_recovery_plan(
    runtime: &RelationalRuntime,
    verification_mode: crate::durability::data::RecoveryVerificationMode,
) -> RecoveryPlan {
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
        verification_mode,
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
    let descriptor_policy = runtime
        .runtime_config()
        .schema
        .descriptor_semantics_policy
        .clone();
    let canonicalization_policy = runtime
        .runtime_config()
        .schema
        .descriptor_canonicalization_policy
        .clone();
    let expected_descriptor_semantics_version = descriptor_policy.current_write_version();
    let expected_descriptor_canonicalization_version =
        canonicalization_policy.current_write_version();
    let mut compatibility =
        RecoveryCompatibilityCheck::verified_at(ReplayVerificationLayer::DigestParity);

    for envelope in checkpoint_envelopes.iter().chain(tail_log.iter()) {
        if !descriptor_policy.supports(envelope.descriptor_semantics_version) {
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
            continue;
        }

        if let Some(found) =
            unsupported_canonicalization_version(envelope, &canonicalization_policy)
        {
            runtime
                .performance_access()
                .count_descriptor_version_mismatch();
            runtime
                .performance_access()
                .count_replay_verification_layer(ReplayVerificationLayer::DigestParity);
            compatibility.descriptor_version_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::DescriptorCanonicalizationVersion {
                    expected: expected_descriptor_canonicalization_version,
                    found,
                },
            );
            compatibility.verification_outcome = RecoveryVerificationOutcome::Rejected {
                layer: ReplayVerificationLayer::DigestParity,
                detail: "descriptor canonicalization version mismatch".to_string(),
            };
            continue;
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

fn unsupported_canonicalization_version(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
    policy: &crate::schema::data::DescriptorCanonicalizationCompatibilityPolicy,
) -> Option<crate::schema::data::DescriptorCanonicalizationVersion> {
    let continuation = envelope
        .schema_continuation_descriptor
        .as_ref()
        .map(|descriptor| descriptor.bridge.canonicalization_version);
    let reconciliation = envelope
        .schema_reconciliation_descriptor
        .as_ref()
        .map(|descriptor| descriptor.canonicalization_version);
    continuation
        .into_iter()
        .chain(reconciliation)
        .find(|version| !policy.supports(*version))
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
            runtime
                .performance_access()
                .count_descriptor_version_mismatch();
            compatibility.descriptor_version_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::DescriptorSemanticsVersion { expected, found },
            );
        }
        SchemaContinuityBundleIssue::DescriptorCanonicalizationVersionMismatch {
            expected,
            found,
        } => {
            runtime
                .performance_access()
                .count_descriptor_version_mismatch();
            compatibility.descriptor_version_parity = RecoveryAuthorityParity::drift();
            compatibility.first_mismatch.get_or_insert(
                RecoveryCompatibilityMismatch::DescriptorCanonicalizationVersion {
                    expected,
                    found,
                },
            );
        }
        SchemaContinuityBundleIssue::VisibleBridgeProofMismatch => {
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
