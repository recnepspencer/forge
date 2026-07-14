use crate::{failure::StoreError, ComplexityStatus};
use serde::{Deserialize, Serialize};

use super::super::constants::{
    FIRST_SHIP_MAX_ADMITTED_ASPECT_SLICES_PER_READ, FIRST_SHIP_MAX_ADMITTED_BLOCK_DECODE_BREADTH,
    FIRST_SHIP_MAX_ADMITTED_CONTROL_REPLAY_BREADTH_FOR_PARITY,
};
use super::{
    core::{AspectLayoutSliceId, EquivalenceContractVersion, StructuralBlockId},
    digests::{canonical_slice_ids, structural_block_id_for_plan},
    scopes::{AspectLayoutReadRequest, AspectReadRegime, AspectScopeClass},
};
use worth_relational::facade::history::{BranchId, CommitId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectLayoutFallbackClass {
    None,
    UnsupportedScopeClass,
    BudgetExceeded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectLayoutPerformanceEnvelope {
    pub strategy: AspectReadRegime,
    pub scope_class: String,
    pub complexity_status: ComplexityStatus,
    pub fallback_class: AspectLayoutFallbackClass,
    pub layout_slices_read: usize,
    pub blocks_decoded: usize,
    pub control_replay_breadth: usize,
    pub chunk_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmittedAspectLayoutReadPlan {
    request: AspectLayoutReadRequest,
    slice_ids: Vec<AspectLayoutSliceId>,
    structural_block_id: StructuralBlockId,
    performance: AspectLayoutPerformanceEnvelope,
}
impl AdmittedAspectLayoutReadPlan {
    pub(crate) fn new(
        request: AspectLayoutReadRequest,
        slice_ids: Vec<AspectLayoutSliceId>,
        structural_block_id: StructuralBlockId,
        performance: AspectLayoutPerformanceEnvelope,
    ) -> Self {
        Self {
            request,
            slice_ids,
            structural_block_id,
            performance,
        }
    }
    pub fn request(&self) -> &AspectLayoutReadRequest {
        &self.request
    }
    pub fn slice_ids(&self) -> &[AspectLayoutSliceId] {
        &self.slice_ids
    }
    pub fn structural_block_id(&self) -> &StructuralBlockId {
        &self.structural_block_id
    }
    pub fn performance(&self) -> &AspectLayoutPerformanceEnvelope {
        &self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExplicitBroadFallbackPlan {
    request: AspectLayoutReadRequest,
    performance: AspectLayoutPerformanceEnvelope,
    reason: String,
}
impl ExplicitBroadFallbackPlan {
    pub(crate) fn new(
        request: AspectLayoutReadRequest,
        performance: AspectLayoutPerformanceEnvelope,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            request,
            performance,
            reason: reason.into(),
        }
    }
    pub fn request(&self) -> &AspectLayoutReadRequest {
        &self.request
    }
    pub fn performance(&self) -> &AspectLayoutPerformanceEnvelope {
        &self.performance
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RejectedAspectLayoutReadPlan {
    request: AspectLayoutReadRequest,
    reason: String,
}
impl RejectedAspectLayoutReadPlan {
    pub(crate) fn new(request: AspectLayoutReadRequest, reason: impl Into<String>) -> Self {
        Self {
            request,
            reason: reason.into(),
        }
    }
    pub fn request(&self) -> &AspectLayoutReadRequest {
        &self.request
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum AspectLayoutReadPlanDecision {
    Admitted(AdmittedAspectLayoutReadPlan),
    Fallback(ExplicitBroadFallbackPlan),
    Rejected(RejectedAspectLayoutReadPlan),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DedupAdmittedBlockReuse {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
    scope_class: String,
    structural_block_id: StructuralBlockId,
    equivalence_contract_version: EquivalenceContractVersion,
    slice_ids: Vec<AspectLayoutSliceId>,
}
impl DedupAdmittedBlockReuse {
    pub(crate) fn new(
        plan: &AdmittedAspectLayoutReadPlan,
        equivalence_contract_version: EquivalenceContractVersion,
    ) -> Self {
        Self {
            branch_id: plan.request.target().branch_id().clone(),
            frontier_commit_id: plan.request.target().frontier_commit_id(),
            scope_class: plan.request.scope_class().label().to_string(),
            structural_block_id: plan.structural_block_id.clone(),
            equivalence_contract_version,
            slice_ids: plan.slice_ids.clone(),
        }
    }
    pub(crate) fn from_parts(
        branch_id: BranchId,
        frontier_commit_id: CommitId,
        scope_class: String,
        structural_block_id: StructuralBlockId,
        equivalence_contract_version: EquivalenceContractVersion,
        slice_ids: Vec<AspectLayoutSliceId>,
    ) -> Self {
        Self {
            branch_id,
            frontier_commit_id,
            scope_class,
            structural_block_id,
            equivalence_contract_version,
            slice_ids,
        }
    }
    pub fn structural_block_id(&self) -> &StructuralBlockId {
        &self.structural_block_id
    }
    pub(crate) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub(crate) fn frontier_commit_id(&self) -> CommitId {
        self.frontier_commit_id
    }
    pub(crate) fn scope_class(&self) -> &str {
        &self.scope_class
    }
    pub fn equivalence_contract_version(&self) -> EquivalenceContractVersion {
        self.equivalence_contract_version
    }
    pub fn slice_ids(&self) -> &[AspectLayoutSliceId] {
        &self.slice_ids
    }
}

pub(crate) fn classify_layout_request(
    request: AspectLayoutReadRequest,
) -> Result<AspectLayoutReadPlanDecision, StoreError> {
    let slice_ids = canonical_slice_ids(&request)?;
    match request.scope_class().clone() {
        AspectScopeClass::Generalized { descriptor } => Ok(AspectLayoutReadPlanDecision::Fallback(
            ExplicitBroadFallbackPlan::new(
                request,
                AspectLayoutPerformanceEnvelope {
                    strategy: AspectReadRegime::ExplicitBroadFallback,
                    scope_class: "generalized".to_string(),
                    complexity_status: ComplexityStatus::Debt,
                    fallback_class: AspectLayoutFallbackClass::UnsupportedScopeClass,
                    layout_slices_read: 0,
                    blocks_decoded: 0,
                    control_replay_breadth: slice_ids.len(),
                    chunk_count: 0,
                },
                format!("generalized aspect scope `{descriptor}` is not admitted for Milestone 6 Phase 1"),
            ),
        )),
        AspectScopeClass::SingleEntity(_) | AspectScopeClass::EntitySetUniform(_) | AspectScopeClass::CdcTouched(_) => {
            if slice_ids.is_empty() {
                return Ok(AspectLayoutReadPlanDecision::Rejected(RejectedAspectLayoutReadPlan::new(
                    request,
                    "aspect layout request does not identify any canonical slices",
                )));
            }
            if slice_ids.len() as u64 > FIRST_SHIP_MAX_ADMITTED_ASPECT_SLICES_PER_READ.value()
                || slice_ids.len() as u64 > FIRST_SHIP_MAX_ADMITTED_BLOCK_DECODE_BREADTH.value()
            {
                return Ok(AspectLayoutReadPlanDecision::Fallback(ExplicitBroadFallbackPlan::new(
                    request.clone(),
                    AspectLayoutPerformanceEnvelope {
                        strategy: AspectReadRegime::ExplicitBroadFallback,
                        scope_class: request.scope_class().label().to_string(),
                        complexity_status: ComplexityStatus::Debt,
                        fallback_class: AspectLayoutFallbackClass::BudgetExceeded,
                        layout_slices_read: slice_ids.len(),
                        blocks_decoded: slice_ids.len(),
                        control_replay_breadth: slice_ids.len(),
                        chunk_count: 0,
                    },
                    "aspect layout request exceeded the first-ship admitted local budget",
                )));
            }
            let structural_block_id = structural_block_id_for_plan(&request, &slice_ids)?;
            let regime = if slice_ids.len() == 1 { AspectReadRegime::DirectLayoutSlice } else { AspectReadRegime::BlockReuseBacked };
            Ok(AspectLayoutReadPlanDecision::Admitted(AdmittedAspectLayoutReadPlan::new(
                request.clone(),
                slice_ids.clone(),
                structural_block_id,
                AspectLayoutPerformanceEnvelope {
                    strategy: regime,
                    scope_class: request.scope_class().label().to_string(),
                    complexity_status: ComplexityStatus::Verified,
                    fallback_class: AspectLayoutFallbackClass::None,
                    layout_slices_read: slice_ids.len(),
                    blocks_decoded: slice_ids.len(),
                    control_replay_breadth: slice_ids.len().min(FIRST_SHIP_MAX_ADMITTED_CONTROL_REPLAY_BREADTH_FOR_PARITY.value() as usize),
                    chunk_count: 0,
                },
            )))
        }
    }
}
