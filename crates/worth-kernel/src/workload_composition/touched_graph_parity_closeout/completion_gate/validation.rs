use crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictArchitectureAlignmentReportRow;

use super::super::ArchitectureClaimLedgerRowKind;
use super::error::{
    WorthTouchedGraphRoadmapCompletionGateError, WorthTouchedGraphRoadmapCompletionGateErrorKind,
};
use super::gate::WorthTouchedGraphRoadmapCompletionGate;

pub(crate) fn validate_roadmap_completion_gate(
    gate: &WorthTouchedGraphRoadmapCompletionGate,
) -> Result<(), WorthTouchedGraphRoadmapCompletionGateError> {
    let matrix = gate.closeout_matrix();
    let readiness = gate.readiness_handoff();
    let representative_path = gate.representative_path();
    let public_closeout = gate.public_closeout();
    let live_ledger = gate.live_coverage_ledger();
    let representative_path_coverage = representative_path.covered_family_kinds();

    if matrix.closeout_architecture_claim_digest() != readiness.architecture_claim_digest()
        || matrix.closeout_architecture_claim_digest()
            != live_ledger.closeout_architecture_claim_digest()
    {
        return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
            WorthTouchedGraphRoadmapCompletionGateErrorKind::MismatchedArchitectureClaim,
            "roadmap completion requires closeout matrix, readiness handoff, and live coverage ledger to certify the same architecture claim",
        ));
    }

    if public_closeout.selected_route_identity_digest()
        != representative_path.selected_route_identity_digest()
        || public_closeout.selected_family_identity()
            != representative_path.selected_family_identity()
        || public_closeout.selected_product_identity_digest()
            != representative_path.selected_product_identity_digest()
        || normalized_witness_identity(public_closeout.selected_witness_identity_digest())
            != normalized_witness_identity(representative_path.selected_witness_identity_digest())
    {
        return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
            WorthTouchedGraphRoadmapCompletionGateErrorKind::RepresentativePathAuthorityMismatch,
            "roadmap completion requires representative path and public closeout to carry the same selected route, family, product, and witness identities",
        ));
    }

    if readiness.residue_digest() != representative_path.residue_digest()
        || readiness.residue_digest() != public_closeout.residue_chain().residue_digest()
        || readiness.source_firewall_digest() != representative_path.source_firewall_digest()
        || readiness.source_firewall_digest() != public_closeout.source_firewall_digest()
        || readiness.source_firewall_digest() != gate.source_firewall_report_digest()
        || public_closeout.deletion_closeout_digest() != gate.deletion_closeout_digest()
    {
        return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
            WorthTouchedGraphRoadmapCompletionGateErrorKind::MismatchedArchitectureClaim,
            "roadmap completion requires readiness, representative path, public closeout, and source-firewall closeout to carry the same residue and firewall authorities",
        ));
    }

    if representative_path.public_proof().closeout_digest() != public_closeout.closeout_digest()
        || representative_path
            .public_proof()
            .architecture_alignment_report()
            .report_digest()
            != public_closeout
                .architecture_alignment_report()
                .report_digest()
        || representative_path
            .evidence_lookup()
            .public_closeout_digest()
            != public_closeout
                .milestone_fifteen_seed()
                .evidence_lookup_public_closeout_digest()
    {
        return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
            WorthTouchedGraphRoadmapCompletionGateErrorKind::RepresentativePathAuthorityMismatch,
            "roadmap completion requires representative public-proof and evidence-lookup steps to bind to the current public closeout authority chain",
        ));
    }

    if gate.covered_forbidden_surface_count() == 0 {
        return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
            WorthTouchedGraphRoadmapCompletionGateErrorKind::SourceFirewallViolation,
            "roadmap completion requires the certified source-firewall closeout to carry covered forbidden surfaces",
        ));
    }

    if representative_path_coverage.is_empty()
        || !representative_path_coverage.iter().all(|family_kind| {
            readiness
                .representative_family_coverage()
                .contains(family_kind)
        })
    {
        return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
            WorthTouchedGraphRoadmapCompletionGateErrorKind::RepresentativePathAuthorityMismatch,
            "roadmap completion requires readiness handoff to carry the actual representative-path family coverage rather than a substituted proxy set",
        ));
    }

    for family_kind in gate.covered_family_kinds() {
        let row = matrix
            .rows()
            .iter()
            .find(|row| row.family_kind() == *family_kind);
        let Some(row) = row else {
            return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
                WorthTouchedGraphRoadmapCompletionGateErrorKind::MissingCoveredFamilyCertification,
                format!(
                    "roadmap completion requires a matrix row for covered family `{}`",
                    family_kind.as_str()
                ),
            ));
        };
        if !row.declare_once_parity_passed()
            || !row.readiness_handoff_passed()
            || !row.public_proof_parity_passed()
            || !row.diagnostic_parity_passed()
        {
            return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
                WorthTouchedGraphRoadmapCompletionGateErrorKind::MissingCoveredFamilyCertification,
                format!(
                    "roadmap completion requires covered family `{}` to carry representative-path, parity, readiness, public-proof, and diagnostic certification",
                    family_kind.as_str()
                ),
            ));
        }
    }

    for family_kind in representative_path_coverage {
        let row = matrix
            .rows()
            .iter()
            .find(|row| row.family_kind() == family_kind);
        let Some(row) = row else {
            return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
                WorthTouchedGraphRoadmapCompletionGateErrorKind::RepresentativePathAuthorityMismatch,
                format!(
                    "roadmap completion requires representative-path family `{}` to have a closeout matrix row",
                    family_kind.as_str()
                ),
            ));
        };
        if !row.representative_path_covered() {
            return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
                WorthTouchedGraphRoadmapCompletionGateErrorKind::RepresentativePathAuthorityMismatch,
                format!(
                    "roadmap completion requires representative-path family `{}` to be marked by the representative-path proof rather than readiness or matrix proxy state",
                    family_kind.as_str()
                ),
            ));
        }
    }

    if public_closeout
        .architecture_alignment_report()
        .ordinary_second_ontology_blockers()
        .iter()
        .any(
            |row: &WorthTouchedGraphConflictArchitectureAlignmentReportRow| {
                !row.mechanically_unreachable_from_ordinary_path()
            },
        )
        || live_ledger.rows().iter().any(|row| {
            row.claim_kind() != ArchitectureClaimLedgerRowKind::Covered
                && !row.mechanically_unreachable_from_ordinary_path()
        })
    {
        return Err(WorthTouchedGraphRoadmapCompletionGateError::new(
            WorthTouchedGraphRoadmapCompletionGateErrorKind::OrdinarySecondOntologyStillReachable,
            "roadmap completion requires every surviving residue or blocker row to be mechanically unreachable from the covered ordinary path",
        ));
    }

    Ok(())
}

fn normalized_witness_identity(identity: Option<&str>) -> &str {
    identity.unwrap_or("not-applicable")
}
