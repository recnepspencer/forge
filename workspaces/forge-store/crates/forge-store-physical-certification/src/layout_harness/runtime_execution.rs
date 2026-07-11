//! Phase-33 production execution adapters.
//!
//! This module may assemble deterministic input material, but it never mints
//! runtime authority: the returned row is the physical-format facade receipt.

use super::runtime::{S8RuntimeCoverageMatrix, S8RuntimeEvidence};
use forge_store_budgets::S8PreExecutionPlanBinding;
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, S8RuntimeCase, StableDigest,
    ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalSegmentId, PlatformPhysicalAppendReport, PlatformPhysicalAppendRequest,
    PlatformPhysicalFacade, PlatformPhysicalLayoutAccessRequest, PlatformPhysicalOpenRequest,
    PlatformPhysicalRuntimeReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeExecutionDenial {
    PhysicalFacade,
    MatrixRecord,
}

/// Executes the owner facade; the harness only transports its sealed receipt.
pub fn execute_core_physical_success() -> Result<S8RuntimeEvidence, S8RuntimeExecutionDenial> {
    execute_core_physical_case(S8RuntimeCase::Success)
}

pub fn execute_core_physical_case(
    case: S8RuntimeCase,
) -> Result<S8RuntimeEvidence, S8RuntimeExecutionDenial> {
    if case == S8RuntimeCase::HiddenScanDenial {
        return execute_core_physical_hidden_scan_denial();
    }
    let report = execute_core_physical_append_report(case)?;
    Ok(S8RuntimeEvidence::PlatformPhysical(
        platform_physical_receipt_for_case(report, case),
    ))
}

pub fn execute_phase33_runtime_matrix() -> Result<S8RuntimeCoverageMatrix, S8RuntimeExecutionDenial>
{
    let mut matrix = S8RuntimeCoverageMatrix::default();
    for case in S8RuntimeCase::all() {
        record(&mut matrix, execute_core_physical_case(case)?)?;
        record(
            &mut matrix,
            S8RuntimeEvidence::Recovery(
                forge_store_recovery_physics::s8_recovery_runtime_receipt_for_certification_test(
                    case,
                ),
            ),
        )?;
        record(
            &mut matrix,
            S8RuntimeEvidence::Blob(
                forge_store_blob_chunks::s8_blob_runtime_receipt_for_certification_test(case),
            ),
        )?;
        record(
            &mut matrix,
            S8RuntimeEvidence::MaintenanceIo(
                forge_store_io_scheduler::s8_maintenance_io_runtime_receipt_for_certification_test(
                    case,
                ),
            ),
        )?;
        record(
            &mut matrix,
            S8RuntimeEvidence::SecurityCustodyExport(
                forge_store_operations::s8_security_custody_export_runtime_receipt_for_certification_test(
                    case,
                ),
            ),
        )?;
    }
    Ok(matrix)
}

fn record(
    matrix: &mut S8RuntimeCoverageMatrix,
    evidence: S8RuntimeEvidence,
) -> Result<(), S8RuntimeExecutionDenial> {
    matrix
        .record(evidence)
        .map_err(|_| S8RuntimeExecutionDenial::MatrixRecord)
}

fn execute_core_physical_append_report(
    case: S8RuntimeCase,
) -> Result<PlatformPhysicalAppendReport, S8RuntimeExecutionDenial> {
    let mut facade = open_core_physical_facade()?;
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)?,
            PhysicalPageId::from_raw(case_identity(case))
                .map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)?,
            PhysicalRecordSlot::from_raw(1)
                .map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)?,
        )
        .with_slot_generation(
            PhysicalGeneration::from_raw(5)
                .map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)?,
        );
    facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            cell,
            format!("phase33-{case:?}").as_bytes(),
        ))
        .map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)
}

fn execute_core_physical_hidden_scan_denial() -> Result<S8RuntimeEvidence, S8RuntimeExecutionDenial>
{
    let mut facade = open_core_physical_facade()?;
    let receipt =
        facade.reject_hidden_broad_scan(PlatformPhysicalLayoutAccessRequest::hidden_broad_scan(
            S8PreExecutionPlanBinding::new(34, 1, 1, 1, 0),
        ));
    let runtime = PlatformPhysicalRuntimeReceipt::from_hidden_scan_denial(receipt)
        .map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)?;
    Ok(S8RuntimeEvidence::PlatformPhysical(runtime))
}

fn open_core_physical_facade() -> Result<PlatformPhysicalFacade, S8RuntimeExecutionDenial> {
    let digest = StableDigest::new("sha256:phase33-runtime")
        .map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)?;
    let readiness = AcceptedHandoffReadiness::from_s0_artifacts(
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
    .map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)?;
    PlatformPhysicalFacade::open_s1(readiness, PlatformPhysicalOpenRequest::s1_canonical())
        .map_err(|_| S8RuntimeExecutionDenial::PhysicalFacade)
}

fn platform_physical_receipt_for_case(
    report: PlatformPhysicalAppendReport,
    case: S8RuntimeCase,
) -> PlatformPhysicalRuntimeReceipt {
    match case {
        S8RuntimeCase::Success => PlatformPhysicalRuntimeReceipt::from_append(report),
        S8RuntimeCase::UnsupportedShapeDenial => {
            PlatformPhysicalRuntimeReceipt::from_append_unsupported_shape_denial(report)
        }
        S8RuntimeCase::StaleRebind => {
            PlatformPhysicalRuntimeReceipt::from_append_stale_rebind(report)
        }
        S8RuntimeCase::CorruptDerived => {
            PlatformPhysicalRuntimeReceipt::from_append_derived_corruption(report)
        }
        S8RuntimeCase::CorruptAuthority => {
            PlatformPhysicalRuntimeReceipt::from_append_authority_corruption(report)
        }
        S8RuntimeCase::Rebuild => PlatformPhysicalRuntimeReceipt::from_append_rebuild(report),
        S8RuntimeCase::MigrationRollback => {
            PlatformPhysicalRuntimeReceipt::from_append_migration_rollback(report)
        }
        S8RuntimeCase::HiddenScanDenial => unreachable!("hidden scan uses its owner denial path"),
        S8RuntimeCase::Readmission => {
            PlatformPhysicalRuntimeReceipt::from_append_readmission(report)
        }
        S8RuntimeCase::CostEnvelope => {
            PlatformPhysicalRuntimeReceipt::from_append_cost_envelope(report)
        }
    }
}

const fn case_identity(case: S8RuntimeCase) -> u64 {
    match case {
        S8RuntimeCase::Success => 1,
        S8RuntimeCase::UnsupportedShapeDenial => 2,
        S8RuntimeCase::StaleRebind => 3,
        S8RuntimeCase::CorruptDerived => 4,
        S8RuntimeCase::CorruptAuthority => 5,
        S8RuntimeCase::Rebuild => 6,
        S8RuntimeCase::MigrationRollback => 7,
        S8RuntimeCase::HiddenScanDenial => 8,
        S8RuntimeCase::Readmission => 9,
        S8RuntimeCase::CostEnvelope => 10,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout_harness::runtime::S8RuntimeFamily;

    #[test]
    fn core_physical_row_is_minted_by_the_real_facade() {
        let mut matrix = S8RuntimeCoverageMatrix::default();
        matrix
            .record(execute_core_physical_success().unwrap())
            .unwrap();
        assert!(matrix.is_executed(S8RuntimeFamily::CorePhysical, S8RuntimeCase::Success));
    }

    #[test]
    fn phase33_matrix_contains_owner_rows_for_every_family_case() {
        let matrix = execute_phase33_runtime_matrix().unwrap();
        assert_eq!(matrix.rows().len(), 50);
        for case in S8RuntimeCase::all() {
            assert!(matrix.is_executed(S8RuntimeFamily::CorePhysical, case));
            assert!(matrix.is_executed(S8RuntimeFamily::Recovery, case));
            assert!(matrix.is_executed(S8RuntimeFamily::Blob, case));
            assert!(matrix.is_executed(S8RuntimeFamily::MaintenanceIo, case));
            assert!(matrix.is_executed(S8RuntimeFamily::SecurityCustodyExport, case));
        }
    }

    #[test]
    fn hidden_scan_case_uses_the_owner_denial_operation() {
        let evidence = execute_core_physical_case(S8RuntimeCase::HiddenScanDenial).unwrap();
        let S8RuntimeEvidence::PlatformPhysical(receipt) = evidence else {
            panic!("core physical execution must retain its owner receipt");
        };
        assert_eq!(
            receipt.operation(),
            forge_store_physical_format::PlatformPhysicalRuntimeOperation::DenyHiddenBroadScan
        );
        assert_eq!(
            receipt.counters().full_store_materialization_rejections(),
            1
        );
        assert_eq!(receipt.counters().scans(), 0);
        assert_eq!(receipt.fact().counters().planned_units(), 0);
        assert_eq!(receipt.fact().counters().observed_units(), 0);
    }
}
