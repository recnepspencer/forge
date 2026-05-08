use std::collections::BTreeSet;

use forge_query::facade::{ForgeQueryEntity, ForgeQueryWorkspace};

use super::super::error::TopologyDomainQueryError;
use super::super::execution::{
    ends_at_vertex_relation_name, execute_shared_neighborhood_read, radial_next_relation_name,
    starts_at_vertex_relation_name, uses_edge_relation_name, ExecutedTopologyReadFamily,
    SharedNeighborhoodReadKind,
};
use super::super::execution::{
    relation_identities, relation_identity, relation_record_identity, row_payload,
};
use super::super::request::TopologyDomainQueryRequest;
use super::super::topology::TopologyDomainQuery;
use super::models::{
    TopologyAdjacentHalfEdgeEvidence, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyRadialCandidateEvidence,
};

impl TopologyDomainQuery {
    pub fn shared_vertex_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyDomainQueryError> {
        let request = TopologyDomainQueryRequest::HalfEdgeSharedVertexNeighborhood {
            source_half_edge_identity: source_identity.to_string(),
        };
        let executed = self.execute_shared_vertex_read(workspace, &request, source_identity)?;
        let request_report = self.record_report(executed.report);
        let rows = executed.result.rows();
        let source_row = row_payload(rows, source_identity, "shared-vertex neighborhood")?;
        let source_edge_identity = relation_identity(
            Some(source_row),
            &uses_edge_relation_name(),
            "shared-vertex neighborhood",
        )?
        .to_string();
        let source_vertex_identities = relation_identities(
            Some(source_row),
            &[
                starts_at_vertex_relation_name(),
                ends_at_vertex_relation_name(),
            ],
            "shared-vertex neighborhood",
        )?;
        let vertex_adjacent_half_edge_identities =
            adjacent_half_edges_sharing_vertices(rows, source_identity, &source_vertex_identities)?;
        let vertex_adjacent_different_edge_half_edge_identities = filter_different_edge_half_edges(
            rows,
            &vertex_adjacent_half_edge_identities,
            &source_edge_identity,
            "shared-vertex neighborhood",
        )?;
        let vertex_adjacent_different_edge_half_edges = adjacent_different_edge_half_edge_evidence(
            rows,
            &source_vertex_identities,
            &vertex_adjacent_different_edge_half_edge_identities,
        )?;
        Ok(TopologyHalfEdgeSharedVertexNeighborhoodView {
            request_report,
            source_half_edge_identity: source_identity.to_string(),
            source_edge_identity,
            source_vertex_identities,
            vertex_adjacent_half_edge_identities,
            vertex_adjacent_different_edge_half_edge_identities,
            vertex_adjacent_different_edge_half_edges,
        })
    }

    pub fn radial_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeRadialNeighborhoodView, TopologyDomainQueryError> {
        let request = TopologyDomainQueryRequest::HalfEdgeRadialNeighborhood {
            source_half_edge_identity: source_identity.to_string(),
        };
        let executed = self.execute_radial_read(workspace, &request, source_identity)?;
        let request_report = self.record_report(executed.report);
        let rows = executed.result.rows();
        let source_row = row_payload(rows, source_identity, "radial neighborhood")?;
        let source_edge_identity = relation_identity(
            Some(source_row),
            &uses_edge_relation_name(),
            "radial neighborhood",
        )?
        .to_string();
        let current_target_half_edge_identity = relation_identity(
            Some(source_row),
            &radial_next_relation_name(),
            "radial neighborhood",
        )?
        .to_string();
        let current_target_edge_identity = relation_identity(
            Some(row_payload(
                rows,
                &current_target_half_edge_identity,
                "radial neighborhood",
            )?),
            &uses_edge_relation_name(),
            "radial neighborhood",
        )?
        .to_string();
        let source_radial_next_relation_identity = relation_record_identity(
            Some(source_row),
            &radial_next_relation_name(),
            "radial neighborhood",
        )?
        .to_string();
        let same_edge_half_edge_identities =
            filter_same_edge_half_edges(rows, source_identity, &source_edge_identity)?;
        let different_edge_half_edge_identities = filter_different_edge_half_edges_from_rows(
            rows,
            source_identity,
            &source_edge_identity,
        )?;
        let different_edge_half_edges =
            radial_different_edge_half_edge_evidence(rows, &different_edge_half_edge_identities)?;
        Ok(TopologyHalfEdgeRadialNeighborhoodView {
            request_report,
            source_half_edge_identity: source_identity.to_string(),
            source_edge_identity,
            current_target_half_edge_identity,
            current_target_edge_identity,
            source_radial_next_relation_identity,
            same_edge_half_edge_identities,
            different_edge_half_edge_identities,
            different_edge_half_edges,
        })
    }

    fn execute_shared_vertex_read(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        request: &TopologyDomainQueryRequest,
        source_identity: &str,
    ) -> Result<ExecutedTopologyReadFamily, TopologyDomainQueryError> {
        execute_shared_neighborhood_read(
            workspace,
            request,
            format!("topology.shared_vertex_neighborhood:{source_identity}"),
            [
                starts_at_vertex_relation_name(),
                ends_at_vertex_relation_name(),
            ],
            SharedNeighborhoodReadKind::SharedEndpoint,
            source_identity,
        )
    }

    fn execute_radial_read(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        request: &TopologyDomainQueryRequest,
        source_identity: &str,
    ) -> Result<ExecutedTopologyReadFamily, TopologyDomainQueryError> {
        execute_shared_neighborhood_read(
            workspace,
            request,
            format!("topology.radial_neighborhood:{source_identity}"),
            [radial_next_relation_name(), uses_edge_relation_name()],
            SharedNeighborhoodReadKind::SharedAttachment,
            source_identity,
        )
    }
}

fn adjacent_different_edge_half_edge_evidence(
    rows: &[ForgeQueryEntity],
    source_vertex_identities: &[String],
    adjacent_half_edge_identities: &[String],
) -> Result<Vec<TopologyAdjacentHalfEdgeEvidence>, TopologyDomainQueryError> {
    adjacent_half_edge_identities
        .iter()
        .map(|identity| {
            let payload = row_payload(rows, identity, "shared-vertex neighborhood")?;
            let edge_identity = relation_identity(
                Some(payload),
                &uses_edge_relation_name(),
                "shared-vertex neighborhood",
            )?
            .to_string();
            let candidate_vertex_identities = relation_identities(
                Some(payload),
                &[
                    starts_at_vertex_relation_name(),
                    ends_at_vertex_relation_name(),
                ],
                "shared-vertex neighborhood",
            )?;
            let shared_vertex_identities = candidate_vertex_identities
                .into_iter()
                .filter(|candidate| source_vertex_identities.contains(candidate))
                .collect();
            Ok(TopologyAdjacentHalfEdgeEvidence {
                half_edge_identity: identity.clone(),
                edge_identity,
                shared_vertex_identities,
            })
        })
        .collect()
}

fn radial_different_edge_half_edge_evidence(
    rows: &[ForgeQueryEntity],
    different_edge_half_edge_identities: &[String],
) -> Result<Vec<TopologyRadialCandidateEvidence>, TopologyDomainQueryError> {
    different_edge_half_edge_identities
        .iter()
        .map(|identity| {
            let edge_identity = relation_identity(
                Some(row_payload(rows, identity, "radial neighborhood")?),
                &uses_edge_relation_name(),
                "radial neighborhood",
            )?
            .to_string();
            Ok(TopologyRadialCandidateEvidence {
                half_edge_identity: identity.clone(),
                edge_identity,
            })
        })
        .collect()
}

fn adjacent_half_edges_sharing_vertices(
    rows: &[ForgeQueryEntity],
    source_identity: &str,
    source_vertex_identities: &[String],
) -> Result<Vec<String>, TopologyDomainQueryError> {
    rows.iter()
        .filter(|row| row.identity != source_identity)
        .try_fold(BTreeSet::new(), |mut identities, row| {
            let row_vertex_identities = relation_identities(
                Some(&row.payload),
                &[
                    starts_at_vertex_relation_name(),
                    ends_at_vertex_relation_name(),
                ],
                "shared-vertex neighborhood",
            )?;
            if row_vertex_identities
                .iter()
                .any(|vertex| source_vertex_identities.contains(vertex))
            {
                identities.insert(row.identity.clone());
            }
            Ok(identities)
        })
        .map(|identities| identities.into_iter().collect())
}

fn filter_same_edge_half_edges(
    rows: &[ForgeQueryEntity],
    source_identity: &str,
    source_edge_identity: &str,
) -> Result<Vec<String>, TopologyDomainQueryError> {
    filter_half_edges_by_edge(rows, source_identity, source_edge_identity, true)
}

fn filter_different_edge_half_edges(
    rows: &[ForgeQueryEntity],
    adjacent_half_edge_identities: &[String],
    source_edge_identity: &str,
    label: &str,
) -> Result<Vec<String>, TopologyDomainQueryError> {
    Ok(adjacent_half_edge_identities
        .iter()
        .filter_map(|identity| {
            row_payload(rows, identity, label)
                .ok()
                .and_then(|payload| {
                    relation_identity(Some(payload), &uses_edge_relation_name(), label)
                        .ok()
                        .filter(|edge_identity| *edge_identity != source_edge_identity)
                })
                .map(|_| identity.clone())
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect())
}

fn filter_different_edge_half_edges_from_rows(
    rows: &[ForgeQueryEntity],
    source_identity: &str,
    source_edge_identity: &str,
) -> Result<Vec<String>, TopologyDomainQueryError> {
    filter_half_edges_by_edge(rows, source_identity, source_edge_identity, false)
}

fn filter_half_edges_by_edge(
    rows: &[ForgeQueryEntity],
    source_identity: &str,
    source_edge_identity: &str,
    same_edge: bool,
) -> Result<Vec<String>, TopologyDomainQueryError> {
    rows.iter()
        .filter(|row| row.identity != source_identity)
        .try_fold(BTreeSet::new(), |mut identities, row| {
            let edge_identity = relation_identity(
                Some(&row.payload),
                &uses_edge_relation_name(),
                "radial neighborhood",
            )?;
            if (edge_identity == source_edge_identity) == same_edge {
                identities.insert(row.identity.clone());
            }
            Ok(identities)
        })
        .map(|identities| identities.into_iter().collect())
}
