use crate::workload_platform::evidence_ledger::spatial_evidence_surface_deletion_ledger;
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;
use crate::workload_platform::evidence_lookup_query_consumer_kit::current_evidence_lookup_query_consumer_kit;
use crate::workload_platform::evidence_lookup_query_surface_matrix::{
    current_evidence_lookup_query_surface_matrix, EvidenceLookupQuerySurfaceTouchpoint,
};
use crate::workload_platform::evidence_lookup_source_firewall::current_evidence_lookup_source_firewall_report;
use crate::workload_platform::evidence_lookup_stage_cutover::current_path::admit_current_family_stage_cutover_path;

use super::assembly_input::EvidenceLookupPublicCloseoutAssemblyInput;
use super::closeout_artifacts::{
    EvidenceLookupPublicCloseout, EvidenceLookupPublicCloseoutFamilyStageRow,
};
use super::error::{EvidenceLookupPublicCloseoutError, EvidenceLookupPublicCloseoutErrorKind};

pub fn current_evidence_lookup_public_closeout(
) -> Result<EvidenceLookupPublicCloseout, EvidenceLookupPublicCloseoutError> {
    let input = current_evidence_lookup_public_closeout_assembly_input()?;
    EvidenceLookupPublicCloseout::assemble_from_proof_products(&input)
}

pub fn current_evidence_lookup_public_closeout_assembly_input(
) -> Result<EvidenceLookupPublicCloseoutAssemblyInput, EvidenceLookupPublicCloseoutError> {
    let catalog = current_evidence_lookup_family_catalog().map_err(|error| {
        EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
            format!("family catalog failed: {:?}", error.kind()),
        )
    })?;
    let query_surface_matrix = current_evidence_lookup_query_surface_matrix().map_err(|error| {
        EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MissingPublicCloseoutQueryRow,
            format!("query surface matrix failed: {:?}", error.kind()),
        )
    })?;
    let query_consumer_kit = current_evidence_lookup_query_consumer_kit().map_err(|error| {
        EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MissingPublicCloseoutQueryRow,
            format!("consumer kit failed: {:?}", error.kind()),
        )
    })?;
    let source_firewall_report =
        current_evidence_lookup_source_firewall_report().map_err(|error| {
            EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::ForbiddenSourceFirewallAuthority,
                format!("source firewall failed: {:?}", error.kind()),
            )
        })?;
    let mut family_stage_rows = Vec::new();
    for family in catalog.declarations() {
        for stage in family.stage_applicability().stages().iter().copied() {
            let query_row = query_surface_matrix
                .require_family_stage_touchpoint_row(
                    family.identity().as_str(),
                    stage,
                    EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
                )
                .map_err(|_| {
                    EvidenceLookupPublicCloseoutError::new(
                        EvidenceLookupPublicCloseoutErrorKind::MissingPublicCloseoutQueryRow,
                        format!(
                            "missing public-closeout query matrix row for family `{}` at stage `{}`",
                            family.identity().as_str(),
                            stage.human_name()
                        ),
                    )
                })?;

            if family.topology_input_posture().requires_topology_receipt() {
                family_stage_rows.push(
                    EvidenceLookupPublicCloseoutFamilyStageRow::blocked_by_topology_seed(
                        family,
                        stage,
                        query_row.row_digest(),
                    ),
                );
                continue;
            }

            let path = admit_current_family_stage_cutover_path(&catalog, family, stage).map_err(
                |error| {
                    EvidenceLookupPublicCloseoutError::new(
                        EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                        error.detail(),
                    )
                },
            )?;
            let proof = path
                .prove_for_family(family.identity().as_str())
                .map_err(|error| {
                    EvidenceLookupPublicCloseoutError::new(
                        EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                        format!(
                            "family `{}` failed proof: {}",
                            family.identity().as_str(),
                            error.detail()
                        ),
                    )
                })?;
            family_stage_rows.push(
                EvidenceLookupPublicCloseoutFamilyStageRow::from_receipt_proof(
                    family,
                    stage,
                    query_row.row_digest(),
                    &proof,
                ),
            );
        }
    }

    EvidenceLookupPublicCloseoutAssemblyInput::admit(
        family_stage_rows,
        query_surface_matrix,
        query_consumer_kit,
        source_firewall_report,
        spatial_evidence_surface_deletion_ledger(),
    )
}
