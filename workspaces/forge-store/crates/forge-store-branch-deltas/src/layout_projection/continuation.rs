use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_live_query::{
    AdmittedNarrowBatchReceipt, BroadenedBatchReceipt, ContinuationRetentionStatus,
    CursorContinuationPlan, StableBasisId,
};

use super::{BranchDeltaLayoutAccessDenial, BranchDeltaLayoutAccessDenialKind};

pub(crate) fn admit_continuation_layout_support(
    plan: &CursorContinuationPlan,
) -> Result<ContinuationLayoutReport, BranchDeltaLayoutAccessDenial> {
    Ok(ContinuationLayoutReport::from_admitted_support(
        ContinuationSupportPlan::from_admitted(plan),
    ))
}

pub(crate) fn reject_broadened_continuation_receipt(
    receipt: &BroadenedBatchReceipt,
) -> Result<(), BranchDeltaLayoutAccessDenial> {
    let _ = receipt;
    Err(BranchDeltaLayoutAccessDenial::new(
        BranchDeltaLayoutAccessDenialKind::BroadenedContinuationCannotStandInForBoundedSupport,
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationLayoutReport {
    family_id: DurableArtifactFamilyId,
    stable_basis_id: StableBasisId,
    declared_window_rows: u32,
    retention_status: ContinuationRetentionStatus,
    support_estimate: ContinuationLayoutSupportEstimate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContinuationSupportPlan {
    family_id: DurableArtifactFamilyId,
    stable_basis_id: StableBasisId,
    declared_window_rows: u32,
    retention_status: ContinuationRetentionStatus,
    support_estimate: ContinuationLayoutSupportEstimate,
}

impl ContinuationSupportPlan {
    fn from_admitted(plan: &CursorContinuationPlan) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::SupportCursor,
            stable_basis_id: plan.stable_basis_id(),
            declared_window_rows: plan.declared_window_rows(),
            retention_status: plan.retention_status(),
            support_estimate: ContinuationLayoutSupportEstimate::from_declared_rows(
                plan.declared_window_rows(),
            ),
        }
    }
}

impl ContinuationLayoutReport {
    fn from_admitted_support(support: ContinuationSupportPlan) -> Self {
        Self {
            family_id: support.family_id,
            stable_basis_id: support.stable_basis_id,
            declared_window_rows: support.declared_window_rows,
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

    pub const fn declared_window_rows(&self) -> u32 {
        self.declared_window_rows
    }

    pub const fn retention_status(&self) -> ContinuationRetentionStatus {
        self.retention_status
    }

    pub fn resume_bounded_continuation(
        &self,
        admitted_window: &AdmittedNarrowBatchReceipt,
    ) -> Result<(), BranchDeltaLayoutAccessDenial> {
        if self.retention_status() == ContinuationRetentionStatus::RetentionRebindRequired {
            return Err(BranchDeltaLayoutAccessDenial::new(
                BranchDeltaLayoutAccessDenialKind::ContinuationRebindRequired,
            ));
        }
        if self.stable_basis_id() != admitted_window.stable_basis_id()
            || admitted_window.admitted_window_rows() > self.declared_window_rows()
        {
            return Err(BranchDeltaLayoutAccessDenial::new(
                BranchDeltaLayoutAccessDenialKind::ContinuationWindowOutOfRange,
            ));
        }
        Ok(())
    }

    pub const fn support_estimate(&self) -> ContinuationLayoutSupportEstimate {
        self.support_estimate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContinuationLayoutSupportEstimate {
    planned_range_lookups: u16,
    planned_maintenance_reads: u16,
    planned_range_steps: u16,
}

impl ContinuationLayoutSupportEstimate {
    const fn from_declared_rows(declared_rows: u32) -> Self {
        Self {
            planned_range_lookups: 1,
            planned_maintenance_reads: 1,
            planned_range_steps: saturating_u16(declared_rows),
        }
    }

    pub const fn planned_range_lookups(self) -> u16 {
        self.planned_range_lookups
    }

    pub const fn planned_maintenance_reads(self) -> u16 {
        self.planned_maintenance_reads
    }

    pub const fn planned_range_steps(self) -> u16 {
        self.planned_range_steps
    }
}

const fn saturating_u16(value: u32) -> u16 {
    if value > u16::MAX as u32 {
        u16::MAX
    } else {
        value as u16
    }
}
