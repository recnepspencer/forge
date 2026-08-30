//! Closed descriptive candidate produced by structurally valid record intake.

use crate::package::{
    WorthQueryPortablePackageManifest, WorthQueryPortablePackageReconstructionLimits,
    WorthQueryPortablePackageReconstructionWork, WorthQueryPortablePackageRecord,
    WorthQueryPortablePackageRecordView,
};

/// Unvalidated package-record candidate.
///
/// The candidate carries claimed identity and typed descriptive records only.
/// It exposes no validated-package, installation, or runtime authority.
///
/// ```compile_fail
/// use worth_query_installation::facade::WorthQueryPortablePackageReconstructionCandidate;
///
/// fn cannot_install_structural_candidate(
///     candidate: WorthQueryPortablePackageReconstructionCandidate,
/// ) {
///     let _installed = candidate.install();
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortablePackageReconstructionCandidate {
    manifest: WorthQueryPortablePackageManifest,
    records: Vec<WorthQueryPortablePackageRecord>,
    limits: WorthQueryPortablePackageReconstructionLimits,
    work: WorthQueryPortablePackageReconstructionWork,
}

impl WorthQueryPortablePackageReconstructionCandidate {
    pub(super) const fn new(
        manifest: WorthQueryPortablePackageManifest,
        records: Vec<WorthQueryPortablePackageRecord>,
        limits: WorthQueryPortablePackageReconstructionLimits,
        work: WorthQueryPortablePackageReconstructionWork,
    ) -> Self {
        Self {
            manifest,
            records,
            limits,
            work,
        }
    }

    pub const fn manifest(&self) -> &WorthQueryPortablePackageManifest {
        &self.manifest
    }

    pub fn records(&self) -> &[WorthQueryPortablePackageRecord] {
        &self.records
    }

    pub const fn work(&self) -> WorthQueryPortablePackageReconstructionWork {
        self.work
    }

    pub fn views(&self) -> impl ExactSizeIterator<Item = WorthQueryPortablePackageRecordView<'_>> {
        self.records.iter().enumerate().map(|(index, record)| {
            WorthQueryPortablePackageRecordView::new(
                u32::try_from(index).expect("manifest already bounded record count"),
                record,
            )
        })
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryPortablePackageManifest,
        Vec<WorthQueryPortablePackageRecord>,
        WorthQueryPortablePackageReconstructionLimits,
        WorthQueryPortablePackageReconstructionWork,
    ) {
        (self.manifest, self.records, self.limits, self.work)
    }
}
