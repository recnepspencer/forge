use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, PhysicalRecordFormatDeclaration, ROOT_SELECTOR_BYTES,
};
use worth_store_physical_integrity::{
    validate_current_root_selector, validate_previous_root_selector, validate_root_manifest,
    CurrentRootSelectorIntegrityValidation, PhysicalArtifactScope, PhysicalByteRange,
    PreviousRootSelectorIntegrityValidation, RootManifestIntegrityValidation,
};

use super::admitted_artifact::{
    IntegrityAdmittedCurrentRootSelector, IntegrityAdmittedPreviousRootSelector,
    IntegrityAdmittedRootManifest,
};
use super::{
    admit_current_root_selector, admit_previous_root_selector, admit_root_manifest,
    RecoveryArtifactNamespaceJoin, RecoveryIntegrityIngressRejection, UntrustedRecoverySource,
};

pub(crate) fn admit_current_selector<'media>(
    join: RecoveryArtifactNamespaceJoin<'media>,
    store: StableStoreIdentity,
    format: PhysicalRecordFormatDeclaration,
) -> Result<IntegrityAdmittedCurrentRootSelector<'media>, RecoveryIntegrityIngressRejection> {
    let observed = join.require_observed()?;
    let scope = PhysicalArtifactScope::current_root_selector(
        store,
        format,
        PhysicalByteRange::new(0, ROOT_SELECTOR_BYTES as u64)
            .expect("the selector declaration has a nonzero fixed width"),
    );
    let source = UntrustedRecoverySource::new(observed, scope);
    let input = source
        .input()
        .ok_or(RecoveryIntegrityIngressRejection::MissingBoundedArtifact)?;
    let (validation, _) = validate_current_root_selector(input, scope);
    match validation {
        CurrentRootSelectorIntegrityValidation::Intact(validated) => {
            admit_current_root_selector(source, validated)
        }
        CurrentRootSelectorIntegrityValidation::Rejected(rejection) => {
            Err(RecoveryIntegrityIngressRejection::Integrity(rejection))
        }
    }
}

pub(crate) fn admit_previous_selector<'media>(
    join: RecoveryArtifactNamespaceJoin<'media>,
    store: StableStoreIdentity,
    format: PhysicalRecordFormatDeclaration,
) -> Result<IntegrityAdmittedPreviousRootSelector<'media>, RecoveryIntegrityIngressRejection> {
    let observed = join.require_observed()?;
    let scope = PhysicalArtifactScope::previous_root_selector(
        store,
        format,
        PhysicalByteRange::new(0, ROOT_SELECTOR_BYTES as u64)
            .expect("the selector declaration has a nonzero fixed width"),
    );
    let source = UntrustedRecoverySource::new(observed, scope);
    let input = source
        .input()
        .ok_or(RecoveryIntegrityIngressRejection::MissingBoundedArtifact)?;
    let (validation, _) = validate_previous_root_selector(input, scope);
    match validation {
        PreviousRootSelectorIntegrityValidation::Intact(validated) => {
            admit_previous_root_selector(source, validated)
        }
        PreviousRootSelectorIntegrityValidation::Rejected(rejection) => {
            Err(RecoveryIntegrityIngressRejection::Integrity(rejection))
        }
    }
}

pub(crate) fn admit_addressed_root<'media>(
    join: RecoveryArtifactNamespaceJoin<'media>,
    store: StableStoreIdentity,
    format: PhysicalRecordFormatDeclaration,
    generation: u64,
) -> Result<IntegrityAdmittedRootManifest<'media>, RecoveryIntegrityIngressRejection> {
    let observed = join.require_observed()?;
    let observed_length = observed.bytes().map_or(1, |bytes| bytes.len().max(1)) as u64;
    let scope = PhysicalArtifactScope::root_manifest(
        store,
        format,
        generation,
        PhysicalByteRange::new(0, observed_length)
            .expect("a present root observation has a bounded validation range"),
    )
    .map_err(|_| RecoveryIntegrityIngressRejection::ScopeMismatch)?;
    let source = UntrustedRecoverySource::new(observed, scope);
    let input = source
        .input()
        .ok_or(RecoveryIntegrityIngressRejection::MissingBoundedArtifact)?;
    let (validation, _) = validate_root_manifest(input, scope);
    match validation {
        RootManifestIntegrityValidation::Intact(validated) => {
            admit_root_manifest(source, validated)
        }
        RootManifestIntegrityValidation::Rejected(rejection) => {
            Err(RecoveryIntegrityIngressRejection::Integrity(rejection))
        }
    }
}
