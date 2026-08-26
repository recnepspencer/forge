use crate::package::{
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
    WorthQueryPortablePackageManifest, WorthQueryPortablePackageRecord,
};

use super::{
    WorthQueryPortablePackageReconstruction, WorthQueryPortablePackageReconstructionCandidate,
    WorthQueryPortablePackageReconstructionLimits,
};

pub(super) fn close_records(
    manifest: &WorthQueryPortablePackageManifest,
    records: Vec<WorthQueryPortablePackageRecord>,
    limits: WorthQueryPortablePackageReconstructionLimits,
) -> WorthQueryPortablePackageReconstructionCandidate {
    let mut reconstruction =
        WorthQueryPortablePackageReconstruction::begin(manifest.clone(), limits).unwrap();
    for (index, record) in records.into_iter().enumerate() {
        reconstruction = reconstruction
            .push_record(u32::try_from(index).unwrap(), record)
            .unwrap();
    }
    reconstruction.close().unwrap()
}

pub(super) fn operation_fixture() -> crate::package::WorthQueryValidatedPortableDomainPackage {
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "portable-operation",
        1,
        0,
    ))
    .domain_operation(
        crate::conditional_application_operation_test_fixture::definition::<(), (), ()>()
            .into_portable(),
    )
    .validate()
    .unwrap()
}
