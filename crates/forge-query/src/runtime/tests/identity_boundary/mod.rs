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
const STOP_CLASS_CONSUMER_ROUTING: &str = include_str!("../stop_class/consumer_support/routing.rs");

const EXPECTED_COVERED_STOP_CLASS_CONTRACTS: &[&str] = &[
    "typed-family-admission-denial",
    "typed-preview-promotion-stop",
    "typed-session-label-collision-stop",
];

const EXPECTED_ORDINARY_SESSION_ENTRYPOINTS: &[&str] = &[
    "runtime.preview",
    "runtime.branch",
    "runtime.try_preview",
    "runtime.try_branch",
    "workspace.preview",
    "workspace.branch",
];

const EXPECTED_ZERO_FORMAT_DIGEST_PATHS: &[&str] = &[
    "application/support/report.rs",
    "runtime/support_matrix.rs",
    "runtime/state_snapshot.rs",
    "runtime/public_api_transcript.rs",
];

const EXPECTED_ZERO_STRING_MATCHING_PATHS: &[&str] =
    &["runtime/tests/stop_class/consumer_support/routing.rs"];

const EXPECTED_ZERO_RAW_SESSION_ADMISSION_PATHS: &[&str] =
    &["runtime/runtime_sessions.rs", "runtime/workspace.rs"];

#[test]
fn identity_boundary_hostile_closure_matrix_holds_under_combined_drift_pressure() {
    let report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = report.identity_boundary_closure();

    assert_eq!(closure.residue_status().as_str(), "zero-folklore-residue");
    assert_eq!(
        closure.evidence_identity().scheme(),
        ForgeQueryEvidenceIdentityScheme::V1
    );
    assert!(
        AI_README.contains("ForgeQueryEvidenceIdentity::compose")
            && AI_README.contains("error.stop_class()")
            && AI_README.contains("ForgeQuerySessionLabel"),
        "AI_README must teach the identity boundary ordinary path explicitly"
    );

    assert_combined_drift_pressure_holds(closure);
    assert_no_format_string_digest_folklore(closure);
    assert_no_string_matched_control_flow(closure);
    assert_no_raw_string_session_admission(closure);
}

fn assert_combined_drift_pressure_holds(
    closure: &crate::application::ForgeQueryIdentityBoundaryClosure,
) {
    assert_eq!(
        closure.stop_class().covered_contracts(),
        EXPECTED_COVERED_STOP_CLASS_CONTRACTS
    );
    assert_eq!(
        closure.session_label().ordinary_entrypoints(),
        EXPECTED_ORDINARY_SESSION_ENTRYPOINTS
    );
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
    let mut runtime = bridge_runtime_with_support_and_intent_authority(
        intent_support_profile(),
        TestIntentAuthority,
    );
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

fn assert_no_format_string_digest_folklore(
    closure: &crate::application::ForgeQueryIdentityBoundaryClosure,
) {
    assert_eq!(
        closure.exact_zero_format_digest_paths(),
        EXPECTED_ZERO_FORMAT_DIGEST_PATHS
    );
    for path in EXPECTED_ZERO_FORMAT_DIGEST_PATHS {
        let source = source_for_format_digest_path(path);
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

fn assert_no_string_matched_control_flow(
    closure: &crate::application::ForgeQueryIdentityBoundaryClosure,
) {
    assert_eq!(
        closure.exact_zero_string_matching_paths(),
        EXPECTED_ZERO_STRING_MATCHING_PATHS
    );
    for path in EXPECTED_ZERO_STRING_MATCHING_PATHS {
        let source = source_for_string_matching_path(path);
        assert!(
            !source.contains("to_string().contains(")
                && !source.contains("message.contains")
                && !source.contains("error_message.contains"),
            "typed stop-class consumer lane still depends on runtime message matching: {path}"
        );
    }
}

fn assert_no_raw_string_session_admission(
    closure: &crate::application::ForgeQueryIdentityBoundaryClosure,
) {
    assert_eq!(
        closure.exact_zero_raw_session_admission_paths(),
        EXPECTED_ZERO_RAW_SESSION_ADMISSION_PATHS
    );
    for path in EXPECTED_ZERO_RAW_SESSION_ADMISSION_PATHS {
        let source = source_for_session_admission_path(path);
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

fn source_for_format_digest_path(path: &str) -> &'static str {
    match path {
        "application/support/report.rs" => SUPPORT_REPORT,
        "runtime/support_matrix.rs" => SUPPORT_MATRIX,
        "runtime/state_snapshot.rs" => STATE_SNAPSHOT,
        "runtime/public_api_transcript.rs" => PUBLIC_API_TRANSCRIPT,
        other => panic!("unexpected format-digest audit path: {other}"),
    }
}

fn source_for_string_matching_path(path: &str) -> &'static str {
    match path {
        "runtime/tests/stop_class/consumer_support/routing.rs" => STOP_CLASS_CONSUMER_ROUTING,
        other => panic!("unexpected string-matching audit path: {other}"),
    }
}

fn source_for_session_admission_path(path: &str) -> &'static str {
    match path {
        "runtime/runtime_sessions.rs" => RUNTIME_SESSIONS,
        "runtime/workspace.rs" => WORKSPACE,
        other => panic!("unexpected session-admission audit path: {other}"),
    }
}
