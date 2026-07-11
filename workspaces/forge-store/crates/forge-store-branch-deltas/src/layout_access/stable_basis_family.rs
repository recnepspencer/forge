use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase23_stable_basis_rule, AdmittedStableBasisLayoutRule,
};
use forge_store_live_query::{ContinuationRetentionStatus, StableBasisId, StableBasisReadPlan};

use super::{
    BranchDeltaLayoutAccessDenial, BranchDeltaLayoutAccessDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmittedStableBasisLayoutFamily {
    _rule: AdmittedStableBasisLayoutRule,
}

impl AdmittedStableBasisLayoutFamily {
    pub(crate) const fn new(rule: AdmittedStableBasisLayoutRule) -> Self {
        Self { _rule: rule }
    }

    pub fn admit_stable_basis_support(
        &self,
        plan: &StableBasisReadPlan,
    ) -> StableBasisLayoutReport {
        StableBasisLayoutReport::from_admitted_support(
            StableBasisSupportPlan::from_admitted(plan),
        )
    }

    pub fn reject_stable_basis_descriptor(
        &self,
        _stable_basis_id: StableBasisId,
    ) -> Result<(), BranchDeltaLayoutAccessDenial> {
        Err(BranchDeltaLayoutAccessDenial::new(
            BranchDeltaLayoutAccessDenialKind::StableBasisDescriptorCannotStandInForLayoutAuthority,
        ))
    }
}

pub(crate) fn admit_stable_basis_layout_support(
    plan: &StableBasisReadPlan,
) -> Result<StableBasisLayoutReport, BranchDeltaLayoutAccessDenial> {
    Ok(
        AdmittedStableBasisLayoutFamily::new(
            phase23_stable_basis_rule().expect("phase 23 stable-basis rule must stay admitted"),
        )
        .admit_stable_basis_support(plan),
    )
}

pub(crate) fn reject_stable_basis_layout_descriptor(
    stable_basis_id: StableBasisId,
) -> Result<(), BranchDeltaLayoutAccessDenial> {
    AdmittedStableBasisLayoutFamily::new(
        phase23_stable_basis_rule().expect("phase 23 stable-basis rule must stay admitted"),
    )
    .reject_stable_basis_descriptor(stable_basis_id)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableBasisLayoutReport {
    family_id: DurableArtifactFamilyId,
    stable_basis_id: StableBasisId,
    declared_support_rows: u32,
    retention_status: ContinuationRetentionStatus,
    support_estimate: StableBasisLayoutSupportEstimate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StableBasisSupportPlan {
    family_id: DurableArtifactFamilyId,
    stable_basis_id: StableBasisId,
    declared_support_rows: u32,
    retention_status: ContinuationRetentionStatus,
    support_estimate: StableBasisLayoutSupportEstimate,
}

impl StableBasisSupportPlan {
    fn from_admitted(plan: &StableBasisReadPlan) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::PlacementStableBasis,
            stable_basis_id: plan.stable_basis_id(),
            declared_support_rows: plan.declared_support_rows(),
            retention_status: plan.retention_status(),
            support_estimate: StableBasisLayoutSupportEstimate::from_declared_rows(
                plan.declared_support_rows(),
            ),
        }
    }
}

impl StableBasisLayoutReport {
    fn from_admitted_support(support: StableBasisSupportPlan) -> Self {
        Self {
            family_id: support.family_id,
            stable_basis_id: support.stable_basis_id,
            declared_support_rows: support.declared_support_rows,
            retention_status: support.retention_status,
            support_estimate: support.support_estimate,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn stable_basis_id(&self) -> StableBasisId {
        self.stable_basis_id
    }

    pub const fn declared_support_rows(&self) -> u32 {
        self.declared_support_rows
    }

    pub const fn retention_status(&self) -> ContinuationRetentionStatus {
        self.retention_status
    }

    pub const fn support_estimate(&self) -> StableBasisLayoutSupportEstimate {
        self.support_estimate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StableBasisLayoutSupportEstimate {
    planned_point_lookups: u16,
    planned_maintenance_reads: u16,
    planned_page_touches: u16,
}

impl StableBasisLayoutSupportEstimate {
    const fn from_declared_rows(declared_rows: u32) -> Self {
        Self {
            planned_point_lookups: 1,
            planned_maintenance_reads: 1,
            planned_page_touches: saturating_u16(declared_rows),
        }
    }

    pub const fn planned_point_lookups(self) -> u16 { self.planned_point_lookups }

    pub const fn planned_maintenance_reads(self) -> u16 { self.planned_maintenance_reads }

    pub const fn planned_page_touches(self) -> u16 { self.planned_page_touches }
}

const fn saturating_u16(value: u32) -> u16 {
    if value > u16::MAX as u32 { u16::MAX } else { value as u16 }
}
