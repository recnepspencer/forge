#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LiveMaintenanceCostClass {
    DetailPatch,
    OrderedCollectionPatch,
    BoundedMaterializationPatch,
    RefreshFallback,
}

impl LiveMaintenanceCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailPatch => "detail_patch",
            Self::OrderedCollectionPatch => "ordered_collection_patch",
            Self::BoundedMaterializationPatch => "bounded_materialization_patch",
            Self::RefreshFallback => "refresh_fallback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LiveMaintenanceWorkUnit {
    ProjectedFieldDeltaCount,
    DerivedFieldRecomputationCount,
    MembershipDeltaCount,
    OrderingRepositionCount,
    PageLocalMoveCount,
    CrossPageMoveCount,
    InScopeNodeDeltaCount,
    InScopeEdgeDeltaCount,
    ScopeExpansionCount,
    ScopeContractionCount,
}

impl LiveMaintenanceWorkUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectedFieldDeltaCount => "projected_field_delta_count",
            Self::DerivedFieldRecomputationCount => "derived_field_recomputation_count",
            Self::MembershipDeltaCount => "membership_delta_count",
            Self::OrderingRepositionCount => "ordering_reposition_count",
            Self::PageLocalMoveCount => "page_local_move_count",
            Self::CrossPageMoveCount => "cross_page_move_count",
            Self::InScopeNodeDeltaCount => "in_scope_node_delta_count",
            Self::InScopeEdgeDeltaCount => "in_scope_edge_delta_count",
            Self::ScopeExpansionCount => "scope_expansion_count",
            Self::ScopeContractionCount => "scope_contraction_count",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveMaintenanceComplexityContract {
    cost_class: LiveMaintenanceCostClass,
    work_units: Vec<LiveMaintenanceWorkUnit>,
}

impl LiveMaintenanceComplexityContract {
    pub fn cost_class(&self) -> &LiveMaintenanceCostClass {
        &self.cost_class
    }

    pub fn work_units(&self) -> &[LiveMaintenanceWorkUnit] {
        &self.work_units
    }

    pub fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![format!("live_cost_class:{}", self.cost_class.as_str())];
        parts.extend(
            self.work_units
                .iter()
                .map(|unit| format!("live_work_unit:{}", unit.as_str())),
        );
        parts
    }

    pub fn detail_patch() -> Self {
        Self {
            cost_class: LiveMaintenanceCostClass::DetailPatch,
            work_units: vec![
                LiveMaintenanceWorkUnit::ProjectedFieldDeltaCount,
                LiveMaintenanceWorkUnit::DerivedFieldRecomputationCount,
            ],
        }
    }

    pub fn ordered_collection_patch() -> Self {
        Self {
            cost_class: LiveMaintenanceCostClass::OrderedCollectionPatch,
            work_units: vec![
                LiveMaintenanceWorkUnit::MembershipDeltaCount,
                LiveMaintenanceWorkUnit::OrderingRepositionCount,
                LiveMaintenanceWorkUnit::PageLocalMoveCount,
                LiveMaintenanceWorkUnit::CrossPageMoveCount,
            ],
        }
    }

    pub fn bounded_materialization_patch() -> Self {
        Self {
            cost_class: LiveMaintenanceCostClass::BoundedMaterializationPatch,
            work_units: vec![
                LiveMaintenanceWorkUnit::InScopeNodeDeltaCount,
                LiveMaintenanceWorkUnit::InScopeEdgeDeltaCount,
                LiveMaintenanceWorkUnit::ScopeExpansionCount,
                LiveMaintenanceWorkUnit::ScopeContractionCount,
            ],
        }
    }

    pub fn refresh_fallback() -> Self {
        Self {
            cost_class: LiveMaintenanceCostClass::RefreshFallback,
            work_units: Vec::new(),
        }
    }
}
