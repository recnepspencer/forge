use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

use super::milestone_four_kernel_evidence_verified_assembly::PrimitiveConstructionMilestoneFourKernelCloseoutAssembly;
use super::milestone_four_kernel_evidence_verified_registry::PrimitiveConstructionMilestoneFourKernelCloseoutRegistry;
use super::milestone_four_kernel_evidence_verified_verification::PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch;

pub(super) fn boundary_gap_mismatches(
    assembly: &PrimitiveConstructionMilestoneFourKernelCloseoutAssembly,
) -> Vec<PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch> {
    let boundary_rows = assembly.query_boundary_gap_register.rows();
    let write_closed = boundary_rows
        .iter()
        .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::Write)
        .is_some_and(|row| row.gap_status().as_str() == "closed");
    let inspect_closed = boundary_rows
        .iter()
        .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::Inspect)
        .is_some_and(|row| row.gap_status().as_str() == "closed");
    let branch_closed = boundary_rows
        .iter()
        .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .is_some_and(|row| row.gap_status().as_str() == "closed");
    let temporal_deferred = boundary_rows
        .iter()
        .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::Temporal)
        .is_some_and(|row| row.gap_status().as_str() == "deferred_unsupported_neighbor");

    if boundary_rows.len() == 6
        && write_closed
        && inspect_closed
        && branch_closed
        && temporal_deferred
        && assembly.query_boundary_gap_register.unresolved_gap_count() >= 1
    {
        Vec::new()
    } else {
        vec![
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::QueryBoundaryGapRegisterDrift,
        ]
    }
}

pub(super) fn closeout_digest(
    registry: &PrimitiveConstructionMilestoneFourKernelCloseoutRegistry,
    assembly: &PrimitiveConstructionMilestoneFourKernelCloseoutAssembly,
) -> String {
    digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            registry.registry_digest.clone(),
            assembly.phase_five_six_closeout.report_digest().to_string(),
            assembly
                .proof_substrate_closeout
                .report_digest()
                .to_string(),
            assembly
                .query_boundary_gap_register
                .report_digest()
                .to_string(),
            assembly
                .query_no_local_runtime_workaround_audit
                .report_digest()
                .to_string(),
            assembly
                .query_existing_truth_binding_report
                .report_digest()
                .to_string(),
            assembly
                .query_graph_composition_parity_report
                .report_digest()
                .to_string(),
            assembly
                .query_basis_preview_parity_report
                .report_digest()
                .to_string(),
            assembly.motion_policy_report.report_digest().to_string(),
            assembly
                .motion_dx_surface_report
                .report_digest()
                .to_string(),
            assembly
                .intent_arbitration_policy_report
                .report_digest()
                .to_string(),
            assembly
                .intent_conflict_dx_surface_report
                .report_digest()
                .to_string(),
            assembly
                .representative_intent_evidence
                .report_digest()
                .to_string(),
            assembly.preview_surface_report.report_digest().to_string(),
            assembly
                .representative_preview_evidence
                .report_digest()
                .to_string(),
            assembly
                .continuity_surface_report
                .report_digest()
                .to_string(),
            assembly
                .representative_continuity_evidence
                .report_digest()
                .to_string(),
            assembly.policy_profile_report.report_digest().to_string(),
            assembly
                .representative_policy_profile_evidence
                .report_digest()
                .to_string(),
            assembly
                .realization_exhaustion_witness_report
                .report_digest()
                .to_string(),
        ],
    )
}
