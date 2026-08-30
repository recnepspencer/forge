use worth_store_physical_integrity::{PhysicalArtifactScope, UntrustedPhysicalArtifact};

use super::source::{ObservedRecoverySource, ObservedWalFrameSource};
use super::RecoveryIntegrityIngressRejection;

pub(super) fn require_observed_recovery_source<'media>(
    source: &ObservedRecoverySource<'media>,
    validated_scope: PhysicalArtifactScope,
    matches: impl FnOnce(UntrustedPhysicalArtifact<'media>) -> bool,
) -> Result<(), RecoveryIntegrityIngressRejection> {
    if source.scope() != validated_scope {
        return Err(RecoveryIntegrityIngressRejection::ScopeMismatch);
    }
    let input = source.input()?;
    if !matches(input) {
        return Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch);
    }
    Ok(())
}

pub(super) fn require_observed_wal_source<'media>(
    source: &ObservedWalFrameSource<'media>,
    validated_scope: PhysicalArtifactScope,
    matches: impl FnOnce(UntrustedPhysicalArtifact<'media>) -> bool,
) -> Result<(), RecoveryIntegrityIngressRejection> {
    if source.scope() != validated_scope {
        return Err(RecoveryIntegrityIngressRejection::ScopeMismatch);
    }
    let input = source.input()?;
    if !matches(input) {
        return Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch);
    }
    Ok(())
}
