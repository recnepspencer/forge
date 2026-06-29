use std::collections::BTreeMap;

use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report,
};
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_kernel::workload_composition::CompletedBooleanSplitHandoff;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitScopeAdmission, PlanarBooleanEdgeSplitScopeAdmissionInput,
    PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput,
    PlanarBooleanSplitSourceEdgeCarrierRecoveryInput, PlanarBooleanSplitSourceEdgeCarrierSet,
};
use worth_spatial::facade::planar_boolean_events::PlanarBooleanSourceIntervalSense;
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanFragmentContinuationEndpointRole, PlanarBooleanFragmentContinuationIndex,
    PlanarBooleanFragmentContinuationIndexInput, PlanarBooleanLoopSourceProvenanceBundle,
    PlanarBooleanLoopSourceProvenanceRecoveryInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExpectedContinuationSemanticRow {
    split_vertex_identity: String,
    fragment_identity: String,
    source_loop_identity: String,
    source_face_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    source_loop_carrier_identity: String,
    fragment_endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
    source_sense: PlanarBooleanSourceIntervalSense,
    endpoint_parameter_bits: u64,
    fragment_parameter_range_bits: [u64; 2],
    local_frame_identity: String,
    precision_basis_identity: String,
    event_group_identities: Vec<String>,
    boundary_roles: Vec<
        worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole,
    >,
}

pub(crate) fn assert_loop_reconstruction_continuation_contract_preserves_real_neighborhoods_and_ordering(
) {
    let subject = MetabossEventExtractionSubject::certify("phase7.4 public continuation contract");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let replay_report = replay_parity_report(&replay_subject);
    let completed_split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    let downstream_consumption = completed_split_handoff
        .admit_batch_execution_cluster()
        .expect("real split evidence should admit batch execution cluster")
        .admit_downstream_split_consumption(
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
        )
        .expect("real split evidence should admit downstream split consumption");
    let loop_split_consumption = PlanarBooleanLoopReconstructionSplitConsumption::admit(
        PlanarBooleanLoopReconstructionSplitConsumptionInput::from_downstream_split_consumption(
            &downstream_consumption,
        ),
    )
    .expect("loop reconstruction should consume the real downstream split product");
    let request = worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionRequest::admit(
        worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionRequestInput::from_split_consumption(
            &loop_split_consumption,
        ),
    )
    .expect("loop reconstruction request should admit from the real loop split consumption");
    let recovered_source_carriers =
        recovered_source_carriers(&subject, &replay_subject.original_products.request);
    let source_provenance = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            replay_subject.original_ledger.ledger(),
            replay_subject.original_ledger.receipt(),
            &recovered_source_carriers,
            &replay_subject.original_products.fragments,
            &replay_subject.original_products.chains,
        ),
    )
    .expect("loop source provenance should recover from the real split chain");
    let continuation_index = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &request,
            &source_provenance,
            &replay_subject.original_products.vertices,
            &replay_subject.original_products.fragments,
            &replay_subject.original_products.chains,
        ),
    )
    .expect("continuation index should admit from real loop reconstruction support");

    assert!(continuation_index.rows().len() > 1);
    assert_eq!(
        continuation_index
            .counters()
            .fragment_continuations_indexed(),
        continuation_index.rows().len()
    );
    assert_eq!(
        continuation_index.request_identity(),
        request.request_identity()
    );
    assert_eq!(
        continuation_index.source_provenance_bundle_identity(),
        source_provenance.bundle_identity()
    );

    let expected_rows = expected_semantic_rows(
        &source_provenance,
        &replay_subject.original_products.vertices,
        &replay_subject.original_products.fragments,
    );
    assert_eq!(continuation_index.rows().len(), expected_rows.len());

    let mut expected_order = expected_rows.clone();
    sort_expected_rows(&mut expected_order);
    let actual_order = continuation_index
        .ordered_rows_with_basis()
        .map(|(row, key)| {
            assert_eq!(key.split_vertex_identity(), row.split_vertex_identity());
            assert_eq!(key.source_sense(), row.source_sense());
            assert_eq!(key.endpoint_parameter_bits(), row.endpoint_parameter_bits());
            assert_eq!(key.fragment_identity(), row.fragment_identity());
            semantic_row(row)
        })
        .collect::<Vec<_>>();
    assert_eq!(actual_order, expected_order);

    for ((split_vertex_identity, source_loop_identity, source_sense), expected_neighborhood) in
        expected_rows_by_neighborhood(&expected_rows)
    {
        let neighborhood = continuation_index
            .neighborhood(
                split_vertex_identity.as_str(),
                source_loop_identity.as_str(),
                source_sense,
            )
            .expect("every expected continuation neighborhood should exist");
        assert_eq!(
            neighborhood.rows().map(semantic_row).collect::<Vec<_>>(),
            expected_neighborhood
        );
        assert!(neighborhood
            .rows()
            .all(|row| row.neighborhood_identity() == neighborhood.neighborhood_identity()));
    }
}

pub(crate) fn completed_split_handoff_for(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &super::edge_splitting_replay_parity_support::EdgeSplitReplayParitySubject,
) -> CompletedBooleanSplitHandoff {
    let event_ledger_lookup_packet = subject
        .pair()
        .left()
        .workload()
        .require_boolean_event_ledger_lookup_execution_packet(subject.ledger())
        .expect("real workload should admit the event-ledger lookup execution packet");
    let completed_split_handoff = subject
        .pair()
        .left()
        .workload()
        .complete_boolean_split_handoff(
            replay_subject.original_ledger.receipt(),
            &event_ledger_lookup_packet,
        )
        .expect("real workload should produce a proof-bearing split completion handoff");
    completed_split_handoff
        .require_boolean_split()
        .expect("completed split handoff should require the exact split ledger receipt");
    completed_split_handoff
}

pub(crate) fn recovered_source_carriers(
    subject: &MetabossEventExtractionSubject,
    request: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest,
) -> PlanarBooleanSplitSourceEdgeCarrierSet {
    let scope = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(request),
    )
    .expect("real split request should admit split scope before source-carrier recovery");
    PlanarBooleanSplitSourceEdgeCarrierSet::recover(
        PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
            &scope,
            subject.ledger(),
        ),
    )
    .expect("real split scope should recover source-edge carriers")
}

fn expected_semantic_rows(
    source_provenance: &PlanarBooleanLoopSourceProvenanceBundle,
    vertices: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentitySet,
    fragments: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet,
) -> Vec<ExpectedContinuationSemanticRow> {
    let split_vertices = vertices
        .vertices()
        .map(|vertex| (vertex.split_vertex_identity().to_string(), vertex))
        .collect::<BTreeMap<_, _>>();
    let mut overlap_roles = BTreeMap::<
        String,
        Vec<
            worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole,
        >,
    >::new();
    for row in source_provenance.overlap_chain_lineage_map().rows() {
        for (fragment_identity, boundary_role) in row
            .fragment_identities()
            .iter()
            .zip(row.boundary_roles().iter().copied())
        {
            overlap_roles
                .entry(fragment_identity.clone())
                .or_default()
                .push(boundary_role);
        }
    }

    let mut rows = Vec::new();
    for fragment in fragments.fragments() {
        let membership = source_provenance
            .fragment_membership_map()
            .membership_for_fragment_identity(fragment.fragment_identity())
            .expect("real source provenance should cover every fragment");
        for source_sense in fragment.source_senses() {
            collect_expected_endpoint_row(
                &mut rows,
                &split_vertices,
                &overlap_roles,
                fragment,
                membership,
                *source_sense,
                fragment.start_endpoint(),
                PlanarBooleanFragmentContinuationEndpointRole::Start,
            );
            collect_expected_endpoint_row(
                &mut rows,
                &split_vertices,
                &overlap_roles,
                fragment,
                membership,
                *source_sense,
                fragment.end_endpoint(),
                PlanarBooleanFragmentContinuationEndpointRole::End,
            );
        }
    }
    rows
}

#[allow(clippy::too_many_arguments)]
fn collect_expected_endpoint_row(
    rows: &mut Vec<ExpectedContinuationSemanticRow>,
    split_vertices: &BTreeMap<
        String,
        &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentityRow,
    >,
    overlap_roles: &BTreeMap<
        String,
        Vec<
            worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole,
        >,
    >,
    fragment: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragment,
    membership: &worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanFragmentMembershipRow,
    source_sense: PlanarBooleanSourceIntervalSense,
    endpoint: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentEndpointRef,
    endpoint_role: PlanarBooleanFragmentContinuationEndpointRole,
) {
    if endpoint.endpoint_kind()
        != worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex
    {
        return;
    }
    let vertex = split_vertices
        .get(endpoint.endpoint_identity())
        .expect("real fragment endpoint should bind a split vertex");

    let mut event_group_identities = fragment.event_group_identities().to_vec();
    event_group_identities.sort();
    let mut boundary_roles = overlap_roles
        .get(fragment.fragment_identity())
        .cloned()
        .unwrap_or_default();
    boundary_roles.sort_by_key(|role| boundary_role_rank(*role));

    rows.push(ExpectedContinuationSemanticRow {
        split_vertex_identity: vertex.split_vertex_identity().to_string(),
        fragment_identity: fragment.fragment_identity().to_string(),
        source_loop_identity: membership.source_loop_identity().to_string(),
        source_face_identity: membership.source_face_identity().to_string(),
        source_edge_identity: fragment.source_edge_identity().to_string(),
        carrier_identity: fragment.carrier_identity().to_string(),
        source_loop_carrier_identity: membership.source_loop_carrier_identity().to_string(),
        fragment_endpoint_role: endpoint_role,
        source_sense,
        endpoint_parameter_bits: endpoint.parameter_bits(),
        fragment_parameter_range_bits: fragment.parameter_range_bits(),
        local_frame_identity: fragment.local_frame_identity().to_string(),
        precision_basis_identity: fragment.precision_basis_identity().to_string(),
        event_group_identities,
        boundary_roles,
    });
}

fn expected_rows_by_neighborhood(
    expected_rows: &[ExpectedContinuationSemanticRow],
) -> BTreeMap<
    (String, String, PlanarBooleanSourceIntervalSense),
    Vec<ExpectedContinuationSemanticRow>,
> {
    let mut neighborhoods = BTreeMap::new();
    for row in expected_rows {
        neighborhoods
            .entry((
                row.split_vertex_identity.clone(),
                row.source_loop_identity.clone(),
                row.source_sense,
            ))
            .or_insert_with(Vec::new)
            .push(row.clone());
    }
    for rows in neighborhoods.values_mut() {
        sort_expected_rows(rows);
    }
    neighborhoods
}

fn semantic_row(
    row: &worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanFragmentContinuationRow,
) -> ExpectedContinuationSemanticRow {
    let mut event_group_identities = row.event_group_identities().to_vec();
    event_group_identities.sort();
    let mut boundary_roles = row.boundary_roles().to_vec();
    boundary_roles.sort_by_key(|role| boundary_role_rank(*role));
    ExpectedContinuationSemanticRow {
        split_vertex_identity: row.split_vertex_identity().to_string(),
        fragment_identity: row.fragment_identity().to_string(),
        source_loop_identity: row.source_loop_identity().to_string(),
        source_face_identity: row.source_face_identity().to_string(),
        source_edge_identity: row.source_edge_identity().to_string(),
        carrier_identity: row.carrier_identity().to_string(),
        source_loop_carrier_identity: row.source_loop_carrier_identity().to_string(),
        fragment_endpoint_role: row.fragment_endpoint_role(),
        source_sense: row.source_sense(),
        endpoint_parameter_bits: row.endpoint_parameter_bits(),
        fragment_parameter_range_bits: row.fragment_parameter_range_bits(),
        local_frame_identity: row.local_frame_identity().to_string(),
        precision_basis_identity: row.precision_basis_identity().to_string(),
        event_group_identities,
        boundary_roles,
    }
}

fn sort_expected_rows(rows: &mut [ExpectedContinuationSemanticRow]) {
    rows.sort_by(|left, right| {
        left.split_vertex_identity
            .cmp(&right.split_vertex_identity)
            .then_with(|| left.source_sense.cmp(&right.source_sense))
            .then_with(|| {
                left.endpoint_parameter_bits
                    .cmp(&right.endpoint_parameter_bits)
            })
            .then_with(|| {
                left.fragment_parameter_range_bits[0].cmp(&right.fragment_parameter_range_bits[0])
            })
            .then_with(|| {
                left.fragment_parameter_range_bits[1].cmp(&right.fragment_parameter_range_bits[1])
            })
            .then_with(|| {
                left.fragment_endpoint_role
                    .cmp(&right.fragment_endpoint_role)
            })
            .then_with(|| left.source_loop_identity.cmp(&right.source_loop_identity))
            .then_with(|| left.fragment_identity.cmp(&right.fragment_identity))
            .then_with(|| left.source_edge_identity.cmp(&right.source_edge_identity))
            .then_with(|| left.carrier_identity.cmp(&right.carrier_identity))
    });
}

fn boundary_role_rank(
    role: worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole,
) -> u8 {
    match role {
        worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan => 0,
        worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary => 1,
        worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment => 2,
        worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary => 3,
    }
}
