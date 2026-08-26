//! Fresh Query validation and exact typed re-entry closure.

use crate::package::{
    WorthQueryPortablePackageManifest, WorthQueryPortablePackageRecord,
    WorthQueryPortablePackageRecordFamily as Family, WorthQueryValidatedPortableDomainPackage,
};

use super::{
    canonical_order::validate_candidate_order, WorthQueryExpectedPortablePackageIdentity,
    WorthQueryPortablePackageReconstructionDenial as Denial,
    WorthQueryReconstructedPortablePackageCandidate,
};

impl WorthQueryReconstructedPortablePackageCandidate {
    pub fn validate_freshly(
        self,
        expected_identity: WorthQueryExpectedPortablePackageIdentity,
    ) -> Result<WorthQueryValidatedPortableDomainPackage, Denial> {
        validate_candidate_order(&self)?;
        let expected_manifest = self.manifest;
        let expected_native_aspects = self.expected_native_aspects;
        let expected_application_operations = self.expected_application_operations;
        let remaining_canonical_work_bytes = self.work.remaining_canonical_work_bytes(self.limits);
        let validated = self
            .package
            .validate_with_canonical_work_limit(remaining_canonical_work_bytes)
            .map_err(|denial| Denial::FreshPackageValidationDenied { denial })?;
        let fresh = validated
            .export_typed_records()
            .map_err(|denial| Denial::FreshPackageExportDenied { denial })?;
        validate_identity(&expected_manifest, fresh.manifest())?;
        validate_expected_identity(&expected_identity, fresh.manifest())?;
        validate_manifest_closure(&expected_manifest, fresh.manifest())?;
        validate_derived_closure(
            fresh.records(),
            &expected_native_aspects,
            &expected_application_operations,
        )?;
        Ok(validated)
    }
}

fn validate_identity(
    expected: &WorthQueryPortablePackageManifest,
    fresh: &WorthQueryPortablePackageManifest,
) -> Result<(), Denial> {
    if fresh.package_identity() == expected.package_identity() {
        return Ok(());
    }
    Err(Denial::ManifestPackageIdentityMismatch {
        claimed: expected.package_identity().clone(),
        recomputed: fresh.package_identity().clone(),
    })
}

fn validate_expected_identity(
    expected: &WorthQueryExpectedPortablePackageIdentity,
    fresh: &WorthQueryPortablePackageManifest,
) -> Result<(), Denial> {
    if fresh.package_identity() == expected.identity() {
        return Ok(());
    }
    Err(Denial::ExpectedPackageIdentityMismatch {
        expected: expected.identity().clone(),
        recomputed: fresh.package_identity().clone(),
    })
}

fn validate_manifest_closure(
    expected: &WorthQueryPortablePackageManifest,
    fresh: &WorthQueryPortablePackageManifest,
) -> Result<(), Denial> {
    if expected.version() == fresh.version()
        && expected.record_count() == fresh.record_count()
        && expected.canonical_source_bytes() == fresh.canonical_source_bytes()
        && expected.logical_export_bytes() == fresh.logical_export_bytes()
        && expected.family_counts() == fresh.family_counts()
    {
        Ok(())
    } else {
        Err(Denial::FreshManifestClosureMismatch)
    }
}

fn validate_derived_closure(
    records: &[WorthQueryPortablePackageRecord],
    expected_native_aspects: &[crate::package::WorthQueryPortableNativeAspectContractRecord],
    expected_application_operations: &[crate::package::WorthQueryPortableApplicationOperationContractRecord],
) -> Result<(), Denial> {
    let fresh_native_aspects_match = records
        .iter()
        .filter_map(|record| match record {
            WorthQueryPortablePackageRecord::NativeAspectContract(value) => Some(value),
            _ => None,
        })
        .eq(expected_native_aspects.iter());
    if !fresh_native_aspects_match {
        return Err(Denial::DerivedContractClosureMismatch {
            family: Family::NativeAspectContract,
        });
    }
    let fresh_application_operations_match = records
        .iter()
        .filter_map(|record| match record {
            WorthQueryPortablePackageRecord::ApplicationOperationContract(value) => Some(value),
            _ => None,
        })
        .eq(expected_application_operations.iter());
    if !fresh_application_operations_match {
        return Err(Denial::DerivedContractClosureMismatch {
            family: Family::ApplicationOperationContract,
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "fresh_validation_tests.rs"]
mod tests;
