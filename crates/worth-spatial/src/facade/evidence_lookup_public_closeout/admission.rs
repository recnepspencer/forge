use std::collections::BTreeSet;

use crate::facade::evidence_lookup_route::EvidenceLookupRoutePacket;
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupFamilyDeclaration;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceTouchpoint;

use super::assembly_input::EvidenceLookupPublicCloseoutAssemblyInput;
use super::error::{EvidenceLookupPublicCloseoutError, EvidenceLookupPublicCloseoutErrorKind};
use super::family_stage_row::EvidenceLookupPublicCloseoutDisposition;
use super::input::{
    AdmittedEvidenceLookupPublicCloseoutAssemblyInput, EvidenceLookupPublicCloseoutRouteInput,
    SelectedEvidenceLookupPublicCloseoutRouteSupport,
};

pub(crate) fn admit_evidence_lookup_public_closeout_assembly_input(
    input: EvidenceLookupPublicCloseoutAssemblyInput,
    families: &[EvidenceLookupFamilyDeclaration],
) -> Result<AdmittedEvidenceLookupPublicCloseoutAssemblyInput, EvidenceLookupPublicCloseoutError> {
    reject_duplicate_family_stage_rows(&input)?;
    reject_mismatched_authority_chain(&input, families)?;
    reject_firewall_without_deletion_pressure(&input)?;
    Ok(AdmittedEvidenceLookupPublicCloseoutAssemblyInput::new(
        input,
    ))
}

pub(crate) fn admit_evidence_lookup_public_closeout_route_input(
    route_packet: EvidenceLookupRoutePacket,
    selected_route_support: SelectedEvidenceLookupPublicCloseoutRouteSupport,
    admitted_assembly_input: AdmittedEvidenceLookupPublicCloseoutAssemblyInput,
) -> Result<EvidenceLookupPublicCloseoutRouteInput, EvidenceLookupPublicCloseoutError> {
    let selected_row = admitted_assembly_input
        .assembly_input()
        .family_stage_rows()
        .iter()
        .find(|row| {
            row.family_identity() == selected_route_support.route_family_identity()
                && row.stage() == route_packet.stage()
                && matches!(
                    row.disposition(),
                    EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. }
                )
        })
        .ok_or_else(|| {
            EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MissingSelectedRouteFamilyRow,
                format!(
                    "public-closeout route input is missing selected family `{}` at stage `{}`",
                    selected_route_support.route_family_identity(),
                    route_packet.stage().human_name()
                ),
            )
        })?;

    if route_packet.route_family_identity() != selected_route_support.route_family_identity()
        || route_packet.stage_receipt_family_identity()
            != selected_route_support.stage_receipt_family_identity()
        || selected_row.stage_receipt_family_identity()
            != selected_route_support.stage_receipt_family_identity()
    {
        return Err(EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MismatchedSelectedRouteFamily,
            format!(
                "public-closeout route family mismatch: packet_family={}, route_input_family={}, packet_stage_receipt={}, route_input_stage_receipt={}",
                route_packet.route_family_identity(),
                selected_route_support.route_family_identity(),
                route_packet.stage_receipt_family_identity(),
                selected_route_support.stage_receipt_family_identity(),
            ),
        ));
    }

    if route_packet.selected_lookup_plan_digest()
        != selected_route_support.selected_lookup_plan_digest()
        || route_packet.lookup_execution_receipt_digest()
            != selected_route_support.lookup_execution_receipt_digest()
        || route_packet.lookup_product_output_digest()
            != selected_route_support.lookup_product_output_digest()
        || route_packet.compiled_product_identity_digest()
            != selected_route_support.compiled_product_identity_digest()
        || route_packet.equivalence_policy_identity_digest()
            != selected_route_support.equivalence_policy_identity_digest()
        || selected_row.spatial_compiled_product_identity_digest()
            != Some(selected_route_support.compiled_product_identity_digest())
        || selected_row.spatial_equivalence_policy_identity_digest()
            != Some(selected_route_support.equivalence_policy_identity_digest())
        || selected_row.selected_lookup_plan_digest()
            != Some(selected_route_support.selected_lookup_plan_digest())
        || selected_row.lookup_execution_receipt_digest()
            != Some(selected_route_support.lookup_execution_receipt_digest())
        || selected_row.lookup_product_output_digest()
            != Some(selected_route_support.lookup_product_output_digest())
    {
        return Err(EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MismatchedSelectedRouteProduct,
            format!(
                "public-closeout route product mismatch: packet_compiled_product={}, route_input_compiled_product={}, packet_lookup_output={}, route_input_lookup_output={}",
                route_packet.compiled_product_identity_digest(),
                selected_route_support.compiled_product_identity_digest(),
                route_packet.lookup_product_output_digest(),
                selected_route_support.lookup_product_output_digest(),
            ),
        ));
    }

    if route_packet.selected_equivalence_family_identity()
        != selected_route_support.selected_equivalence_family_identity()
        || selected_row.spatial_selected_equivalence_family_identity()
            != Some(selected_route_support.selected_equivalence_family_identity())
    {
        return Err(EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MismatchedSelectedRouteFamily,
            format!(
                "public-closeout selected family mismatch: packet_selected_family={}, route_input_selected_family={}",
                route_packet.selected_equivalence_family_identity(),
                selected_route_support.selected_equivalence_family_identity(),
            ),
        ));
    }

    if route_packet.selected_reuse_basis_identity_digest()
        != selected_route_support.selected_reuse_basis_identity_digest()
    {
        return Err(EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::MismatchedSelectedRouteSupport,
            format!(
                "public-closeout selected reuse support mismatch: packet_selected_reuse_basis={}, route_input_selected_reuse_basis={}",
                route_packet.selected_reuse_basis_identity_digest(),
                selected_route_support.selected_reuse_basis_identity_digest(),
            ),
        ));
    }

    Ok(EvidenceLookupPublicCloseoutRouteInput::new(
        route_packet,
        selected_route_support,
        admitted_assembly_input,
    ))
}

fn reject_duplicate_family_stage_rows(
    input: &EvidenceLookupPublicCloseoutAssemblyInput,
) -> Result<(), EvidenceLookupPublicCloseoutError> {
    let mut identities = BTreeSet::new();
    for row in input.family_stage_rows() {
        let identity = format!("{}::{:?}", row.family_identity(), row.stage());
        if !identities.insert(identity) {
            return Err(EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::DuplicateFamilyStageRow,
                format!(
                    "duplicate public closeout family-stage row for `{}` at stage `{:?}`",
                    row.family_identity(),
                    row.stage()
                ),
            ));
        }
    }
    Ok(())
}

fn reject_mismatched_authority_chain(
    input: &EvidenceLookupPublicCloseoutAssemblyInput,
    families: &[EvidenceLookupFamilyDeclaration],
) -> Result<(), EvidenceLookupPublicCloseoutError> {
    for row in input.family_stage_rows() {
        let query_row = input
            .query_surface_matrix()
            .require_family_stage_touchpoint_row(
                row.family_identity(),
                row.stage(),
                EvidenceLookupQuerySurfaceTouchpoint::PublicCloseoutProof,
            )
            .map_err(|_| {
                EvidenceLookupPublicCloseoutError::new(
                    EvidenceLookupPublicCloseoutErrorKind::MissingPublicCloseoutQueryRow,
                    format!(
                        "missing public-closeout query matrix row for family `{}` at stage `{}`",
                        row.family_identity(),
                        row.stage().human_name()
                    ),
                )
            })?;
        if query_row.row_digest() != row.query_surface_row_digest() {
            return Err(EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MismatchedFamilyAuthorityChain,
                format!(
                    "family `{}` at stage `{}` carries a mismatched public-closeout query row digest",
                    row.family_identity(),
                    row.stage().human_name()
                ),
            ));
        }

        let family = families
            .iter()
            .find(|family| family.identity().as_str() == row.family_identity())
            .ok_or_else(|| {
                EvidenceLookupPublicCloseoutError::new(
                    EvidenceLookupPublicCloseoutErrorKind::MissingFamilyCoverageDisposition,
                    format!(
                        "closeout row references undeclared family `{}`",
                        row.family_identity()
                    ),
                )
            })?;
        if family.declaration_digest() != row.family_declaration_digest()
            || family
                .stage_applicability()
                .stage_receipt_family_identity()
                .digest()
                != row.stage_receipt_family_identity()
            || family.query_posture().imported_evidence_digest()
                != row.query_import_evidence_digest()
        {
            return Err(EvidenceLookupPublicCloseoutError::new(
                EvidenceLookupPublicCloseoutErrorKind::MismatchedFamilyAuthorityChain,
                format!(
                    "family `{}` at stage `{}` is not bound to the current catalog authority chain",
                    row.family_identity(),
                    row.stage().human_name()
                ),
            ));
        }

        let topology_requires_receipt = family.topology_input_posture().requires_topology_receipt();
        match row.disposition() {
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. } => {
                let missing_topology_receipt_proof = topology_requires_receipt
                    && (row.topology_query_backed_cutover_digest().is_none()
                        || row.topology_read_family_row_digest().is_none());
                if row.spatial_touch_digest().is_none() || missing_topology_receipt_proof {
                    return Err(EvidenceLookupPublicCloseoutError::new(
                        EvidenceLookupPublicCloseoutErrorKind::ImpossibleResidueSuccessMix,
                        format!(
                            "family `{}` at stage `{}` cannot publish receipt proof with the current topology/spatial-touch posture",
                            row.family_identity(),
                            row.stage().human_name()
                        ),
                    ));
                }
            }
            EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => {
                if !topology_requires_receipt {
                    return Err(EvidenceLookupPublicCloseoutError::new(
                        EvidenceLookupPublicCloseoutErrorKind::ImpossibleResidueSuccessMix,
                        format!(
                            "family `{}` at stage `{}` cannot publish residue without a topology-backed blocker",
                            row.family_identity(),
                            row.stage().human_name()
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

fn reject_firewall_without_deletion_pressure(
    input: &EvidenceLookupPublicCloseoutAssemblyInput,
) -> Result<(), EvidenceLookupPublicCloseoutError> {
    if input
        .source_firewall_report()
        .counters()
        .forbidden_row_count()
        > 0
        && input.spatial_deletion_ledger_rows().is_empty()
    {
        return Err(EvidenceLookupPublicCloseoutError::new(
            EvidenceLookupPublicCloseoutErrorKind::SourceFirewallDeletionPressureMismatch,
            "forbidden source-firewall authority requires concrete deletion-ledger pressure",
        ));
    }
    Ok(())
}
