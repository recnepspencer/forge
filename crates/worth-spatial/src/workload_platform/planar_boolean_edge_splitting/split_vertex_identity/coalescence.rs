use std::collections::BTreeMap;

use super::decision_record::{
    PlanarBooleanSplitVertexCoalescenceDecision, PlanarBooleanSplitVertexCoalescenceReason,
};
use super::denial::{
    PlanarBooleanSplitVertexIdentityDenial, PlanarBooleanSplitVertexIdentityDenialKind,
};
use super::identity::{coalescence_decision_identity, split_vertex_identity};
use super::input_rows::{
    canonical_strings, SplitVertexCoalescenceKey, SplitVertexInputKind, SplitVertexInputRow,
};
use super::vertex_set::PlanarBooleanSplitVertexIdentityRow;

pub(super) struct SplitVertexCoalescence {
    vertices: Vec<PlanarBooleanSplitVertexIdentityRow>,
    decisions: Vec<PlanarBooleanSplitVertexCoalescenceDecision>,
    coalesced_vertices: usize,
    interval_point_collisions: usize,
}

impl SplitVertexCoalescence {
    pub(super) fn from_inputs(
        inputs: Vec<SplitVertexInputRow>,
    ) -> Result<Self, PlanarBooleanSplitVertexIdentityDenial> {
        let mut groups = BTreeMap::<SplitVertexCoalescenceKey, Vec<SplitVertexInputRow>>::new();
        for input in inputs {
            groups
                .entry(input.coalescence_key())
                .or_default()
                .push(input);
        }

        let mut vertices = Vec::with_capacity(groups.len());
        let mut decisions = Vec::new();
        let mut coalesced_vertices = 0;
        let mut interval_point_collisions = 0;
        for (key, mut group) in groups {
            group.sort_by(|left, right| left.input_identity.cmp(&right.input_identity));
            reject_coordinate_only_group(&group)?;
            let input_refs = group.iter().collect::<Vec<_>>();
            let vertex_identity = split_vertex_identity(
                &key.source_edge_identity,
                &key.carrier_identity,
                key.parameter_bits,
                &key.local_frame_identity,
                &key.precision_basis_identity,
                &input_refs,
            );
            let vertex = vertex_from_group(&vertex_identity, &key, &group);
            if group.len() > 1 {
                coalesced_vertices += 1;
                let decision = decision_from_group(&vertex_identity, &key, &group);
                if decision.reason()
                    == PlanarBooleanSplitVertexCoalescenceReason::IntervalEndpointAndPointCut
                {
                    interval_point_collisions += 1;
                }
                decisions.push(decision);
            }
            vertices.push(vertex);
        }
        Ok(Self {
            vertices,
            decisions,
            coalesced_vertices,
            interval_point_collisions,
        })
    }

    pub(super) fn vertices(&self) -> &[PlanarBooleanSplitVertexIdentityRow] {
        &self.vertices
    }
    pub(super) fn into_parts(
        self,
    ) -> (
        Vec<PlanarBooleanSplitVertexIdentityRow>,
        Vec<PlanarBooleanSplitVertexCoalescenceDecision>,
    ) {
        (self.vertices, self.decisions)
    }
    pub(super) fn coalesced_vertices(&self) -> usize {
        self.coalesced_vertices
    }
    pub(super) fn interval_point_collisions(&self) -> usize {
        self.interval_point_collisions
    }
}

fn vertex_from_group(
    vertex_identity: &str,
    key: &SplitVertexCoalescenceKey,
    group: &[SplitVertexInputRow],
) -> PlanarBooleanSplitVertexIdentityRow {
    PlanarBooleanSplitVertexIdentityRow::new(
        vertex_identity.to_string(),
        key.source_edge_identity.clone(),
        key.carrier_identity.clone(),
        f64::from_bits(key.parameter_bits),
        key.parameter_bits,
        key.local_frame_identity.clone(),
        key.precision_basis_identity.clone(),
        canonical_strings(
            group
                .iter()
                .filter_map(|input| input.point_cut_identity.clone())
                .collect(),
        ),
        canonical_strings(
            group
                .iter()
                .flat_map(|input| input.parameter_fact_identities.iter().cloned())
                .collect(),
        ),
        canonical_strings(
            group
                .iter()
                .filter_map(|input| input.interval_subdivision_identity.clone())
                .collect(),
        ),
        canonical_strings(
            group
                .iter()
                .filter_map(|input| input.normalized_interval_identity.clone())
                .collect(),
        ),
        canonical_strings(
            group
                .iter()
                .filter_map(|input| input.coordinate_fact_identity.clone())
                .collect(),
        ),
        canonical_strings(
            group
                .iter()
                .flat_map(|input| input.provenance_identities.iter().cloned())
                .collect(),
        ),
        canonical_strings(
            group
                .iter()
                .flat_map(|input| input.event_group_identities.iter().cloned())
                .collect(),
        ),
    )
}

fn decision_from_group(
    vertex_identity: &str,
    key: &SplitVertexCoalescenceKey,
    group: &[SplitVertexInputRow],
) -> PlanarBooleanSplitVertexCoalescenceDecision {
    let input_identities = canonical_strings(
        group
            .iter()
            .map(|input| input.input_identity.clone())
            .collect(),
    );
    let point_cut_identities = canonical_strings(
        group
            .iter()
            .filter_map(|input| input.point_cut_identity.clone())
            .collect(),
    );
    let interval_subdivision_identities = canonical_strings(
        group
            .iter()
            .filter_map(|input| input.interval_subdivision_identity.clone())
            .collect(),
    );
    let event_group_identities = canonical_strings(
        group
            .iter()
            .flat_map(|input| input.event_group_identities.iter().cloned())
            .collect(),
    );
    let decision_identity = coalescence_decision_identity(vertex_identity, &input_identities);
    PlanarBooleanSplitVertexCoalescenceDecision::new(
        decision_identity,
        vertex_identity.to_string(),
        key.source_edge_identity.clone(),
        key.carrier_identity.clone(),
        key.parameter_bits,
        reason_for_group(group),
        input_identities,
        point_cut_identities,
        interval_subdivision_identities,
        event_group_identities,
    )
}

fn reason_for_group(group: &[SplitVertexInputRow]) -> PlanarBooleanSplitVertexCoalescenceReason {
    let has_point = group
        .iter()
        .any(|input| input.input_kind == SplitVertexInputKind::PointCut);
    let has_interval = group
        .iter()
        .any(|input| input.input_kind != SplitVertexInputKind::PointCut);
    if has_point && has_interval {
        PlanarBooleanSplitVertexCoalescenceReason::IntervalEndpointAndPointCut
    } else if has_point {
        PlanarBooleanSplitVertexCoalescenceReason::DuplicatePointCutReports
    } else {
        PlanarBooleanSplitVertexCoalescenceReason::RedundantIntervalEndpoints
    }
}

fn reject_coordinate_only_group(
    group: &[SplitVertexInputRow],
) -> Result<(), PlanarBooleanSplitVertexIdentityDenial> {
    if group
        .iter()
        .any(|input| !input.provenance_identities.is_empty())
    {
        return Ok(());
    }
    Err(PlanarBooleanSplitVertexIdentityDenial::new(
        PlanarBooleanSplitVertexIdentityDenialKind::CoordinateOnlySplitVertexIdentity,
        group
            .first()
            .map(|input| input.input_identity.as_str())
            .unwrap_or("empty-split-vertex-group"),
        "split vertex identity cannot be minted from coordinate evidence alone",
    ))
}
