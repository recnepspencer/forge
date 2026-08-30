use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedRootManifest, PhysicalArtifactScope, PhysicalIntegrityValidationRecord,
    UntrustedPhysicalArtifact,
};

use super::{
    IntegrityAdmittedRecoveryArtifact, RecoveryIntegrityIngressRejection, UntrustedRecoverySource,
};

pub(crate) fn admit_current_root_selector<'media>(
    source: UntrustedRecoverySource<'media>,
    validated: IntegrityValidatedCurrentRootSelector<'media>,
) -> Result<IntegrityAdmittedRecoveryArtifact<'media>, RecoveryIntegrityIngressRejection> {
    let scope = validated.scope();
    let input = require_source(&source, scope)?;
    if !validated.matches_input(input) {
        return Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch);
    }
    admit_record(source, scope, validated.into_validation_record())
}

pub(crate) fn admit_previous_root_selector<'media>(
    source: UntrustedRecoverySource<'media>,
    validated: IntegrityValidatedPreviousRootSelector<'media>,
) -> Result<IntegrityAdmittedRecoveryArtifact<'media>, RecoveryIntegrityIngressRejection> {
    let scope = validated.scope();
    let input = require_source(&source, scope)?;
    if !validated.matches_input(input) {
        return Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch);
    }
    admit_record(source, scope, validated.into_validation_record())
}

pub(crate) fn admit_root_manifest<'media>(
    source: UntrustedRecoverySource<'media>,
    validated: IntegrityValidatedRootManifest<'media>,
) -> Result<IntegrityAdmittedRecoveryArtifact<'media>, RecoveryIntegrityIngressRejection> {
    let scope = validated.scope();
    let input = require_source(&source, scope)?;
    if !validated.matches_input(input) {
        return Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch);
    }
    admit_record(source, scope, validated.into_validation_record())
}

fn require_source<'media>(
    source: &UntrustedRecoverySource<'media>,
    validated_scope: PhysicalArtifactScope,
) -> Result<UntrustedPhysicalArtifact<'media>, RecoveryIntegrityIngressRejection> {
    if source.scope() != validated_scope {
        return Err(RecoveryIntegrityIngressRejection::ScopeMismatch);
    }
    source
        .input()
        .ok_or(RecoveryIntegrityIngressRejection::MissingBoundedArtifact)
}

fn admit_record<'media>(
    source: UntrustedRecoverySource<'media>,
    validated_scope: PhysicalArtifactScope,
    record: PhysicalIntegrityValidationRecord,
) -> Result<IntegrityAdmittedRecoveryArtifact<'media>, RecoveryIntegrityIngressRejection> {
    if !record.matches_scope(validated_scope) {
        return Err(RecoveryIntegrityIngressRejection::ScopeMismatch);
    }
    Ok(IntegrityAdmittedRecoveryArtifact::new(
        source.observed(),
        validated_scope,
        record,
    ))
}
