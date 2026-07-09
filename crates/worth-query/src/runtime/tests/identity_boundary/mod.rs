use super::support::*;
use crate::application::{
    format_digest_folklore_pattern_in, identity_boundary_hostile_matrix_artifact,
    identity_boundary_hostile_matrix_digest, source_for_format_digest_path,
    source_for_session_admission_path, source_for_string_carried_session_identity_path,
    source_for_string_matching_path, WorthQueryFolkloreResidueStatus,
    EXACT_ZERO_FORMAT_DIGEST_PATHS, EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS,
    EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS, EXACT_ZERO_STRING_MATCHING_PATHS,
    STOP_CLASS_COVERED_CONTRACTS,
};
use crate::facade::WorthQueryApplicationFacade;
use crate::intent_admission::{
    WorthQueryAuthoritativeMutationIntentSeed, WorthQueryAuthoritativeMutationPreflight,
};
use crate::WorthQueryEvidenceIdentityScheme;

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
        AI_README.contains("WorthQueryEvidenceIdentity::compose")
            && AI_README.contains("error.stop_class()")
            && AI_README.contains("WorthQuerySessionLabel"),
        "AI_README must teach the identity boundary ordinary path explicitly"
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

#[test]
fn inventory_reports_zero_format_digest_residue_when_covered_paths_are_clean() {
    let residue_paths = crate::application::scan_format_digest_residue_paths();
    let residue_patterns = crate::application::scan_format_digest_residue_path_patterns();
    assert!(
        residue_paths.is_empty(),
        "format-digest residue remains in covered paths: {residue_paths:?}; patterns: {residue_patterns:?}"
    );
}

#[test]
fn lower_runtime_identity_shims_are_removed_from_covered_surfaces_and_feeders() {
    assert!(crate::application::scan_lower_runtime_identity_shim_paths().is_empty());
}

#[test]
fn milestone_nine_six_certification_modules_do_not_use_hash_parts() {
    use crate::application::EXCLUDED_FOLKLORE_PATHS;

    let sources = [
        include_str!("../stop_class/digests.rs"),
        include_str!("../session_label.rs"),
    ];
    for source in sources {
        assert!(
            !source.contains("hash_parts("),
            "milestone 9.6 certification module must not call hash_parts"
        );
    }
    assert!(!EXCLUDED_FOLKLORE_PATHS.is_empty());
}

#[test]
fn inventory_documents_excluded_folklore_paths() {
    use crate::application::{
        EXACT_ZERO_FORMAT_DIGEST_PATHS, EXCLUDED_FOLKLORE_DEFERRALS, EXCLUDED_FOLKLORE_PATHS,
    };

    assert_eq!(
        EXCLUDED_FOLKLORE_PATHS.len(),
        EXCLUDED_FOLKLORE_DEFERRALS.len(),
        "every excluded folklore prefix must carry a named owner milestone"
    );
    for (path, _owner) in EXCLUDED_FOLKLORE_DEFERRALS {
        assert!(
            EXCLUDED_FOLKLORE_PATHS.contains(path),
            "deferral entry must remain listed in EXCLUDED_FOLKLORE_PATHS: {path}"
        );
    }

    assert!(!EXCLUDED_FOLKLORE_PATHS.contains(&"subscription/"));
    assert!(
        !EXCLUDED_FOLKLORE_PATHS.contains(&"projection_consumption/"),
        "projection_consumption is same-class 9.6 identity-boundary scope"
    );
    assert!(
        !EXCLUDED_FOLKLORE_PATHS.contains(&"workflow/"),
        "workflow is same-class 9.6 identity-boundary scope"
    );
    assert!(
        !EXCLUDED_FOLKLORE_PATHS.contains(&"domain_capabilities/"),
        "domain_capabilities is same-class 9.6 identity-boundary scope"
    );
    assert!(EXCLUDED_FOLKLORE_PATHS.contains(&"runtime/intent/declaration.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"subscription/input.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"subscription/diagnostic/trace.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"subscription/support/profile.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"projection_consumption/receipt.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"workflow/foundation.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"workflow/lowering/writeback.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"domain_capabilities/identity/mod.rs"));
    assert!(!EXCLUDED_FOLKLORE_PATHS.contains(&"runtime/intent/receipt.rs"));
    assert!(!EXCLUDED_FOLKLORE_PATHS.contains(&"runtime/intent/receipt_identity.rs"));
    assert!(!EXCLUDED_FOLKLORE_PATHS.contains(&"runtime/intent/effect_triggered.rs"));
    assert!(!EXCLUDED_FOLKLORE_PATHS.contains(&"runtime/intent/preview_receipt_identity.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"runtime/intent/receipt.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"runtime/intent/receipt_identity.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"runtime/intent/effect_triggered.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS.contains(&"runtime/intent/preview_receipt_identity.rs"));
    assert!(EXACT_ZERO_FORMAT_DIGEST_PATHS
        .contains(&"runtime/inspection/preview/intent_receipt_identity.rs"));
}

#[test]
fn unified_inspection_request_labels_remain_typed_artifacts() {
    let seed_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/intent_admission/eligibility/seeds/generic_inspection.rs"
    ));
    let receipt_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/runtime/surface/unified_inspection_receipt.rs"
    ));

    assert!(
        seed_source.contains("WorthQueryGenericInspectionRequestLabel")
            && !seed_source.contains("request_label: String"),
        "generic inspection seeds must carry typed request labels instead of raw strings"
    );
    assert!(
        receipt_source.contains("target_label: WorthQueryGenericInspectionRequestLabel")
            && !receipt_source.contains("target_label: String"),
        "unified inspection receipts must retain the typed request-label artifact"
    );
    assert!(
        seed_source.contains("outcome.session_label().identity_digest()")
            && !seed_source.contains("WorthQueryEvidenceTag::new(\"label_identity\"),\n                    outcome.label(),"),
        "generic preview-outcome inspection seeds must use typed label identity, not display text"
    );
    assert!(
        !seed_source.contains("hash_parts(")
            && !seed_source.contains("receipt.snapshot_token()")
            && seed_source.contains("WorthQueryEvidenceScope::GenericInspectionIntentSeed"),
        "generic inspection seeds must compose typed evidence identity rather than hash receipt token text"
    );
}

#[test]
fn authoritative_mutation_seed_identity_resists_delimiter_pressure() {
    let left = WorthQueryAuthoritativeMutationIntentSeed::new(
        test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("entity|a:1"),
            "profile.name",
            "left",
        ),
        WorthQueryAuthoritativeMutationPreflight::Admitted {
            verified_existing_truth_assertion: None,
        },
    );
    let right = WorthQueryAuthoritativeMutationIntentSeed::new(
        test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("entity"),
            "a:1|profile.name",
            "left",
        ),
        WorthQueryAuthoritativeMutationPreflight::Admitted {
            verified_existing_truth_assertion: None,
        },
    );

    assert!(left
        .command_input_digest()
        .starts_with("worth.query.evidence-identity.v1:"));
    assert_ne!(
        left.command_input_digest(),
        right.command_input_digest(),
        "mutation seed identity must compose typed entity evidence rather than flattening delimiter-shaped labels"
    );
}

#[test]
fn authoritative_mutation_batch_seed_composes_component_evidence_identities() {
    let left = test_update_string_aspect_command(
        crate::memory_workspace::admit_authored_entity_label("batch:left|1"),
        "profile.name",
        "left",
    );
    let right = test_update_string_aspect_command(
        crate::memory_workspace::admit_authored_entity_label("batch:right:1"),
        "profile.name",
        "right",
    );
    let batch = crate::intent_admission::WorthQueryAuthoritativeMutationBatchIntentSeed::new(
        vec![left.clone(), right.clone()],
        crate::runtime::WorthQueryGraphCompositionBreadth::empty(),
        crate::runtime::WorthQueryGraphCompositionProgram::empty(),
    );

    assert!(batch
        .batch_input_digest()
        .starts_with("worth.query.evidence-identity.v1:"));
}

#[test]
fn relational_mutation_execution_uses_typed_authority_binding() {
    let execution_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/effect_lifecycle/execution_relational_scalar.rs"
    ));
    let lowering_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/workflow/lowering/mutation.rs"
    ));

    assert!(
        execution_source.contains("runtime_target_branch()")
            && execution_source.contains("runtime_snapshot_identity()"),
        "relational mutation execution must consume typed target branch and snapshot identity"
    );
    assert!(
        !execution_source.contains("parse_target_branch_binding")
            && !execution_source.contains("rsplit_once")
            && !execution_source.contains("strip_prefix(\"relational-branch:\")"),
        "relational mutation execution must not recover authority target from binding digest text"
    );
    assert!(
        lowering_source.contains("runtime_target_branch: Option<BranchId>")
            && lowering_source
                .contains("runtime_snapshot_identity: Option<WorthQuerySnapshotIdentity>"),
        "mutation lowering must retain typed authority handles from workflow binding"
    );
}

#[test]
fn continuation_readmission_witnesses_keep_typed_basis_identities() {
    let witness_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/continuation_pipeline/readmission.rs"
    ));
    let execution_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/continuation_pipeline/execution/readmission.rs"
    ));
    let observation_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/application/domain_handle/operating_context.rs"
    ));

    assert!(
        witness_source.contains("basis_identity: WorthQueryEvidenceIdentity")
            && witness_source.contains(
                "expected_lower_runtime_binding_identity: Option<WorthQueryEvidenceIdentity>"
            )
            && witness_source.contains("source_basis_identity: Option<WorthQueryEvidenceIdentity>"),
        "prepared continuation basis witness must retain typed evidence identities"
    );
    assert!(
        observation_source.contains("basis_identity: WorthQueryEvidenceIdentity")
            && observation_source
                .contains("lower_runtime_binding_identity: Option<WorthQueryEvidenceIdentity>"),
        "continuation readmission observation must retain typed evidence identities"
    );
    assert!(
        execution_source.contains("bridge_commit_evidence_identity")
            && execution_source.contains("bridge_snapshot_evidence_identity")
            && !execution_source.contains("request.commit_identity().to_string()")
            && !execution_source.contains("bridge_commit_evidence_digest")
            && !execution_source.contains("bridge_snapshot_evidence_digest"),
        "continuation readmission must compose typed bridge commit/snapshot evidence identities"
    );
}

#[test]
fn bridge_mutation_lowering_keeps_resolved_targets_typed() {
    let bridge_lowering_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/runtime/bridge_mutation_lowering.rs"
    ));

    assert!(
        bridge_lowering_source.contains("bridge_resolved_target_identity")
            && bridge_lowering_source.contains("identity.relational_record_parts()")
            && bridge_lowering_source
                .contains("BridgeNamingResolvedTargetIdentity::from_relational_record")
            && bridge_lowering_source.contains("BridgeContinuityResolvedTargetIdentity::new"),
        "bridge mutation lowering must lower typed relational record handles through bridge-native identity constructors"
    );
    assert!(
        !bridge_lowering_source
            .contains("identity.terminal_projection_for_reporting().to_string()"),
        "bridge mutation lowering must not smuggle Query evidence strings into bridge-native target identity slots"
    );
}

#[test]
fn inventory_derived_residue_status_matches_support_report() {
    let report = WorthQueryApplicationFacade::runtime_backed_default().support_report();
    assert!(matches!(
        report.identity_boundary_closure().residue_status(),
        WorthQueryFolkloreResidueStatus::ZeroFolkloreResidue
    ));
}
