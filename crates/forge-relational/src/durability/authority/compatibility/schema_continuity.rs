use crate::capabilities::{
    RuntimeConfigSource, RuntimeIdentitySource, SchemaSource, SchemaVersionSource,
};
use crate::durability::data::{
    DurabilityError, RecoveryCompatibilityMismatch, RecoveryFailureClass, RecoveryPlan,
};
use crate::replay::data::CanonicalCommitEnvelope;
use crate::schema::logic::{validate_schema_continuity_bundle, SchemaContinuityBundleIssue};

pub(super) fn validate_schema_continuity_compatibility(
    runtime: &(impl SchemaSource + RuntimeIdentitySource + SchemaVersionSource + RuntimeConfigSource),
    plan: &RecoveryPlan,
) -> Result<(), DurabilityError> {
    let descriptor_policy = runtime
        .runtime_config()
        .schema
        .descriptor_semantics_policy
        .clone();
    let canonical_basis_policy = runtime
        .runtime_config()
        .schema
        .descriptor_canonical_basis_policy
        .clone();
    let runtime_descriptor_version = descriptor_policy.current_write_version();
    let runtime_canonical_basis_version = canonical_basis_policy.current_write_version();
    if !descriptor_policy.supports(plan.descriptor_semantics_version) {
        return Err(DurabilityError::new(
            RecoveryFailureClass::SchemaMismatch,
            "recovery descriptor semantics version mismatch",
        )
        .with_compatibility_mismatch(
            RecoveryCompatibilityMismatch::DescriptorSemanticsVersion {
                expected: plan.descriptor_semantics_version,
                found: runtime_descriptor_version,
            },
        ));
    }

    let checkpoint_envelopes = plan
        .checkpoint
        .as_ref()
        .map(|checkpoint| checkpoint.envelopes.as_slice())
        .unwrap_or(&[]);
    for envelope in checkpoint_envelopes.iter().chain(plan.tail_log.iter()) {
        if envelope.descriptor_semantics_version != plan.descriptor_semantics_version {
            return Err(DurabilityError::new(
                RecoveryFailureClass::SchemaMismatch,
                "recovery envelope descriptor semantics version mismatch",
            )
            .with_compatibility_mismatch(
                RecoveryCompatibilityMismatch::DescriptorSemanticsVersion {
                    expected: plan.descriptor_semantics_version,
                    found: envelope.descriptor_semantics_version,
                },
            ));
        }

        if let Some(found) = envelope
            .schema_continuation_descriptor
            .as_ref()
            .map(|descriptor| descriptor.bridge.canonical_basis_version)
            .into_iter()
            .chain(
                envelope
                    .schema_reconciliation_descriptor
                    .as_ref()
                    .map(|descriptor| descriptor.canonical_basis_version),
            )
            .find(|version| !canonical_basis_policy.supports(*version))
        {
            return Err(DurabilityError::new(
                RecoveryFailureClass::SchemaMismatch,
                "recovery envelope descriptor canonical basis version mismatch",
            )
            .with_compatibility_mismatch(
                RecoveryCompatibilityMismatch::DescriptorCanonicalBasisVersion {
                    expected: runtime_canonical_basis_version,
                    found,
                },
            ));
        }

        validate_schema_continuity_bundle(envelope)
            .map_err(|issue| schema_continuity_recovery_error(envelope, issue))?;
    }

    Ok(())
}

fn schema_continuity_recovery_error(
    envelope: &CanonicalCommitEnvelope,
    issue: SchemaContinuityBundleIssue,
) -> DurabilityError {
    let detail = issue.detail();
    let mismatch = match issue {
        SchemaContinuityBundleIssue::IncompleteBundle => {
            RecoveryCompatibilityMismatch::SchemaTransitionArtifact {
                commit_id: envelope.commit.commit_id.0,
                detail,
            }
        }
        SchemaContinuityBundleIssue::ContinuationDescriptorDrift {
            boundary_fingerprint,
        } => RecoveryCompatibilityMismatch::ContinuationDescriptor {
            commit_id: envelope.commit.commit_id.0,
            boundary_fingerprint,
            detail,
        },
        SchemaContinuityBundleIssue::ContinuationBoundaryFingerprintMismatch {
            boundary_fingerprint,
        } => RecoveryCompatibilityMismatch::ContinuationDescriptor {
            commit_id: envelope.commit.commit_id.0,
            boundary_fingerprint: Some(boundary_fingerprint),
            detail,
        },
        SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch { expected, found } => {
            RecoveryCompatibilityMismatch::DescriptorSemanticsVersion { expected, found }
        }
        SchemaContinuityBundleIssue::DescriptorCanonicalBasisVersionMismatch {
            expected,
            found,
        } => RecoveryCompatibilityMismatch::DescriptorCanonicalBasisVersion { expected, found },
        SchemaContinuityBundleIssue::VisibleBridgeProofMismatch => {
            RecoveryCompatibilityMismatch::ContinuationDescriptor {
                commit_id: envelope.commit.commit_id.0,
                boundary_fingerprint: envelope
                    .schema_continuation_descriptor
                    .as_ref()
                    .map(|descriptor| descriptor.boundary_fingerprint),
                detail,
            }
        }
        SchemaContinuityBundleIssue::ReconciliationDescriptorDrift => {
            RecoveryCompatibilityMismatch::ReconciliationDescriptor {
                commit_id: envelope.commit.commit_id.0,
                detail,
            }
        }
        SchemaContinuityBundleIssue::TargetSchemaVersionMismatch => {
            RecoveryCompatibilityMismatch::SchemaTransitionArtifact {
                commit_id: envelope.commit.commit_id.0,
                detail,
            }
        }
        SchemaContinuityBundleIssue::LineageSchemaVersionMismatch => {
            RecoveryCompatibilityMismatch::SchemaLineage {
                commit_id: envelope.commit.commit_id.0,
                detail,
            }
        }
        SchemaContinuityBundleIssue::HistoricalReinterpretationViolation => {
            RecoveryCompatibilityMismatch::ContinuationDescriptor {
                commit_id: envelope.commit.commit_id.0,
                boundary_fingerprint: envelope
                    .schema_continuation_descriptor
                    .as_ref()
                    .map(|descriptor| descriptor.boundary_fingerprint),
                detail,
            }
        }
    };
    DurabilityError::new(
        RecoveryFailureClass::SchemaMismatch,
        "recovery schema continuity compatibility failure",
    )
    .with_compatibility_mismatch(mismatch)
}
