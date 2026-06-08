use super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::facade::runtime::{
    admit_runtime_intent_request, certify_intent_admission,
    forge_query_intent_admission_certification_output_manifest,
    forge_query_intent_admission_closeout_extension_outputs,
    forge_query_intent_admission_compile_fail_targets,
    forge_query_intent_admission_coverage_inventory, forge_query_intent_admission_family_inventory,
    forge_query_intent_admission_golden_transcripts, forge_query_intent_admission_mutation_audit,
    forge_query_intent_admission_required_certification_outputs,
    forge_query_intent_admission_support_matrix, ForgeQueryAdmittedIntentExecutionHandoff,
    ForgeQueryAuthoritativeIntentExecutionHandoff, ForgeQueryAuthorityLane,
    ForgeQueryEffectTriggeredIntentExecutionHandoff,
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionCoverageStatus,
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionDecisionClass, ForgeQueryIntentAdmissionEligibilityAuthority,
    ForgeQueryIntentAdmissionExecutionBoundary, ForgeQueryIntentAdmissionExecutionHandoffInventory,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPlanKind, ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionPreDecisionPosture,
    ForgeQueryIntentAdmissionProjectionSourceEligibility, ForgeQueryIntentAdmissionResultArtifact,
    ForgeQueryIntentAdmissionRoutingSupportEligibility, ForgeQueryIntentAdmissionSlopeLane,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportDetail,
    ForgeQueryIntentAdmissionSupportEligibility, ForgeQueryIntentAdmissionSupportPosture,
    ForgeQueryIntentAdmissionSurfaceDescriptor, ForgeQueryIntentAdmissionWidthRunScale,
    ForgeQueryIntentConsumerOutcomeClass, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentDecisionTraceEnvelopeKind, ForgeQueryIntentDecisionTraceEvidence,
    ForgeQueryIntentDecisionTraceEvidenceOwner, ForgeQueryIntentDecisionTraceStage,
    ForgeQueryIntentDeclaration, ForgeQueryIntentNonAdmittedStop, ForgeQueryIntentSourceLane,
    ForgeQueryWorkspace,
};
use crate::facade::{
    admit_query_basis_context, bind_query_basis_context, preflight_execution_basis,
    resolve_snapshot_basis, AdmittedQueryBasisContext, BasisAuthorityFamily, BasisResolutionMode,
    ExecutionBasisIntent, QueryBasisContextRequest, QueryContextBindingSource,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView};
use std::cell::Cell;
use std::rc::Rc;

mod certification;
mod dx;
mod execution;
mod inventory;
mod inventory_mutation;
mod phases;

pub(in crate::runtime::tests) fn intent_runtime_with_authority<
    T: ForgeQueryIntentAuthorityAdapter + 'static,
>(
    authority: T,
) -> ForgeQueryRuntime {
    bridge_runtime_with_support_and_intent_authority(intent_support_profile(), authority)
}

fn trace_stages(
    envelope: &ForgeQueryIntentDecisionTraceEnvelope,
) -> Vec<ForgeQueryIntentDecisionTraceStage> {
    envelope.rows().iter().map(|row| row.stage()).collect()
}

fn read_runtime() -> ForgeQueryRuntime {
    bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ))
}

fn manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-composition",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 1)],
    )
}

fn identity_read_family(
    workspace: &mut ForgeQueryWorkspace,
    family_name: &str,
) -> crate::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.local_detail(
                "user",
                manager_schema(),
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
        })
        .expect("read family should define")
}

fn current_context_for_family(
    family: &crate::runtime::ForgeQueryReadFamily,
    snapshot_token: &str,
) -> AdmittedQueryBasisContext {
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
        .expect("runtime basis should resolve");
    let preflight = preflight_execution_basis(family.read_graph().execution_plan().clone(), basis)
        .expect("family preflight should build");
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .expect("current basis context should bind");
    admit_query_basis_context(binding).expect("current basis context should admit")
}

fn profile_read_family(
    workspace: &mut ForgeQueryWorkspace,
    family_name: &str,
) -> crate::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.local_detail(
                "user",
                manager_schema(),
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .project(
                            AspectFieldSelector::new("profile", "display_name")
                                .expect("profile projection should build"),
                        )
                },
                |shape| {
                    shape
                        .field(
                            AuthoredResultShapeField::new("identity", "id", "id")
                                .expect("identity result-shape field should build"),
                        )
                        .field(
                            AuthoredResultShapeField::new(
                                "profile",
                                "display_name",
                                "display_name",
                            )
                            .expect("profile result-shape field should build"),
                        )
                },
            )
        })
        .expect("profile read family should define")
}
