use crate::graph_read_access_inventory::{
    WorthGraphReadAccessInventoryRowContext, WorthGraphReadReadFamilyTarget,
    WorthGraphReadRequirementVocabulary,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum WorthGraphReadCatalogAccessShape {
    TopologyNeighborhoodGraphRead,
    TopologyLocalRewireGraphRead,
    SpatialContinuationIndexGraphRead,
    BroadBooleanPredicateGraphRead,
}

impl WorthGraphReadCatalogAccessShape {
    pub(crate) const fn from_target(target: WorthGraphReadReadFamilyTarget) -> Self {
        match target {
            WorthGraphReadReadFamilyTarget::TopologyHalfEdgeSharedVertexNeighborhood
            | WorthGraphReadReadFamilyTarget::TopologyHalfEdgeRadialNeighborhood
            | WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood => {
                Self::TopologyNeighborhoodGraphRead
            }
            WorthGraphReadReadFamilyTarget::TopologyLocalRewireNeighborhood => {
                Self::TopologyLocalRewireGraphRead
            }
            WorthGraphReadReadFamilyTarget::SpatialPlanarBooleanContinuationIndex => {
                Self::SpatialContinuationIndexGraphRead
            }
            WorthGraphReadReadFamilyTarget::BroadBooleanPredicateGraphRead => {
                Self::BroadBooleanPredicateGraphRead
            }
        }
    }

    pub(crate) const fn digest_part(&self) -> &'static str {
        match self {
            Self::TopologyNeighborhoodGraphRead => "topology_neighborhood_graph_read",
            Self::TopologyLocalRewireGraphRead => "topology_local_rewire_graph_read",
            Self::SpatialContinuationIndexGraphRead => "spatial_continuation_index_graph_read",
            Self::BroadBooleanPredicateGraphRead => "broad_boolean_predicate_graph_read",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthGraphReadCatalogSelectivityPosture {
    digest_part: String,
}

impl WorthGraphReadCatalogSelectivityPosture {
    pub(crate) fn from_context(context: &WorthGraphReadAccessInventoryRowContext) -> Self {
        Self {
            digest_part: format!("cost_posture:{:?}", context.cost_posture()),
        }
    }

    pub(crate) fn digest_part(&self) -> &str {
        &self.digest_part
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthGraphReadCatalogBasisSnapshotPosture {
    digest_part: String,
}

impl WorthGraphReadCatalogBasisSnapshotPosture {
    pub(crate) fn from_vocabulary(vocabulary: &WorthGraphReadRequirementVocabulary) -> Self {
        Self {
            digest_part: format!(
                "rebuild:{:?}|invalidate:{:?}",
                vocabulary.rebuild_basis(),
                vocabulary.invalidation_basis()
            ),
        }
    }

    pub(crate) fn digest_part(&self) -> &str {
        &self.digest_part
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthGraphReadCatalogPolicyTenantPosture {
    digest_part: String,
}

impl WorthGraphReadCatalogPolicyTenantPosture {
    pub(crate) fn from_context(context: &WorthGraphReadAccessInventoryRowContext) -> Self {
        Self {
            digest_part: format!(
                "worth_policy_tenant_not_declared_in_phase_2|owner:{:?}",
                context.identity().owner()
            ),
        }
    }

    pub(crate) fn digest_part(&self) -> &str {
        &self.digest_part
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthGraphReadCatalogSupportPosture {
    digest_part: String,
}

impl WorthGraphReadCatalogSupportPosture {
    pub(crate) fn from_context(context: &WorthGraphReadAccessInventoryRowContext) -> Self {
        Self {
            digest_part: format!(
                "phase_2_catalog_support_only|classification:{:?}|disposition:{:?}",
                context.classification(),
                context.milestone_seven_disposition()
            ),
        }
    }

    pub(crate) fn digest_part(&self) -> &str {
        &self.digest_part
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct WorthGraphReadCatalogMilestoneEightAdoptionTarget {
    digest_part: String,
}

impl WorthGraphReadCatalogMilestoneEightAdoptionTarget {
    pub(crate) fn from_target(target: WorthGraphReadReadFamilyTarget) -> Self {
        Self {
            digest_part: format!("milestone_8_access_plan_adoption:{}", target.as_str()),
        }
    }

    pub(crate) fn digest_part(&self) -> &str {
        &self.digest_part
    }
}
