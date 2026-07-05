use std::sync::OnceLock;

use crate::facade::evidence_lookup_route::current_evidence_lookup_route_packet;
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
use topology::facade::{
    current_topology_query_backed_consumer_cutover, TopologyReadModelReusePosture,
};
use topology::query_domain::TopologyReadRequestFamily;

use super::admission::{
    admit_evidence_lookup_public_closeout_assembly_input,
    admit_evidence_lookup_public_closeout_route_input,
};
use super::assembly_input::EvidenceLookupPublicCloseoutAssemblyInput;
use super::closeout::EvidenceLookupPublicCloseout;
use super::error::{EvidenceLookupPublicCloseoutError, EvidenceLookupPublicCloseoutErrorKind};
use super::family_stage_row::EvidenceLookupPublicCloseoutFamilyStageRow;
use super::input::{
    EvidenceLookupPublicCloseoutRouteInput, SelectedEvidenceLookupPublicCloseoutRouteSupport,
};
use super::query_boundary_support::compose_query_boundary_support_digest;

pub fn current_evidence_lookup_public_closeout(
) -> Result<EvidenceLookupPublicCloseout, EvidenceLookupPublicCloseoutError> {
    static CACHE: OnceLock<EvidenceLookupPublicCloseout> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let route_input = current_evidence_lookup_public_closeout_route_input()?;
    let closeout = EvidenceLookupPublicCloseout::assemble_from_route_input(&route_input)?;
    let _ = CACHE.set(closeout.clone());
    Ok(closeout)
}

pub(crate) fn current_evidence_lookup_public_closeout_assembly_input(
) -> Result<EvidenceLookupPublicCloseoutAssemblyInput, EvidenceLookupPublicCloseoutError> {
    current_evidence_lookup_public_closeout_route_input()
        .map(|route_input| route_input.assembly_input().clone())
}

pub(crate) fn current_evidence_lookup_public_closeout_with_selected_route_support(
    selected_route_support: SelectedEvidenceLookupPublicCloseoutRouteSupport,
) -> Result<EvidenceLookupPublicCloseout, EvidenceLookupPublicCloseoutError> {
    let route_input =
        current_evidence_lookup_public_closeout_route_input_with_selected_route_support(
            selected_route_support,
        )?;
    EvidenceLookupPublicCloseout::assemble_from_route_input(&route_input)
}

pub(crate) fn current_evidence_lookup_public_closeout_route_input_with_selected_route_support(
    selected_route_support: SelectedEvidenceLookupPublicCloseoutRouteSupport,
) -> Result<EvidenceLookupPublicCloseoutRouteInput, EvidenceLookupPublicCloseoutError> {
    let current_route_input = current_evidence_lookup_public_closeout_route_input()?;
    admit_evidence_lookup_public_closeout_route_input(
        current_route_input.route_packet().clone(),
        selected_route_support,
        current_route_input.admitted_assembly_input().clone(),
    )
}

pub fn current_evidence_lookup_public_closeout_route_input(
) -> Result<EvidenceLookupPublicCloseoutRouteInput, EvidenceLookupPublicCloseoutError> {
    static CACHE: OnceLock<EvidenceLookupPublicCloseoutRouteInput> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let route_packet = current_evidence_lookup_route_packet().map_err(|error| {
        EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MissingSelectedRouteFamilyRow,
            format!(
                "planner-owned public closeout route requires evidence lookup route packet: {}",
                error.detail()
            ),
        )
    })?;
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
    let topology_query_backed_cutover =
        current_topology_query_backed_consumer_cutover().map_err(|error| {
            EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                error.detail(),
            )
        })?;
    let topology_loop_cycle_row = topology_query_backed_cutover
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
        .ok_or_else(|| {
            EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                "topology query-backed consumer cutover did not expose a loop-cycle family row",
            )
        })?;
    if topology_loop_cycle_row.reuse_posture() == TopologyReadModelReusePosture::Denied {
        return Err(EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
            "topology loop-cycle public-read proof remains denied and cannot satisfy evidence-lookup public closeout",
        ));
    }

    let mut family_stage_rows = Vec::new();
    let mut selected_route_support = None;

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

            if family.identity().as_str() == route_packet.route_family_identity()
                && stage == route_packet.stage()
            {
                selected_route_support =
                    Some(SelectedEvidenceLookupPublicCloseoutRouteSupport::new(
                        family.identity().as_str().to_string(),
                        family
                            .stage_applicability()
                            .stage_receipt_family_identity()
                            .digest()
                            .to_string(),
                        proof.selected_lookup_plan_digest().to_string(),
                        proof.lookup_execution_receipt_digest().to_string(),
                        proof.lookup_product_output_digest().to_string(),
                        lowered_identity
                            .compiled_product_identity()
                            .identity_digest()
                            .to_string(),
                        lowered_identity
                            .equivalence_policy_identity()
                            .identity_digest()
                            .to_string(),
                        proof.selected_equivalence_family_identity().to_string(),
                        proof.selected_reuse_basis_identity_digest().to_string(),
                    ));
            }

            let row = if family.topology_input_posture().requires_topology_receipt() {
                EvidenceLookupPublicCloseoutFamilyStageRow::from_receipt_proof_with_topology_read_receipt(
                    family,
                    stage,
                    query_row.row_digest(),
                    &proof,
                    &lowered_identity,
                    topology_query_backed_cutover.closeout_digest(),
                    topology_loop_cycle_row,
                )
            } else {
                EvidenceLookupPublicCloseoutFamilyStageRow::from_receipt_proof(
                    family,
                    stage,
                    query_row.row_digest(),
                    &proof,
                    &lowered_identity,
                )
            };
            family_stage_rows.push(row);
        }
    }

    let query_boundary_support_digest = compose_query_boundary_support_digest(
        &family_stage_rows,
        &query_surface_matrix,
        &query_consumer_kit,
    );
    let assembly_input = EvidenceLookupPublicCloseoutAssemblyInput::admit(
        spatial_compiled_product_family_digest,
        family_stage_rows,
        query_surface_matrix,
        query_consumer_kit,
        query_boundary_support_digest,
        source_firewall_report,
        spatial_evidence_surface_deletion_ledger(),
    )?;
    let selected_route_support = selected_route_support.ok_or_else(|| {
        EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MissingSelectedRouteFamilyRow,
            format!(
                "public-closeout route input could not lower selected support for family `{}` at stage `{}`",
                route_packet.route_family_identity(),
                route_packet.stage().human_name()
            ),
        )
    })?;

    let admitted_assembly_input = admit_evidence_lookup_public_closeout_assembly_input(
        assembly_input,
        catalog.declarations(),
    )?;

    let route_input = admit_evidence_lookup_public_closeout_route_input(
        route_packet,
        selected_route_support,
        admitted_assembly_input,
    )?;
    let _ = CACHE.set(route_input.clone());
    Ok(route_input)
}
