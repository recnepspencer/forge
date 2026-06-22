use super::super::edge_splitting_decision_log_support::DecisionLogMetabossProducts;
use std::collections::{BTreeMap, BTreeSet};
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitDecisionKind, PlanarBooleanSplitDecisionLogQueryResult,
    PlanarBooleanSplitDecisionPhase, PlanarBooleanSplitNamedArtifactKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ObservedSplitEdgeChainLedgerManifest {
    pub(crate) chains: BTreeMap<EdgeKey, ObservedSplitEdgeChainLedgerRow>,
    pub(crate) persistent_name_rows_bound: usize,
    pub(crate) decision_rows_bound: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ObservedSplitEdgeChainLedgerRow {
    pub(crate) endpoint_boundary_schedule_identity: String,
    pub(crate) interval_subdivision_schedule_identity: String,
    pub(crate) split_vertex_schedule_identity: String,
    pub(crate) split_fragment_schedule_identity: String,
    pub(crate) fragment_identities: Vec<String>,
    pub(crate) split_vertex_identities: Vec<String>,
    pub(crate) overlap_chain_identities: Vec<String>,
    pub(crate) persistent_name_row_identities: Vec<String>,
    pub(crate) persistent_name_artifact_kinds: BTreeSet<PlanarBooleanSplitNamedArtifactKind>,
    pub(crate) decision_identities: Vec<String>,
    pub(crate) decision_phases: BTreeSet<PlanarBooleanSplitDecisionPhase>,
    pub(crate) decision_kinds: BTreeSet<PlanarBooleanSplitDecisionKind>,
    pub(crate) validation_fragment_coverage_identities: Vec<String>,
    pub(crate) validation_overlap_coverage_identities: Vec<String>,
}

pub(crate) type EdgeKey = (String, String);

impl ObservedSplitEdgeChainLedgerManifest {
    pub(crate) fn from_products(
        products: &DecisionLogMetabossProducts,
        decision_log: &PlanarBooleanSplitDecisionLogQueryResult,
    ) -> Self {
        let mut chains = BTreeMap::<EdgeKey, ObservedSplitEdgeChainLedgerRow>::new();
        bind_endpoint_schedules(products, &mut chains);
        bind_interval_schedules(products, &mut chains);
        bind_vertices(products, &mut chains);
        bind_fragments(products, &mut chains);
        bind_overlap_chains(products, &mut chains);
        bind_validation(products, &mut chains);
        bind_names_and_decisions(products, decision_log, &mut chains);
        Self {
            chains,
            persistent_name_rows_bound: products.naming.persistent_name_rows().len(),
            decision_rows_bound: decision_log.receipt().decision_rows().len(),
        }
    }

    pub(crate) fn total_fragments(&self) -> usize {
        self.chains
            .values()
            .map(|row| row.fragment_identities.len())
            .sum()
    }
}

fn bind_endpoint_schedules(
    products: &DecisionLogMetabossProducts,
    chains: &mut BTreeMap<EdgeKey, ObservedSplitEdgeChainLedgerRow>,
) {
    for schedule in products.endpoint_boundary.schedules() {
        chains
            .entry(edge_key(
                schedule.source_edge_identity(),
                schedule.carrier_identity(),
            ))
            .or_default()
            .endpoint_boundary_schedule_identity = schedule.schedule_identity().to_string();
    }
}

fn bind_interval_schedules(
    products: &DecisionLogMetabossProducts,
    chains: &mut BTreeMap<EdgeKey, ObservedSplitEdgeChainLedgerRow>,
) {
    for schedule in products.interval_subdivision.schedules() {
        chains
            .entry(edge_key(
                schedule.source_edge_identity(),
                schedule.carrier_identity(),
            ))
            .or_default()
            .interval_subdivision_schedule_identity = schedule.schedule_identity().to_string();
    }
}

fn bind_vertices(
    products: &DecisionLogMetabossProducts,
    chains: &mut BTreeMap<EdgeKey, ObservedSplitEdgeChainLedgerRow>,
) {
    for schedule in products.vertices.schedules() {
        let row = chains
            .entry(edge_key(
                schedule.source_edge_identity(),
                schedule.carrier_identity(),
            ))
            .or_default();
        row.split_vertex_schedule_identity = schedule.schedule_identity().to_string();
        row.split_vertex_identities.extend(
            schedule
                .vertices()
                .iter()
                .map(|vertex| vertex.split_vertex_identity().to_string()),
        );
    }
}

fn bind_fragments(
    products: &DecisionLogMetabossProducts,
    chains: &mut BTreeMap<EdgeKey, ObservedSplitEdgeChainLedgerRow>,
) {
    for schedule in products.fragments.schedules() {
        let row = chains
            .entry(edge_key(
                schedule.source_edge_identity(),
                schedule.carrier_identity(),
            ))
            .or_default();
        row.split_fragment_schedule_identity = schedule.schedule_identity().to_string();
        row.fragment_identities.extend(
            schedule
                .fragments()
                .iter()
                .map(|fragment| fragment.fragment_identity().to_string()),
        );
    }
}

fn bind_overlap_chains(
    products: &DecisionLogMetabossProducts,
    chains: &mut BTreeMap<EdgeKey, ObservedSplitEdgeChainLedgerRow>,
) {
    for chain in products.chains.chains() {
        for member in chain.members() {
            chains
                .entry(edge_key(
                    member.source_edge_identity(),
                    member.carrier_identity(),
                ))
                .or_default()
                .overlap_chain_identities
                .push(chain.chain_identity().to_string());
        }
    }
    for row in chains.values_mut() {
        row.overlap_chain_identities.sort();
        row.overlap_chain_identities.dedup();
    }
}

fn bind_validation(
    products: &DecisionLogMetabossProducts,
    chains: &mut BTreeMap<EdgeKey, ObservedSplitEdgeChainLedgerRow>,
) {
    for row in products.validation.fragment_coverage_rows() {
        chains
            .entry(edge_key(row.source_edge_identity(), row.carrier_identity()))
            .or_default()
            .validation_fragment_coverage_identities
            .push(row.row_identity().to_string());
    }
    for row in products.validation.overlap_coverage_rows() {
        chains
            .entry(edge_key(row.source_edge_identity(), row.carrier_identity()))
            .or_default()
            .validation_overlap_coverage_identities
            .push(row.row_identity().to_string());
    }
}

fn bind_names_and_decisions(
    products: &DecisionLogMetabossProducts,
    decision_log: &PlanarBooleanSplitDecisionLogQueryResult,
    chains: &mut BTreeMap<EdgeKey, ObservedSplitEdgeChainLedgerRow>,
) {
    let names_by_artifact = products
        .naming
        .persistent_name_rows()
        .iter()
        .map(|row| {
            (
                row.artifact_identity(),
                (row.row_identity().to_string(), row.artifact_kind()),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut decisions_by_artifact = BTreeMap::<
        &str,
        Vec<(
            String,
            PlanarBooleanSplitDecisionPhase,
            PlanarBooleanSplitDecisionKind,
        )>,
    >::new();
    for decision in decision_log.receipt().decision_rows() {
        decisions_by_artifact
            .entry(decision.affected_artifact_identity())
            .or_default()
            .push((
                decision.decision_identity().to_string(),
                decision.phase(),
                decision.kind(),
            ));
    }
    for row in chains.values_mut() {
        let artifact_identities = row
            .fragment_identities
            .iter()
            .chain(row.split_vertex_identities.iter())
            .chain(row.overlap_chain_identities.iter())
            .cloned()
            .collect::<Vec<_>>();
        for identity in &artifact_identities {
            if let Some((row_identity, kind)) = names_by_artifact.get(identity.as_str()) {
                row.persistent_name_row_identities
                    .push(row_identity.clone());
                row.persistent_name_artifact_kinds.insert(*kind);
            }
            if let Some(decisions) = decisions_by_artifact.get(identity.as_str()) {
                for (decision_identity, phase, kind) in decisions {
                    row.decision_identities.push(decision_identity.clone());
                    row.decision_phases.insert(*phase);
                    row.decision_kinds.insert(*kind);
                }
            }
        }
    }
}

pub(crate) fn edge_key(source_edge_identity: &str, carrier_identity: &str) -> EdgeKey {
    (
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
    )
}
