use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::facade::{
    admit_query_basis_context, bind_query_basis_context, preflight_execution_basis,
    resolve_snapshot_basis, AdmittedQueryBasisContext, BasisAuthorityFamily, BasisResolutionMode,
    ExecutionBasisIntent, QueryBasisContextRequest, QueryContextBindingSource,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::intent_admission::certification_runtime;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{ForgeQueryReadFamily, ForgeQueryReadResult, ForgeQueryWorkspace};
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
    )
}

pub(crate) fn representative_execute_read_family_in_basis_context_row() -> RepresentativeArtifacts {
    let mut workspace = certification_workspace("lower-runtime-execute-read-family-basis");
    let family = certification_read_family(&mut workspace, "lower-runtime-basis-family");
    let snapshot_identity = workspace.snapshot_identity();
    let snapshot_evidence_identity = snapshot_identity.evidence_identity();
    let context = branch_context_for_family(&family, snapshot_evidence_identity.terminal_projection_for_reporting());
    let result = workspace
        .execute_read_family_in_basis_context(&family, &context)
        .expect("execute-read-family-in-basis-context fixture should execute");

    route_planned_read_row(
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamilyInBasisContext,
        "Basis-context read-family execution",
        &result,
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
    )
}

pub(crate) fn representative_runtime_basis_context_read_graph_row() -> RepresentativeArtifacts {
    let mut workspace = certification_workspace("lower-runtime-runtime-basis-read");
    let family = certification_read_family(&mut workspace, "lower-runtime-runtime-basis-family");
    let snapshot_identity = workspace.snapshot_identity();
    let snapshot_evidence_identity = snapshot_identity.evidence_identity();
    let context = branch_context_for_family(&family, snapshot_evidence_identity.terminal_projection_for_reporting());
    let result = workspace
        .execute_read_family_in_basis_context(&family, &context)
        .expect("runtime basis-context read fixture should execute");

    route_planned_read_row(
        ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeBasisContextReadGraph,
        "Basis-context read execution",
        &result,
    )
}

fn route_planned_read_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    capability_label: &str,
    result: &ForgeQueryReadResult,
) -> RepresentativeArtifacts {
    let read_evidence = read_result_evidence_identity(result);
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        capability_label,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "phase-six-read-execution-route-subject",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("read"), &read_evidence)
        .seal(),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request.clone(),
        &read_evidence,
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "read-execution-engine-route",
            &read_evidence,
        ),
    );
    let retained_evidence_identity =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "phase-six-read-execution-route",
            &read_evidence,
        );
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence_identity,
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam_key,
        &route_plan,
        &boundary_receipt,
        &retained_evidence_identity,
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

fn read_result_evidence_identity(result: &ForgeQueryReadResult) -> ForgeQueryEvidenceIdentity {
    let receipt = result.receipt();
    let snapshot_evidence_identity = receipt.snapshot_evidence_identity();
    let mut builder =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("engine"),
                receipt.execution_engine().as_str(),
            )
            .field_value(ForgeQueryEvidenceTag::new("query"), receipt.query_digest())
            .field_value(ForgeQueryEvidenceTag::new("basis"), receipt.basis_digest())
            .field_value(
                ForgeQueryEvidenceTag::new("result"),
                receipt.result_digest(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("snapshot"),
                &snapshot_evidence_identity,
            );
    if let Some(provenance) = receipt.execution_provenance_chain_digest() {
        builder = builder.field_value(ForgeQueryEvidenceTag::new("provenance"), provenance);
    }
    builder.seal()
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
        crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
        )
        .field_value(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture_snapshot"),
            snapshot_token,
        )
        .seal(),
        family.read_graph().schema_basis().clone(),
        SnapshotLineageClass::CurrentHead,
    );
    let basis = resolve_snapshot_basis(intent, identity, BasisResolutionMode::RuntimeDirect)
        .expect("read fixture runtime basis should resolve");
    preflight_execution_basis(family.read_graph().execution_plan().clone(), basis)
        .expect("read fixture preflight should build")
}
