use super::runtime::{LayoutRuntimeCoverageMatrix, LayoutRuntimeEvidence};
use forge_store_budgets::S8PreExecutionPlanBinding;
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalSegmentId, PlatformPhysicalAppendRequest, PlatformPhysicalFacade,
    PlatformPhysicalLayoutAccessRequest, PlatformPhysicalOpenRequest,
    PlatformPhysicalRuntimeReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRuntimeExecutionDenial {
    PhysicalFacade,
    MatrixRecord,
}

pub fn execute_layout_runtime_observations(
) -> Result<LayoutRuntimeCoverageMatrix, LayoutRuntimeExecutionDenial> {
    let mut matrix = LayoutRuntimeCoverageMatrix::default();
    matrix
        .record(execute_core_physical_append()?)
        .map_err(|_| LayoutRuntimeExecutionDenial::MatrixRecord)?;
    matrix
        .record(execute_core_physical_hidden_scan_denial()?)
        .map_err(|_| LayoutRuntimeExecutionDenial::MatrixRecord)?;
    Ok(matrix)
}

pub fn execute_core_physical_append() -> Result<LayoutRuntimeEvidence, LayoutRuntimeExecutionDenial>
{
    let mut facade = open_core_physical_facade()?;
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(1)
                .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)?,
            PhysicalPageId::from_raw(1)
                .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)?,
            PhysicalRecordSlot::from_raw(1)
                .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)?,
        )
        .with_slot_generation(
            PhysicalGeneration::from_raw(5)
                .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)?,
        );
    let report = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            cell,
            b"layout-runtime-append",
        ))
        .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)?;
    Ok(LayoutRuntimeEvidence::PlatformPhysical(
        PlatformPhysicalRuntimeReceipt::from_append(report),
    ))
}

pub fn execute_core_physical_hidden_scan_denial(
) -> Result<LayoutRuntimeEvidence, LayoutRuntimeExecutionDenial> {
    let mut facade = open_core_physical_facade()?;
    let receipt =
        facade.reject_hidden_broad_scan(PlatformPhysicalLayoutAccessRequest::hidden_broad_scan(
            S8PreExecutionPlanBinding::new(34, 1, 1, 1, 0),
        ));
    let runtime = PlatformPhysicalRuntimeReceipt::from_hidden_scan_denial(receipt)
        .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)?;
    Ok(LayoutRuntimeEvidence::PlatformPhysical(runtime))
}

fn open_core_physical_facade() -> Result<PlatformPhysicalFacade, LayoutRuntimeExecutionDenial> {
    let digest = StableDigest::new("sha256:layout-runtime")
        .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)?;
    let readiness = AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
        ROADMAP_2_S1_SCOPE,
        HandoffEvidenceDigestSet::new(
            digest.clone(),
            digest.clone(),
            digest.clone(),
            digest.clone(),
            digest.clone(),
            digest.clone(),
            digest,
        ),
    )
    .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)?;
    PlatformPhysicalFacade::open_physical_format(readiness, PlatformPhysicalOpenRequest::physical_format_canonical())
        .map_err(|_| LayoutRuntimeExecutionDenial::PhysicalFacade)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout_harness::runtime::{LayoutRuntimeFamily, LayoutRuntimeObligation};

    #[test]
    fn matrix_records_only_observed_owner_operations() {
        let matrix = execute_layout_runtime_observations().unwrap();
        assert_eq!(matrix.rows().len(), 2);
        assert!(matrix.is_executed(
            LayoutRuntimeFamily::CorePhysical,
            LayoutRuntimeObligation::Success
        ));
        assert!(matrix.is_executed(
            LayoutRuntimeFamily::CorePhysical,
            LayoutRuntimeObligation::HiddenScanDenial
        ));
        assert!(!matrix.is_executed(LayoutRuntimeFamily::Blob, LayoutRuntimeObligation::Success));
    }
}
