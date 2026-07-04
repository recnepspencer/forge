use topology::facade::TopologyQueryBackedConsumerCutover;
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::facade::evidence_lookup_route::EvidenceLookupRoutePacket;

use crate::workload_composition::planner_owned_routing::{
    CompiledProductReusePlannerRoutePacket, ReplayUndoPlannerRoutePacket,
    WorthTouchedGraphConflictPublicFacade, WorthTouchedGraphConflictSelectedRoutePacket,
};

use super::consumer_step::{
    RepresentativeSelectedRouteAuthority, RepresentativeSelectedRouteConsumerStep,
    RepresentativeSelectedRouteDiagnosticStep, RepresentativeSelectedRouteEvidenceLookupStep,
    RepresentativeSelectedRoutePublicProofStep, RepresentativeSelectedRouteQueryBackedReadStep,
    RepresentativeSelectedRouteReplayConsumerStep, RepresentativeSelectedRouteReuseConsumerStep,
};
use super::path::{
    RepresentativeSelectedRouteParityPath, RepresentativeSelectedRouteParityPathError,
};
use super::validation::validate_representative_path_sources;

pub(crate) fn build_representative_selected_route_parity_path(
    selected_route_packet: WorthTouchedGraphConflictSelectedRoutePacket,
    public_facade: WorthTouchedGraphConflictPublicFacade,
    query_cutover: TopologyQueryBackedConsumerCutover,
    evidence_route: EvidenceLookupRoutePacket,
    replay_route: ReplayUndoPlannerRoutePacket,
    reuse_route: CompiledProductReusePlannerRoutePacket,
) -> Result<RepresentativeSelectedRouteParityPath, RepresentativeSelectedRouteParityPathError> {
    validate_representative_path_sources(
        &selected_route_packet,
        &public_facade,
        &query_cutover,
        &evidence_route,
        &replay_route,
        &reuse_route,
    )?;
    let query_row = query_cutover
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
        .expect("loop-cycle query-backed read row should exist")
        .clone();
    let consumers = vec![
        RepresentativeSelectedRouteConsumerStep::QueryBackedRead(
            RepresentativeSelectedRouteQueryBackedReadStep::new(
                query_cutover,
                query_row,
                public_facade
                    .public_proof()
                    .milestone_fifteen_seed()
                    .evidence_lookup_query_boundary_support_digest()
                    .to_string(),
            ),
        ),
        RepresentativeSelectedRouteConsumerStep::EvidenceLookup(
            RepresentativeSelectedRouteEvidenceLookupStep::new(
                evidence_route,
                selected_route_packet
                    .evidence_lookup_public_closeout_digest()
                    .to_string(),
            ),
        ),
        RepresentativeSelectedRouteConsumerStep::ReplayOrConflict(
            RepresentativeSelectedRouteReplayConsumerStep::new(replay_route),
        ),
        RepresentativeSelectedRouteConsumerStep::CompiledProductReuse(
            RepresentativeSelectedRouteReuseConsumerStep::new(reuse_route),
        ),
        RepresentativeSelectedRouteConsumerStep::PublicProof(
            RepresentativeSelectedRoutePublicProofStep::new(public_facade.public_proof().clone()),
        ),
        RepresentativeSelectedRouteConsumerStep::Diagnostic(
            RepresentativeSelectedRouteDiagnosticStep::new(
                public_facade.derived_diagnostics().clone(),
            ),
        ),
    ];

    Ok(RepresentativeSelectedRouteParityPath::new(
        RepresentativeSelectedRouteAuthority::new(selected_route_packet),
        consumers,
    ))
}
