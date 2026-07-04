use topology::facade::current_topology_query_backed_consumer_cutover;
use worth_spatial::facade::evidence_lookup_route::current_evidence_lookup_route_packet;

use super::closeout_matrix::{
    closeout_matrix_from_authorities, WorthTouchedGraphCrossFamilyCloseoutMatrix,
};
use super::coverage_inventory::{
    cross_family_coverage_inventory_from_authorities, live_coverage_ledger_from_authorities,
    LiveCoverageLedger,
};
use super::readiness_handoff::{
    current_representative_family_coverage, touched_graph_readiness_handoff_from_authorities,
};
use super::representative_path::representative_selected_route_parity_path_from_authorities;
use super::{ReadinessHandoffError, RepresentativeSelectedRouteParityPath};
use crate::workload_composition::{
    current_worth_touched_graph_conflict_source_firewall_closeout, LiveCoverageLedgerError,
    WorthTouchedGraphConflictSourceFirewallCloseout,
};
use crate::workload_composition::planner_owned_routing::{
    current_replay_undo_transaction_route_packet,
    current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    public_proof::{current_public_closeout_components, publish_from_parts},
    require_matching_projection_authority, select_rich_localization,
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    WorthTouchedGraphConflictDerivedDiagnosticProjection, WorthTouchedGraphConflictPublicFacade,
    WorthTouchedGraphConflictPublicFacadeError, WorthTouchedGraphConflictPublicFacadeErrorKind,
    WorthTouchedGraphConflictPublicProofInspection,
    CompiledProductReusePlannerRoutePacket, ReplayUndoPlannerRoutePacket,
    WorthTouchedGraphConflictPublicCloseout,
};
use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;

#[derive(Clone)]
pub(crate) struct CurrentTouchedGraphParityCloseoutAuthorities {
    public_closeout: WorthTouchedGraphConflictPublicCloseout,
    source_firewall_closeout: WorthTouchedGraphConflictSourceFirewallCloseout,
    live_coverage_ledger: LiveCoverageLedger,
    representative_path: RepresentativeSelectedRouteParityPath,
    readiness_handoff: TouchedGraphParityReadinessInput,
    closeout_matrix: WorthTouchedGraphCrossFamilyCloseoutMatrix,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CurrentTouchedGraphParityCloseoutAuthoritiesError {
    detail: String,
}

pub(crate) fn current_touched_graph_parity_closeout_authorities(
) -> Result<
    CurrentTouchedGraphParityCloseoutAuthorities,
    CurrentTouchedGraphParityCloseoutAuthoritiesError,
> {
    let public_closeout_components = current_public_closeout_components()
        .map_err(|error| authorities_error(error.detail()))?;
    let public_closeout = publish_from_parts(
        public_closeout_components
            .input()
            .map_err(|error| authorities_error(error.detail()))?,
        public_closeout_components.cutover(),
        public_closeout_components.selected_route_packet(),
        public_closeout_components.admitted_public_proof_input(),
    )
    .map_err(|error| authorities_error(error.detail()))?;
    let selected_route_packet = public_closeout_components.selected_route_packet();
    let cutover = public_closeout_components.cutover();
    let public_facade = build_public_facade_from_authorities(
        public_closeout.clone(),
        &selected_route_packet,
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::RichLocalization,
    )
    .map_err(|error| authorities_error(error.detail()))?;
    let source_firewall_closeout = current_worth_touched_graph_conflict_source_firewall_closeout()
        .map_err(|error| authorities_error(error.detail()))?;
    let inventory = cross_family_coverage_inventory_from_authorities(
        &selected_route_packet,
        &public_facade,
        &cutover,
    )
    .map_err(|error| authorities_error(format!("{error:?}")))?;
    let query_cutover = match current_topology_query_backed_consumer_cutover() {
        Ok(value) => value,
        Err(error) => return Err(authorities_error(error.detail())),
    };
    let evidence_route = match current_evidence_lookup_route_packet() {
        Ok(value) => value,
        Err(error) => return Err(authorities_error(format!("{error:?}"))),
    };
    let replay_route: ReplayUndoPlannerRoutePacket =
        current_replay_undo_transaction_route_packet()
            .map_err(|error| authorities_error(error.detail()))?;
    let reuse_route: CompiledProductReusePlannerRoutePacket =
        current_worth_touched_graph_conflict_compiled_product_reuse_route_packet()
            .map_err(|error| authorities_error(error.detail()))?;
    let representative_path = representative_selected_route_parity_path_from_authorities(
        selected_route_packet.clone(),
        public_facade,
        query_cutover,
        evidence_route,
        replay_route,
        reuse_route,
    )
    .map_err(|error| authorities_error(error.detail()))?;
    let live_coverage_ledger = live_coverage_ledger_from_authorities(
        &inventory,
        &public_closeout,
        selected_route_packet,
    )
    .map_err(live_coverage_ledger_error)?;
    let readiness_handoff = touched_graph_readiness_handoff_from_authorities(
        &representative_path,
        &live_coverage_ledger,
        selected_route_packet,
        current_representative_family_coverage().map_err(readiness_error)?,
    )
    .map_err(readiness_error)?;
    let closeout_matrix = closeout_matrix_from_authorities(
        &live_coverage_ledger,
        &readiness_handoff,
        &representative_path,
        &public_closeout,
    )
    .map_err(|error| authorities_error(error.detail()))?;
    Ok(CurrentTouchedGraphParityCloseoutAuthorities {
        public_closeout,
        source_firewall_closeout,
        live_coverage_ledger,
        representative_path,
        readiness_handoff,
        closeout_matrix,
    })
}

fn authorities_error(detail: impl Into<String>) -> CurrentTouchedGraphParityCloseoutAuthoritiesError {
    CurrentTouchedGraphParityCloseoutAuthoritiesError {
        detail: detail.into(),
    }
}

fn readiness_error(error: ReadinessHandoffError) -> CurrentTouchedGraphParityCloseoutAuthoritiesError {
    authorities_error(error.detail())
}

fn live_coverage_ledger_error(
    error: LiveCoverageLedgerError,
) -> CurrentTouchedGraphParityCloseoutAuthoritiesError {
    authorities_error(format!("{error:?}"))
}

impl CurrentTouchedGraphParityCloseoutAuthorities {
    pub(crate) fn public_closeout(&self) -> &WorthTouchedGraphConflictPublicCloseout {
        &self.public_closeout
    }

    pub(crate) fn source_firewall_closeout(&self) -> &WorthTouchedGraphConflictSourceFirewallCloseout {
        &self.source_firewall_closeout
    }

    pub(crate) fn live_coverage_ledger(&self) -> &LiveCoverageLedger {
        &self.live_coverage_ledger
    }

    pub(crate) fn representative_path(&self) -> &RepresentativeSelectedRouteParityPath {
        &self.representative_path
    }

    pub(crate) fn readiness_handoff(&self) -> &TouchedGraphParityReadinessInput {
        &self.readiness_handoff
    }

    pub(crate) fn closeout_matrix(&self) -> &WorthTouchedGraphCrossFamilyCloseoutMatrix {
        &self.closeout_matrix
    }
}

impl CurrentTouchedGraphParityCloseoutAuthoritiesError {
    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

fn build_public_facade_from_authorities(
    public_closeout: WorthTouchedGraphConflictPublicCloseout,
    selected_route_packet: &crate::workload_composition::WorthTouchedGraphConflictSelectedRoutePacket,
    artifact_policy: WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
) -> Result<WorthTouchedGraphConflictPublicFacade, WorthTouchedGraphConflictPublicFacadeError> {
    let derived_diagnostics = WorthTouchedGraphConflictDerivedDiagnosticProjection::from_selected_route_packet(
        selected_route_packet,
        artifact_policy,
        select_rich_localization(artifact_policy, selected_route_packet),
    );
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
