//! Materialized package meaning that has not received package validation.

use crate::package::{
    WorthQueryPortableApplicationOperationContractRecord, WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage, WorthQueryPortableNativeAspectContractRecord,
    WorthQueryPortablePackageManifest, WorthQueryPortablePackageReconstructionLimits,
    WorthQueryPortablePackageReconstructionWork,
};

/// Reconstructed semantic package candidate with no validation authority.
///
/// ```compile_fail
/// fn cannot_validate(
///     candidate: worth_query_installation::facade::WorthQueryReconstructedPortablePackageCandidate,
/// ) {
///     let _validated = candidate.validate();
/// }
/// ```
///
/// ```compile_fail
/// fn cannot_install(
///     candidate: worth_query_installation::facade::WorthQueryReconstructedPortablePackageCandidate,
/// ) {
///     let _installed = candidate.install();
/// }
/// ```
///
/// The identity claimed inside decoded records cannot substitute for the
/// caller's separately supplied expectation:
///
/// ```compile_fail
/// fn manifest_claim_is_not_an_expected_identity(
///     candidate: worth_query_installation::facade::WorthQueryReconstructedPortablePackageCandidate,
/// ) {
///     let _validated = candidate.validate_freshly();
/// }
/// ```
#[derive(Clone, Debug)]
pub struct WorthQueryReconstructedPortablePackageCandidate {
    pub(super) manifest: WorthQueryPortablePackageManifest,
    pub(super) package: WorthQueryPortableDomainPackage,
    pub(super) expected_native_aspects: Vec<WorthQueryPortableNativeAspectContractRecord>,
    pub(super) expected_application_operations:
        Vec<WorthQueryPortableApplicationOperationContractRecord>,
    pub(super) limits: WorthQueryPortablePackageReconstructionLimits,
    pub(super) work: WorthQueryPortablePackageReconstructionWork,
}

impl WorthQueryReconstructedPortablePackageCandidate {
    pub const fn manifest(&self) -> &WorthQueryPortablePackageManifest {
        &self.manifest
    }

    pub const fn domain_identity(&self) -> &WorthQueryPortableDomainIdentity {
        &self.package.identity
    }

    pub fn expected_native_aspects(&self) -> &[WorthQueryPortableNativeAspectContractRecord] {
        &self.expected_native_aspects
    }

    pub fn expected_application_operations(
        &self,
    ) -> &[WorthQueryPortableApplicationOperationContractRecord] {
        &self.expected_application_operations
    }

    pub const fn work(&self) -> WorthQueryPortablePackageReconstructionWork {
        self.work
    }
}
