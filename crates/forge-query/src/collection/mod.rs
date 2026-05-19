use crate::authoring::QueryFamily;
use crate::identity::CollectionPlanDigest;
use crate::validation::ValidatedQueryBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CollectionPlanningMode {
    Ordinary,
    Cdc,
    #[cfg(test)]
    AggregateRollupCount,
    #[cfg(test)]
    DerivedDisplayLabel,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OrderingKeyPath {
    aspect: String,
    field: String,
}

impl OrderingKeyPath {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub(crate) fn new(aspect: impl Into<String>, field: impl Into<String>) -> Self {
        Self {
            aspect: aspect.into(),
            field: field.into(),
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
            self.key_path.aspect(),
            self.key_path.field(),
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
            CollectionResultFamily::OrdinaryCollection => "ordinary_collection",
            CollectionResultFamily::CdcCollection => "cdc_collection",
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AggregateFunctionFamily {
    NoneAdmittedYet,
    CountRows,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AggregateGroupingShape {
    grouping_key_count: usize,
}

impl AggregateGroupingShape {
    pub fn grouping_key_count(&self) -> usize {
        self.grouping_key_count
    }

    pub(crate) fn new(grouping_key_count: usize) -> Self {
        Self { grouping_key_count }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AggregateInputBreadth(usize);

impl AggregateInputBreadth {
    pub fn value(&self) -> usize {
        self.0
    }

    pub(crate) fn new(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AggregateShapeArtifact {
    function_family: AggregateFunctionFamily,
    grouping_shape: AggregateGroupingShape,
    input_breadth: AggregateInputBreadth,
}

impl AggregateShapeArtifact {
    pub fn function_family(&self) -> &AggregateFunctionFamily {
        &self.function_family
    }

    pub fn grouping_shape(&self) -> &AggregateGroupingShape {
        &self.grouping_shape
    }

    pub fn input_breadth(&self) -> &AggregateInputBreadth {
        &self.input_breadth
    }

    pub(crate) fn new(input_breadth: AggregateInputBreadth) -> Self {
        Self {
            function_family: AggregateFunctionFamily::NoneAdmittedYet,
            grouping_shape: AggregateGroupingShape::new(0),
            input_breadth,
        }
    }

    #[cfg(test)]
    pub(crate) fn count_rows(input_breadth: AggregateInputBreadth) -> Self {
        Self {
            function_family: AggregateFunctionFamily::CountRows,
            grouping_shape: AggregateGroupingShape::new(1),
            input_breadth,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RollupEdgeClass {
    NoneAdmittedYet,
    RootCollection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RollupShapeArtifact {
    edge_class: RollupEdgeClass,
}

impl RollupShapeArtifact {
    pub fn edge_class(&self) -> &RollupEdgeClass {
        &self.edge_class
    }

    pub(crate) fn none_admitted_yet() -> Self {
        Self {
            edge_class: RollupEdgeClass::NoneAdmittedYet,
        }
    }

    #[cfg(test)]
    pub(crate) fn root_collection() -> Self {
        Self {
            edge_class: RollupEdgeClass::RootCollection,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DerivedFieldComputationClass {
    NoneAdmittedYet,
    DisplayLabelFromIdentityAndProfile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DerivedFieldPlanArtifact {
    computation_class: DerivedFieldComputationClass,
    derived_field_count: usize,
}

impl DerivedFieldPlanArtifact {
    pub fn computation_class(&self) -> &DerivedFieldComputationClass {
        &self.computation_class
    }

    pub fn derived_field_count(&self) -> usize {
        self.derived_field_count
    }

    pub(crate) fn none_admitted_yet() -> Self {
        Self {
            computation_class: DerivedFieldComputationClass::NoneAdmittedYet,
            derived_field_count: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn display_label() -> Self {
        Self {
            computation_class: DerivedFieldComputationClass::DisplayLabelFromIdentityAndProfile,
            derived_field_count: 1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CollectionResultFamily {
    OrdinaryCollection,
    CdcCollection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostReadShapingPlan {
    aggregate_shape: AggregateShapeArtifact,
    rollup_shape: RollupShapeArtifact,
    derived_field_plan: DerivedFieldPlanArtifact,
    result_family: CollectionResultFamily,
}

impl PostReadShapingPlan {
    pub fn aggregate_shape(&self) -> &AggregateShapeArtifact {
        &self.aggregate_shape
    }

    pub fn rollup_shape(&self) -> &RollupShapeArtifact {
        &self.rollup_shape
    }

    pub fn derived_field_plan(&self) -> &DerivedFieldPlanArtifact {
        &self.derived_field_plan
    }

    pub fn result_family(&self) -> &CollectionResultFamily {
        &self.result_family
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        vec![
            format!(
                "aggregate_family:{}",
                match self.aggregate_shape.function_family {
                    AggregateFunctionFamily::NoneAdmittedYet => "none_admitted_yet",
                    AggregateFunctionFamily::CountRows => "count_rows",
                }
            ),
            format!(
                "aggregate_grouping_key_count:{}",
                self.aggregate_shape.grouping_shape.grouping_key_count()
            ),
            format!(
                "aggregate_input_breadth:{}",
                self.aggregate_shape.input_breadth.value()
            ),
            format!(
                "rollup_edge_class:{}",
                match self.rollup_shape.edge_class {
                    RollupEdgeClass::NoneAdmittedYet => "none_admitted_yet",
                    RollupEdgeClass::RootCollection => "root_collection",
                }
            ),
            format!(
                "derived_field_class:{}",
                match self.derived_field_plan.computation_class {
                    DerivedFieldComputationClass::NoneAdmittedYet => "none_admitted_yet",
                    DerivedFieldComputationClass::DisplayLabelFromIdentityAndProfile => {
                        "display_label_from_identity_and_profile"
                    }
                }
            ),
            format!(
                "derived_field_count:{}",
                self.derived_field_plan.derived_field_count()
            ),
            format!(
                "collection_result_family:{}",
                match self.result_family {
                    CollectionResultFamily::OrdinaryCollection => "ordinary_collection",
                    CollectionResultFamily::CdcCollection => "cdc_collection",
                }
            ),
        ]
    }

    pub(crate) fn for_mode(input_breadth: usize, mode: &CollectionPlanningMode) -> Self {
        let aggregate_input_breadth = AggregateInputBreadth::new(input_breadth);
        match mode {
            CollectionPlanningMode::Ordinary => Self {
                aggregate_shape: AggregateShapeArtifact::new(aggregate_input_breadth),
                rollup_shape: RollupShapeArtifact::none_admitted_yet(),
                derived_field_plan: DerivedFieldPlanArtifact::none_admitted_yet(),
                result_family: CollectionResultFamily::OrdinaryCollection,
            },
            CollectionPlanningMode::Cdc => Self {
                aggregate_shape: AggregateShapeArtifact::new(aggregate_input_breadth),
                rollup_shape: RollupShapeArtifact::none_admitted_yet(),
                derived_field_plan: DerivedFieldPlanArtifact::none_admitted_yet(),
                result_family: CollectionResultFamily::CdcCollection,
            },
            #[cfg(test)]
            CollectionPlanningMode::AggregateRollupCount => Self {
                aggregate_shape: AggregateShapeArtifact::count_rows(aggregate_input_breadth),
                rollup_shape: RollupShapeArtifact::root_collection(),
                derived_field_plan: DerivedFieldPlanArtifact::none_admitted_yet(),
                result_family: CollectionResultFamily::OrdinaryCollection,
            },
            #[cfg(test)]
            CollectionPlanningMode::DerivedDisplayLabel => Self {
                aggregate_shape: AggregateShapeArtifact::new(aggregate_input_breadth),
                rollup_shape: RollupShapeArtifact::none_admitted_yet(),
                derived_field_plan: DerivedFieldPlanArtifact::display_label(),
                result_family: CollectionResultFamily::OrdinaryCollection,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionPlanningContext {
    query_family: QueryFamily,
    result_family: CollectionResultFamily,
}

impl CollectionPlanningContext {
    pub fn query_family(&self) -> &QueryFamily {
        &self.query_family
    }

    pub fn result_family(&self) -> &CollectionResultFamily {
        &self.result_family
    }

    pub(crate) fn new(result_family: CollectionResultFamily) -> Self {
        Self {
            query_family: QueryFamily::Collection,
            result_family,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionPlanBundle {
    digest: CollectionPlanDigest,
    planning_context: CollectionPlanningContext,
    ordering_basis: CollectionOrderingBasis,
    window_policy: CollectionWindowPolicy,
    cursor_contract: CursorAdvanceContract,
    traversal_bound: TraversalBoundContract,
    post_read_shaping: PostReadShapingPlan,
}

impl CollectionPlanBundle {
    pub fn digest(&self) -> &CollectionPlanDigest {
        &self.digest
    }

    pub fn planning_context(&self) -> &CollectionPlanningContext {
        &self.planning_context
    }

    pub fn ordering_basis(&self) -> &CollectionOrderingBasis {
        &self.ordering_basis
    }

    pub fn window_policy(&self) -> &CollectionWindowPolicy {
        &self.window_policy
    }

    pub fn cursor_contract(&self) -> &CursorAdvanceContract {
        &self.cursor_contract
    }

    pub fn traversal_bound(&self) -> &TraversalBoundContract {
        &self.traversal_bound
    }

    pub fn post_read_shaping(&self) -> &PostReadShapingPlan {
        &self.post_read_shaping
    }

    pub(crate) fn from_validated_bundle_for_mode(
        bundle: &ValidatedQueryBundle,
        mode: CollectionPlanningMode,
    ) -> Option<Self> {
        if bundle.query().family() != &QueryFamily::Collection {
            return None;
        }

        let ordering_entries = if bundle.query().ordering().entries().is_empty() {
            vec![CollectionOrderingEntry::new(
                OrderingKeyPath::new("identity", "id"),
                CollectionOrderingDirection::Ascending,
            )]
        } else {
            bundle
                .query()
                .ordering()
                .entries()
                .iter()
                .map(|entry| {
                    CollectionOrderingEntry::new(
                        OrderingKeyPath::new(entry.aspect(), entry.field()),
                        CollectionOrderingDirection::from_validated_direction(entry.direction()),
                    )
                })
                .collect()
        };
        let max_depth = bundle
            .query()
            .traversal()
            .iter()
            .map(|entry| entry.max_depth())
            .max()
            .unwrap_or(0);
        let edge_classes = bundle
            .query()
            .traversal()
            .iter()
            .map(|entry| TraversalEdgeClass::new(entry.relation()))
            .collect();
        let traversal_bound = TraversalBoundContract::new(
            TraversalDepthLimit::new(max_depth),
            edge_classes,
            if bundle.query().traversal().is_empty() {
                MaterializationBreadthClass::ScalarOnly
            } else {
                MaterializationBreadthClass::RootPlusTraversal
            },
        );
        let input_breadth = bundle.query().projection().len()
            + bundle.query().predicates().entries().len()
            + bundle.query().traversal().len()
            + bundle.query().ordering().entries().len();

        let planning_context = CollectionPlanningContext::new(match mode {
            CollectionPlanningMode::Cdc => CollectionResultFamily::CdcCollection,
            CollectionPlanningMode::Ordinary => CollectionResultFamily::OrdinaryCollection,
            #[cfg(test)]
            CollectionPlanningMode::AggregateRollupCount => {
                CollectionResultFamily::OrdinaryCollection
            }
            #[cfg(test)]
            CollectionPlanningMode::DerivedDisplayLabel => {
                CollectionResultFamily::OrdinaryCollection
            }
        });
        let ordering_basis = CollectionOrderingBasis::new(ordering_entries);
        let window_policy = CollectionWindowPolicy::FullSnapshotRead;
        let cursor_contract = CursorAdvanceContract::BasisBoundOpaque;
        let post_read_shaping = PostReadShapingPlan::for_mode(input_breadth, &mode);
        let mut digest_parts = vec![
            format!("query_family:{:?}", planning_context.query_family()),
            format!(
                "result_family:{}",
                match planning_context.result_family() {
                    CollectionResultFamily::OrdinaryCollection => "ordinary_collection",
                    CollectionResultFamily::CdcCollection => "cdc_collection",
                }
            ),
            window_policy.digest_part(),
            cursor_contract.digest_part(),
        ];
        digest_parts.extend(ordering_basis.digest_parts());
        digest_parts.extend(traversal_bound.digest_parts());
        digest_parts.extend(post_read_shaping.digest_parts());

        Some(Self {
            digest: CollectionPlanDigest::from_parts(&digest_parts),
            planning_context,
            ordering_basis,
            window_policy,
            cursor_contract,
            traversal_bound,
            post_read_shaping,
        })
    }
}
