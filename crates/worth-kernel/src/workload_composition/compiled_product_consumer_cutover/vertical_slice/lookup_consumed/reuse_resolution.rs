use worth_spatial::facade::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use worth_spatial::facade::evidence_lookup_index_product::EvidenceLookupIndexReuseResolution;
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanEventLedgerLookupExecutionPacket;
use worth_spatial::facade::spatial_compiled_product_consumer_cutover::{
    admit_lookup_product_handoff_match, reuse_evidence_lookup_index_product,
    SpatialLookupConsumerRouteDenialKind,
};

use super::admitted_slice::LookupConsumedVerticalSlice;
use crate::workload_composition::{
    LookupConsumedWorkloadDenial, LookupConsumedWorkloadReuseResolutionDenied,
    WorkloadCompositionError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LookupConsumedVerticalSliceReuseProduct {
    Reused(EvidenceLookupIndexProduct),
    Rebuilt(EvidenceLookupIndexProduct),
}

#[derive(Clone, Debug)]
pub(crate) struct ResolvedLookupConsumedVerticalSlice<'a> {
    slice: &'a LookupConsumedVerticalSlice,
    reuse_product: LookupConsumedVerticalSliceReuseProduct,
}

impl LookupConsumedVerticalSlice {
    pub(crate) fn resolve_prior_lookup_product<'a>(
        &'a self,
        prior_product: &'a EvidenceLookupIndexProduct,
    ) -> Result<ResolvedLookupConsumedVerticalSlice<'a>, WorkloadCompositionError> {
        let resolution = reuse_evidence_lookup_index_product(
            self.boundary().selected_plan(),
            self.boundary().selected_lookup_slice(),
            prior_product,
        )
        .map_err(|error| {
            WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::CutoverProof(format!(
                    "phase 10 lookup-consumed vertical slice could not lower reuse resolution: {:?}",
                    error.kind()
                )),
            )
        })?;
        let reuse_product = match resolution {
            worth_spatial::facade::evidence_lookup_index_product::EvidenceLookupIndexReuseResolution::Denied { denial, .. } => {
                return Err(WorkloadCompositionError::LookupConsumedWorkload(
                    LookupConsumedWorkloadDenial::ReuseResolutionDenied(
                        LookupConsumedWorkloadReuseResolutionDenied::from_spatial_denial(&denial),
                    ),
                ));
            }
            worth_spatial::facade::evidence_lookup_index_product::EvidenceLookupIndexReuseResolution::Reused { product, .. } => {
                self.require_resolution_product(&product)?;
                LookupConsumedVerticalSliceReuseProduct::Reused(product)
            }
            worth_spatial::facade::evidence_lookup_index_product::EvidenceLookupIndexReuseResolution::Rebuilt { product, .. } => {
                self.require_resolution_product(&product)?;
                LookupConsumedVerticalSliceReuseProduct::Rebuilt(product)
            }
        };
        Ok(ResolvedLookupConsumedVerticalSlice {
            slice: self,
            reuse_product,
        })
    }

    fn require_resolution_product(
        &self,
        product: &EvidenceLookupIndexProduct,
    ) -> Result<(), WorkloadCompositionError> {
        admit_lookup_product_handoff_match(self.boundary().workload_handoff(), product).map_err(
            |denial| {
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
            },
        )
    }
}

pub(crate) fn resolve_lookup_reuse_for_handoff(
    handoff: &EvidenceLookupConsumedWorkloadHandoff,
    selected_plan: &EvidenceLookupSelectedPlan,
    packet: &PlanarBooleanEventLedgerLookupExecutionPacket,
    prior_product: &EvidenceLookupIndexProduct,
) -> Result<EvidenceLookupIndexReuseResolution, WorkloadCompositionError> {
    let resolution = reuse_evidence_lookup_index_product(
        selected_plan,
        packet.selected_lookup_slice(),
        prior_product,
    )
    .map_err(|error| {
        WorkloadCompositionError::LookupConsumedWorkload(
            LookupConsumedWorkloadDenial::CutoverProof(format!(
                "lookup-consumed workload could not lower reuse resolution: {:?}",
                error.kind()
            )),
        )
    })?;
    match &resolution {
        EvidenceLookupIndexReuseResolution::Reused { product, .. }
        | EvidenceLookupIndexReuseResolution::Rebuilt { product, .. } => {
            require_lookup_resolution_product(handoff, product)?;
        }
        EvidenceLookupIndexReuseResolution::Denied { .. } => {}
    }
    Ok(resolution)
}

fn require_lookup_resolution_product(
    handoff: &EvidenceLookupConsumedWorkloadHandoff,
    product: &EvidenceLookupIndexProduct,
) -> Result<(), WorkloadCompositionError> {
    admit_lookup_product_handoff_match(handoff, product).map_err(|denial| {
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

impl<'a> ResolvedLookupConsumedVerticalSlice<'a> {
    pub(crate) fn slice(&self) -> &'a LookupConsumedVerticalSlice {
        self.slice
    }

    pub(crate) fn reuse_product(&self) -> &LookupConsumedVerticalSliceReuseProduct {
        &self.reuse_product
    }
}
