use crate::performance::basis::FoundationalPerformanceBundle;
use crate::performance::claims::FoundationalPerformanceClaimSurface;
use crate::performance::policy::FoundationalPolicyAdmissionReceipt;
use crate::performance::receipts::FoundationalCounterBackedPerformanceReceipt;

use super::targets::FoundationalPerformanceAttachmentTargetKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalPerformanceAttachmentDenial {
    ClaimBundlesCannotAttachToBoundaryReceipts,
    ClaimBundlesCannotAttachToCertificationBundles,
    CounterBackedReceiptsCannotAttachToCertificationBundles,
    PolicyReceiptsCannotAttachToBoundarySummaries,
    PolicyReceiptsCannotAttachToCertificationBundles,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalAttachedPerformanceBundle<Claim> {
    target: FoundationalPerformanceAttachmentTargetKind,
    bundle: FoundationalPerformanceBundle<Claim>,
}

impl<Claim> FoundationalAttachedPerformanceBundle<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    pub fn new(
        target: FoundationalPerformanceAttachmentTargetKind,
        bundle: FoundationalPerformanceBundle<Claim>,
    ) -> Result<Self, FoundationalPerformanceAttachmentDenial> {
        match target {
            FoundationalPerformanceAttachmentTargetKind::BoundaryReceipt => {
                Err(FoundationalPerformanceAttachmentDenial::ClaimBundlesCannotAttachToBoundaryReceipts)
            }
            FoundationalPerformanceAttachmentTargetKind::CertificationBundle => Err(
                FoundationalPerformanceAttachmentDenial::ClaimBundlesCannotAttachToCertificationBundles,
            ),
            _ => Ok(Self { target, bundle }),
        }
    }

    pub const fn target(&self) -> FoundationalPerformanceAttachmentTargetKind {
        self.target
    }

    pub const fn bundle(&self) -> &FoundationalPerformanceBundle<Claim> {
        &self.bundle
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalAttachedPolicyAdmissionReceipt {
    target: FoundationalPerformanceAttachmentTargetKind,
    receipt: FoundationalPolicyAdmissionReceipt,
}

impl FoundationalAttachedPolicyAdmissionReceipt {
    pub fn new(
        target: FoundationalPerformanceAttachmentTargetKind,
        receipt: FoundationalPolicyAdmissionReceipt,
    ) -> Result<Self, FoundationalPerformanceAttachmentDenial> {
        match target {
            FoundationalPerformanceAttachmentTargetKind::BoundarySummary => Err(
                FoundationalPerformanceAttachmentDenial::PolicyReceiptsCannotAttachToBoundarySummaries,
            ),
            FoundationalPerformanceAttachmentTargetKind::CertificationBundle => Err(
                FoundationalPerformanceAttachmentDenial::PolicyReceiptsCannotAttachToCertificationBundles,
            ),
            _ => Ok(Self { target, receipt }),
        }
    }

    pub const fn target(&self) -> FoundationalPerformanceAttachmentTargetKind {
        self.target
    }

    pub const fn receipt(&self) -> &FoundationalPolicyAdmissionReceipt {
        &self.receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalAttachedCounterBackedPerformanceReceipt<Claim> {
    target: FoundationalPerformanceAttachmentTargetKind,
    receipt: FoundationalCounterBackedPerformanceReceipt<Claim>,
}

impl<Claim> FoundationalAttachedCounterBackedPerformanceReceipt<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    pub fn new(
        target: FoundationalPerformanceAttachmentTargetKind,
        receipt: FoundationalCounterBackedPerformanceReceipt<Claim>,
    ) -> Result<Self, FoundationalPerformanceAttachmentDenial> {
        match target {
            FoundationalPerformanceAttachmentTargetKind::CertificationBundle => Err(
                FoundationalPerformanceAttachmentDenial::CounterBackedReceiptsCannotAttachToCertificationBundles,
            ),
            _ => Ok(Self { target, receipt }),
        }
    }

    pub const fn target(&self) -> FoundationalPerformanceAttachmentTargetKind {
        self.target
    }

    pub const fn receipt(&self) -> &FoundationalCounterBackedPerformanceReceipt<Claim> {
        &self.receipt
    }
}

pub fn attach_performance_bundle<Claim>(
    target: FoundationalPerformanceAttachmentTargetKind,
    bundle: FoundationalPerformanceBundle<Claim>,
) -> Result<FoundationalAttachedPerformanceBundle<Claim>, FoundationalPerformanceAttachmentDenial>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    FoundationalAttachedPerformanceBundle::new(target, bundle)
}

pub fn attach_policy_admission_receipt(
    target: FoundationalPerformanceAttachmentTargetKind,
    receipt: FoundationalPolicyAdmissionReceipt,
) -> Result<FoundationalAttachedPolicyAdmissionReceipt, FoundationalPerformanceAttachmentDenial> {
    FoundationalAttachedPolicyAdmissionReceipt::new(target, receipt)
}

pub fn attach_counter_backed_performance_receipt<Claim>(
    target: FoundationalPerformanceAttachmentTargetKind,
    receipt: FoundationalCounterBackedPerformanceReceipt<Claim>,
) -> Result<
    FoundationalAttachedCounterBackedPerformanceReceipt<Claim>,
    FoundationalPerformanceAttachmentDenial,
>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    FoundationalAttachedCounterBackedPerformanceReceipt::new(target, receipt)
}
