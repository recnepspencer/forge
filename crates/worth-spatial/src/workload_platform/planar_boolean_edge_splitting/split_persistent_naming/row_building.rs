use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::{
    overlap_edge_chains::PlanarBooleanOverlapEdgeChainSet,
    split_edge_fragments::PlanarBooleanSplitEdgeFragmentSet,
    split_vertex_identity::PlanarBooleanSplitVertexIdentitySet,
};

use super::counters::PlanarBooleanSplitPersistentNamingCounters;
use super::denial::{
    PlanarBooleanSplitPersistentNamingDenial, PlanarBooleanSplitPersistentNamingDenialKind,
};
use super::naming_row::{PlanarBooleanSplitNamedArtifactKind, PlanarBooleanSplitPersistentNameRow};
use super::query_evolution::PlanarBooleanSplitIdentityEvolutionRow;

pub(crate) fn build_persistent_name_rows(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    vertices: &PlanarBooleanSplitVertexIdentitySet,
    chains: &PlanarBooleanOverlapEdgeChainSet,
    evolution_rows: &[PlanarBooleanSplitIdentityEvolutionRow],
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
) -> Result<Vec<PlanarBooleanSplitPersistentNameRow>, PlanarBooleanSplitPersistentNamingDenial> {
    let evolution_by_source = evolution_rows
        .iter()
        .map(|row| (row.source_edge_identity().to_string(), row))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    push_fragment_name_rows(fragments, &evolution_by_source, counters, &mut rows)?;
    push_vertex_name_rows(vertices, &evolution_by_source, counters, &mut rows)?;
    push_overlap_chain_name_rows(chains, &evolution_by_source, counters, &mut rows)?;
    rows.sort_by(|left, right| row_order_key(left).cmp(&row_order_key(right)));
    rows.dedup_by(|left, right| left.row_identity() == right.row_identity());
    counters.set_named_split_artifacts(rows.len());
    Ok(rows)
}

fn push_fragment_name_rows(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    evolution_by_source: &BTreeMap<String, &PlanarBooleanSplitIdentityEvolutionRow>,
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
    rows: &mut Vec<PlanarBooleanSplitPersistentNameRow>,
) -> Result<(), PlanarBooleanSplitPersistentNamingDenial> {
    for fragment in fragments.fragments() {
        let evolution = required_evolution(
            evolution_by_source,
            fragment.source_edge_identity(),
            counters,
        )?;
        rows.push(PlanarBooleanSplitPersistentNameRow::new(
            fragment.source_edge_identity(),
            PlanarBooleanSplitNamedArtifactKind::SplitFragment,
            fragment.fragment_identity(),
            fragment.cause_provenance_identities().to_vec(),
            evolution,
        ));
        counters.named_split_artifact();
        for cause in fragment.cause_provenance_identities() {
            rows.push(PlanarBooleanSplitPersistentNameRow::new(
                fragment.source_edge_identity(),
                PlanarBooleanSplitNamedArtifactKind::EventCause,
                cause,
                vec![fragment.fragment_identity().to_string()],
                evolution,
            ));
            counters.named_split_artifact();
        }
    }
    Ok(())
}

fn push_vertex_name_rows(
    vertices: &PlanarBooleanSplitVertexIdentitySet,
    evolution_by_source: &BTreeMap<String, &PlanarBooleanSplitIdentityEvolutionRow>,
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
    rows: &mut Vec<PlanarBooleanSplitPersistentNameRow>,
) -> Result<(), PlanarBooleanSplitPersistentNamingDenial> {
    for vertex in vertices.vertices() {
        let evolution =
            required_evolution(evolution_by_source, vertex.source_edge_identity(), counters)?;
        rows.push(PlanarBooleanSplitPersistentNameRow::new(
            vertex.source_edge_identity(),
            PlanarBooleanSplitNamedArtifactKind::SplitVertex,
            vertex.split_vertex_identity(),
            vertex.coalescence_provenance().to_vec(),
            evolution,
        ));
        counters.named_split_artifact();
    }
    Ok(())
}

fn push_overlap_chain_name_rows(
    chains: &PlanarBooleanOverlapEdgeChainSet,
    evolution_by_source: &BTreeMap<String, &PlanarBooleanSplitIdentityEvolutionRow>,
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
    rows: &mut Vec<PlanarBooleanSplitPersistentNameRow>,
) -> Result<(), PlanarBooleanSplitPersistentNamingDenial> {
    for chain in chains.chains() {
        for member in chain.members() {
            let evolution =
                required_evolution(evolution_by_source, member.source_edge_identity(), counters)?;
            rows.push(PlanarBooleanSplitPersistentNameRow::new(
                member.source_edge_identity(),
                PlanarBooleanSplitNamedArtifactKind::OverlapChain,
                &format!(
                    "{}:member:{}",
                    chain.chain_identity(),
                    member.member_identity()
                ),
                chain.event_group_identities().to_vec(),
                evolution,
            ));
            counters.named_split_artifact();
            rows.push(PlanarBooleanSplitPersistentNameRow::new(
                member.source_edge_identity(),
                PlanarBooleanSplitNamedArtifactKind::RetainedInterval,
                member.source_interval_identity(),
                vec![chain.interval_event_identity().to_string()],
                evolution,
            ));
            counters.named_split_artifact();
        }
    }
    Ok(())
}

fn row_order_key(row: &PlanarBooleanSplitPersistentNameRow) -> String {
    format!(
        "{}:{}:{}",
        row.source_edge_identity(),
        row.artifact_kind().as_str(),
        row.artifact_identity()
    )
}

fn required_evolution<'a>(
    evolution_by_source: &'a BTreeMap<String, &PlanarBooleanSplitIdentityEvolutionRow>,
    source_edge_identity: &str,
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
) -> Result<&'a PlanarBooleanSplitIdentityEvolutionRow, PlanarBooleanSplitPersistentNamingDenial> {
    evolution_by_source
        .get(source_edge_identity)
        .copied()
        .ok_or_else(|| {
            counters.rejected_foreign_artifact();
            PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference,
                source_edge_identity,
                "split persistent naming requires every split artifact to bind a Query identity-evolution row",
            )
        })
}
