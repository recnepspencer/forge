use super::support::*;
use crate::application::{
    format_digest_folklore_pattern_in, identity_boundary_hostile_matrix_artifact,
    identity_boundary_hostile_matrix_digest, source_for_format_digest_path,
    source_for_session_admission_path, source_for_string_carried_session_identity_path,
    source_for_string_matching_path, EXACT_ZERO_FORMAT_DIGEST_PATHS,
    EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS, EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS,
    EXACT_ZERO_STRING_MATCHING_PATHS, STOP_CLASS_COVERED_CONTRACTS,
};
use crate::facade::foundation::WorthQueryApplicationFacade;
use crate::WorthQueryEvidenceIdentityScheme;

mod certification_surfaces;

const AI_README: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/AI_README.md"));

#[test]
fn identity_boundary_hostile_closure_matrix_holds_under_combined_drift_pressure() {
    let report = WorthQueryApplicationFacade::runtime_backed_default().support_report();
    let closure = report.identity_boundary_closure();
    let hostile_matrix = identity_boundary_hostile_matrix_artifact();

    assert!(closure.residue_status().is_zero());
    assert_eq!(closure.residue_status().as_str(), "zero-folklore-residue");
    assert_eq!(
        closure.evidence_identity().scheme(),
        WorthQueryEvidenceIdentityScheme::V1
    );
    assert_eq!(
        closure.hostile_matrix_digest(),
        identity_boundary_hostile_matrix_digest()
    );
    assert!(closure.hostile_matrix_certified());
    assert!(hostile_matrix.certified());
    assert_eq!(
        hostile_matrix.suite_name(),
        crate::application::MILESTONE_NINE_SIX_SUITE_NAME
    );
    assert_eq!(
        hostile_matrix
            .canonical_rows()
            .iter()
            .map(|row| row.name())
            .collect::<Vec<_>>(),
        crate::application::MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES
    );
    assert_eq!(
        hostile_matrix
            .rejection_rows()
            .iter()
            .map(|row| row.name())
            .collect::<Vec<_>>(),
        crate::application::MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES
    );
    assert!(
        AI_README.contains("ordinary runtime-backed identity boundary")
            && AI_README.contains("typed stops")
            && AI_README.contains("./foundations/workspace-overview.md")
            && AI_README.contains("./foundations/downstream-runtime-integration.md")
            && AI_README.contains("WorthQuerySessionLabel"),
        "AI_README must route identity-boundary discovery to the owning feature docs"
    );

    assert_combined_drift_pressure_holds(closure);
    assert_no_format_string_digest_folklore(closure);
    assert_no_string_matched_control_flow(closure);
    assert_no_raw_string_session_admission(closure);
    assert_no_string_carried_session_identity(closure);
}

fn assert_combined_drift_pressure_holds(
    closure: &crate::application::WorthQueryIdentityBoundaryClosure,
) {
    assert_eq!(
        closure.stop_class().covered_contracts(),
        STOP_CLASS_COVERED_CONTRACTS
    );
    assert_eq!(
        closure.session_label().ordinary_entrypoints(),
        crate::application::SESSION_LABEL_ORDINARY_ENTRYPOINTS
    );
    assert_evidence_identity_resists_joined_string_folklore();
    assert_stop_class_remains_typed_under_message_rewording();
    assert_session_label_identity_holds_under_collision_pressure();
}

fn assert_evidence_identity_resists_joined_string_folklore() {
    let authority = crate::runtime::WorthQueryRuntimeEvidenceAuthority::new();
    let left = crate::runtime::WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "alpha",
            "beta|gamma",
        ]),
    );
    let right = crate::runtime::WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "basis|alpha",
            "beta|gamma",
        ]),
    );

    assert_ne!(
        left.admission_digest(),
        right.admission_digest(),
        "canonical evidence identity must resist joined-string delimiter collisions"
    );
}

fn assert_stop_class_remains_typed_under_message_rewording() {
    let first_error = bridge_runtime_with_support(
        WorthQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            WorthQueryRuntimeFamilySupport::supported_with_teaching_posture_and_reason(
                WorthQueryRuntimeFacadeFamily::Temporal,
                WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [WorthQueryAuthorityLane::TemporalExecutionState],
                [],
                ["runtime-backed-temporal-basis-state-inspection"],
                "first temporal wording",
            ),
        ),
    )
    .workspace("identity-boundary-reword-first")
    .expect("workspace should open")
    .admit_public_api_family(WorthQueryRuntimeFacadeFamily::Temporal)
    .expect_err("temporal admission should fail closed");
    let second_error = bridge_runtime_with_support(
        WorthQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            WorthQueryRuntimeFamilySupport::supported_with_teaching_posture_and_reason(
                WorthQueryRuntimeFacadeFamily::Temporal,
                WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly,
                [WorthQueryAuthorityLane::TemporalExecutionState],
                [],
                ["runtime-backed-temporal-basis-state-inspection"],
                "second temporal wording",
            ),
        ),
    )
    .workspace("identity-boundary-reword-second")
    .expect("workspace should open")
    .admit_public_api_family(WorthQueryRuntimeFacadeFamily::Temporal)
    .expect_err("temporal admission should fail closed");

    for error in [&first_error, &second_error] {
        match error.stop_class() {
            WorthQueryStopClass::FamilyAdmissionDenied {
                family,
                status,
                teaching_posture,
                ..
            } => {
                assert_eq!(family, WorthQueryRuntimeFacadeFamily::Temporal);
                assert_eq!(status, WorthQueryRuntimeFamilySupportStatus::Supported);
                assert_eq!(
                    teaching_posture,
                    Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly)
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
        WorthQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let render_collision =
        WorthQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");

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
        WorthQueryStopClass::SessionLabelCollision {
            authority_lane,
            label,
        } => {
            assert_eq!(authority_lane, WorthQueryAuthorityLane::BranchLocalTruth);
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
    closure: &crate::application::WorthQueryIdentityBoundaryClosure,
) {
    assert_eq!(
        closure.exact_zero_format_digest_paths(),
        EXACT_ZERO_FORMAT_DIGEST_PATHS
    );
    for path in EXACT_ZERO_FORMAT_DIGEST_PATHS {
        let source = source_for_format_digest_path(path).unwrap_or_else(|| {
            panic!("unexpected format-digest audit path: {path}");
        });
        if let Some(pattern) = format_digest_folklore_pattern_in(source) {
            panic!(
                "covered digest surface still uses joined-string digest folklore pattern {pattern}: {path}"
            );
        }
    }
}

fn assert_no_string_matched_control_flow(
    closure: &crate::application::WorthQueryIdentityBoundaryClosure,
) {
    assert_eq!(
        closure.exact_zero_string_matching_paths(),
        EXACT_ZERO_STRING_MATCHING_PATHS
    );
    for path in EXACT_ZERO_STRING_MATCHING_PATHS {
        let source = source_for_string_matching_path(path).unwrap_or_else(|| {
            panic!("unexpected string-matching audit path: {path}");
        });
        assert!(
            !source.contains("to_string().contains(")
                && !source.contains("message.contains")
                && !source.contains("error_message.contains"),
            "typed stop-class consumer lane still depends on runtime message matching: {path}"
        );
    }
}

fn assert_no_raw_string_session_admission(
    closure: &crate::application::WorthQueryIdentityBoundaryClosure,
) {
    use crate::application::normalize_source_text;

    assert_eq!(
        closure.exact_zero_raw_session_admission_paths(),
        EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS
    );
    for path in EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS {
        let source = source_for_session_admission_path(path).unwrap_or_else(|| {
            panic!("unexpected session-admission audit path: {path}");
        });
        let normalized = normalize_source_text(source);
        assert!(
            !normalized.contains("label: impl Into<String>"),
            "raw-string session admission survived on ordinary path: {path}"
        );
        assert!(
            normalized.contains("label: WorthQuerySessionLabel"),
            "ordinary session entrypoint must require typed session labels: {path}"
        );
    }
}

fn assert_no_string_carried_session_identity(
    closure: &crate::application::WorthQueryIdentityBoundaryClosure,
) {
    use crate::application::normalize_source_text;

    assert_eq!(
        closure.exact_zero_string_carried_session_identity_paths(),
        EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS
    );
    for path in EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS {
        let source = source_for_string_carried_session_identity_path(path).unwrap_or_else(|| {
            panic!("unexpected string-carried session-identity audit path: {path}");
        });
        let normalized = normalize_source_text(source);
        let carries_string_identity = normalized.contains("label: String")
            || normalized.contains("label: &str")
            || normalized.contains("self.label.to_string()")
            || normalized.contains("label.to_string(),")
            || normalized.contains("self.label.display(),")
            || normalized.contains("format!(\"preview:{label}:{sequence}\")");
        let preserves_typed_identity = normalized.contains("label: WorthQuerySessionLabel")
            || normalized.contains("label: &WorthQuerySessionLabel")
            || normalized.contains("&self.label")
            || normalized.contains("self.label.clone()");
        assert!(
            !carries_string_identity,
            "ordinary runtime/product path still stores session identity as string: {path}"
        );
        assert!(
            preserves_typed_identity,
            "ordinary runtime/product path must keep the typed session-label artifact: {path}"
        );
    }
}
