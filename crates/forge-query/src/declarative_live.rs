use std::collections::{BTreeMap, BTreeSet};

use forge_foundational::facade::{AspectKey, AspectValue, InternedString};

use crate::authoring::{
    AspectFieldKey, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    GuidedAuthoringPath, IntegerComparisonOperator, IntegerComparisonPredicate, OrderingDirection,
    OrderingSelector, PresencePredicate, RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
    ScalarPredicateValue, SetMembershipPredicate, StringContainsPredicate, TraversalSelector,
};
use crate::basis::{
    preflight_execution_basis, resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode,
    ExecutionBasisIntent, ExecutionPreflightBundle, ResolvedSnapshotBasis,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::canonicalization::CanonicalQueryBundle;
use crate::identity::hash_parts;
use crate::identity_evolution::{InspectorIdentityArtifact, InspectorIdentityClassification};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::schema_view::QuerySchemaView;
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor, ViewShapePlanArtifact,
};
use crate::view_shape_live::{
    lower_view_shape_plan_to_live, materialize_authoritative_grouped_baseline_from_members,
    AuthoritativeGroupedBaselineArtifact, ForgeQueryGroupedBaselineMember, LiveViewShapeArtifact,
};
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, lower_query_writeback_declaration,
    QueryWritebackDeclaration, WorkflowAuthorityTargetFamily, WorkflowBindingSource,
    WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
    WorkflowFreshnessPolicy, WritebackLoweringInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeProjectionField {
    source: AspectFieldKey,
    delivered_name: String,
}

impl DeclarativeProjectionField {
    pub fn new(source: AspectFieldKey) -> Self {
        let delivered_name = source.field().as_str().to_string();
        Self {
            delivered_name,
            source,
        }
    }

    pub(crate) fn from_authoring_parts(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Self {
        Self::new(
            AspectFieldKey::from_authoring_parts(aspect, field)
                .expect("declarative projection fields require non-empty aspect and field names"),
        )
    }

    pub fn delivered_as(mut self, delivered_name: impl Into<String>) -> Self {
        self.delivered_name = delivered_name.into();
        self
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn delivered_name(&self) -> &str {
        &self.delivered_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeOrderingField {
    source: AspectFieldKey,
    direction: OrderingDirection,
}

impl DeclarativeOrderingField {
    pub fn ascending(source: AspectFieldKey) -> Self {
        Self {
            source,
            direction: OrderingDirection::Ascending,
        }
    }

    pub fn descending(source: AspectFieldKey) -> Self {
        Self {
            source,
            direction: OrderingDirection::Descending,
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn direction(&self) -> OrderingDirection {
        self.direction
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeEqualityFilter {
    source: AspectFieldKey,
    value: ScalarPredicateValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeIntegerComparisonFilter {
    source: AspectFieldKey,
    operator: IntegerComparisonOperator,
    value: i64,
}

impl DeclarativeIntegerComparisonFilter {
    pub fn greater_than(source: AspectFieldKey, value: i64) -> Self {
        Self {
            source,
            operator: IntegerComparisonOperator::GreaterThan,
            value,
        }
    }

    pub fn less_than(source: AspectFieldKey, value: i64) -> Self {
        Self {
            source,
            operator: IntegerComparisonOperator::LessThan,
            value,
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn operator(&self) -> IntegerComparisonOperator {
        self.operator
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeStringContainsFilter {
    source: AspectFieldKey,
    value: String,
}

impl DeclarativeStringContainsFilter {
    pub fn new(source: AspectFieldKey, value: impl Into<String>) -> Self {
        Self {
            source,
            value: value.into(),
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeSetMembershipFilter {
    source: AspectFieldKey,
    values: Vec<ScalarPredicateValue>,
}

impl DeclarativeSetMembershipFilter {
    pub fn new(
        source: AspectFieldKey,
        values: impl IntoIterator<Item = ScalarPredicateValue>,
    ) -> Self {
        Self {
            source,
            values: values.into_iter().collect(),
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn values(&self) -> &[ScalarPredicateValue] {
        &self.values
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclarativePresenceFilterKind {
    IsPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativePresenceFilter {
    source: AspectFieldKey,
    kind: DeclarativePresenceFilterKind,
}

impl DeclarativePresenceFilter {
    pub fn is_present(source: AspectFieldKey) -> Self {
        Self {
            source,
            kind: DeclarativePresenceFilterKind::IsPresent,
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn kind(&self) -> DeclarativePresenceFilterKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativePredicateFilter {
    Equality(DeclarativeEqualityFilter),
    IntegerComparison(DeclarativeIntegerComparisonFilter),
    StringContains(DeclarativeStringContainsFilter),
    SetMembership(DeclarativeSetMembershipFilter),
    Presence(DeclarativePresenceFilter),
}

impl DeclarativePredicateFilter {
    pub fn source_field_key(&self) -> &AspectFieldKey {
        match self {
            Self::Equality(filter) => filter.source_field_key(),
            Self::IntegerComparison(filter) => filter.source_field_key(),
            Self::StringContains(filter) => filter.source_field_key(),
            Self::SetMembership(filter) => filter.source_field_key(),
            Self::Presence(filter) => filter.source_field_key(),
        }
    }
}

impl DeclarativeEqualityFilter {
    pub fn new(source: AspectFieldKey, value: ScalarPredicateValue) -> Self {
        Self { source, value }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn value(&self) -> &ScalarPredicateValue {
        &self.value
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativeLiveViewShape {
    ListSplice,
    Table,
    Detail,
    InspectorObserved,
    InspectorFocused {
        focused_aspect: AspectKey,
    },
    IdentityAwareInspectorFocused {
        focused_aspect: AspectKey,
        classification: InspectorIdentityClassification,
    },
    KanbanGrouped {
        grouping_aspect: AspectKey,
    },
}

impl DeclarativeLiveViewShape {
    pub fn list_splice() -> Self {
        Self::ListSplice
    }

    pub fn table() -> Self {
        Self::Table
    }

    pub fn detail() -> Self {
        Self::Detail
    }

    pub fn inspector_observed() -> Self {
        Self::InspectorObserved
    }

    pub fn inspector_focused(focused_aspect: AspectKey) -> Self {
        Self::InspectorFocused { focused_aspect }
    }

    pub fn identity_aware_inspector_focused(
        focused_aspect: AspectKey,
        classification: InspectorIdentityClassification,
    ) -> Self {
        Self::IdentityAwareInspectorFocused {
            focused_aspect,
            classification,
        }
    }

    pub fn kanban_grouped(grouping_aspect: AspectKey) -> Self {
        Self::KanbanGrouped { grouping_aspect }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ListSplice => "list_splice",
            Self::Table => "table",
            Self::Detail => "detail",
            Self::InspectorObserved => "inspector_observed",
            Self::InspectorFocused { .. } => "inspector_focused",
            Self::IdentityAwareInspectorFocused { .. } => "identity_aware_inspector_focused",
            Self::KanbanGrouped { .. } => "kanban_grouped",
        }
    }

    fn collection_backed(&self) -> bool {
        matches!(
            self,
            Self::ListSplice | Self::Table | Self::KanbanGrouped { .. }
        )
    }

    fn view_shape_descriptor(&self) -> ViewShapeDescriptor {
        match self {
            Self::ListSplice | Self::Table => ViewShapeDescriptor::table(),
            Self::Detail => ViewShapeDescriptor::detail(),
            Self::InspectorObserved => ViewShapeDescriptor::inspector_detail_observed(),
            Self::InspectorFocused { focused_aspect } => {
                ViewShapeDescriptor::inspector_detail_focused(focused_aspect.clone())
            }
            Self::IdentityAwareInspectorFocused {
                focused_aspect,
                classification,
            } => ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                focused_aspect.clone(),
                *classification,
            ),
            Self::KanbanGrouped { grouping_aspect } => {
                ViewShapeDescriptor::kanban_grouped(grouping_aspect.clone())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeLiveQueryRequest {
    target: String,
    view_shape: DeclarativeLiveViewShape,
    query_projection: Vec<DeclarativeProjectionField>,
    result_fields: Vec<DeclarativeProjectionField>,
    predicate_filters: Vec<DeclarativePredicateFilter>,
    traversal: Vec<TraversalSelector>,
    ordering: Vec<DeclarativeOrderingField>,
    inspector_identity: Option<InspectorIdentityArtifact>,
}

impl DeclarativeLiveQueryRequest {
    pub fn new(target: impl Into<String>, view_shape: DeclarativeLiveViewShape) -> Self {
        Self {
            target: target.into(),
            view_shape,
            query_projection: Vec::new(),
            result_fields: Vec::new(),
            predicate_filters: Vec::new(),
            traversal: Vec::new(),
            ordering: Vec::new(),
            inspector_identity: None,
        }
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn target_collection_identity(
        &self,
    ) -> crate::runtime::ForgeQueryMutationTargetCollectionIdentity {
        crate::runtime::ForgeQueryMutationTargetCollectionIdentity::new(
            "declarative-live-request-target",
            self.target.clone(),
        )
    }

    pub fn view_shape(&self) -> &DeclarativeLiveViewShape {
        &self.view_shape
    }

    pub fn query_projection(&self) -> &[DeclarativeProjectionField] {
        &self.query_projection
    }

    pub fn projection(&self) -> &[DeclarativeProjectionField] {
        self.result_fields()
    }

    pub fn result_fields(&self) -> &[DeclarativeProjectionField] {
        &self.result_fields
    }

    pub fn predicate_filters(&self) -> &[DeclarativePredicateFilter] {
        &self.predicate_filters
    }

    pub fn traversal(&self) -> &[TraversalSelector] {
        &self.traversal
    }

    pub fn ordering(&self) -> &[DeclarativeOrderingField] {
        &self.ordering
    }

    pub fn inspector_identity(&self) -> Option<&InspectorIdentityArtifact> {
        self.inspector_identity.as_ref()
    }

    pub fn project(mut self, field: DeclarativeProjectionField) -> Self {
        self.query_projection.push(field.clone());
        self.result_fields.push(field);
        self
    }

    pub fn where_equal(mut self, filter: DeclarativeEqualityFilter) -> Self {
        self.predicate_filters
            .push(DeclarativePredicateFilter::Equality(filter));
        self
    }

    pub fn where_greater_than(mut self, filter: DeclarativeIntegerComparisonFilter) -> Self {
        self.predicate_filters
            .push(DeclarativePredicateFilter::IntegerComparison(filter));
        self
    }

    pub fn where_less_than(mut self, filter: DeclarativeIntegerComparisonFilter) -> Self {
        self.predicate_filters
            .push(DeclarativePredicateFilter::IntegerComparison(filter));
        self
    }

    pub fn where_contains(mut self, filter: DeclarativeStringContainsFilter) -> Self {
        self.predicate_filters
            .push(DeclarativePredicateFilter::StringContains(filter));
        self
    }

    pub fn where_in(mut self, filter: DeclarativeSetMembershipFilter) -> Self {
        self.predicate_filters
            .push(DeclarativePredicateFilter::SetMembership(filter));
        self
    }

    pub fn where_present(mut self, filter: DeclarativePresenceFilter) -> Self {
        self.predicate_filters
            .push(DeclarativePredicateFilter::Presence(filter));
        self
    }

    pub fn traverse(mut self, selector: TraversalSelector) -> Self {
        self.traversal.push(selector);
        self
    }

    pub fn order_by(mut self, field: DeclarativeProjectionField) -> Self {
        self.ordering.push(DeclarativeOrderingField::ascending(
            field.source_field_key().clone(),
        ));
        self
    }

    pub fn order_by_direction(mut self, field: DeclarativeOrderingField) -> Self {
        self.ordering.push(field);
        self
    }

    pub fn with_inspector_identity(mut self, artifact: InspectorIdentityArtifact) -> Self {
        self.inspector_identity = Some(artifact);
        self
    }

    pub(crate) fn project_query_only(mut self, field: DeclarativeProjectionField) -> Self {
        self.query_projection.push(field);
        self
    }

    pub(crate) fn result_field(mut self, field: DeclarativeProjectionField) -> Self {
        self.result_fields.push(field);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativeLiveQueryError {
    InvalidTarget,
    Authoring(String),
    Canonicalization(String),
    ViewShape(String),
    DuplicateTraversal {
        relation: String,
        depth: u8,
    },
    TraversalNotDeclaredInSchema {
        relation: String,
        requested_depth: u8,
    },
    TraversalExceedsSchemaDepth {
        relation: String,
        requested_depth: u8,
        max_depth: u8,
    },
    BasisResolution(String),
    BasisPreflight(String),
    LiveLowering(String),
    Writeback(String),
    InspectorIdentityBindingRequired,
    EmptyWritebackIntent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeLiveQuerySession {
    request: DeclarativeLiveQueryRequest,
    canonical: CanonicalQueryBundle,
    view_plan: ViewShapePlanArtifact,
    preflight: ExecutionPreflightBundle,
    live_view: LiveViewShapeArtifact,
}

impl DeclarativeLiveQuerySession {
    pub fn request(&self) -> &DeclarativeLiveQueryRequest {
        &self.request
    }

    pub fn canonical(&self) -> &CanonicalQueryBundle {
        &self.canonical
    }

    pub fn view_plan(&self) -> &ViewShapePlanArtifact {
        &self.view_plan
    }

    pub fn preflight(&self) -> &ExecutionPreflightBundle {
        &self.preflight
    }

    pub fn live_view(&self) -> &LiveViewShapeArtifact {
        &self.live_view
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareValue {
    source: AspectFieldKey,
    value: String,
}

impl DeclarativeBranchCompareValue {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            source: AspectFieldKey::from_authoring_parts(aspect, field).expect(
                "declarative branch compare values require non-empty aspect and field names",
            ),
            value: value.into(),
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn key(&self) -> AspectFieldKey {
        self.source.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareInputRow {
    identity: String,
    label: String,
    values: Vec<DeclarativeBranchCompareValue>,
}

impl DeclarativeBranchCompareInputRow {
    pub fn new(
        identity: impl Into<String>,
        label: impl Into<String>,
        values: impl IntoIterator<Item = DeclarativeBranchCompareValue>,
    ) -> Self {
        Self {
            identity: identity.into(),
            label: label.into(),
            values: values.into_iter().collect(),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn values(&self) -> &[DeclarativeBranchCompareValue] {
        &self.values
    }

    fn value_for(&self, key: &AspectFieldKey) -> Option<&DeclarativeBranchCompareValue> {
        self.values
            .iter()
            .find(|value| value.source_field_key() == key)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativeBranchCompareChangeFamily {
    Added,
    Removed,
    Modified,
}

impl DeclarativeBranchCompareChangeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Modified => "modified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativeBranchCompareIdentityClass {
    AuthoritativeIdentity,
    BranchLocalAddition,
    BranchLocalRemoval,
}

impl DeclarativeBranchCompareIdentityClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeIdentity => "authoritative_identity",
            Self::BranchLocalAddition => "branch_local_addition",
            Self::BranchLocalRemoval => "branch_local_removal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareFieldDelta {
    source: AspectFieldKey,
    left_value: Option<String>,
    right_value: Option<String>,
    family: DeclarativeBranchCompareChangeFamily,
}

impl DeclarativeBranchCompareFieldDelta {
    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn left_value(&self) -> Option<&str> {
        self.left_value.as_deref()
    }

    pub fn right_value(&self) -> Option<&str> {
        self.right_value.as_deref()
    }

    pub fn family(&self) -> &DeclarativeBranchCompareChangeFamily {
        &self.family
    }

    fn digest_part(&self) -> String {
        format!(
            "delta:{}:{}:{}:{}:{}",
            self.source_field_key().aspect().as_str(),
            self.source_field_key().field().as_str(),
            self.left_value.as_deref().unwrap_or("none"),
            self.right_value.as_deref().unwrap_or("none"),
            self.family.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareRow {
    left_identity: Option<String>,
    right_identity: Option<String>,
    label: String,
    identity_class: DeclarativeBranchCompareIdentityClass,
    field_deltas: Vec<DeclarativeBranchCompareFieldDelta>,
}

impl DeclarativeBranchCompareRow {
    pub fn left_identity(&self) -> Option<&str> {
        self.left_identity.as_deref()
    }

    pub fn right_identity(&self) -> Option<&str> {
        self.right_identity.as_deref()
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn identity_class(&self) -> &DeclarativeBranchCompareIdentityClass {
        &self.identity_class
    }

    pub fn field_deltas(&self) -> &[DeclarativeBranchCompareFieldDelta] {
        &self.field_deltas
    }

    fn digest_part(&self) -> String {
        let mut deltas = self
            .field_deltas
            .iter()
            .map(DeclarativeBranchCompareFieldDelta::digest_part)
            .collect::<Vec<_>>();
        deltas.sort();
        format!(
            "compare_row:{}:{}:{}:{}",
            self.left_identity.as_deref().unwrap_or("none"),
            self.right_identity.as_deref().unwrap_or("none"),
            self.identity_class.as_str(),
            deltas.join("|")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeBranchCompareArtifact {
    left_live_view_digest: String,
    right_live_view_digest: String,
    left_basis_digest: String,
    right_basis_digest: String,
    query_digest: String,
    result_digest: String,
    rows: Vec<DeclarativeBranchCompareRow>,
}

impl DeclarativeBranchCompareArtifact {
    pub fn left_live_view_digest(&self) -> &str {
        &self.left_live_view_digest
    }

    pub fn right_live_view_digest(&self) -> &str {
        &self.right_live_view_digest
    }

    pub fn left_basis_digest(&self) -> &str {
        &self.left_basis_digest
    }

    pub fn right_basis_digest(&self) -> &str {
        &self.right_basis_digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn rows(&self) -> &[DeclarativeBranchCompareRow] {
        &self.rows
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackValue {
    value: AspectValue,
}

impl DeclarativeWritebackValue {
    pub fn string(value: impl Into<String>) -> Self {
        Self {
            value: crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(value),
        }
    }

    pub fn integer(value: i64) -> Self {
        Self {
            value: AspectValue::Int64(value),
        }
    }

    pub fn boolean(value: bool) -> Self {
        Self {
            value: AspectValue::Bool(value),
        }
    }

    pub fn aspect_value(&self) -> &AspectValue {
        &self.value
    }

    fn digest_part(&self) -> String {
        format!(
            "aspect_value:{}",
            declarative_writeback_value_digest_text(&self.value)
        )
    }
}

fn declarative_writeback_value_digest_text(value: &AspectValue) -> String {
    match value {
        AspectValue::Null => "null".to_string(),
        AspectValue::Bool(value) => format!("bool:{value}"),
        AspectValue::Int8(value) => format!("i8:{value}"),
        AspectValue::Int16(value) => format!("i16:{value}"),
        AspectValue::Int32(value) => format!("i32:{value}"),
        AspectValue::Int64(value) => format!("i64:{value}"),
        AspectValue::UInt8(value) => format!("u8:{value}"),
        AspectValue::UInt16(value) => format!("u16:{value}"),
        AspectValue::UInt32(value) => format!("u32:{value}"),
        AspectValue::UInt64(value) => format!("u64:{value}"),
        AspectValue::Float32(value) => format!("f32-bits:{}", value.bits()),
        AspectValue::Float64(value) => format!("f64-bits:{}", value.bits()),
        AspectValue::Decimal(value) => format!("decimal:{}", value.as_str()),
        AspectValue::BigInt(value) => format!("bigint:{}", value.as_str()),
        AspectValue::Rational(value) => {
            format!(
                "rational:{}/{}",
                value.numerator.as_str(),
                value.denominator.as_str()
            )
        }
        AspectValue::String(value) => format!("string:{}", declarative_interned_string_text(value)),
        AspectValue::Bytes(value) => format!("bytes-ref:{}", value.0),
        AspectValue::Uuid(value) => value.iter().map(|byte| format!("{byte:02x}")).collect(),
        AspectValue::Date(value) => format!("date-days:{}", value.days_from_unix_epoch),
        AspectValue::Time(value) => format!("time-nanos:{}", value.nanos_since_midnight),
        AspectValue::Timestamp(value) => {
            format!("timestamp-micros:{}", value.micros_since_unix_epoch)
        }
        AspectValue::TimestampTz(value) => format!(
            "timestamp-tz:{}:{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        ),
        AspectValue::EntityRef(value) => {
            format!(
                "entity-ref:{}:{}:{}",
                value.partition_id.0, value.local_slot.0, value.generation.0
            )
        }
        AspectValue::ContentRef(value) => format!("content-ref:{}", value.0),
    }
}

fn declarative_interned_string_text(value: &InternedString) -> String {
    match value {
        InternedString::Raw(value) => value.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackChange {
    source: AspectFieldKey,
    value: DeclarativeWritebackValue,
}

impl DeclarativeWritebackChange {
    pub fn new(source: AspectFieldKey, value: DeclarativeWritebackValue) -> Self {
        Self { source, value }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn value(&self) -> &DeclarativeWritebackValue {
        &self.value
    }

    fn digest_part(&self) -> String {
        format!(
            "change:{}:{}:{}",
            self.source_field_key().aspect().as_str(),
            self.source_field_key().field().as_str(),
            self.value.digest_part()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackIntent {
    changes: Vec<DeclarativeWritebackChange>,
}

impl DeclarativeWritebackIntent {
    pub fn new(changes: impl IntoIterator<Item = DeclarativeWritebackChange>) -> Self {
        Self {
            changes: changes.into_iter().collect(),
        }
    }

    pub fn update_aspect(source: AspectFieldKey, value: DeclarativeWritebackValue) -> Self {
        Self::new([DeclarativeWritebackChange::new(source, value)])
    }

    pub fn changes(&self) -> &[DeclarativeWritebackChange] {
        &self.changes
    }

    fn digest(&self) -> String {
        let mut parts = vec![format!("change_count:{}", self.changes.len())];
        parts.extend(
            self.changes
                .iter()
                .map(DeclarativeWritebackChange::digest_part),
        );
        hash_parts(&parts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackArtifact {
    live_view_basis_digest: String,
    intent_digest: String,
    changes: Vec<DeclarativeWritebackChange>,
    declaration: QueryWritebackDeclaration,
    artifact_digest: String,
}

impl DeclarativeWritebackArtifact {
    pub fn live_view_basis_digest(&self) -> &str {
        &self.live_view_basis_digest
    }

    pub fn intent_digest(&self) -> &str {
        &self.intent_digest
    }

    pub fn changes(&self) -> &[DeclarativeWritebackChange] {
        &self.changes
    }

    pub fn declaration(&self) -> &QueryWritebackDeclaration {
        &self.declaration
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

pub fn declare_runtime_live_query_session(
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    snapshot_identity: ForgeQuerySnapshotIdentity,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    declare_runtime_live_query_session_with_grouped_baseline(
        request,
        schema_view,
        snapshot_identity,
        None::<Vec<ForgeQueryGroupedBaselineMember>>,
    )
}

pub fn declare_runtime_live_query_session_with_grouped_baseline(
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    grouped_baseline_members: Option<impl IntoIterator<Item = ForgeQueryGroupedBaselineMember>>,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    let basis_intent = ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    );
    validate_declared_traversal_contract(&request, &schema_view)?;
    let canonical = canonicalize_declarative_request(&request)?;
    let view_plan =
        plan_declarative_request(&request, &canonical, schema_view, basis_intent.clone())?;
    let identity = ResolvedSnapshotIdentity::new(
        BasisAuthorityFamily::Runtime,
        None,
        snapshot_identity.evidence_identity(),
        view_plan.validated().query().schema_basis().clone(),
        SnapshotLineageClass::CurrentHead,
    );
    let basis = resolve_snapshot_basis(basis_intent, identity, BasisResolutionMode::RuntimeDirect)
        .map_err(|error| DeclarativeLiveQueryError::BasisResolution(format!("{error:?}")))?;

    let grouped_baseline = grouped_baseline_members
        .map(|members| {
            materialize_authoritative_grouped_baseline_from_members(
                &view_plan,
                basis.clone(),
                members,
            )
            .map_err(|error| DeclarativeLiveQueryError::LiveLowering(format!("{error:?}")))
        })
        .transpose()?;

    finish_declarative_live_query_session(request, canonical, view_plan, basis, grouped_baseline)
}

pub fn declare_live_query_session(
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    basis_intent: ExecutionBasisIntent,
    basis: ResolvedSnapshotBasis,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    validate_declared_traversal_contract(&request, &schema_view)?;
    let canonical = canonicalize_declarative_request(&request)?;
    let view_plan = plan_declarative_request(&request, &canonical, schema_view, basis_intent)?;
    finish_declarative_live_query_session(request, canonical, view_plan, basis, None)
}

pub fn declare_writeback_from_live_session(
    session: &DeclarativeLiveQuerySession,
    intent: DeclarativeWritebackIntent,
) -> Result<DeclarativeWritebackArtifact, DeclarativeLiveQueryError> {
    if intent.changes().is_empty() {
        return Err(DeclarativeLiveQueryError::EmptyWritebackIntent);
    }

    let binding =
        bind_workflow_context(WorkflowBindingSource::RuntimePreflight(session.preflight()))
            .map_err(|error| DeclarativeLiveQueryError::Writeback(format!("{error:?}")))?;
    let workflow = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
            WorkflowCostClass::WritebackLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .map_err(|error| DeclarativeLiveQueryError::Writeback(format!("{error:?}")))?;
    let declaration = lower_query_writeback_declaration(
        &workflow,
        WritebackLoweringInput::projected_state_diff(),
    )
    .map_err(|error| DeclarativeLiveQueryError::Writeback(format!("{error:?}")))?;

    let live_view_basis_digest = session
        .preflight()
        .basis()
        .proof()
        .digest()
        .as_str()
        .to_string();
    let intent_digest = intent.digest();
    let artifact_digest = hash_parts(&[
        format!("basis:{live_view_basis_digest}"),
        format!("intent:{intent_digest}"),
        format!("writeback:{}", declaration.lowering_for_reporting()),
    ]);

    Ok(DeclarativeWritebackArtifact {
        live_view_basis_digest,
        intent_digest,
        changes: intent.changes,
        declaration,
        artifact_digest,
    })
}

pub fn declare_branch_compare_from_live_sessions(
    left: &DeclarativeLiveQuerySession,
    right: &DeclarativeLiveQuerySession,
    left_rows: impl IntoIterator<Item = DeclarativeBranchCompareInputRow>,
    right_rows: impl IntoIterator<Item = DeclarativeBranchCompareInputRow>,
) -> Result<DeclarativeBranchCompareArtifact, DeclarativeLiveQueryError> {
    let left_query_digest = left.canonical().query().digest().as_str();
    let right_query_digest = right.canonical().query().digest().as_str();
    if left_query_digest != right_query_digest {
        return Err(DeclarativeLiveQueryError::ViewShape(
            "branch compare requires matching canonical query identity".to_string(),
        ));
    }

    let left_by_identity = left_rows
        .into_iter()
        .map(|row| (row.identity().to_string(), row))
        .collect::<BTreeMap<_, _>>();
    let right_by_identity = right_rows
        .into_iter()
        .map(|row| (row.identity().to_string(), row))
        .collect::<BTreeMap<_, _>>();
    let identities = left_by_identity
        .keys()
        .chain(right_by_identity.keys())
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut rows = Vec::new();
    for identity in identities {
        let left_row = left_by_identity.get(&identity);
        let right_row = right_by_identity.get(&identity);
        let value_keys = left_row
            .into_iter()
            .flat_map(|row| row.values().iter().map(DeclarativeBranchCompareValue::key))
            .chain(
                right_row
                    .into_iter()
                    .flat_map(|row| row.values().iter().map(DeclarativeBranchCompareValue::key)),
            )
            .collect::<BTreeSet<_>>();
        let mut field_deltas = Vec::new();
        for key in value_keys {
            let left_value = left_row
                .and_then(|row| row.value_for(&key))
                .map(|value| value.value().to_string());
            let right_value = right_row
                .and_then(|row| row.value_for(&key))
                .map(|value| value.value().to_string());
            let family = match (&left_value, &right_value) {
                (Some(left), Some(right)) if left == right => continue,
                (Some(_), Some(_)) => DeclarativeBranchCompareChangeFamily::Modified,
                (None, Some(_)) => DeclarativeBranchCompareChangeFamily::Added,
                (Some(_), None) => DeclarativeBranchCompareChangeFamily::Removed,
                (None, None) => continue,
            };
            field_deltas.push(DeclarativeBranchCompareFieldDelta {
                source: key,
                left_value,
                right_value,
                family,
            });
        }
        if field_deltas.is_empty() {
            continue;
        }
        rows.push(DeclarativeBranchCompareRow {
            left_identity: left_row.map(|row| row.identity().to_string()),
            right_identity: right_row.map(|row| row.identity().to_string()),
            label: right_row
                .or(left_row)
                .map(|row| row.label().to_string())
                .unwrap_or_else(|| identity.clone()),
            identity_class: match (left_row, right_row) {
                (Some(_), Some(_)) => DeclarativeBranchCompareIdentityClass::AuthoritativeIdentity,
                (None, Some(_)) => DeclarativeBranchCompareIdentityClass::BranchLocalAddition,
                (Some(_), None) => DeclarativeBranchCompareIdentityClass::BranchLocalRemoval,
                (None, None) => continue,
            },
            field_deltas,
        });
    }

    let result_digest = hash_parts(
        &rows
            .iter()
            .map(DeclarativeBranchCompareRow::digest_part)
            .chain([
                format!("query:{left_query_digest}"),
                format!(
                    "left_basis:{}",
                    left.preflight().basis().proof().digest().as_str()
                ),
                format!(
                    "right_basis:{}",
                    right.preflight().basis().proof().digest().as_str()
                ),
                format!(
                    "left_live:{}",
                    left.live_view()
                        .core_live_plan()
                        .subscription_digest()
                        .as_str()
                ),
                format!(
                    "right_live:{}",
                    right
                        .live_view()
                        .core_live_plan()
                        .subscription_digest()
                        .as_str()
                ),
            ])
            .collect::<Vec<_>>(),
    );

    Ok(DeclarativeBranchCompareArtifact {
        left_live_view_digest: left
            .live_view()
            .core_live_plan()
            .subscription_digest()
            .as_str()
            .to_string(),
        right_live_view_digest: right
            .live_view()
            .core_live_plan()
            .subscription_digest()
            .as_str()
            .to_string(),
        left_basis_digest: left
            .preflight()
            .basis()
            .proof()
            .digest()
            .as_str()
            .to_string(),
        right_basis_digest: right
            .preflight()
            .basis()
            .proof()
            .digest()
            .as_str()
            .to_string(),
        query_digest: left_query_digest.to_string(),
        result_digest,
        rows,
    })
}

fn plan_declarative_request(
    request: &DeclarativeLiveQueryRequest,
    canonical: &CanonicalQueryBundle,
    schema_view: QuerySchemaView,
    basis_intent: ExecutionBasisIntent,
) -> Result<ViewShapePlanArtifact, DeclarativeLiveQueryError> {
    let admitted = admit_view_shape(&canonical, request.view_shape().view_shape_descriptor())
        .map_err(|error| DeclarativeLiveQueryError::ViewShape(format!("{error:?}")))?;
    let validated =
        validate_canonical_bundle_for_admitted_view_shape(&canonical, schema_view, admitted)
            .map_err(|error| DeclarativeLiveQueryError::ViewShape(format!("{error:?}")))?;
    plan_admitted_view_shape(validated, basis_intent)
        .map_err(|error| DeclarativeLiveQueryError::ViewShape(format!("{error:?}")))
}

fn finish_declarative_live_query_session(
    request: DeclarativeLiveQueryRequest,
    canonical: CanonicalQueryBundle,
    view_plan: ViewShapePlanArtifact,
    basis: ResolvedSnapshotBasis,
    grouped_baseline: Option<AuthoritativeGroupedBaselineArtifact>,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    let preflight = preflight_execution_basis(view_plan.execution_plan().clone(), basis)
        .map_err(|error| DeclarativeLiveQueryError::BasisPreflight(format!("{error:?}")))?;

    if matches!(
        request.view_shape(),
        DeclarativeLiveViewShape::IdentityAwareInspectorFocused { .. }
    ) && request.inspector_identity.is_none()
    {
        return Err(DeclarativeLiveQueryError::InspectorIdentityBindingRequired);
    }

    let live_view = lower_view_shape_plan_to_live(
        &view_plan,
        preflight.basis().clone(),
        grouped_baseline,
        request.inspector_identity.clone(),
    )
    .map_err(|error| DeclarativeLiveQueryError::LiveLowering(format!("{error:?}")))?;

    Ok(DeclarativeLiveQuerySession {
        request,
        canonical,
        view_plan,
        preflight,
        live_view,
    })
}

pub(crate) fn canonicalize_declarative_request(
    request: &DeclarativeLiveQueryRequest,
) -> Result<CanonicalQueryBundle, DeclarativeLiveQueryError> {
    let root = RootEntityKey::new(request.target())
        .map_err(|_| DeclarativeLiveQueryError::InvalidTarget)?;
    let query_projection = normalized_query_projection(request);
    let result_fields = normalized_result_fields(request, &query_projection);

    if request.view_shape().collection_backed() {
        let ordering = normalized_ordering(request);
        let mut query = RawAuthoredQuery::collection_builder(root);
        for field in &query_projection {
            query = query.project(AspectFieldSelector::from_source_field_key(
                field.source_field_key().clone(),
            ));
        }
        for filter in request.predicate_filters() {
            query = apply_declarative_predicate_filter(query, filter)?;
        }
        for traversal in request.traversal() {
            query = query.traverse(traversal.clone());
        }
        for ordering in &ordering {
            query = apply_declarative_ordering(query, ordering)?;
        }

        let mut shape = RawAuthoredResultShape::collection_builder();
        for field in &result_fields {
            shape = shape.field(
                AuthoredResultShapeField::from_source_field_key(
                    field.source_field_key().clone(),
                    field.delivered_name(),
                )
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            );
        }
        GuidedAuthoringPath::canonicalize_collection(
            query
                .build()
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            shape
                .build()
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
        )
        .map_err(|error| DeclarativeLiveQueryError::Canonicalization(format!("{error:?}")))
    } else {
        let mut query = RawAuthoredQuery::detail_builder(root);
        for field in &query_projection {
            query = query.project(AspectFieldSelector::from_source_field_key(
                field.source_field_key().clone(),
            ));
        }
        for filter in request.predicate_filters() {
            query = apply_declarative_predicate_filter(query, filter)?;
        }
        for traversal in request.traversal() {
            query = query.traverse(traversal.clone());
        }

        let mut shape = RawAuthoredResultShape::detail_builder();
        for field in &result_fields {
            shape = shape.field(
                AuthoredResultShapeField::from_source_field_key(
                    field.source_field_key().clone(),
                    field.delivered_name(),
                )
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            );
        }
        GuidedAuthoringPath::canonicalize_detail(
            query
                .build()
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            shape
                .build()
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
        )
        .map_err(|error| DeclarativeLiveQueryError::Canonicalization(format!("{error:?}")))
    }
}

fn normalized_query_projection(
    request: &DeclarativeLiveQueryRequest,
) -> Vec<DeclarativeProjectionField> {
    let mut fields = request.query_projection().to_vec();
    if fields.is_empty() {
        fields.push(DeclarativeProjectionField::from_authoring_parts(
            "identity", "id",
        ));
        for filter in request.predicate_filters() {
            push_unique_field(&mut fields, declarative_field_from_predicate(filter));
        }
    }
    if request.view_shape().collection_backed() {
        let ordering = normalized_ordering(request);
        for field in ordering {
            push_unique_field(
                &mut fields,
                DeclarativeProjectionField::new(field.source_field_key().clone()),
            );
        }
    }
    fields
}

fn normalized_result_fields(
    request: &DeclarativeLiveQueryRequest,
    query_projection: &[DeclarativeProjectionField],
) -> Vec<DeclarativeProjectionField> {
    if request.result_fields().is_empty() {
        query_projection.to_vec()
    } else {
        request.result_fields().to_vec()
    }
}

fn normalized_ordering(request: &DeclarativeLiveQueryRequest) -> Vec<DeclarativeOrderingField> {
    if request.ordering().is_empty() && request.view_shape().collection_backed() {
        vec![DeclarativeOrderingField::ascending(
            AspectFieldKey::from_authoring_parts("identity", "id")
                .expect("default collection ordering requires identity.id"),
        )]
    } else {
        request.ordering().to_vec()
    }
}

fn declarative_field_from_predicate(
    filter: &DeclarativePredicateFilter,
) -> DeclarativeProjectionField {
    DeclarativeProjectionField::new(filter.source_field_key().clone())
}

pub(crate) fn validate_declared_traversal_contract(
    request: &DeclarativeLiveQueryRequest,
    schema_view: &QuerySchemaView,
) -> Result<(), DeclarativeLiveQueryError> {
    let mut seen = BTreeSet::new();
    for traversal in request.traversal() {
        let relation = traversal
            .terminal_relation_projection_for_boundary()
            .to_string();
        let depth = traversal.depth();
        if !seen.insert((relation.clone(), depth)) {
            return Err(DeclarativeLiveQueryError::DuplicateTraversal { relation, depth });
        }
        let Some(schema_relation) = schema_view.relation(traversal.relation_name()) else {
            return Err(DeclarativeLiveQueryError::TraversalNotDeclaredInSchema {
                relation,
                requested_depth: depth,
            });
        };
        if depth > schema_relation.max_depth() {
            return Err(DeclarativeLiveQueryError::TraversalExceedsSchemaDepth {
                relation,
                requested_depth: depth,
                max_depth: schema_relation.max_depth(),
            });
        }
    }
    Ok(())
}

fn push_unique_field(
    fields: &mut Vec<DeclarativeProjectionField>,
    candidate: DeclarativeProjectionField,
) {
    if !fields
        .iter()
        .any(|field| field.source_field_key() == candidate.source_field_key())
    {
        fields.push(candidate);
    }
}

fn apply_declarative_predicate_filter<F: crate::authoring::QueryAuthoringFamily>(
    mut query: crate::authoring::QueryBuilder<F>,
    filter: &DeclarativePredicateFilter,
) -> Result<crate::authoring::QueryBuilder<F>, DeclarativeLiveQueryError> {
    query = match filter {
        DeclarativePredicateFilter::Equality(filter) => {
            query.where_equal(EqualityPredicate::from_target_field_key(
                filter.source_field_key().clone(),
                filter.value().clone(),
            ))
        }
        DeclarativePredicateFilter::IntegerComparison(filter) => match filter.operator() {
            IntegerComparisonOperator::GreaterThan => {
                query.where_greater_than(IntegerComparisonPredicate::greater_than_target_field_key(
                    filter.source_field_key().clone(),
                    filter.value(),
                ))
            }
            IntegerComparisonOperator::LessThan => {
                query.where_less_than(IntegerComparisonPredicate::less_than_target_field_key(
                    filter.source_field_key().clone(),
                    filter.value(),
                ))
            }
        },
        DeclarativePredicateFilter::StringContains(filter) => {
            query.where_contains(StringContainsPredicate::from_target_field_key(
                filter.source_field_key().clone(),
                filter.value(),
            ))
        }
        DeclarativePredicateFilter::SetMembership(filter) => query.where_in(
            SetMembershipPredicate::from_target_field_key(
                filter.source_field_key().clone(),
                filter.values().iter().cloned(),
            )
            .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
        ),
        DeclarativePredicateFilter::Presence(filter) => query.where_present(
            PresencePredicate::is_present_target_field_key(filter.source_field_key().clone()),
        ),
    };
    Ok(query)
}

fn apply_declarative_ordering<F: crate::authoring::QueryAuthoringFamily>(
    query: crate::authoring::QueryBuilder<F>,
    ordering: &DeclarativeOrderingField,
) -> Result<crate::authoring::QueryBuilder<F>, DeclarativeLiveQueryError> {
    let selector = match ordering.direction() {
        OrderingDirection::Ascending => {
            OrderingSelector::ascending_source_field_key(ordering.source_field_key().clone())
        }
        OrderingDirection::Descending => {
            OrderingSelector::descending_source_field_key(ordering.source_field_key().clone())
        }
    };
    Ok(query.order_by(selector))
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::FieldKey;

    use super::*;
    use crate::evidence_identity::{
        ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
    };
    use crate::schema_view::{SchemaFieldKind, SchemaFieldView, SchemaRelationView};
    use crate::view_shape_live::LiveViewShapeFamily;
    use crate::workflow::{WorkflowFreshnessBinding, WorkflowStalenessClass};

    fn todo_schema() -> QuerySchemaView {
        QuerySchemaView::new(
            "todo-demo-schema",
            [
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("identity")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("id")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("status")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("state")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("title")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("value")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
            ],
            [],
        )
    }

    fn test_snapshot_identity(label: &'static str) -> ForgeQuerySnapshotIdentity {
        ForgeQuerySnapshotIdentity::preview(
            ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
            )
            .field_shape(ForgeQueryEvidenceTag::new("test_snapshot"), label)
            .seal(),
        )
    }

    fn test_field_key(aspect: &str, field: &str) -> AspectFieldKey {
        let aspect = AspectKey::new(aspect).expect("test aspect key should be valid");
        let field = FieldKey::new(field).expect("test field key should be valid");
        AspectFieldKey::from_native_keys(&aspect, &field)
    }

    #[test]
    fn runtime_list_splice_declaration_mints_real_live_session_with_hidden_basis() {
        let request =
            DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::list_splice())
                .where_equal(DeclarativeEqualityFilter::new(
                    test_field_key("status", "state"),
                    ScalarPredicateValue::String("incomplete".to_string()),
                ));

        let snapshot_identity = test_snapshot_identity("runtime-head-demo");
        let session =
            declare_runtime_live_query_session(request, todo_schema(), snapshot_identity.clone())
                .expect("declarative list splice should plan, preflight, and lower to live");

        assert_eq!(session.request().target(), "Todo");
        assert_eq!(
            session.live_view().lowering().family(),
            LiveViewShapeFamily::Table
        );
        assert_eq!(
            session.preflight().basis().identity().snapshot_identity(),
            &snapshot_identity.evidence_identity()
        );
        assert_eq!(
            session.preflight().basis().identity().schema_basis(),
            session.view_plan().validated().query().schema_basis()
        );
        assert_eq!(
            session.preflight().basis().proof().digest(),
            session.live_view().basis().proof().digest()
        );
    }

    #[test]
    fn projection_defaults_include_filter_and_ordering_fields_without_host_knobs() {
        let request =
            DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::list_splice())
                .where_equal(DeclarativeEqualityFilter::new(
                    test_field_key("status", "state"),
                    ScalarPredicateValue::String("incomplete".to_string()),
                ));

        let fields = normalized_query_projection(&request);

        assert_eq!(
            fields
                .iter()
                .map(|field| {
                    (
                        field.source_field_key().aspect().as_str(),
                        field.source_field_key().field().as_str(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![("identity", "id"), ("status", "state")]
        );
    }

    #[test]
    fn writeback_from_live_session_preserves_basis_and_detected_aspect_intent() {
        let session = declare_runtime_live_query_session(
            DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::list_splice())
                .where_equal(DeclarativeEqualityFilter::new(
                    test_field_key("status", "state"),
                    ScalarPredicateValue::String("incomplete".to_string()),
                )),
            todo_schema(),
            test_snapshot_identity("runtime-head-writeback"),
        )
        .expect("runtime live query should admit");

        let artifact = declare_writeback_from_live_session(
            &session,
            DeclarativeWritebackIntent::update_aspect(
                AspectFieldKey::from_authoring_parts("title", "value").unwrap(),
                DeclarativeWritebackValue::string("Buy oat milk"),
            ),
        )
        .expect("SDK-detected local proxy edit should lower to bridge writeback declaration");

        assert_eq!(artifact.changes().len(), 1);
        assert_eq!(
            artifact.changes()[0].source_field_key().aspect().as_str(),
            "title"
        );
        assert_eq!(
            artifact.changes()[0].source_field_key().field().as_str(),
            "value"
        );
        assert_eq!(
            artifact.live_view_basis_digest(),
            session.preflight().basis().proof().digest().as_str()
        );
        assert!(!artifact.intent_digest().is_empty());
        assert!(!artifact.artifact_digest().is_empty());
        assert_eq!(
            artifact.declaration().freshness_binding(),
            &WorkflowFreshnessBinding::RuntimeBasisExact
        );
        assert_eq!(
            artifact.declaration().staleness_class(),
            &WorkflowStalenessClass::AuthorityValidationRequired
        );
    }

    #[test]
    fn empty_writeback_intent_is_rejected_before_bridge_lowering() {
        let session = declare_runtime_live_query_session(
            DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::table()),
            todo_schema(),
            test_snapshot_identity("runtime-head-empty-writeback"),
        )
        .expect("runtime live query should admit");

        let error =
            declare_writeback_from_live_session(&session, DeclarativeWritebackIntent::new([]))
                .expect_err("empty proxy flushes should never mint writeback authority");

        assert_eq!(error, DeclarativeLiveQueryError::EmptyWritebackIntent);
    }

    #[test]
    fn runtime_declarative_request_preserves_traversal_into_canonical_query() {
        let request = DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::detail())
            .project(DeclarativeProjectionField::from_authoring_parts(
                "identity", "id",
            ))
            .traverse(TraversalSelector::bounded("worth.todo_parent", 2).unwrap());
        let schema = QuerySchemaView::new(
            "todo-demo-schema-with-traversal",
            [
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("identity")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("id")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("status")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("state")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("title")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("value")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
            ],
            [SchemaRelationView::new(
                crate::authoring::RelationName::new("worth.todo_parent")
                    .expect("schema relation literal must be valid"),
                2,
            )],
        );

        let session = declare_runtime_live_query_session(
            request,
            schema,
            test_snapshot_identity("runtime-head-traversal"),
        )
        .expect("declarative traversal should lower into the canonical query");

        assert_eq!(session.request().traversal().len(), 1);
        assert_eq!(session.canonical().query().traversal().len(), 1);
        assert_eq!(
            session.canonical().query().traversal()[0].relation.as_str(),
            "worth.todo_parent"
        );
        assert_eq!(session.canonical().query().traversal()[0].depth, 2);
    }

    #[test]
    fn runtime_declarative_request_rejects_duplicate_traversal_before_canonicalization() {
        let request = DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::detail())
            .project(DeclarativeProjectionField::from_authoring_parts(
                "identity", "id",
            ))
            .traverse(TraversalSelector::bounded("worth.todo_parent", 2).unwrap())
            .traverse(TraversalSelector::bounded("worth.todo_parent", 2).unwrap());
        let schema = QuerySchemaView::new(
            "todo-demo-schema-with-traversal",
            [
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("identity")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("id")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("status")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("state")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
                SchemaFieldView::new(
                    crate::authoring::AspectName::new("title")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("value")
                        .expect("schema field literal must be valid"),
                    SchemaFieldKind::String,
                ),
            ],
            [SchemaRelationView::new(
                crate::authoring::RelationName::new("worth.todo_parent")
                    .expect("schema relation literal must be valid"),
                2,
            )],
        );

        let error = declare_runtime_live_query_session(
            request,
            schema,
            test_snapshot_identity("runtime-head-traversal"),
        )
        .expect_err("duplicate traversal should fail at the declarative boundary");

        assert!(matches!(
            error,
            DeclarativeLiveQueryError::DuplicateTraversal { .. }
        ));
    }

    #[test]
    fn declarative_request_preserves_query_only_projection_and_delivered_result_fields() {
        let request = DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::table())
            .project_query_only(DeclarativeProjectionField::from_authoring_parts(
                "identity", "id",
            ))
            .result_field(
                DeclarativeProjectionField::from_authoring_parts("title", "value")
                    .delivered_as("title"),
            )
            .order_by_direction(DeclarativeOrderingField::descending(
                AspectFieldKey::from_authoring_parts("title", "value")
                    .expect("test ordering key should be valid"),
            ));

        let query_projection = normalized_query_projection(&request);
        let result_fields = normalized_result_fields(&request, &query_projection);

        assert_eq!(
            query_projection
                .iter()
                .map(|field| {
                    (
                        field.source_field_key().aspect().as_str(),
                        field.source_field_key().field().as_str(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![("identity", "id"), ("title", "value")]
        );
        assert_eq!(
            result_fields
                .iter()
                .map(|field| {
                    (
                        field.source_field_key().aspect().as_str(),
                        field.source_field_key().field().as_str(),
                        field.delivered_name(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![("title", "value", "title")]
        );
    }

    #[test]
    fn runtime_declarative_request_preserves_non_equality_predicates_and_descending_ordering() {
        let request = DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::table())
            .project(DeclarativeProjectionField::from_authoring_parts(
                "identity", "id",
            ))
            .where_greater_than(DeclarativeIntegerComparisonFilter::greater_than(
                test_field_key("metrics", "priority"),
                5,
            ))
            .where_contains(DeclarativeStringContainsFilter::new(
                test_field_key("title", "value"),
                "milk",
            ))
            .where_in(DeclarativeSetMembershipFilter::new(
                test_field_key("status", "state"),
                [
                    ScalarPredicateValue::String("todo".to_string()),
                    ScalarPredicateValue::String("doing".to_string()),
                ],
            ))
            .where_present(DeclarativePresenceFilter::is_present(test_field_key(
                "owner", "name",
            )))
            .order_by_direction(DeclarativeOrderingField::descending(test_field_key(
                "metrics", "priority",
            )));

        let canonical = canonicalize_declarative_request(&request)
            .expect("declarative request should preserve full predicate families");

        assert_eq!(canonical.query().predicates().len(), 4);
        assert_eq!(canonical.query().ordering().len(), 1);
        assert_eq!(
            canonical.query().ordering()[0].direction,
            OrderingDirection::Descending
        );
    }
}
