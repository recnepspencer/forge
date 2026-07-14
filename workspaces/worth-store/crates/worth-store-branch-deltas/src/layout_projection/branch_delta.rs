use worth_store_contracts::DurableArtifactFamilyId;

use crate::{BranchDeltaLayerId, BranchDeltaReadPlan, SameBranchDescendantWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchDeltaLayoutAccessDenialKind {
    BranchDeltaPlanCannotStandInForLayoutAuthority,
    BranchDeltaLineageDoesNotMatchWitness,
    StableBasisDescriptorCannotStandInForLayoutAuthority,
    BroadenedContinuationCannotStandInForBoundedSupport,
    ContinuationWindowOutOfRange,
    ContinuationRebindRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaLayoutAccessDenial {
    kind: BranchDeltaLayoutAccessDenialKind,
}

impl BranchDeltaLayoutAccessDenial {
    pub(crate) const fn new(kind: BranchDeltaLayoutAccessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> BranchDeltaLayoutAccessDenialKind {
        self.kind
    }
}

pub(crate) fn admit_branch_delta_layout(
    plan: &BranchDeltaReadPlan,
    witness: &SameBranchDescendantWitness,
) -> Result<BranchDeltaLayoutReport, BranchDeltaLayoutAccessDenial> {
    if plan.request().branch_lineage() != witness.branch_lineage() {
        return Err(BranchDeltaLayoutAccessDenial::new(
            BranchDeltaLayoutAccessDenialKind::BranchDeltaLineageDoesNotMatchWitness,
        ));
    }
    Ok(BranchDeltaLayoutReport::from_admitted_support(
        BranchDeltaLayerSupportPlan::from_admitted(plan),
    ))
}

pub(crate) fn reject_branch_delta_read_plan(
    plan: &BranchDeltaReadPlan,
) -> Result<(), BranchDeltaLayoutAccessDenial> {
    let _ = plan;
    Err(BranchDeltaLayoutAccessDenial::new(
        BranchDeltaLayoutAccessDenialKind::BranchDeltaPlanCannotStandInForLayoutAuthority,
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaLayoutReport {
    family_id: DurableArtifactFamilyId,
    layer_id: BranchDeltaLayerId,
    branch_lineage: String,
    declared_delta_rows: u32,
    support_estimate: BranchDeltaLayoutSupportEstimate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BranchDeltaLayerSupportPlan {
    family_id: DurableArtifactFamilyId,
    layer_id: BranchDeltaLayerId,
    branch_lineage: String,
    declared_delta_rows: u32,
    support_estimate: BranchDeltaLayoutSupportEstimate,
}

impl BranchDeltaLayerSupportPlan {
    fn from_admitted(plan: &BranchDeltaReadPlan) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::BranchDeltaArtifact,
            layer_id: plan.request().layer_id(),
            branch_lineage: plan.request().branch_lineage().to_owned(),
            declared_delta_rows: plan.declared_delta_rows(),
            support_estimate: BranchDeltaLayoutSupportEstimate::from_declared_rows(
                plan.declared_delta_rows(),
            ),
        }
    }
}

impl BranchDeltaLayoutReport {
    fn from_admitted_support(support: BranchDeltaLayerSupportPlan) -> Self {
        Self {
            family_id: support.family_id,
            layer_id: support.layer_id,
            branch_lineage: support.branch_lineage,
            declared_delta_rows: support.declared_delta_rows,
            support_estimate: support.support_estimate,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn layer_id(&self) -> BranchDeltaLayerId {
        self.layer_id
    }

    pub fn branch_lineage(&self) -> &str {
        &self.branch_lineage
    }

    pub const fn declared_delta_rows(&self) -> u32 {
        self.declared_delta_rows
    }

    pub const fn support_estimate(&self) -> BranchDeltaLayoutSupportEstimate {
        self.support_estimate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchDeltaLayoutSupportEstimate {
    planned_range_lookups: u16,
    planned_maintenance_reads: u16,
    planned_range_steps: u16,
}

impl BranchDeltaLayoutSupportEstimate {
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
