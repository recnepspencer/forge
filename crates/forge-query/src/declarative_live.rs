use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, GuidedAuthoringPath,
    OrderingSelector, RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
    ScalarPredicateValue,
};
use crate::basis::{
    preflight_execution_basis, resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode,
    ExecutionBasisIntent, ExecutionPreflightBundle, ResolvedSnapshotBasis,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::canonicalization::CanonicalQueryBundle;
use crate::identity::hash_parts;
use crate::identity_evolution::{InspectorIdentityArtifact, InspectorIdentityClassification};
use crate::schema_view::QuerySchemaView;
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor, ViewShapePlanArtifact,
};
use crate::view_shape_live::{lower_view_shape_plan_to_live, LiveViewShapeArtifact};
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, lower_query_writeback_declaration,
    QueryWritebackDeclaration, WorkflowAuthorityTargetFamily, WorkflowBindingSource,
    WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
    WorkflowFreshnessPolicy, WritebackLoweringInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeProjectionField {
    aspect: String,
    field: String,
    delivered_name: String,
}

impl DeclarativeProjectionField {
    pub fn new(aspect: impl Into<String>, field: impl Into<String>) -> Self {
        let aspect = aspect.into();
        let field = field.into();
        Self {
            delivered_name: field.clone(),
            aspect,
            field,
        }
    }

    pub fn delivered_as(mut self, delivered_name: impl Into<String>) -> Self {
        self.delivered_name = delivered_name.into();
        self
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn delivered_name(&self) -> &str {
        &self.delivered_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeEqualityFilter {
    aspect: String,
    field: String,
    value: ScalarPredicateValue,
}

impl DeclarativeEqualityFilter {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: ScalarPredicateValue,
    ) -> Self {
        Self {
            aspect: aspect.into(),
            field: field.into(),
            value,
        }
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
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
        focused_aspect: String,
    },
    IdentityAwareInspectorFocused {
        focused_aspect: String,
        classification: InspectorIdentityClassification,
    },
    KanbanGrouped {
        grouping_aspect: String,
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

    pub fn inspector_focused(focused_aspect: impl Into<String>) -> Self {
        Self::InspectorFocused {
            focused_aspect: focused_aspect.into(),
        }
    }

    pub fn identity_aware_inspector_focused(
        focused_aspect: impl Into<String>,
        classification: InspectorIdentityClassification,
    ) -> Self {
        Self::IdentityAwareInspectorFocused {
            focused_aspect: focused_aspect.into(),
            classification,
        }
    }

    pub fn kanban_grouped(grouping_aspect: impl Into<String>) -> Self {
        Self::KanbanGrouped {
            grouping_aspect: grouping_aspect.into(),
        }
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
                ViewShapeDescriptor::inspector_detail_focused(focused_aspect)
            }
            Self::IdentityAwareInspectorFocused {
                focused_aspect,
                classification,
            } => ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                focused_aspect,
                *classification,
            ),
            Self::KanbanGrouped { grouping_aspect } => {
                ViewShapeDescriptor::kanban_grouped(grouping_aspect)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeLiveQueryRequest {
    target: String,
    view_shape: DeclarativeLiveViewShape,
    projection: Vec<DeclarativeProjectionField>,
    equality_filters: Vec<DeclarativeEqualityFilter>,
    ordering: Option<DeclarativeProjectionField>,
    inspector_identity: Option<InspectorIdentityArtifact>,
}

impl DeclarativeLiveQueryRequest {
    pub fn new(target: impl Into<String>, view_shape: DeclarativeLiveViewShape) -> Self {
        Self {
            target: target.into(),
            view_shape,
            projection: Vec::new(),
            equality_filters: Vec::new(),
            ordering: None,
            inspector_identity: None,
        }
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn view_shape(&self) -> &DeclarativeLiveViewShape {
        &self.view_shape
    }

    pub fn projection(&self) -> &[DeclarativeProjectionField] {
        &self.projection
    }

    pub fn equality_filters(&self) -> &[DeclarativeEqualityFilter] {
        &self.equality_filters
    }

    pub fn ordering(&self) -> Option<&DeclarativeProjectionField> {
        self.ordering.as_ref()
    }

    pub fn inspector_identity(&self) -> Option<&InspectorIdentityArtifact> {
        self.inspector_identity.as_ref()
    }

    pub fn project(mut self, field: DeclarativeProjectionField) -> Self {
        self.projection.push(field);
        self
    }

    pub fn where_equal(mut self, filter: DeclarativeEqualityFilter) -> Self {
        self.equality_filters.push(filter);
        self
    }

    pub fn order_by(mut self, field: DeclarativeProjectionField) -> Self {
        self.ordering = Some(field);
        self
    }

    pub fn with_inspector_identity(mut self, artifact: InspectorIdentityArtifact) -> Self {
        self.inspector_identity = Some(artifact);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativeLiveQueryError {
    InvalidTarget,
    Authoring(String),
    Canonicalization(String),
    ViewShape(String),
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
pub enum DeclarativeWritebackValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    StructuredJson(String),
}

impl DeclarativeWritebackValue {
    fn digest_part(&self) -> String {
        match self {
            Self::String(value) => format!("string:{value}"),
            Self::Integer(value) => format!("integer:{value}"),
            Self::Boolean(value) => format!("boolean:{value}"),
            Self::StructuredJson(value) => format!("structured_json:{value}"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeWritebackChange {
    aspect: String,
    field: String,
    value: DeclarativeWritebackValue,
}

impl DeclarativeWritebackChange {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: DeclarativeWritebackValue,
    ) -> Self {
        Self {
            aspect: aspect.into(),
            field: field.into(),
            value,
        }
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn value(&self) -> &DeclarativeWritebackValue {
        &self.value
    }

    fn digest_part(&self) -> String {
        format!(
            "change:{}:{}:{}",
            self.aspect,
            self.field,
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

    pub fn update_aspect(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: DeclarativeWritebackValue,
    ) -> Self {
        Self::new([DeclarativeWritebackChange::new(aspect, field, value)])
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
    snapshot_token: impl Into<String>,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    let basis_intent = ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    );
    let canonical = canonicalize_declarative_request(&request)?;
    let view_plan =
        plan_declarative_request(&request, &canonical, schema_view, basis_intent.clone())?;
    let identity = ResolvedSnapshotIdentity::new(
        BasisAuthorityFamily::Runtime,
        None,
        snapshot_token,
        view_plan.validated().query().schema_basis().clone(),
        SnapshotLineageClass::CurrentHead,
    );
    let basis = resolve_snapshot_basis(basis_intent, identity, BasisResolutionMode::RuntimeDirect)
        .map_err(|error| DeclarativeLiveQueryError::BasisResolution(format!("{error:?}")))?;

    finish_declarative_live_query_session(request, canonical, view_plan, basis)
}

pub fn declare_live_query_session(
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    basis_intent: ExecutionBasisIntent,
    basis: ResolvedSnapshotBasis,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    let canonical = canonicalize_declarative_request(&request)?;
    let view_plan = plan_declarative_request(&request, &canonical, schema_view, basis_intent)?;
    finish_declarative_live_query_session(request, canonical, view_plan, basis)
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
        format!("writeback:{}", declaration.lowering_digest()),
    ]);

    Ok(DeclarativeWritebackArtifact {
        live_view_basis_digest,
        intent_digest,
        changes: intent.changes,
        declaration,
        artifact_digest,
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
        None,
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

fn canonicalize_declarative_request(
    request: &DeclarativeLiveQueryRequest,
) -> Result<CanonicalQueryBundle, DeclarativeLiveQueryError> {
    let root = RootEntityKey::new(request.target())
        .map_err(|_| DeclarativeLiveQueryError::InvalidTarget)?;
    let projection = normalized_projection(request);

    if request.view_shape().collection_backed() {
        let mut query = RawAuthoredQuery::collection_builder(root);
        for field in &projection {
            query = query.project(
                AspectFieldSelector::new(field.aspect(), field.field())
                    .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            );
        }
        for filter in request.equality_filters() {
            query = query.where_equal(
                EqualityPredicate::new(filter.aspect(), filter.field(), filter.value().clone())
                    .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            );
        }
        let ordering = request
            .ordering
            .clone()
            .unwrap_or_else(|| DeclarativeProjectionField::new("identity", "id"));
        query = query.order_by(
            OrderingSelector::ascending(ordering.aspect(), ordering.field())
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
        );

        let mut shape = RawAuthoredResultShape::collection_builder();
        for field in &projection {
            shape = shape.field(
                AuthoredResultShapeField::new(
                    field.aspect(),
                    field.field(),
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
        for field in &projection {
            query = query.project(
                AspectFieldSelector::new(field.aspect(), field.field())
                    .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            );
        }
        for filter in request.equality_filters() {
            query = query.where_equal(
                EqualityPredicate::new(filter.aspect(), filter.field(), filter.value().clone())
                    .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            );
        }

        let mut shape = RawAuthoredResultShape::detail_builder();
        for field in &projection {
            shape = shape.field(
                AuthoredResultShapeField::new(
                    field.aspect(),
                    field.field(),
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

fn normalized_projection(request: &DeclarativeLiveQueryRequest) -> Vec<DeclarativeProjectionField> {
    let mut fields = request.projection().to_vec();
    if fields.is_empty() {
        fields.push(DeclarativeProjectionField::new("identity", "id"));
        for filter in request.equality_filters() {
            push_unique_field(
                &mut fields,
                DeclarativeProjectionField::new(filter.aspect(), filter.field()),
            );
        }
    }
    if request.view_shape().collection_backed() {
        let ordering = request
            .ordering
            .clone()
            .unwrap_or_else(|| DeclarativeProjectionField::new("identity", "id"));
        push_unique_field(&mut fields, ordering);
    }
    fields
}

fn push_unique_field(
    fields: &mut Vec<DeclarativeProjectionField>,
    candidate: DeclarativeProjectionField,
) {
    if !fields
        .iter()
        .any(|field| field.aspect() == candidate.aspect() && field.field() == candidate.field())
    {
        fields.push(candidate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_view::{SchemaFieldKind, SchemaFieldView};
    use crate::view_shape_live::LiveViewShapeFamily;
    use crate::workflow::{WorkflowFreshnessBinding, WorkflowStalenessClass};

    fn todo_schema() -> QuerySchemaView {
        QuerySchemaView::new(
            "todo-demo-schema",
            [
                SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                SchemaFieldView::new("status", "state", SchemaFieldKind::String),
                SchemaFieldView::new("title", "value", SchemaFieldKind::String),
            ],
            [],
        )
    }

    #[test]
    fn runtime_list_splice_declaration_mints_real_live_session_with_hidden_basis() {
        let request =
            DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::list_splice())
                .where_equal(DeclarativeEqualityFilter::new(
                    "status",
                    "state",
                    ScalarPredicateValue::String("incomplete".to_string()),
                ));

        let session =
            declare_runtime_live_query_session(request, todo_schema(), "runtime-head-demo")
                .expect("declarative list splice should plan, preflight, and lower to live");

        assert_eq!(session.request().target(), "Todo");
        assert_eq!(
            session.live_view().lowering().family(),
            LiveViewShapeFamily::Table
        );
        assert_eq!(
            session.preflight().basis().identity().snapshot_token(),
            "runtime-head-demo"
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
                    "status",
                    "state",
                    ScalarPredicateValue::String("incomplete".to_string()),
                ));

        let fields = normalized_projection(&request);

        assert_eq!(
            fields
                .iter()
                .map(|field| (field.aspect(), field.field()))
                .collect::<Vec<_>>(),
            vec![("identity", "id"), ("status", "state")]
        );
    }

    #[test]
    fn writeback_from_live_session_preserves_basis_and_detected_aspect_intent() {
        let session = declare_runtime_live_query_session(
            DeclarativeLiveQueryRequest::new("Todo", DeclarativeLiveViewShape::list_splice())
                .where_equal(DeclarativeEqualityFilter::new(
                    "status",
                    "state",
                    ScalarPredicateValue::String("incomplete".to_string()),
                )),
            todo_schema(),
            "runtime-head-writeback",
        )
        .expect("runtime live query should admit");

        let artifact = declare_writeback_from_live_session(
            &session,
            DeclarativeWritebackIntent::update_aspect(
                "title",
                "value",
                DeclarativeWritebackValue::String("Buy oat milk".to_string()),
            ),
        )
        .expect("SDK-detected local proxy edit should lower to bridge writeback declaration");

        assert_eq!(artifact.changes().len(), 1);
        assert_eq!(artifact.changes()[0].aspect(), "title");
        assert_eq!(artifact.changes()[0].field(), "value");
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
            "runtime-head-empty-writeback",
        )
        .expect("runtime live query should admit");

        let error =
            declare_writeback_from_live_session(&session, DeclarativeWritebackIntent::new([]))
                .expect_err("empty proxy flushes should never mint writeback authority");

        assert_eq!(error, DeclarativeLiveQueryError::EmptyWritebackIntent);
    }
}
