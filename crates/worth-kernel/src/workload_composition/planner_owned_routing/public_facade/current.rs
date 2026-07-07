use std::sync::OnceLock;

use crate::workload_composition::planner_owned_routing::derived_diagnostics::{
    current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy,
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
};
use crate::workload_composition::planner_owned_routing::public_proof::{
    current_worth_touched_graph_conflict_public_closeout, WorthTouchedGraphConflictPublicCloseout,
};

use super::inspection::{
    WorthTouchedGraphConflictPublicFacade, WorthTouchedGraphConflictPublicFacadeError,
    WorthTouchedGraphConflictPublicFacadeErrorKind, WorthTouchedGraphConflictPublicProofInspection,
};
use super::require_matching_projection_authority;

pub fn current_worth_touched_graph_conflict_public_facade(
) -> Result<WorthTouchedGraphConflictPublicFacade, WorthTouchedGraphConflictPublicFacadeError> {
    current_worth_touched_graph_conflict_public_facade_with_artifact_policy(
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::RichLocalization,
    )
}

pub fn current_worth_touched_graph_conflict_public_facade_with_artifact_policy(
    artifact_policy: WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
) -> Result<WorthTouchedGraphConflictPublicFacade, WorthTouchedGraphConflictPublicFacadeError> {
    static MINIMAL_CACHE: OnceLock<WorthTouchedGraphConflictPublicFacade> = OnceLock::new();
    static RICH_CACHE: OnceLock<WorthTouchedGraphConflictPublicFacade> = OnceLock::new();
    let cache = match artifact_policy {
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth => {
            &MINIMAL_CACHE
        }
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::RichLocalization => &RICH_CACHE,
    };
    if let Some(cached) = cache.get() {
        return Ok(cached.clone());
    }

    let public_closeout =
        current_worth_touched_graph_conflict_public_closeout().map_err(|error| {
            WorthTouchedGraphConflictPublicFacadeError::new(
                WorthTouchedGraphConflictPublicFacadeErrorKind::CurrentPublicProofUnavailable,
                error.detail(),
            )
        })?;
    let derived_diagnostics =
    current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy(
            artifact_policy,
        )
        .map_err(|error| {
            WorthTouchedGraphConflictPublicFacadeError::new(
                WorthTouchedGraphConflictPublicFacadeErrorKind::CurrentDerivedDiagnosticsUnavailable,
                error.detail(),
            )
        })?;
    require_matching_projection_authority(&public_closeout, &derived_diagnostics).map_err(
        |detail| {
            WorthTouchedGraphConflictPublicFacadeError::new(
                WorthTouchedGraphConflictPublicFacadeErrorKind::MismatchedProjectionAuthority,
                detail,
            )
        },
    )?;

    Ok(WorthTouchedGraphConflictPublicFacade::new(
        public_proof_inspection(public_closeout),
        derived_diagnostics,
    ))
    .inspect(|facade| {
        let _ = cache.set(facade.clone());
    })
}

fn public_proof_inspection(
    public_closeout: WorthTouchedGraphConflictPublicCloseout,
) -> WorthTouchedGraphConflictPublicProofInspection {
    WorthTouchedGraphConflictPublicProofInspection::new(
        public_closeout.selected_route_identity_digest().to_string(),
        public_closeout.selected_family_identity().to_string(),
        public_closeout
            .selected_product_identity_digest()
            .to_string(),
        public_closeout
            .selected_witness_identity_digest()
            .map(str::to_string),
        public_closeout.closeout_digest().to_string(),
        public_closeout
            .proof_chain()
            .proof_chain_digest()
            .to_string(),
        public_closeout.source_firewall_digest().to_string(),
        public_closeout.deletion_closeout_digest().to_string(),
        public_closeout.residue_chain().clone(),
        public_closeout.architecture_alignment_report().clone(),
        public_closeout.milestone_fifteen_seed().clone(),
    )
}
