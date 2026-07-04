use crate::workload_composition::planner_owned_routing::{
    WorthTouchedGraphConflictDerivedDiagnosticProjection, WorthTouchedGraphConflictPublicCloseout,
};

pub(crate) fn require_matching_projection_authority(
    public_closeout: &WorthTouchedGraphConflictPublicCloseout,
    derived_diagnostics: &WorthTouchedGraphConflictDerivedDiagnosticProjection,
) -> Result<(), String> {
    if derived_diagnostics.selected_route_identity_digest()
        != public_closeout.selected_route_identity_digest()
        || derived_diagnostics.selected_family_identity()
            != public_closeout.selected_family_identity()
        || derived_diagnostics.selected_product_identity_digest()
            != public_closeout.selected_product_identity_digest()
        || derived_diagnostics.selected_witness_identity_digest()
            != public_closeout.selected_witness_identity_digest()
    {
        return Err(
            "planner-owned public facade requires public proof and diagnostic projections from one selected-route authority chain".to_string(),
        );
    }

    if public_closeout
        .proof_chain()
        .proof_chain_digest()
        .is_empty()
        || public_closeout
            .milestone_fifteen_seed()
            .seed_digest()
            .is_empty()
        || public_closeout.residue_chain().residue_digest().is_empty()
        || public_closeout.source_firewall_digest().is_empty()
        || public_closeout.residue_chain().residue_digest()
            != public_closeout.milestone_fifteen_seed().residue_digest()
        || public_closeout.source_firewall_digest()
            != public_closeout
                .milestone_fifteen_seed()
                .source_firewall_digest()
    {
        return Err(
            "planner-owned public facade requires public proof seed, residue, and source-firewall digests from one carried authority chain".to_string(),
        );
    }

    Ok(())
}
