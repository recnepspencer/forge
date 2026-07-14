use super::super::support::*;
use crate::application::WorthQueryFolkloreResidueStatus;
use crate::facade::foundation::WorthQueryApplicationFacade;
use crate::intent_admission::{
    WorthQueryAuthoritativeMutationIntentSeed, WorthQueryAuthoritativeMutationPreflight,
};

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
