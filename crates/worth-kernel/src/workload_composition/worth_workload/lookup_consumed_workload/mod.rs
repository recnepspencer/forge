mod reuse_resolution_denial;

#[cfg(test)]
mod tests;

use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::evidence_lookup_index_product::{
    EvidenceLookupIndexProduct, EvidenceLookupIndexReuseResolution,
};
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanEventLedgerLookupExecutionPacket;
use worth_spatial::facade::spatial_compiled_product_consumer_cutover::{
    admit_lookup_product_handoff_match, SpatialLookupConsumerRouteDenialKind,
};
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

use super::{LookupConsumedWorkloadDenial, WorkloadCompositionError, WorthWorkload};
use crate::workload_composition::{
    admit_spatial_conflict_input,
    compiled_product_consumer_cutover::vertical_slice::lookup_consumed::resolve_lookup_reuse_for_handoff,
    AdmittedSpatialConflictInput, SpatialConflictInputRequest,
};

pub use reuse_resolution_denial::LookupConsumedWorkloadReuseResolutionDenied;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LookupConsumedWorkloadComposition {
    workload: WorthWorkload,
    handoff: EvidenceLookupConsumedWorkloadHandoff,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LookupConsumedWorkloadReuseProduct<'a> {
    Reused(&'a EvidenceLookupIndexProduct),
    Rebuilt(&'a EvidenceLookupIndexProduct),
}

impl LookupConsumedWorkloadComposition {
    pub(crate) fn admit(
        workload: &WorthWorkload,
        handoff: &EvidenceLookupConsumedWorkloadHandoff,
    ) -> Result<Self, WorkloadCompositionError> {
        let workload_stage_index_identity =
            workload.evidence_ledger().stage_index().index_identity();
        if workload_stage_index_identity != handoff.workload_stage_index_identity() {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::StageIndexIdentityMismatch,
            ));
        }
        if handoff.counters().raw_row_scan_count() != 0
            || handoff.counters().broad_receipt_scan_count() != 0
        {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::BroadEvidenceFallbackScan,
            ));
        }
        if handoff.counters().caller_owned_scan_count() != 0 {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::CallerOwnedLookupScan,
            ));
        }

        Ok(Self {
            workload: workload.clone(),
            handoff: handoff.clone(),
        })
    }

    pub fn workload(&self) -> &WorthWorkload {
        &self.workload
    }

    pub fn handoff(&self) -> &EvidenceLookupConsumedWorkloadHandoff {
        &self.handoff
    }

    pub fn admit_lookup_reuse_resolution<'a>(
        &self,
        resolution: &'a EvidenceLookupIndexReuseResolution,
    ) -> Result<LookupConsumedWorkloadReuseProduct<'a>, WorkloadCompositionError> {
        match resolution {
            EvidenceLookupIndexReuseResolution::Denied { denial, .. } => {
                Err(WorkloadCompositionError::LookupConsumedWorkload(
                    LookupConsumedWorkloadDenial::ReuseResolutionDenied(
                        LookupConsumedWorkloadReuseResolutionDenied::from_spatial_denial(denial),
                    ),
                ))
            }
            EvidenceLookupIndexReuseResolution::Reused { product, .. } => {
                self.require_resolution_product(product)?;
                Ok(LookupConsumedWorkloadReuseProduct::Reused(product))
            }
            EvidenceLookupIndexReuseResolution::Rebuilt { product, .. } => {
                self.require_resolution_product(product)?;
                Ok(LookupConsumedWorkloadReuseProduct::Rebuilt(product))
            }
        }
    }

    pub fn route_lookup_reuse_resolution(
        &self,
        selected_plan: &EvidenceLookupSelectedPlan,
        packet: &PlanarBooleanEventLedgerLookupExecutionPacket,
        prior_product: &EvidenceLookupIndexProduct,
    ) -> Result<EvidenceLookupIndexReuseResolution, WorkloadCompositionError> {
        resolve_lookup_reuse_for_handoff(self.handoff(), selected_plan, packet, prior_product)
    }

    pub fn admit_spatial_conflict_input<'a>(
        &'a self,
        authority: &'a SpatialGeometryEvidenceTouchAuthority,
        execution_receipt: &'a EvidenceLookupExecutionReceipt,
    ) -> Result<AdmittedSpatialConflictInput<'a>, WorkloadCompositionError> {
        admit_spatial_conflict_input(
            SpatialConflictInputRequest::new(authority)
                .with_evidence_lookup(self.handoff(), execution_receipt),
        )
    }

    fn require_resolution_product(
        &self,
        product: &EvidenceLookupIndexProduct,
    ) -> Result<(), WorkloadCompositionError> {
        admit_lookup_product_handoff_match(&self.handoff, product).map_err(|denial| {
            let mapped = match denial.kind() {
                SpatialLookupConsumerRouteDenialKind::SelectedPlanMismatch => {
                    LookupConsumedWorkloadDenial::ReuseResolutionSelectedPlanMismatch
                }
                SpatialLookupConsumerRouteDenialKind::SelectedEquivalenceFamilyMismatch => {
                    LookupConsumedWorkloadDenial::ReuseResolutionSelectedFamilyMismatch
                }
                SpatialLookupConsumerRouteDenialKind::SelectedReuseBasisMismatch => {
                    LookupConsumedWorkloadDenial::ReuseResolutionSelectedReuseBasisMismatch
                }
                SpatialLookupConsumerRouteDenialKind::LookupExecutionReceiptMismatch => {
                    LookupConsumedWorkloadDenial::CutoverProof(denial.detail().to_string())
                }
            };
            WorkloadCompositionError::LookupConsumedWorkload(mapped)
        })
    }
}

impl WorthWorkload {
    pub(crate) fn admit_lookup_consumed_workload(
        &self,
        handoff: &EvidenceLookupConsumedWorkloadHandoff,
    ) -> Result<LookupConsumedWorkloadComposition, WorkloadCompositionError> {
        LookupConsumedWorkloadComposition::admit(self, handoff)
    }
}
