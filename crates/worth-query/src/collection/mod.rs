use worth_foundational::facade::{AspectKey, FieldKey};

mod plan_bundle;
mod post_read_shaping;

pub use plan_bundle::{CollectionPlanBundle, CollectionPlanningContext};
pub(crate) use post_read_shaping::CollectionPlanningMode;
pub use post_read_shaping::{
    AggregateFunctionFamily, AggregateGroupingShape, AggregateInputBreadth, AggregateShapeArtifact,
    CollectionResultFamily, DerivedFieldComputationClass, DerivedFieldPlanArtifact,
    PostReadShapingPlan, RollupEdgeClass, RollupShapeArtifact,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OrderingKeyPath {
    aspect_key: AspectKey,
    field_key: FieldKey,
}

impl OrderingKeyPath {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field_key
    }

    pub(crate) fn new(aspect: impl Into<String>, field: impl Into<String>) -> Self {
        let aspect = aspect.into();
        let field = field.into();
        Self {
            aspect_key: AspectKey::new(aspect).expect("ordering aspect must be foundational"),
            field_key: FieldKey::new(field).expect("ordering field must be foundational"),
        }
    }

    pub(crate) fn from_native_keys(aspect_key: AspectKey, field_key: FieldKey) -> Self {
        Self {
            aspect_key,
            field_key,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CollectionOrderingDirection {
    Ascending,
    Descending,
}

impl CollectionOrderingDirection {
    pub(crate) fn from_validated_direction(direction: &str) -> Self {
        match direction {
            "descending" => Self::Descending,
            _ => Self::Ascending,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderingTieBreakContract {
    RootEntityIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StableOrderingContract {
    tie_break: OrderingTieBreakContract,
}

impl StableOrderingContract {
    pub fn tie_break(&self) -> &OrderingTieBreakContract {
        &self.tie_break
    }

    pub(crate) fn digest_part(&self) -> String {
        match self.tie_break {
            OrderingTieBreakContract::RootEntityIdentity => {
                "stable_tie_break:root_entity_identity".to_string()
            }
        }
    }

    pub(crate) fn root_entity_identity() -> Self {
        Self {
            tie_break: OrderingTieBreakContract::RootEntityIdentity,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionOrderingEntry {
    key_path: OrderingKeyPath,
    direction: CollectionOrderingDirection,
}

impl CollectionOrderingEntry {
    pub fn key_path(&self) -> &OrderingKeyPath {
        &self.key_path
    }

    pub fn direction(&self) -> &CollectionOrderingDirection {
        &self.direction
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "ordering:{}:{}:{}",
            self.key_path.native_aspect_key().as_str(),
            self.key_path.native_field_key().as_str(),
            match self.direction {
                CollectionOrderingDirection::Ascending => "ascending",
                CollectionOrderingDirection::Descending => "descending",
            }
        )
    }

    pub(crate) fn new(key_path: OrderingKeyPath, direction: CollectionOrderingDirection) -> Self {
        Self {
            key_path,
            direction,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionOrderingBasis {
    entries: Vec<CollectionOrderingEntry>,
    stable_ordering: StableOrderingContract,
}

impl CollectionOrderingBasis {
    pub fn entries(&self) -> &[CollectionOrderingEntry] {
        &self.entries
    }

    pub fn stable_ordering(&self) -> &StableOrderingContract {
        &self.stable_ordering
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts: Vec<String> = self
            .entries
            .iter()
            .map(CollectionOrderingEntry::digest_part)
            .collect();
        parts.push(self.stable_ordering.digest_part());
        parts
    }

    pub(crate) fn new(entries: Vec<CollectionOrderingEntry>) -> Self {
        Self {
            entries,
            stable_ordering: StableOrderingContract::root_entity_identity(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CollectionWindowPolicy {
    FullSnapshotRead,
}

impl CollectionWindowPolicy {
    pub(crate) fn digest_part(&self) -> String {
        match self {
            Self::FullSnapshotRead => "window_policy:full_snapshot_read".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CursorBoundaryDigest(String);

impl CursorBoundaryDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[cfg(test)]
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CursorAdvanceContract {
    BasisBoundOpaque,
}

impl CursorAdvanceContract {
    pub(crate) fn digest_part(&self) -> String {
        match self {
            Self::BasisBoundOpaque => "cursor_contract:basis_bound_opaque".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpaquePageCursor {
    boundary: CursorBoundaryDigest,
}

impl OpaquePageCursor {
    pub fn boundary(&self) -> &CursorBoundaryDigest {
        &self.boundary
    }

    #[cfg(test)]
    pub(crate) fn new(boundary: CursorBoundaryDigest) -> Self {
        Self { boundary }
    }
}

#[cfg(test)]
pub(crate) fn page_cursor_for_collection(
    collection: &CollectionPlanBundle,
    plan_digest: &str,
    basis_digest: &str,
    page_width: usize,
) -> OpaquePageCursor {
    OpaquePageCursor::new(CursorBoundaryDigest::new(format!(
        "cursor:{}:{}:{}:{}:{}",
        plan_digest,
        basis_digest,
        match collection.planning_context().result_family() {
            family => family.digest_label(),
        },
        collection.ordering_basis().entries().len(),
        page_width
    )))
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TraversalDepthLimit(u8);

impl TraversalDepthLimit {
    pub fn value(&self) -> u8 {
        self.0
    }

    pub(crate) fn new(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TraversalEdgeClass(String);

impl TraversalEdgeClass {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MaterializationBreadthClass {
    ScalarOnly,
    RootPlusTraversal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraversalBoundContract {
    depth_limit: TraversalDepthLimit,
    edge_classes: Vec<TraversalEdgeClass>,
    materialization_breadth: MaterializationBreadthClass,
}

impl TraversalBoundContract {
    pub fn depth_limit(&self) -> &TraversalDepthLimit {
        &self.depth_limit
    }

    pub fn edge_classes(&self) -> &[TraversalEdgeClass] {
        &self.edge_classes
    }

    pub fn materialization_breadth(&self) -> &MaterializationBreadthClass {
        &self.materialization_breadth
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("traversal_depth_limit:{}", self.depth_limit.value()),
            format!(
                "materialization_breadth:{}",
                match self.materialization_breadth {
                    MaterializationBreadthClass::ScalarOnly => "scalar_only",
                    MaterializationBreadthClass::RootPlusTraversal => "root_plus_traversal",
                }
            ),
        ];
        parts.extend(
            self.edge_classes
                .iter()
                .map(|edge_class| format!("traversal_edge_class:{}", edge_class.as_str())),
        );
        parts
    }

    pub(crate) fn new(
        depth_limit: TraversalDepthLimit,
        edge_classes: Vec<TraversalEdgeClass>,
        materialization_breadth: MaterializationBreadthClass,
    ) -> Self {
        Self {
            depth_limit,
            edge_classes,
            materialization_breadth,
        }
    }
}
