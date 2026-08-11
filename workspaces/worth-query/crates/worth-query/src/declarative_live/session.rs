use crate::basis::{
    preflight_execution_basis, resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode,
    ExecutionBasisIntent, ExecutionPreflightBundle, ResolvedSnapshotBasis,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::canonicalization::CanonicalQueryBundle;
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::planning::ExecutionPlanBundle;
use crate::schema_view::QuerySchemaView;
use crate::validation::ValidatedQueryBundle;
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, plan_admitted_view_shape_from_execution_plan,
    validate_canonical_bundle_for_admitted_view_shape, ViewShapePlanArtifact,
    ViewShapeValidatedBundle,
};
use crate::view_shape_live::{
    lower_view_shape_plan_to_live, materialize_authoritative_grouped_baseline_from_members,
    AuthoritativeGroupedBaselineArtifact, LiveViewShapeArtifact, WorthQueryGroupedBaselineMember,
};

use super::canonicalization::{
    canonicalize_declarative_request, validate_declared_traversal_contract,
};
use super::request::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};

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

#[cfg(test)]
pub fn declare_runtime_live_query_session(
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    snapshot_identity: WorthQuerySnapshotIdentity,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    declare_runtime_live_query_session_with_grouped_baseline(
        request,
        schema_view,
        snapshot_identity,
        None::<Vec<WorthQueryGroupedBaselineMember>>,
    )
}

pub fn declare_runtime_live_query_session_with_grouped_baseline(
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
    snapshot_identity: WorthQuerySnapshotIdentity,
    grouped_baseline_members: Option<impl IntoIterator<Item = WorthQueryGroupedBaselineMember>>,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    let basis_intent = runtime_current_basis_intent();
    validate_declared_traversal_contract(&request, &schema_view)?;
    let canonical = canonicalize_declarative_request(&request)?;
    let view_plan =
        plan_declarative_request(&request, &canonical, schema_view, basis_intent.clone())?;
    let basis = resolve_runtime_current_basis(&view_plan, snapshot_identity, basis_intent)?;
    let grouped_baseline =
        materialize_optional_grouped_baseline(&view_plan, &basis, grouped_baseline_members)?;

    finish_declarative_live_query_session(request, canonical, view_plan, basis, grouped_baseline)
}

pub(crate) fn declare_runtime_live_query_session_from_admitted_read(
    request: DeclarativeLiveQueryRequest,
    canonical: CanonicalQueryBundle,
    validated: ValidatedQueryBundle,
    execution_plan: ExecutionPlanBundle,
    snapshot_identity: WorthQuerySnapshotIdentity,
    grouped_baseline_members: Option<impl IntoIterator<Item = WorthQueryGroupedBaselineMember>>,
) -> Result<DeclarativeLiveQuerySession, DeclarativeLiveQueryError> {
    let basis_intent = runtime_current_basis_intent();
    let admitted = admit_view_shape(&canonical, request.view_shape().view_shape_descriptor())
        .map_err(|error| DeclarativeLiveQueryError::ViewShape(format!("{error:?}")))?;
    let validated_view = ViewShapeValidatedBundle::new(canonical.clone(), admitted, validated);
    let view_plan = plan_admitted_view_shape_from_execution_plan(validated_view, execution_plan)
        .map_err(|error| DeclarativeLiveQueryError::ViewShape(format!("{error:?}")))?;
    let basis = resolve_runtime_current_basis(&view_plan, snapshot_identity, basis_intent)?;
    let grouped_baseline =
        materialize_optional_grouped_baseline(&view_plan, &basis, grouped_baseline_members)?;

    finish_declarative_live_query_session(request, canonical, view_plan, basis, grouped_baseline)
}

fn runtime_current_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

fn resolve_runtime_current_basis(
    view_plan: &ViewShapePlanArtifact,
    snapshot_identity: WorthQuerySnapshotIdentity,
    basis_intent: ExecutionBasisIntent,
) -> Result<ResolvedSnapshotBasis, DeclarativeLiveQueryError> {
    let identity = ResolvedSnapshotIdentity::new(
        BasisAuthorityFamily::Runtime,
        None,
        snapshot_identity.evidence_identity(),
        view_plan.validated().query().schema_basis().clone(),
        SnapshotLineageClass::CurrentHead,
    );
    resolve_snapshot_basis(basis_intent, identity, BasisResolutionMode::RuntimeDirect)
        .map_err(|error| DeclarativeLiveQueryError::BasisResolution(format!("{error:?}")))
}

fn materialize_optional_grouped_baseline<I>(
    view_plan: &ViewShapePlanArtifact,
    basis: &ResolvedSnapshotBasis,
    grouped_baseline_members: Option<I>,
) -> Result<Option<AuthoritativeGroupedBaselineArtifact>, DeclarativeLiveQueryError>
where
    I: IntoIterator<Item = WorthQueryGroupedBaselineMember>,
{
    grouped_baseline_members
        .map(|members| {
            materialize_authoritative_grouped_baseline_from_members(
                view_plan,
                basis.clone(),
                members,
            )
            .map_err(|error| DeclarativeLiveQueryError::LiveLowering(format!("{error:?}")))
        })
        .transpose()
}

fn plan_declarative_request(
    request: &DeclarativeLiveQueryRequest,
    canonical: &CanonicalQueryBundle,
    schema_view: QuerySchemaView,
    basis_intent: ExecutionBasisIntent,
) -> Result<ViewShapePlanArtifact, DeclarativeLiveQueryError> {
    let admitted = admit_view_shape(canonical, request.view_shape().view_shape_descriptor())
        .map_err(|error| DeclarativeLiveQueryError::ViewShape(format!("{error:?}")))?;
    let validated =
        validate_canonical_bundle_for_admitted_view_shape(canonical, schema_view, admitted)
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
    ) && request.inspector_identity().is_none()
    {
        return Err(DeclarativeLiveQueryError::InspectorIdentityBindingRequired);
    }

    let live_view = lower_view_shape_plan_to_live(
        &view_plan,
        preflight.basis().clone(),
        grouped_baseline,
        request.inspector_identity().cloned(),
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
