use schema::facade::platform::authority::planner_owned_routing_semantic_graph::{
    admit_planner_admitted_explanation_input, admit_planner_public_proof_identity,
    admit_planner_selected_family_identity, admit_planner_selected_product_identity,
    admit_planner_selected_route_identity, admit_planner_witness_identity, PlannerMismatchLocus,
    PlannerWitnessRole,
};
use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityFamilyKind, TouchedGraphParityReadinessInput,
    TouchedGraphParityResidueClassification,
};
use schema::facade::platform::authority::touched_graph_parity_closeout_internal::{
    admit_touched_graph_parity_readiness_claim, admit_touched_graph_parity_readiness_input,
};

use super::error::{ReadinessHandoffError, ReadinessHandoffErrorKind};
use crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_selected_route_packet;
use crate::workload_composition::touched_graph_parity_closeout::{
    current_conflict_family_parity_claim, current_live_coverage_ledger,
    current_replay_undo_family_parity_claim, current_reuse_family_parity_claim,
    current_representative_selected_route_parity_path, current_spatial_family_parity_claim,
    current_topology_family_declare_once_parity_claim,
};

pub fn current_touched_graph_readiness_handoff(
) -> Result<TouchedGraphParityReadinessInput, ReadinessHandoffError> {
    let representative_path =
        current_representative_selected_route_parity_path().map_err(|error| {
            ReadinessHandoffError::new(
                ReadinessHandoffErrorKind::CurrentRepresentativePathUnavailable,
                error.detail(),
            )
        })?;
    let live_coverage_ledger = current_live_coverage_ledger().map_err(|error| {
        ReadinessHandoffError::new(
            ReadinessHandoffErrorKind::CurrentLiveCoverageLedgerUnavailable,
            format!("{error:?}"),
        )
    })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            ReadinessHandoffError::new(
                ReadinessHandoffErrorKind::CurrentSelectedRouteUnavailable,
                error.detail(),
            )
        })?;
    touched_graph_readiness_handoff_from_authorities(
        &representative_path,
        &live_coverage_ledger,
        &selected_route,
        current_representative_family_coverage()?,
    )
}

pub(crate) fn touched_graph_readiness_handoff_from_authorities(
    representative_path: &crate::workload_composition::RepresentativeSelectedRouteParityPath,
    live_coverage_ledger: &crate::workload_composition::LiveCoverageLedger,
    selected_route: &crate::workload_composition::WorthTouchedGraphConflictSelectedRoutePacket,
    representative_family_coverage: Vec<TouchedGraphParityFamilyKind>,
) -> Result<TouchedGraphParityReadinessInput, ReadinessHandoffError> {
    let explanation_input = admit_planner_admitted_explanation_input(
        "worth-kernel:touched-graph-readiness-handoff",
        representative_path.selected_route_identity_digest(),
    )
    .map_err(schema_error)?;
    let selected_family_identity = admit_planner_selected_family_identity(
        &explanation_input,
        representative_path.selected_family_identity(),
    )
    .map_err(schema_error)?;
    let selected_route_identity = admit_planner_selected_route_identity(
        &selected_family_identity,
        representative_path.selected_route_identity_digest(),
    )
    .map_err(schema_error)?;
    let selected_product_identity = admit_planner_selected_product_identity(
        &selected_route_identity,
        representative_path.selected_product_identity_digest(),
    )
    .map_err(schema_error)?;
    let selected_witness_identity = admit_planner_witness_identity(
        &selected_route_identity,
        PlannerWitnessRole::DenialOrAdvisory,
        PlannerMismatchLocus::QuerySupportPosture,
        representative_path
            .selected_witness_identity_digest()
            .unwrap_or("not-applicable"),
    )
    .map_err(schema_error)?;
    let public_proof_identity = admit_planner_public_proof_identity(
        &selected_route_identity,
        &selected_product_identity,
        representative_path.public_proof().proof_chain_digest(),
    )
    .map_err(schema_error)?;
    let claim = admit_touched_graph_parity_readiness_claim(
        TouchedGraphParityFamilyKind::PublicProof,
        selected_route_identity,
        selected_family_identity,
        selected_product_identity,
        selected_witness_identity,
        public_proof_identity,
    );

    admit_touched_graph_parity_readiness_input(
        claim,
        TouchedGraphParityResidueClassification::OrdinaryPathCarried,
        selected_route.touched_closure_digest(),
        selected_route.overlap_identity_digests().to_vec(),
        representative_family_coverage,
        representative_path
            .query_posture()
            .cutover()
            .support_snapshot_digest(),
        representative_path
            .evidence_lookup()
            .packet()
            .query_support_digest(),
        representative_path.residue_digest(),
        representative_path.source_firewall_digest(),
        live_coverage_ledger.closeout_architecture_claim_digest(),
    )
    .map_err(|error| {
        ReadinessHandoffError::new(
            ReadinessHandoffErrorKind::SchemaContractRejected,
            error.detail(),
        )
    })
}

pub(crate) fn current_representative_family_coverage(
) -> Result<Vec<TouchedGraphParityFamilyKind>, ReadinessHandoffError> {
    let topology = current_topology_family_declare_once_parity_claim().map_err(|error| {
        ReadinessHandoffError::new(
            ReadinessHandoffErrorKind::CurrentCoverageInventoryUnavailable,
            error.detail(),
        )
    })?;
    let spatial = current_spatial_family_parity_claim().map_err(|error| {
        ReadinessHandoffError::new(
            ReadinessHandoffErrorKind::CurrentCoverageInventoryUnavailable,
            error.detail(),
        )
    })?;
    let replay_undo = current_replay_undo_family_parity_claim().map_err(|error| {
        ReadinessHandoffError::new(
            ReadinessHandoffErrorKind::CurrentCoverageInventoryUnavailable,
            error.detail(),
        )
    })?;
    let conflict = current_conflict_family_parity_claim().map_err(|error| {
        ReadinessHandoffError::new(
            ReadinessHandoffErrorKind::CurrentCoverageInventoryUnavailable,
            error.detail(),
        )
    })?;
    let reuse = current_reuse_family_parity_claim().map_err(|error| {
        ReadinessHandoffError::new(
            ReadinessHandoffErrorKind::CurrentCoverageInventoryUnavailable,
            error.detail(),
        )
    })?;
    let mut coverage = topology
        .rows()
        .iter()
        .map(|row| row.family_kind())
        .chain(spatial.rows().iter().map(|row| row.family_kind()))
        .chain(replay_undo.rows().iter().map(|row| row.family_kind()))
        .chain(conflict.rows().iter().map(|row| row.family_kind()))
        .chain(reuse.rows().iter().map(|row| row.family_kind()))
        .chain([
            TouchedGraphParityFamilyKind::PublicProof,
            TouchedGraphParityFamilyKind::DerivedDiagnostics,
        ])
        .collect::<Vec<_>>();
    coverage.sort();
    coverage.dedup();
    if coverage.is_empty() {
        return Err(ReadinessHandoffError::new(
            ReadinessHandoffErrorKind::MissingRepresentativeFamilyCoverage,
            "readiness handoff requires representative family coverage from live family parity claims",
        ));
    }
    Ok(coverage)
}

fn schema_error(
    error: schema::facade::platform::authority::planner_owned_routing_semantic_graph::PlannerOwnedRoutingSemanticGraphVocabularyError,
) -> ReadinessHandoffError {
    ReadinessHandoffError::new(
        ReadinessHandoffErrorKind::PlannerSemanticGraphUnavailable,
        error.detail(),
    )
}
