use super::{
    RuntimeApiStabilizationAdapter, RUNTIME_API_STABILIZATION_REQUIRED_CANONICAL_ROW_NAMES,
    RUNTIME_API_STABILIZATION_REQUIRED_REJECTION_ROW_NAMES,
};
use crate::harness::certification::{
    contains_row, unmet_required_assertion_classes, HostileExpectation, ParityAnchor,
    RequiredAssertionClass,
};
use crate::runtime::{ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus};

const CLOSEOUT_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/forge-query/runtime-api-public-stabilization-closeout.md"
));

#[test]
fn runtime_api_stabilization_adapter_emits_named_matrix() {
    let artifact =
        RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_artifact();

    assert_eq!(
        artifact.suite_name,
        "Runtime API Golden DX And Async-Safe Facade Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
    assert!(!artifact.closeout.closeout_digest.is_empty());
}

#[test]
fn runtime_api_stabilization_matrix_covers_required_rows() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();

    for row_name in RUNTIME_API_STABILIZATION_REQUIRED_CANONICAL_ROW_NAMES {
        assert!(contains_row(&matrix, row_name), "missing {row_name}");
    }
    for row_name in RUNTIME_API_STABILIZATION_REQUIRED_REJECTION_ROW_NAMES {
        assert!(contains_row(&matrix, row_name), "missing {row_name}");
    }
}

#[test]
fn runtime_api_stabilization_rows_have_required_outputs_and_meaningful_assertions() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();

    for row in &matrix.rows {
        for lane in [&row.control_lane, &row.hostile_lane, &row.parity_lane] {
            assert!(lane.has_required_outputs(), "row '{}'", row.row_name);
            assert!(lane.public_facade_only, "row '{}'", row.row_name);
            assert_eq!(
                lane.lower_runtime_plumbing_count, 0,
                "row '{}'",
                row.row_name
            );
            assert!(
                lane.meaningful_assertion_count >= 8,
                "golden DX rows must assert proof artifacts, not only compilation"
            );
            assert!(
                lane.unsupported_neighbor_denial_count >= 1,
                "golden DX rows must prove future-neighbor denials"
            );
            assert!(
                lane.delivery_residue_count >= 1,
                "golden DX rows must prove delivery or pending-intent residue"
            );
            assert!(
                lane.counter_snapshot.contains("denials=")
                    && lane.counter_snapshot.contains("residue="),
                "runtime API counter snapshot must include executable transcript proof counters"
            );
            assert!(
                lane.counter_snapshot.contains("preferred_names=")
                    && lane.counter_snapshot.contains("alternate_names="),
                "runtime API counter snapshot must include canonical naming contract counters"
            );
            assert!(
                lane.counter_snapshot.contains("support_deferred_rows=6")
                    && lane.counter_snapshot.contains("support_fail_closed=")
                    && lane.counter_snapshot.contains("parallel_api_forbidden="),
                "runtime API counter snapshot must include Phase 5 support-matrix gates"
            );
            assert!(!lane.public_api_naming_contract_digest.is_empty());
            assert_ne!(
                lane.public_api_naming_contract_digest, lane.public_api_surface_digest,
                "naming contract must be distinct from backend family support posture"
            );
            assert!(lane.stable_family_count >= 7);
            assert!(lane.deferred_family_count >= 6);
        }
    }
}

#[test]
fn runtime_api_stabilization_golden_transcripts_are_executable_and_distinct() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();
    let mut digests = matrix
        .rows
        .iter()
        .map(|row| {
            assert!(
                row.control_lane.executable_transcript_digest
                    != row.control_lane.golden_transcript_digest,
                "row '{}' should carry runtime-executed evidence beyond label evidence",
                row.row_name
            );
            (
                row.row_name,
                row.control_lane.executable_transcript_digest.clone(),
            )
        })
        .collect::<Vec<_>>();
    digests.sort_by(|left, right| left.1.cmp(&right.1));
    for pair in digests.windows(2) {
        assert_ne!(
            pair[0].1, pair[1].1,
            "rows '{}' and '{}' should not collapse to the same executable transcript digest",
            pair[0].0, pair[1].0
        );
    }
}

#[test]
fn runtime_api_stabilization_rows_enforce_required_assertion_classes() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();
    let mut covered = Vec::new();

    for row in &matrix.rows {
        let control = row.control_lane.semantic_signature();
        let hostile = row.hostile_lane.semantic_signature();
        let parity = row.parity_lane.semantic_signature();
        match row.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                assert_eq!(control, hostile, "row '{}'", row.row_name);
                covered.push(RequiredAssertionClass::Equality);
            }
            HostileExpectation::DistinctFromControl => {
                assert_ne!(control, hostile, "row '{}'", row.row_name);
                covered.push(RequiredAssertionClass::Inequality);
            }
        }
        match row.parity_anchor {
            ParityAnchor::Control => assert_eq!(parity, control, "row '{}'", row.row_name),
            ParityAnchor::Hostile => assert_eq!(parity, hostile, "row '{}'", row.row_name),
        }
    }

    for row in &matrix.rejection_rows {
        covered.push(RequiredAssertionClass::TypedFailure);
        assert_eq!(
            row.hostile_lane.status,
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert!(row
            .hostile_lane
            .counter_snapshot
            .contains("authority_residue=0"));
        covered.push(RequiredAssertionClass::ZeroResidue);
    }

    covered.sort();
    covered.dedup();
    let required = [
        RequiredAssertionClass::Equality,
        RequiredAssertionClass::TypedFailure,
        RequiredAssertionClass::ZeroResidue,
    ];
    let missing = unmet_required_assertion_classes(&covered, &required);
    assert!(missing.is_empty(), "missing assertion classes: {missing:?}");
}

#[test]
fn runtime_api_stabilization_deferred_gates_name_future_owners() {
    let matrix = RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_test();
    let expected = [
        (
            "temporal-basis-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::Temporal,
        ),
        (
            "async-resource-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
        ),
        (
            "mixed-cause-delivery-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
        ),
        (
            "store-backed-parity-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
        ),
        (
            "durable-restart-deferred-gate",
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
        ),
    ];

    for (row_name, family) in expected {
        let row = matrix
            .rejection_rows
            .iter()
            .find(|row| row.row_name == row_name)
            .expect("deferred gate row should exist");
        assert_eq!(row.hostile_lane.family, family);
        assert_eq!(
            row.hostile_lane.status,
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert_ne!(
            row.hostile_lane.deferred_temporal_async_gate_digest,
            row.hostile_lane.failure_digest
        );
    }
}

#[test]
fn runtime_api_stabilization_closeout_answers_required_questions() {
    let artifact =
        RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_artifact();
    let closeout = &artifact.closeout;

    assert_eq!(
        closeout.golden_transcript_count,
        RUNTIME_API_STABILIZATION_REQUIRED_CANONICAL_ROW_NAMES.len()
    );
    assert_eq!(
        closeout.hostile_rejection_count,
        RUNTIME_API_STABILIZATION_REQUIRED_REJECTION_ROW_NAMES.len()
    );
    assert_eq!(
        closeout.lower_runtime_plumbing_count, 0,
        "closeout cannot pass if ordinary DX still reaches lower runtime plumbing"
    );
    assert!(closeout
        .stable_runtime_surfaces
        .iter()
        .any(|row| row == "live"));
    assert!(closeout
        .stable_runtime_surfaces
        .iter()
        .any(|row| row == "computed"));
    assert!(closeout
        .stable_runtime_surfaces
        .iter()
        .any(|row| row == "branch-preview"));
    assert!(closeout
        .deferred_runtime_surfaces
        .iter()
        .any(|row| row == "temporal:Milestone 9.4"));
    assert!(closeout
        .deferred_runtime_surfaces
        .iter()
        .any(|row| row == "async-resource:Milestone 9.5"));
    assert!(closeout
        .deferred_runtime_surfaces
        .iter()
        .any(|row| row == "mixed-cause-delivery:Milestone 9.6"));
    assert!(closeout
        .deferred_runtime_surfaces
        .iter()
        .any(|row| row == "temporal-async-certification:Milestone 9.7"));
    assert!(closeout
        .deferred_runtime_surfaces
        .iter()
        .any(|row| row == "store-backed-execution:Milestone 10"));
    assert!(closeout
        .deferred_runtime_surfaces
        .iter()
        .any(|row| row == "durable-artifacts:Milestone 11"));
    assert!(closeout
        .unsupported_runtime_surfaces
        .iter()
        .any(|row| row == "intent"));

    for required in [
        "golden transcripts are executable through the public facade",
        "unsupported future neighbors fail typed and early",
        "ordinary DX uses no lower-runtime plumbing",
        "support metadata is synchronized with admission gates",
        "handle/state/inspection contract is extension-ready",
        "temporal/async/store/durable behavior remains explicitly deferred",
        "downstream examples are pressure tests",
    ] {
        assert!(
            closeout
                .closeout_self_check_answers
                .iter()
                .any(|answer| answer.contains(required)),
            "closeout self-check must answer `{required}`"
        );
    }
}

#[test]
fn runtime_api_stabilization_closeout_doc_uses_mutation_surface_language() {
    assert!(
        CLOSEOUT_DOC.contains("`workspace.public_mutation_surface_report()`"),
        "runtime API closeout doc must point callers at the mutation surface report"
    );
    assert!(
        !CLOSEOUT_DOC.contains("public_mutation_api_compatibility_report"),
        "runtime API closeout doc must not teach the deleted compatibility report name"
    );
}

#[test]
fn runtime_api_stabilization_closeout_carries_migration_and_verification_contract() {
    let artifact =
        RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_artifact();
    let closeout = &artifact.closeout;

    assert!(closeout
        .alternate_names
        .iter()
        .any(|row| row == "computed_definition=>computed"));
    assert!(closeout
        .alternate_names
        .iter()
        .any(|row| row == "preview_with_options=>preview"));
    assert!(closeout
        .alternate_names
        .iter()
        .all(|row| !row.contains("computed_declaration")));
    assert!(closeout
        .safe_to_build_now
        .iter()
        .any(|row| row.contains("handles as durable inspectable app surfaces")));
    assert!(closeout
        .safe_to_build_now
        .iter()
        .any(|row| row.contains("state/inspect access")));
    assert!(closeout
        .must_not_assume_yet
        .iter()
        .any(|row| row.contains("temporal basis")));
    assert!(closeout
        .must_not_assume_yet
        .iter()
        .any(|row| row.contains("store-backed parity")));
    assert!(closeout
        .must_not_assume_yet
        .iter()
        .any(|row| row.contains("domain-specific")));
    assert!(closeout
        .migration_guidance
        .iter()
        .any(|row| row.contains("ForgeQueryRuntime::workspace")));
    assert!(closeout
        .migration_guidance
        .iter()
        .any(|row| row.contains("extend existing handle/state/inspection contracts")));

    for command in [
        "cargo fmt -p forge-query",
        "cargo check -p forge-query --tests",
        "cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail",
        "cargo test -p forge-query",
        "cargo test -p forge-query runtime_api_stabilization",
        "cargo test -p forge-query runtime_public_support",
        "git diff --check",
    ] {
        assert!(
            closeout
                .required_verification_commands
                .iter()
                .any(|actual| actual == command),
            "closeout must require `{command}`"
        );
    }
}

#[test]
fn runtime_api_stabilization_closeout_document_matches_certified_contract() {
    let artifact =
        RuntimeApiStabilizationAdapter::runtime_api_golden_dx_and_async_safe_facade_artifact();
    let closeout = &artifact.closeout;

    for required in [
        "`workspace.live_view(...)`",
        "`workspace.computed(...)`",
        "`workspace.effect(...)`",
        "`workspace.preview(...)` / `workspace.branch(...)`",
        "`workspace.write(...)`",
        "`workspace.read(...)`",
        "`workspace.observe(...)`",
        "`workspace.materialize(...)`",
        "`workspace.state(...)`",
        "`workspace.inspect(...)`",
        "`workspace.public_api_contract()`",
        "`workspace.public_handle_contract()`",
        "`workspace.public_support_matrix()`",
        "`workspace.admit_public_api_family(...)`",
    ] {
        assert!(
            CLOSEOUT_DOC.contains(required),
            "closeout doc must include `{required}`"
        );
    }

    assert!(
        CLOSEOUT_DOC.contains("`workspace.intent(...)` remains part of the public vocabulary"),
        "closeout doc must explain that intent is vocabulary but not yet in the stable support set"
    );
    assert!(
        CLOSEOUT_DOC.contains("not in\nthe stable support set yet")
            || CLOSEOUT_DOC.contains("not in the stable support set yet"),
        "closeout doc must state the intent support boundary explicitly"
    );

    for required in &closeout.deferred_runtime_surfaces {
        let (surface, owner) = required
            .split_once(':')
            .expect("deferred closeout surface should carry milestone ownership");
        let expected_phrase = match surface {
            "temporal" => "temporal basis and time-aware subscriptions",
            "async-resource" => "async/resource query families",
            "mixed-cause-delivery" => "mixed truth/time/async delivery",
            "temporal-async-certification" => "temporal/async certification",
            "store-backed-execution" => "store-backed parity",
            "durable-artifacts" => "durable restart and artifact reload",
            other => panic!("unexpected deferred surface `{other}`"),
        };
        assert!(
            CLOSEOUT_DOC.contains(expected_phrase) && CLOSEOUT_DOC.contains(owner),
            "closeout doc must name deferred surface `{surface}` and owner `{owner}`"
        );
    }

    for command in &closeout.required_verification_commands {
        assert!(
            CLOSEOUT_DOC.contains(command),
            "closeout doc must cite verification command `{command}`"
        );
    }

    assert!(
        CLOSEOUT_DOC.contains("Method presence is not a support claim."),
        "closeout doc must preserve support-matrix honesty"
    );
}
