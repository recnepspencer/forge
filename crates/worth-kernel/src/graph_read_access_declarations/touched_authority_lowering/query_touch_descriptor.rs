use forge_query::facade::{
    ForgeQueryGraphReadTouchShape, ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb,
};

use crate::graph_read_access_inventory::WorthGraphReadReadFamilyTarget;

use super::lowering_errors::{
    WorthGraphReadTouchedAuthorityLoweringError, WorthGraphReadTouchedAuthorityLoweringErrorKind,
};
use super::source_family::WorthGraphReadTouchedAuthoritySourceFamily;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthGraphReadQueryTouchDescriptorEvidence {
    descriptor_digest: String,
    collection_label: String,
    read_verb_digest: String,
}

impl WorthGraphReadQueryTouchDescriptorEvidence {
    pub(crate) fn from_lowered_authority_parts(
        source_family: WorthGraphReadTouchedAuthoritySourceFamily,
        target: WorthGraphReadReadFamilyTarget,
    ) -> Result<Self, WorthGraphReadTouchedAuthorityLoweringError> {
        let collection_label = collection_label(source_family, target).to_string();
        let read_verbs = read_verbs(source_family, target);
        let read_verb_digest = read_verbs
            .iter()
            .map(|verb| verb.as_str())
            .collect::<Vec<_>>()
            .join("|");
        let descriptor = ForgeQueryGraphTouchDescriptor::read_family_shape(
            collection_label.clone(),
            read_verbs,
            ForgeQueryGraphReadTouchShape::default(),
        )
        .map_err(|_| {
            WorthGraphReadTouchedAuthorityLoweringError::new(
                WorthGraphReadTouchedAuthorityLoweringErrorKind::QueryTouchDescriptorDenied,
            )
        })?;

        Ok(Self {
            descriptor_digest: descriptor.descriptor_digest().to_string(),
            collection_label,
            read_verb_digest,
        })
    }

    pub fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }

    pub fn collection_label(&self) -> &str {
        &self.collection_label
    }

    pub fn read_verb_digest(&self) -> &str {
        &self.read_verb_digest
    }
}

fn collection_label(
    source_family: WorthGraphReadTouchedAuthoritySourceFamily,
    target: WorthGraphReadReadFamilyTarget,
) -> &'static str {
    match source_family {
        WorthGraphReadTouchedAuthoritySourceFamily::TopologyClosure => match target {
            WorthGraphReadReadFamilyTarget::TopologyHalfEdgeSharedVertexNeighborhood => {
                "worth_topology_half_edge_shared_vertex_neighborhood"
            }
            WorthGraphReadReadFamilyTarget::TopologyHalfEdgeRadialNeighborhood => {
                "worth_topology_half_edge_radial_neighborhood"
            }
            WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood => {
                "worth_topology_loop_cycle_neighborhood"
            }
            WorthGraphReadReadFamilyTarget::TopologyLocalRewireNeighborhood => {
                "worth_topology_local_rewire_neighborhood"
            }
            WorthGraphReadReadFamilyTarget::SpatialPlanarBooleanContinuationIndex => {
                "worth_spatial_planar_boolean_continuation_index"
            }
            WorthGraphReadReadFamilyTarget::BroadBooleanPredicateGraphRead => {
                "worth_spatial_broad_boolean_predicate_graph_read"
            }
        },
        WorthGraphReadTouchedAuthoritySourceFamily::SpatialContinuation => match target {
            WorthGraphReadReadFamilyTarget::BroadBooleanPredicateGraphRead => {
                "worth_spatial_broad_boolean_predicate_graph_read"
            }
            _ => "worth_spatial_planar_boolean_continuation_index",
        },
    }
}

fn read_verbs(
    source_family: WorthGraphReadTouchedAuthoritySourceFamily,
    target: WorthGraphReadReadFamilyTarget,
) -> Vec<ForgeQueryGraphTouchReadVerb> {
    let mut verbs = vec![
        ForgeQueryGraphTouchReadVerb::ObservesCollection,
        ForgeQueryGraphTouchReadVerb::ObservesRelationKind,
        ForgeQueryGraphTouchReadVerb::ObservesAspect,
    ];
    if source_family == WorthGraphReadTouchedAuthoritySourceFamily::TopologyClosure
        || target == WorthGraphReadReadFamilyTarget::SpatialPlanarBooleanContinuationIndex
        || target == WorthGraphReadReadFamilyTarget::BroadBooleanPredicateGraphRead
    {
        verbs.push(ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology);
    }
    verbs
}
