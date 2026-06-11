use super::support::*;
use crate::facade::ForgeQueryApplicationFacade;
use crate::ForgeQueryEvidenceIdentityScheme;

const AI_README: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/AI_README.md"));
const SUPPORT_REPORT: &str = include_str!("../../../application/support/report.rs");
const SUPPORT_MATRIX: &str = include_str!("../../support_matrix.rs");
const STATE_SNAPSHOT: &str = include_str!("../../state_snapshot.rs");
const PUBLIC_API_TRANSCRIPT: &str = include_str!("../../public_api_transcript.rs");
const RUNTIME_SESSIONS: &str = include_str!("../../runtime_sessions.rs");
const WORKSPACE: &str = include_str!("../../workspace.rs");
const STOP_CLASS_CONSUMER_MATCHING: &str = include_str!("../stop_class/consumer_matching.rs");
const STOP_CLASS_CONSUMER_RUNTIME_PATHS: &str =
    include_str!("../stop_class/consumer_runtime_paths.rs");

#[test]
fn identity_boundary_hostile_closure_matrix_holds_under_combined_drift_pressure() {
    let report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();

    assert_eq!(
        report.identity_boundary_closure().residue_status().as_str(),
        "zero-folklore-residue"
    );
    assert_eq!(
        report
            .identity_boundary_closure()
            .evidence_identity()
            .scheme(),
        ForgeQueryEvidenceIdentityScheme::V1
    );
    assert!(
        AI_README.contains("ForgeQueryEvidenceIdentity::compose")
            && AI_README.contains("error.stop_class()")
            && AI_README.contains("ForgeQuerySessionLabel"),
        "AI_README must teach the identity boundary ordinary path explicitly"
    );

    assert_combined_drift_pressure_holds();
    assert_no_format_string_digest_folklore();
    assert_no_string_matched_control_flow();
    assert_no_raw_string_session_admission();
}

fn assert_combined_drift_pressure_holds() {
    assert_evidence_identity_resists_joined_string_folklore();
    assert_stop_class_remains_typed_under_message_rewording();
    assert_session_label_identity_holds_under_collision_pressure();
}

fn assert_evidence_identity_resists_joined_string_folklore() {
    let authority = crate::runtime::ForgeQueryRuntimeEvidenceAuthority::new();
    let left = crate::runtime::ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ["alpha", "beta|gamma"],
    );
    let right = crate::runtime::ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ["basis|alpha", "beta|gamma"],
    );

    assert_ne!(
        left.admission_digest(),
        right.admission_digest(),
        "canonical evidence identity must resist joined-string delimiter collisions"
    );
}

fn assert_stop_class_remains_typed_under_message_rewording() {
    let first_error = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::supported_with_teaching_posture_and_reason(
                ForgeQueryRuntimeFacadeFamily::Temporal,
                ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [ForgeQueryAuthorityLane::TemporalExecutionState],
                [],
                ["runtime-backed-temporal-basis-state-inspection"],
                "first temporal wording",
            ),
        ),
    )
    .workspace("identity-boundary-reword-first")
    .expect("workspace should open")
    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Temporal)
    .expect_err("temporal admission should fail closed");
    let second_error = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::supported_with_teaching_posture_and_reason(
                ForgeQueryRuntimeFacadeFamily::Temporal,
                ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [ForgeQueryAuthorityLane::TemporalExecutionState],
                [],
                ["runtime-backed-temporal-basis-state-inspection"],
                "second temporal wording",
            ),
        ),
    )
    .workspace("identity-boundary-reword-second")
    .expect("workspace should open")
    .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Temporal)
    .expect_err("temporal admission should fail closed");

    for error in [&first_error, &second_error] {
        match error.stop_class() {
            ForgeQueryStopClass::FamilyAdmissionDenied {
                family,
                status,
                teaching_posture,
                ..
            } => {
                assert_eq!(family, ForgeQueryRuntimeFacadeFamily::Temporal);
                assert_eq!(status, ForgeQueryRuntimeFamilySupportStatus::Supported);
                assert_eq!(
                    teaching_posture,
                    Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly)
                );
            }
            other => panic!("expected typed family-admission stop class, got {other:?}"),
        }
    }
    assert_ne!(
        first_error.to_string(),
        second_error.to_string(),
        "message wording must remain presentation while stop-class meaning stays stable"
    );
}

fn assert_session_label_identity_holds_under_collision_pressure() {
    let mut runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("session-entry runtime should build");
    let preview_label =
        ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let render_collision =
        ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");

    runtime
        .preview(preview_label.clone())
        .expect("first preview label should admit")
        .discard();
    runtime
        .preview(render_collision.clone())
        .expect("display-colliding but identity-distinct preview label should admit")
        .discard();
    runtime
        .branch(preview_label.clone())
        .expect("branch family should admit same identity independently");

    let error = match runtime.branch(preview_label.clone()) {
        Ok(_) => panic!("same-family branch replay should collide"),
        Err(error) => error,
    };
    match error.stop_class() {
        ForgeQueryStopClass::SessionLabelCollision {
            authority_lane,
            label,
        } => {
            assert_eq!(authority_lane, ForgeQueryAuthorityLane::BranchLocalTruth);
            assert_eq!(label, &preview_label);
        }
        other => panic!("expected typed session-label collision, got {other:?}"),
    }

    assert_eq!(preview_label.display(), render_collision.display());
    assert_ne!(
        preview_label.identity_digest(),
        render_collision.identity_digest()
    );
}

fn assert_no_format_string_digest_folklore() {
    for (path, source) in [
        ("application/support/report.rs", SUPPORT_REPORT),
        ("runtime/support_matrix.rs", SUPPORT_MATRIX),
        ("runtime/state_snapshot.rs", STATE_SNAPSHOT),
        ("runtime/public_api_transcript.rs", PUBLIC_API_TRANSCRIPT),
    ] {
        assert!(
            !source.contains("hash_parts("),
            "covered digest surface still uses joined-string digest folklore: {path}"
        );
        assert!(
            !source.contains("format!(\"{:?}\""),
            "covered digest surface still depends on Debug formatting: {path}"
        );
        assert!(
            !source.contains(".join(\"|\")"),
            "covered digest surface still joins evidence with pipe delimiters: {path}"
        );
    }
}

fn assert_no_string_matched_control_flow() {
    for (path, source) in [
        (
            "runtime/tests/stop_class/consumer_matching.rs",
            STOP_CLASS_CONSUMER_MATCHING,
        ),
        (
            "runtime/tests/stop_class/consumer_runtime_paths.rs",
            STOP_CLASS_CONSUMER_RUNTIME_PATHS,
        ),
    ] {
        assert!(
            !source.contains("to_string().contains(")
                && !source.contains("message.contains")
                && !source.contains("error_message.contains"),
            "typed stop-class consumer lane still depends on runtime message matching: {path}"
        );
    }
}

fn assert_no_raw_string_session_admission() {
    for (path, source) in [
        ("runtime/runtime_sessions.rs", RUNTIME_SESSIONS),
        ("runtime/workspace.rs", WORKSPACE),
    ] {
        assert!(
            !source.contains("label: impl Into<String>"),
            "raw-string session admission survived on ordinary path: {path}"
        );
        assert!(
            source.contains("label: ForgeQuerySessionLabel"),
            "ordinary session entrypoint must require typed session labels: {path}"
        );
    }
}
