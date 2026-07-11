use forge_store_layout_indexes::layout_strategy_admission::{
    phase23_snapshot_rule, AdmittedSnapshotLayoutRule,
};
use forge_store_contracts::DurableArtifactFamilyId;

use crate::{PublishedSnapshotHandle, SnapshotImageBundle, SnapshotReadRequest, SnapshotId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotLayoutAccessDenialKind {
    SnapshotBundleCannotStandInForLayoutAuthority,
    SnapshotHandleDoesNotMatchReadRequest,
    SnapshotReadBroadensBeyondPublishedImage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotLayoutAccessDenial {
    kind: SnapshotLayoutAccessDenialKind,
}

impl SnapshotLayoutAccessDenial {
    const fn new(kind: SnapshotLayoutAccessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> SnapshotLayoutAccessDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SnapshotLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SnapshotLayoutAdmission {
    _private: (),
}

impl SnapshotLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    fn admit(
        self,
        _rule: &AdmittedSnapshotLayoutRule,
    ) -> Result<SnapshotLayoutAdmission, SnapshotLayoutAccessDenial> {
        Ok(SnapshotLayoutAdmission { _private: () })
    }
}

fn snapshot_layout(
    rule: &AdmittedSnapshotLayoutRule,
) -> Result<AdmittedSnapshotLayoutFamily, SnapshotLayoutAccessDenial> {
    let admission = SnapshotLayoutFamilyHome::s8().admit(rule)?;
    Ok(AdmittedSnapshotLayoutFamily::new(admission))
}

pub(crate) fn admit_snapshot_image_support(
    handle: &PublishedSnapshotHandle,
    request: &SnapshotReadRequest,
) -> Result<SnapshotLayoutReport, SnapshotLayoutAccessDenial> {
    snapshot_layout(&phase23_snapshot_rule().expect("phase 23 snapshot rule must stay admitted"))?
        .admit_snapshot_image(handle, request)
}

pub(crate) fn reject_snapshot_bundle_layout_authority(
    bundle: &SnapshotImageBundle,
) -> Result<(), SnapshotLayoutAccessDenial> {
    snapshot_layout(&phase23_snapshot_rule().expect("phase 23 snapshot rule must stay admitted"))?
        .reject_snapshot_bundle(bundle)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedSnapshotLayoutFamily {
    _admission: SnapshotLayoutAdmission,
}

impl AdmittedSnapshotLayoutFamily {
    pub(crate) const fn new(admission: SnapshotLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn admit_snapshot_image(
        &self,
        handle: &PublishedSnapshotHandle,
        request: &SnapshotReadRequest,
    ) -> Result<SnapshotLayoutReport, SnapshotLayoutAccessDenial> {
        if handle.snapshot_id() != request.snapshot_id() {
            return Err(SnapshotLayoutAccessDenial::new(
                SnapshotLayoutAccessDenialKind::SnapshotHandleDoesNotMatchReadRequest,
            ));
        }
        if request.requested_page_count() > handle.declared_page_count() {
            return Err(SnapshotLayoutAccessDenial::new(
                SnapshotLayoutAccessDenialKind::SnapshotReadBroadensBeyondPublishedImage,
            ));
        }
        Ok(SnapshotLayoutReport::from_admitted_support(
            SnapshotImageSupportPlan::from_admitted(handle, request.requested_page_count()),
        ))
    }

    pub fn reject_snapshot_bundle(
        &self,
        _bundle: &SnapshotImageBundle,
    ) -> Result<(), SnapshotLayoutAccessDenial> {
        Err(SnapshotLayoutAccessDenial::new(
            SnapshotLayoutAccessDenialKind::SnapshotBundleCannotStandInForLayoutAuthority,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotLayoutReport {
    family_id: DurableArtifactFamilyId,
    snapshot_id: SnapshotId,
    image_digest: String,
    declared_page_count: u32,
    requested_page_count: u32,
    support_estimate: SnapshotLayoutSupportEstimate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnapshotImageSupportPlan {
    family_id: DurableArtifactFamilyId,
    snapshot_id: SnapshotId,
    image_digest: String,
    declared_page_count: u32,
    requested_page_count: u32,
    support_estimate: SnapshotLayoutSupportEstimate,
}

impl SnapshotImageSupportPlan {
    fn from_admitted(
        handle: &PublishedSnapshotHandle,
        requested_page_count: u32,
    ) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::PublicationSnapshotImage,
            snapshot_id: handle.snapshot_id().clone(),
            image_digest: handle.image_digest().to_owned(),
            declared_page_count: handle.declared_page_count(),
            requested_page_count,
            support_estimate: SnapshotLayoutSupportEstimate::from_declared_support(
                handle.declared_page_count(),
                requested_page_count,
            ),
        }
    }
}

impl SnapshotLayoutReport {
    fn from_admitted_support(support: SnapshotImageSupportPlan) -> Self {
        Self {
            family_id: support.family_id,
            snapshot_id: support.snapshot_id,
            image_digest: support.image_digest,
            declared_page_count: support.declared_page_count,
            requested_page_count: support.requested_page_count,
            support_estimate: support.support_estimate,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn snapshot_id(&self) -> &SnapshotId {
        &self.snapshot_id
    }

    pub fn image_digest(&self) -> &str {
        &self.image_digest
    }

    pub const fn declared_page_count(&self) -> u32 {
        self.declared_page_count
    }

    pub const fn requested_page_count(&self) -> u32 {
        self.requested_page_count
    }

    pub const fn support_estimate(&self) -> SnapshotLayoutSupportEstimate {
        self.support_estimate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotLayoutSupportEstimate {
    planned_page_touches: u16,
    planned_publications: u16,
    planned_maintenance_reads: u16,
}

impl SnapshotLayoutSupportEstimate {
    const fn from_declared_support(declared_page_count: u32, requested_page_count: u32) -> Self {
        Self {
            planned_page_touches: saturating_u16(requested_page_count),
            planned_publications: if declared_page_count == 0 { 0 } else { 1 },
            planned_maintenance_reads: 1,
        }
    }

    pub const fn planned_page_touches(self) -> u16 {
        self.planned_page_touches
    }

    pub const fn planned_publications(self) -> u16 {
        self.planned_publications
    }

    pub const fn planned_maintenance_reads(self) -> u16 {
        self.planned_maintenance_reads
    }
}

const fn saturating_u16(value: u32) -> u16 {
    if value > u16::MAX as u32 { u16::MAX } else { value as u16 }
}
