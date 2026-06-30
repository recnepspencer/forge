use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupFamilyCatalogCloseout;
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupFamilyDeclaration;

use super::current_path::current_query_surface_witness;
use super::error::EvidenceLookupQuerySurfaceMatrixError;
use super::row::{EvidenceLookupQuerySurfaceMatrixRow, EvidenceLookupQuerySurfaceTouchpoint};

pub(super) fn current_query_surface_rows(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
) -> Result<Vec<EvidenceLookupQuerySurfaceMatrixRow>, EvidenceLookupQuerySurfaceMatrixError> {
    let mut rows = Vec::new();

    for family in catalog.declarations() {
        for stage in family.stage_applicability().stages().iter().copied() {
            let declaration_contract =
                crate::workload_platform::evidence_lookup_query_surface_contract::EvidenceLookupQuerySurfaceContract::from_family_query_posture(
                    family.query_posture(),
                );
            rows.push(
                EvidenceLookupQuerySurfaceMatrixRow::from_family_stage_touchpoint_contract(
                    family,
                    stage,
                    EvidenceLookupQuerySurfaceTouchpoint::FamilyCatalogQueryPosture,
                    declaration_contract.as_ref(),
                ),
            );

            if family.topology_input_posture().requires_topology_receipt() {
                rows.extend(fallback_rows_for_runtime_touchpoints(
                    family,
                    stage,
                    declaration_contract.as_ref(),
                ));
                continue;
            }

            let witness = current_query_surface_witness(catalog, family, stage)?;

            rows.push(
                EvidenceLookupQuerySurfaceMatrixRow::from_family_stage_touchpoint_contract(
                    family,
                    stage,
                    EvidenceLookupQuerySurfaceTouchpoint::InputAdmissionQuerySupport,
                    witness
                        .admitted_input()
                        .query_support()
                        .iter()
                        .find(|support| support.family_identity() == family.identity().as_str())
                        .and_then(|support| support.query_surface_contract()),
                ),
            );
            rows.push(
                EvidenceLookupQuerySurfaceMatrixRow::from_family_stage_touchpoint_contract(
                    family,
                    stage,
                    EvidenceLookupQuerySurfaceTouchpoint::PlanSelectionQueryPosture,
                    witness
                        .selected_plan()
                        .rows()
                        .iter()
                        .find(|row| row.family_identity() == family.identity().as_str())
                        .and_then(|row| row.query_surface_contract()),
                ),
            );
            rows.push(
                EvidenceLookupQuerySurfaceMatrixRow::from_family_stage_touchpoint_contract(
                    family,
                    stage,
                    EvidenceLookupQuerySurfaceTouchpoint::IndexProductQuerySupport,
                    witness
                        .index_product()
                        .query_surface_contract_for_family(family.identity().as_str()),
                ),
            );
            rows.push(
                EvidenceLookupQuerySurfaceMatrixRow::from_family_stage_touchpoint_contract(
                    family,
                    stage,
                    EvidenceLookupQuerySurfaceTouchpoint::ExecutionReceiptQuerySupport,
                    witness
                        .execution_receipt()
                        .query_surface_contract_for_family(family.identity().as_str()),
                ),
            );
            rows.push(
                EvidenceLookupQuerySurfaceMatrixRow::from_family_stage_touchpoint_contract(
                    family,
                    stage,
                    EvidenceLookupQuerySurfaceTouchpoint::DiagnosticWitnessContract,
                    witness
                        .diagnostics()
                        .require_family_stage_witness(family.identity().as_str(), stage)
                        .ok()
                        .and_then(|row| row.query_surface_contract()),
                ),
            );
            rows.push(
                EvidenceLookupQuerySurfaceMatrixRow::from_family_stage_touchpoint_contract(
                    family,
                    stage,
                    EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
                    witness
                        .execution_receipt()
                        .query_surface_contract_for_family(family.identity().as_str()),
                ),
            );
        }
    }

    Ok(rows)
}

fn fallback_rows_for_runtime_touchpoints(
    family: &EvidenceLookupFamilyDeclaration,
    stage: WorkloadEvidenceStage,
    contract: Option<
        &crate::workload_platform::evidence_lookup_query_surface_contract::EvidenceLookupQuerySurfaceContract,
    >,
) -> Vec<EvidenceLookupQuerySurfaceMatrixRow> {
    [
        EvidenceLookupQuerySurfaceTouchpoint::InputAdmissionQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::PlanSelectionQueryPosture,
        EvidenceLookupQuerySurfaceTouchpoint::IndexProductQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::ExecutionReceiptQuerySupport,
        EvidenceLookupQuerySurfaceTouchpoint::DiagnosticWitnessContract,
        EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
    ]
    .into_iter()
    .map(|touchpoint| {
        EvidenceLookupQuerySurfaceMatrixRow::from_family_stage_touchpoint_contract(
            family, stage, touchpoint, contract,
        )
    })
    .collect()
}
