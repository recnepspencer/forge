use std::cell::RefCell;
use std::collections::BTreeMap;

use super::error::WorthTopologyDomainQueryError;
use super::fallback::WorthTopologyDomainQueryFallbackPosture;
use super::lowering::lower_topology_domain_query;
use super::proof::WorthTopologyDomainQueryProofLedger;
use super::report::WorthTopologyDomainQueryAggregateReport;
use super::report::{WorthTopologyDomainQueryRequestFamily, WorthTopologyDomainQueryRequestReport};
use super::request::WorthTopologyDomainQueryRequest;
use super::views::{
    WorthTopologyHalfEdgeRadialNeighborhoodView, WorthTopologyHalfEdgeSharedVertexNeighborhoodView,
    WorthTopologyLocalRewireNeighborhoodView,
};
use crate::query::{WorthTopologyQueryAssembly, WorthTopologyQuerySnapshotIndex};
use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::identity::{EntityId, RelationId};
use serde_json::Value;
use worth_schema::facade::WorthTopologyRelationKind;
pub(crate) struct WorthTopologyDomainQuery {
    pub(super) snapshot_index: WorthTopologyQuerySnapshotIndex,
    #[allow(dead_code)]
    fallback_posture: WorthTopologyDomainQueryFallbackPosture,
    pub(super) request_reports: RefCell<Vec<WorthTopologyDomainQueryRequestReport>>,
    pub(super) proof_ledger: WorthTopologyDomainQueryProofLedger,
    local_rewire_cache:
        RefCell<BTreeMap<(String, usize), WorthTopologyLocalRewireNeighborhoodView>>,
}

impl WorthTopologyDomainQuery {
    const MAX_SUPPORTED_TRAVERSAL_DEPTH: usize = 64;
    fn snapshot_fallback_report(
        &self,
        request: &WorthTopologyDomainQueryRequest,
    ) -> Result<WorthTopologyDomainQueryRequestReport, WorthTopologyDomainQueryError> {
        let lowering_artifact = lower_topology_domain_query(request)?;
        Ok(WorthTopologyDomainQueryRequestReport::snapshot_indexed_fallback(lowering_artifact))
    }

    pub(super) fn record_report(
        &self,
        report: WorthTopologyDomainQueryRequestReport,
    ) -> WorthTopologyDomainQueryRequestReport {
        self.request_reports.borrow_mut().push(report.clone());
        report
    }

    pub(super) fn require_supported_traversal_depth(
        request_family: WorthTopologyDomainQueryRequestFamily,
        requested_depth: usize,
    ) -> Result<(), WorthTopologyDomainQueryError> {
        if requested_depth == 0 || requested_depth > Self::MAX_SUPPORTED_TRAVERSAL_DEPTH {
            return Err(WorthTopologyDomainQueryError::unsupported_traversal_depth(
                request_family,
                requested_depth,
                Self::MAX_SUPPORTED_TRAVERSAL_DEPTH,
            ));
        }
        Ok(())
    }

    pub(crate) fn load(
        workspace: &ForgeQueryWorkspace,
        assembly: &WorthTopologyQueryAssembly,
    ) -> Result<Self, WorthTopologyDomainQueryError> {
        let entity_rows = workspace.read::<Value>(assembly.entities());
        let relation_rows = workspace.read::<Value>(assembly.relations());
        let snapshot_index = WorthTopologyQuerySnapshotIndex::new(&entity_rows, &relation_rows)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        Ok(Self {
            snapshot_index,
            fallback_posture: WorthTopologyDomainQueryFallbackPosture::SnapshotIndexedFallback,
            request_reports: RefCell::new(Vec::new()),
            proof_ledger: WorthTopologyDomainQueryProofLedger::default(),
            local_rewire_cache: RefCell::new(BTreeMap::new()),
        })
    }

    #[allow(dead_code)]
    pub(crate) fn fallback_posture(&self) -> WorthTopologyDomainQueryFallbackPosture {
        self.fallback_posture
    }

    pub(crate) fn first_source_identity_for_relation_kind(
        &self,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<String, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .first_source_identity_for_relation_kind(relation_kind)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    pub(crate) fn find_entity_id_by_identity(
        &self,
        identity: &str,
    ) -> Result<EntityId, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .find_entity_id_by_identity(identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    pub(crate) fn find_entity_identity_by_id(
        &self,
        entity_id: EntityId,
    ) -> Result<String, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .find_entity_identity_by_id(entity_id)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    pub(crate) fn relation_id_for_source_kind(
        &self,
        source_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<RelationId, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .relation_id_for_source_kind(source_identity, relation_kind)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    pub(crate) fn relation_id_by_kind_and_endpoints(
        &self,
        source_identity: &str,
        target_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<RelationId, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .relation_id_by_kind_and_endpoints(source_identity, target_identity, relation_kind)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    pub(crate) fn outgoing_target_identity(
        &self,
        source_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<String, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .outgoing_target_identity(source_identity, relation_kind)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    #[cfg(test)]
    pub(crate) fn incoming_source_identity(
        &self,
        target_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<String, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .incoming_source_identity(target_identity, relation_kind)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    pub(crate) fn edge_identity_of_half_edge(
        &self,
        half_edge_identity: &str,
    ) -> Result<String, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .edge_identity_of_half_edge(half_edge_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    pub(crate) fn half_edge_vertex_identities(
        &self,
        half_edge_identity: &str,
    ) -> Result<Vec<String>, WorthTopologyDomainQueryError> {
        self.snapshot_index
            .half_edge_vertex_identities(half_edge_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })
    }

    #[allow(dead_code)]
    pub(crate) fn supported_request_families(&self) -> Vec<WorthTopologyDomainQueryRequestFamily> {
        vec![
            WorthTopologyDomainQueryRequest::HalfEdgeSharedVertexNeighborhood {
                source_half_edge_identity: "worth.topology.support".to_string(),
            }
            .family(),
            WorthTopologyDomainQueryRequest::HalfEdgeRadialNeighborhood {
                source_half_edge_identity: "worth.topology.support".to_string(),
            }
            .family(),
            WorthTopologyDomainQueryRequest::LoopCycleNeighborhood {
                start_half_edge_identity: "worth.topology.support".to_string(),
                depth: 1,
            }
            .family(),
            WorthTopologyDomainQueryRequest::LocalRewireNeighborhood {
                moved_half_edge_identity: "worth.topology.support".to_string(),
                cycle_depth: 1,
            }
            .family(),
        ]
    }

    #[allow(dead_code)]
    pub(crate) fn aggregate_report(&self) -> WorthTopologyDomainQueryAggregateReport {
        WorthTopologyDomainQueryAggregateReport::from_request_reports(
            self.request_reports.borrow().as_slice(),
        )
    }

    pub(crate) fn shared_vertex_half_edge_neighborhood(
        &self,
        source_identity: &str,
    ) -> Result<WorthTopologyHalfEdgeSharedVertexNeighborhoodView, WorthTopologyDomainQueryError>
    {
        let request = WorthTopologyDomainQueryRequest::HalfEdgeSharedVertexNeighborhood {
            source_half_edge_identity: source_identity.to_string(),
        };
        let source_edge_identity = self
            .snapshot_index
            .edge_identity_of_half_edge(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let source_vertex_identities = self
            .snapshot_index
            .half_edge_vertex_identities(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let vertex_adjacent_half_edge_identities = self
            .snapshot_index
            .half_edge_identities_sharing_vertex(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let vertex_adjacent_different_edge_half_edge_identities =
            vertex_adjacent_half_edge_identities
                .iter()
                .filter(|identity| {
                    self.snapshot_index
                        .edge_identity_of_half_edge(identity)
                        .is_ok_and(|edge_identity| edge_identity != source_edge_identity)
                })
                .cloned()
                .collect();
        Ok(WorthTopologyHalfEdgeSharedVertexNeighborhoodView {
            request_report: self.record_report(self.snapshot_fallback_report(&request)?),
            source_half_edge_identity: source_identity.to_string(),
            source_edge_identity,
            source_vertex_identities,
            vertex_adjacent_half_edge_identities,
            vertex_adjacent_different_edge_half_edge_identities,
        })
    }

    pub(crate) fn radial_half_edge_neighborhood(
        &self,
        source_identity: &str,
    ) -> Result<WorthTopologyHalfEdgeRadialNeighborhoodView, WorthTopologyDomainQueryError> {
        let request = WorthTopologyDomainQueryRequest::HalfEdgeRadialNeighborhood {
            source_half_edge_identity: source_identity.to_string(),
        };
        let source_edge_identity = self
            .snapshot_index
            .edge_identity_of_half_edge(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let current_target_half_edge_identity = self
            .snapshot_index
            .outgoing_target_identity(
                source_identity,
                WorthTopologyRelationKind::HalfEdgeRadialNext,
            )
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let current_target_edge_identity = self
            .snapshot_index
            .edge_identity_of_half_edge(&current_target_half_edge_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let same_edge_half_edge_identities = self
            .snapshot_index
            .half_edge_identities_on_same_edge(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let different_edge_half_edge_identities = self
            .snapshot_index
            .half_edge_identities_on_different_edge(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        Ok(WorthTopologyHalfEdgeRadialNeighborhoodView {
            request_report: self.record_report(self.snapshot_fallback_report(&request)?),
            source_half_edge_identity: source_identity.to_string(),
            source_edge_identity,
            current_target_half_edge_identity,
            current_target_edge_identity,
            same_edge_half_edge_identities,
            different_edge_half_edge_identities,
        })
    }
    pub(crate) fn local_rewire_neighborhood(
        &self,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<WorthTopologyLocalRewireNeighborhoodView, WorthTopologyDomainQueryError> {
        let request = WorthTopologyDomainQueryRequest::LocalRewireNeighborhood {
            moved_half_edge_identity: moved_identity.to_string(),
            cycle_depth: u8::try_from(cycle_count)
                .expect("supported traversal depth must fit in u8"),
        };
        if let Some(cached) = self
            .local_rewire_cache
            .borrow()
            .get(&(moved_identity.to_string(), cycle_count))
            .cloned()
        {
            return Ok(cached);
        }
        Self::require_supported_traversal_depth(
            WorthTopologyDomainQueryRequestFamily::LocalRewireNeighborhood,
            cycle_count,
        )?;
        let old_successor_identity = self
            .snapshot_index
            .outgoing_target_identity(moved_identity, WorthTopologyRelationKind::HalfEdgeNext)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let old_predecessor_identity = self
            .snapshot_index
            .outgoing_target_identity(moved_identity, WorthTopologyRelationKind::HalfEdgePrev)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let cycle_identities = self
            .snapshot_index
            .successor_cycle_identities(moved_identity, cycle_count)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let neighborhood = WorthTopologyLocalRewireNeighborhoodView {
            request_report: self.record_report(self.snapshot_fallback_report(&request)?),
            moved_half_edge_identity: moved_identity.to_string(),
            old_successor_identity,
            old_predecessor_identity,
            cycle_identities,
        };
        self.local_rewire_cache.borrow_mut().insert(
            (moved_identity.to_string(), cycle_count),
            neighborhood.clone(),
        );
        Ok(neighborhood)
    }
}
