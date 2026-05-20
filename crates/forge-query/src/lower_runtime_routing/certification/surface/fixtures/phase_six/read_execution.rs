use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::facade::{
    admit_query_basis_context, bind_query_basis_context, preflight_execution_basis,
    resolve_snapshot_basis, AdmittedQueryBasisContext, BasisAuthorityFamily, BasisResolutionMode,
    ExecutionBasisIntent, QueryBasisContextRequest, QueryContextBindingSource,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::identity::hash_parts;
use crate::intent_admission::certification_runtime;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    ForgeQueryReadExecutionEngine, ForgeQueryReadFamily, ForgeQueryReadResult, ForgeQueryWorkspace,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use std::cell::Cell;

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};

pub(crate) fn representative_compose_read_row() -> RepresentativeArtifacts {
    let mut workspace = certification_workspace("lower-runtime-compose-read");
    let result = workspace
        .compose_read(read_declaration())
        .expect("compose-read fixture should execute");

    route_planned_read_row(
        ForgeQueryLowerRuntimeSeamKey::ComposeRead,
        "Composed current read",
        &result,
        &[
            "compose_read_subject_v1".to_string(),
            format!("query:{}", result.receipt().query_digest()),
            format!("graph:{}", result.receipt().read_graph_digest()),
        ],
    )
}

pub(crate) fn representative_compose_read_with_invariant_pack_row() -> RepresentativeArtifacts {
    let mut workspace = certification_workspace("lower-runtime-compose-read-invariant");
    let invoked = Cell::new(false);
    let result = workspace
        .compose_read_with_invariant_pack(read_declaration(), |_context| {
            invoked.set(true);
            Ok(())
        })
        .expect("compose-read-with-invariant-pack fixture should execute");
    assert!(
        invoked.get(),
        "compose-read-with-invariant-pack fixture must prove invariant admission executes"
    );

    route_planned_read_row(
        ForgeQueryLowerRuntimeSeamKey::ComposeReadWithInvariantPack,
        "Composed current read with invariant pack",
        &result,
        &[
            "compose_read_with_invariant_pack_subject_v1".to_string(),
            format!("query:{}", result.receipt().query_digest()),
            format!("graph:{}", result.receipt().read_graph_digest()),
        ],
    )
}

pub(crate) fn representative_execute_read_family_row() -> RepresentativeArtifacts {
    let mut workspace = certification_workspace("lower-runtime-execute-read-family");
    let family = certification_read_family(&mut workspace, "lower-runtime-read-family");
    let result = workspace
        .execute_read_family(&family)
        .expect("execute-read-family fixture should execute");

    route_planned_read_row(
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamily,
        "Defined read-family execution",
        &result,
        &[
            "execute_read_family_subject_v1".to_string(),
            format!("family:{}", family.family_name()),
            format!("query:{}", result.receipt().query_digest()),
        ],
    )
}

pub(crate) fn representative_execute_read_family_in_basis_context_row() -> RepresentativeArtifacts {
    let mut workspace = certification_workspace("lower-runtime-execute-read-family-basis");
    let family = certification_read_family(&mut workspace, "lower-runtime-basis-family");
    let context = branch_context_for_family(&family, workspace.snapshot_token().as_str());
    let result = workspace
        .execute_read_family_in_basis_context(&family, &context)
        .expect("execute-read-family-in-basis-context fixture should execute");

    route_planned_read_row(
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamilyInBasisContext,
        "Basis-context read-family execution",
        &result,
        &[
            "execute_read_family_in_basis_context_subject_v1".to_string(),
            format!("family:{}", family.family_name()),
            format!("basis:{}", context.basis_digest()),
        ],
    )
}

pub(crate) fn representative_runtime_current_read_graph_row() -> RepresentativeArtifacts {
    let mut workspace = certification_workspace("lower-runtime-runtime-current-read");
    let family = certification_read_family(&mut workspace, "lower-runtime-current-family");
    let result = workspace
        .execute_read_family(&family)
        .expect("runtime current-read fixture should execute");

    route_planned_read_row(
        ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeCurrentReadGraph,
        "Current read execution",
        &result,
        &[
            "execute_runtime_current_read_graph_subject_v1".to_string(),
            format!("family:{}", family.family_name()),
            format!("snapshot:{}", result.receipt().snapshot_token()),
        ],
    )
}

pub(crate) fn representative_runtime_basis_context_read_graph_row() -> RepresentativeArtifacts {
    let mut workspace = certification_workspace("lower-runtime-runtime-basis-read");
    let family = certification_read_family(&mut workspace, "lower-runtime-runtime-basis-family");
    let context = branch_context_for_family(&family, workspace.snapshot_token().as_str());
    let result = workspace
        .execute_read_family_in_basis_context(&family, &context)
        .expect("runtime basis-context read fixture should execute");

    route_planned_read_row(
        ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeBasisContextReadGraph,
        "Basis-context read execution",
        &result,
        &[
            "execute_runtime_basis_context_read_graph_subject_v1".to_string(),
            format!("family:{}", family.family_name()),
            format!("basis:{}", context.basis_digest()),
        ],
    )
}

fn route_planned_read_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    capability_label: &str,
    result: &ForgeQueryReadResult,
    subject_parts: &[String],
) -> RepresentativeArtifacts {
    let retained = result
        .receipt()
        .execution_provenance_chain_digest()
        .unwrap_or_else(|| result.receipt().result_digest())
        .to_string();
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        capability_label,
        hash_parts(subject_parts),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        hash_parts(&[
            result.receipt().query_digest().to_string(),
            result.receipt().basis_digest().to_string(),
            retained.clone(),
        ]),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        execution_engine_label(result.receipt().execution_engine()),
    );
    let boundary_receipt =
        ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(&route_plan, &retained);
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam_key,
        &route_plan,
        &boundary_receipt,
        &retained,
    );
    RepresentativeArtifacts {
        seam_key,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

fn execution_engine_label(engine: &ForgeQueryReadExecutionEngine) -> &'static str {
    match engine {
        ForgeQueryReadExecutionEngine::QueryRuntimeCurrent => "query-runtime-current",
        ForgeQueryReadExecutionEngine::QueryRuntimeBranch => "query-runtime-branch",
        ForgeQueryReadExecutionEngine::QueryRuntimeHistorical => "query-runtime-historical",
        ForgeQueryReadExecutionEngine::QueryRuntimePreviewDerived => {
            "query-runtime-preview-derived"
        }
    }
}

fn certification_workspace(label: &str) -> ForgeQueryWorkspace {
    certification_runtime()
        .workspace(label)
        .expect("lower-runtime read fixture workspace should build")
}

fn certification_read_family(
    workspace: &mut ForgeQueryWorkspace,
    family_name: &str,
) -> ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, read_declaration())
        .expect("lower-runtime read fixture family should define")
}

fn read_declaration() -> impl FnOnce(
    crate::runtime::ForgeQueryReadBuilder,
) -> Result<
    crate::runtime::ForgeQueryReadGraph,
    crate::runtime::ForgeQueryReadDenial,
> {
    |read| {
        read.local_detail(
            "user",
            QuerySchemaView::new(
                "lower-runtime-read-fixture",
                [
                    SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                    SchemaFieldView::new("title", "value", SchemaFieldKind::String),
                ],
                [],
            ),
            |query| {
                query.project(
                    AspectFieldSelector::new("identity", "id")
                        .expect("identity projection should build"),
                )
            },
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("identity", "id", "id")
                        .expect("identity result-shape field should build"),
                )
            },
        )
    }
}

fn branch_context_for_family(
    family: &ForgeQueryReadFamily,
    snapshot_token: &str,
) -> AdmittedQueryBasisContext {
    let preflight = runtime_preflight_for_family(family, snapshot_token);
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::branch_head(snapshot_token),
        QueryContextBindingSource::RuntimeBranch(&preflight),
    )
    .expect("branch read fixture context should bind");
    admit_query_basis_context(binding).expect("branch read fixture context should admit")
}

fn runtime_preflight_for_family(
    family: &ForgeQueryReadFamily,
    snapshot_token: &str,
) -> crate::facade::ExecutionPreflightBundle {
    let intent = ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    );
    let identity = ResolvedSnapshotIdentity::new(
        BasisAuthorityFamily::Runtime,
        None,
        snapshot_token.to_string(),
        family.read_graph().schema_basis().clone(),
        SnapshotLineageClass::CurrentHead,
    );
    let basis = resolve_snapshot_basis(intent, identity, BasisResolutionMode::RuntimeDirect)
        .expect("read fixture runtime basis should resolve");
    preflight_execution_basis(family.read_graph().execution_plan().clone(), basis)
        .expect("read fixture preflight should build")
}
