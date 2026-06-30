use crate::spatial_compiled_product_family::{
    current_spatial_compiled_product_family_catalog, select_spatial_compiled_product_family,
    SpatialCompiledProductConsumer,
};
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
};
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
    let spatial_family_catalog = current_spatial_compiled_product_family_catalog();
    let spatial_compiled_product_family_digest = spatial_family_catalog
        .family_for_consumer(SpatialCompiledProductConsumer::EvidenceLookupPublicCloseout)
        .ok_or_else(|| {
            EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                "spatial compiled-product family catalog has no public-closeout declaration",
            )
        })?
        .family_digest()
        .to_string();
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
            let lowered_identity = select_spatial_compiled_product_family(
                &spatial_family_catalog,
                admit_spatial_compiled_product_input(
                    &spatial_family_catalog,
                    SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
                        SpatialCompiledProductConsumer::EvidenceLookupPublicCloseout,
                        path.selected_plan(),
                        path.index_product(),
                    ),
                )
                .map(|admitted| admitted.family_admitted_input())
                .map_err(|error| {
                    EvidenceLookupPublicCloseoutError::new(
                        EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                        format!(
                            "family `{}` could not admit public-closeout compiled-product basis: {:?}",
                            family.identity().as_str(),
                            error.kind()
                        ),
                    )
                })?,
            )
            .map_err(|error| {
                EvidenceLookupPublicCloseoutError::new(
                    EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                    format!(
                        "family `{}` could not select public-closeout compiled-product family: {:?}",
                        family.identity().as_str(),
                        error.kind()
                    ),
                )
            })?
            .compile_product_identity()
            .map_err(|error| {
                EvidenceLookupPublicCloseoutError::new(
                    EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                    format!(
                        "family `{}` could not lower public-closeout compiled-product identity: {:?}",
                        family.identity().as_str(),
                        error.kind()
                    ),
                )
            })?;
            family_stage_rows.push(
                EvidenceLookupPublicCloseoutFamilyStageRow::from_receipt_proof(
                    family,
                    stage,
                    query_row.row_digest(),
                    &proof,
                    &lowered_identity,
                ),
            );
        }
    }

    EvidenceLookupPublicCloseoutAssemblyInput::admit(
        spatial_compiled_product_family_digest,
        family_stage_rows,
        query_surface_matrix,
        query_consumer_kit,
        source_firewall_report,
        spatial_evidence_surface_deletion_ledger(),
    )
}
