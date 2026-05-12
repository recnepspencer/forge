use super::super::error::TopologyDomainQueryError;
use super::super::request::TopologyDomainQueryRequest;
use super::super::topology::TopologyDomainQuery;
use crate::projection::diagnostic_surfaces::read_proof::report::TopologyDomainQueryRequestFamily;
use crate::projection::read_views::{
    TopologyLocalRewireNeighborhoodView, TopologyLoopNeighborEvidence,
};
use crate::projection::runtime_boundary::read_execution::{
    execute_local_rewire_read, prev_relation_name, successor_relation_name,
};
use crate::projection::runtime_boundary::read_execution::{
    relation_identity, relation_record_identity, row_payload,
};
use forge_query::facade::ForgeQueryWorkspace;

impl TopologyDomainQuery {
    pub fn local_rewire_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyDomainQueryError> {
        let request = TopologyDomainQueryRequest::LocalRewireNeighborhood {
            moved_half_edge_identity: moved_identity.to_string(),
            cycle_depth: u8::try_from(cycle_count)
                .expect("supported traversal depth must fit in u8"),
        };
        Self::require_supported_traversal_depth(
            TopologyDomainQueryRequestFamily::LocalRewireNeighborhood,
            cycle_count,
        )?;
        let executed = execute_local_rewire_read(workspace, &request, moved_identity, cycle_count)?;
        let request_report = self.record_report(executed.report);
        let moved_row = row_payload(
            executed.result.rows(),
            moved_identity,
            "local rewire neighborhood",
        )?;
        let old_successor_identity = relation_identity(
            Some(moved_row),
            &successor_relation_name(),
            "local rewire neighborhood",
        )?;
        let old_predecessor_identity = relation_identity(
            Some(moved_row),
            &prev_relation_name(),
            "local rewire neighborhood",
        )?;
        let cycle_identities =
            decode_cycle_identities(executed.result.rows(), moved_identity, cycle_count)?;
        let cycle_half_edges = loop_neighbor_evidence(
            executed.result.rows(),
            &cycle_identities,
            "local rewire neighborhood",
        )?;
        let neighborhood = TopologyLocalRewireNeighborhoodView {
            request_report,
            moved_half_edge_identity: moved_identity.to_string(),
            old_successor_identity: old_successor_identity.to_string(),
            old_predecessor_identity: old_predecessor_identity.to_string(),
            cycle_identities,
            cycle_half_edges,
        };
        Ok(neighborhood)
    }
}

fn loop_neighbor_evidence(
    rows: &[forge_query::facade::ForgeQueryEntity],
    cycle_identities: &[String],
    label: &str,
) -> Result<Vec<TopologyLoopNeighborEvidence>, TopologyDomainQueryError> {
    cycle_identities
        .iter()
        .map(|identity| {
            let payload = row_payload(rows, identity, label)?;
            Ok(TopologyLoopNeighborEvidence {
                half_edge_identity: identity.clone(),
                next_half_edge_identity: relation_identity(
                    Some(payload),
                    &successor_relation_name(),
                    label,
                )?
                .to_string(),
                previous_half_edge_identity: relation_identity(
                    Some(payload),
                    &prev_relation_name(),
                    label,
                )?
                .to_string(),
                next_relation_identity: relation_record_identity(
                    Some(payload),
                    &successor_relation_name(),
                    label,
                )?
                .to_string(),
                previous_relation_identity: relation_record_identity(
                    Some(payload),
                    &prev_relation_name(),
                    label,
                )?
                .to_string(),
            })
        })
        .collect()
}

fn decode_cycle_identities(
    rows: &[forge_query::facade::ForgeQueryEntity],
    moved_identity: &str,
    count: usize,
) -> Result<Vec<String>, TopologyDomainQueryError> {
    let mut cycle = Vec::with_capacity(count);
    let mut current = moved_identity;
    for _ in 0..count {
        cycle.push(current.to_string());
        current = relation_identity(
            Some(row_payload(rows, current, "local rewire neighborhood")?),
            &successor_relation_name(),
            "local rewire neighborhood",
        )?;
    }
    Ok(cycle)
}
