use super::query_contract::QueryFieldKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeFieldDelta {
    field: QueryFieldKey,
    old_value: Option<String>,
    new_value: Option<String>,
}

impl BridgeFieldDelta {
    pub fn field_key(&self) -> &QueryFieldKey {
        &self.field
    }

    pub fn old_value(&self) -> Option<&str> {
        self.old_value.as_deref()
    }

    pub fn new_value(&self) -> Option<&str> {
        self.new_value.as_deref()
    }

    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        old_value: Option<impl Into<String>>,
        new_value: Option<impl Into<String>>,
    ) -> Self {
        Self {
            field: QueryFieldKey::new(aspect, field),
            old_value: old_value.map(Into::into),
            new_value: new_value.map(Into::into),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeRelationDelta {
    relation: String,
}

impl BridgeRelationDelta {
    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn new(relation: impl Into<String>) -> Self {
        Self {
            relation: relation.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BridgeSliceCategory {
    EntityRegion,
    EntityPartition,
    CoarseFallback,
}

impl BridgeSliceCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EntityRegion => "entity_region",
            Self::EntityPartition => "entity_partition",
            Self::CoarseFallback => "coarse_fallback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeLocalitySlice {
    category: BridgeSliceCategory,
    scope: String,
}

impl BridgeLocalitySlice {
    pub fn category(&self) -> &BridgeSliceCategory {
        &self.category
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn region(scope: impl Into<String>) -> Self {
        Self {
            category: BridgeSliceCategory::EntityRegion,
            scope: scope.into(),
        }
    }

    pub fn partition(scope: impl Into<String>) -> Self {
        Self {
            category: BridgeSliceCategory::EntityPartition,
            scope: scope.into(),
        }
    }

    pub fn coarse_fallback(scope: impl Into<String>) -> Self {
        Self {
            category: BridgeSliceCategory::CoarseFallback,
            scope: scope.into(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BridgeChangeSummary {
    field_deltas: Vec<BridgeFieldDelta>,
    relation_deltas: Vec<BridgeRelationDelta>,
    membership_transition: Option<MembershipTransition>,
    materialization_scope_transition: Option<MaterializationScopeTransition>,
    locality_slices: Vec<BridgeLocalitySlice>,
}

impl BridgeChangeSummary {
    pub fn field_deltas(&self) -> &[BridgeFieldDelta] {
        &self.field_deltas
    }

    pub fn relation_deltas(&self) -> &[BridgeRelationDelta] {
        &self.relation_deltas
    }

    pub fn membership_changed(&self) -> bool {
        self.membership_transition
            .as_ref()
            .is_some_and(MembershipTransition::changed)
    }

    pub fn materialization_scope_changed(&self) -> bool {
        self.materialization_scope_transition
            .as_ref()
            .is_some_and(MaterializationScopeTransition::changed)
    }

    pub fn membership_transition(&self) -> Option<&MembershipTransition> {
        self.membership_transition.as_ref()
    }

    pub fn materialization_scope_transition(&self) -> Option<&MaterializationScopeTransition> {
        self.materialization_scope_transition.as_ref()
    }

    pub fn locality_slices(&self) -> &[BridgeLocalitySlice] {
        &self.locality_slices
    }

    pub fn with_field_delta(mut self, delta: BridgeFieldDelta) -> Self {
        self.field_deltas.push(delta);
        self
    }

    pub fn with_relation_delta(mut self, delta: BridgeRelationDelta) -> Self {
        self.relation_deltas.push(delta);
        self
    }

    pub fn with_membership_transition(mut self, was_member: bool, is_member: bool) -> Self {
        self.membership_transition = Some(MembershipTransition::new(was_member, is_member));
        self
    }

    pub fn with_materialization_scope_transition(
        mut self,
        was_in_scope: bool,
        is_in_scope: bool,
    ) -> Self {
        self.materialization_scope_transition = Some(MaterializationScopeTransition::new(
            was_in_scope,
            is_in_scope,
        ));
        self
    }

    pub fn with_locality_slice(mut self, slice: BridgeLocalitySlice) -> Self {
        self.locality_slices.push(slice);
        self
    }

    pub fn with_region_slice(self, scope: impl Into<String>) -> Self {
        self.with_locality_slice(BridgeLocalitySlice::region(scope))
    }

    pub fn with_partition_slice(self, scope: impl Into<String>) -> Self {
        self.with_locality_slice(BridgeLocalitySlice::partition(scope))
    }

    pub fn with_coarse_fallback_slice(self, scope: impl Into<String>) -> Self {
        self.with_locality_slice(BridgeLocalitySlice::coarse_fallback(scope))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MembershipTransition {
    was_member: bool,
    is_member: bool,
}

impl MembershipTransition {
    pub fn was_member(&self) -> bool {
        self.was_member
    }

    pub fn is_member(&self) -> bool {
        self.is_member
    }

    pub fn changed(&self) -> bool {
        self.was_member != self.is_member
    }

    fn new(was_member: bool, is_member: bool) -> Self {
        Self {
            was_member,
            is_member,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterializationScopeTransition {
    was_in_scope: bool,
    is_in_scope: bool,
}

impl MaterializationScopeTransition {
    pub fn was_in_scope(&self) -> bool {
        self.was_in_scope
    }

    pub fn is_in_scope(&self) -> bool {
        self.is_in_scope
    }

    pub fn changed(&self) -> bool {
        self.was_in_scope != self.is_in_scope
    }

    fn new(was_in_scope: bool, is_in_scope: bool) -> Self {
        Self {
            was_in_scope,
            is_in_scope,
        }
    }
}
