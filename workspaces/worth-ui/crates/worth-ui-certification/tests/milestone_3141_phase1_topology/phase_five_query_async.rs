use super::{repository_document, workspace_source_inventory};

const QUERY_BINDING_ROOT: &str = "crates/worth-ui-query-binding/src/presentation_async";
const QUERY_BINDING_FILES: [&str; 7] = [
    "declaration.rs",
    "request_basis.rs",
    "runtime_bridge.rs",
    "semantic_invalidation.rs",
    "observation.rs",
    "retained_posture.rs",
    "terminal_projection.rs",
];

#[test]
fn phase_five_query_async_dependency_direction_is_explicit_and_enforced() {
    let gate_e_active = future_binding_file_count() > 0;
    let binding_manifest =
        repository_document("workspaces/worth-ui/crates/worth-ui-query-binding/Cargo.toml");
    for dependency in ["worth-query", "worth-runtime-bridge", "worth-signal"] {
        assert!(
            binding_manifest.contains(dependency),
            "the sole WUI Query audience edge omits {dependency}"
        );
    }

    for manifest in ["workspaces/worth-ui/crates/worth-ui-runtime/Cargo.toml"] {
        let document = repository_document(manifest);
        let governed = if gate_e_active {
            document.as_str()
        } else {
            document
                .split("[dev-dependencies]")
                .next()
                .unwrap_or(&document)
        };
        for forbidden in ["worth-query", "worth-signal"] {
            assert!(
                !governed.contains(forbidden),
                "{manifest} gives a governed target direct {forbidden} access"
            );
        }
    }

    let inventory = workspace_source_inventory();
    for root in ["crates/worth-ui-runtime/src"] {
        for source in inventory.rust_files_under(root) {
            let path = source.relative_path().to_string_lossy();
            if !gate_e_active && is_test_source(&path) {
                continue;
            }
            for forbidden in ["worth_query", "worth_signal"] {
                assert!(
                    !source.text().contains(forbidden),
                    "{} imports {forbidden} outside the Query audience edge",
                    source.relative_path().display()
                );
            }
        }
    }
    if !gate_e_active {
        let plan =
            repository_document("_docs/worth-ui/milestone-3.14.1-phase-5-implementation-plan.md");
        assert!(plan.contains("direct test-only `worth-query-host` installation"));
        assert!(plan.contains("no direct"));
        assert!(plan.contains("runtime imports neither Signal nor Query"));
    }

    let native_manifest =
        repository_document("workspaces/worth-ui/crates/worth-ui-host-native/Cargo.toml");
    assert!(
        !native_manifest.contains("worth-query"),
        "host-native may own physical Signal but never Query"
    );
    for source in inventory.rust_files_under("crates/worth-ui-host-native/src") {
        assert!(
            !source.text().contains("worth_query"),
            "{} imports Query into the native physical owner",
            source.relative_path().display()
        );
    }
}

#[test]
fn phase_five_query_async_destination_owns_the_missing_substrate_extensions() {
    let specification = repository_document("_docs/worth-ui/milestone-3.14.1-phase-5.md");
    let plan =
        repository_document("_docs/worth-ui/milestone-3.14.1-phase-5-implementation-plan.md");
    for required in [
        "application/declaration/async_resource/request_identity.rs",
        "runtime/async_result_state.rs",
        "source/async_declaration/completion/",
        "effects-indeterminate completion class",
        "Query-owned `Unresolved` retained-result state",
    ] {
        assert!(
            specification.contains(required) || plan.contains(required),
            "Phase 5 leaves the required Query substrate owner implicit: {required}"
        );
    }
    assert!(plan.contains("current identity part"));
    assert!(plan.contains("string-only"));

    let current_identity = repository_document(
        "workspaces/worth-query/crates/worth-query/src/application/declaration/async_resource.rs",
    );
    assert!(current_identity.contains("pub fn text("));
    let current_state = repository_document(
        "workspaces/worth-query/crates/worth-query/src/runtime/async_result_state.rs",
    );
    if !current_state.contains("Unresolved") {
        assert!(plan.contains("Add Query's retained `Unresolved` async result state"));
    }

    assert_complete_future_binding_or_absent();
}

#[test]
fn terminal_projection_and_string_identity_cannot_enter_operational_text_flow() {
    let inventory = workspace_source_inventory();
    for root in [
        "crates/worth-ui-runtime/src/native_platform/text_presentation",
        "crates/worth-ui-host-native/src/native/text_atlas",
        "crates/worth-ui-host-native/src/native/presentation/text",
    ] {
        for source in inventory.rust_files_under(root) {
            let path = source.relative_path().to_string_lossy();
            if is_test_source(&path) {
                continue;
            }
            for forbidden in [
                "terminal_projection",
                "serde_json::Value",
                "WorthQueryAsyncRequestIdentityPart::text",
            ] {
                assert!(
                    !source.text().contains(forbidden),
                    "{} admits operational authority through {forbidden}",
                    source.relative_path().display()
                );
            }
        }
    }

    let fixture = repository_document(
        "workspaces/worth-ui/crates/worth-ui/tests/ui/facade/query_binding/fail/reporting_projection_cannot_enter_consumption.rs",
    );
    assert!(fixture.contains("reporting.into()"));
    let compile_manifest = repository_document(
        "workspaces/worth-ui/crates/worth-ui-certification/tests/fixtures/compile_contracts/Cargo.toml",
    );
    assert!(compile_manifest.contains("reporting_projection_cannot_enter_consumption.rs"));
}

#[test]
fn presentation_transition_counter_names_exactly_ten_events() {
    let specification = repository_document("_docs/worth-ui/milestone-3.14.1-phase-5.md");
    for event in [
        "1. attempt A becomes `pending`;",
        "2. attempt B supersedes A;",
        "3. A's stale completion is rejected without changing B;",
        "4. B becomes `completed` from its owner-issued native observation;",
        "5. a duplicate or out-of-order B observation is rejected without a new result;",
        "6. attempt C becomes `pending`;",
        "7. C becomes `unresolved` from effects-indeterminate;",
        "8. C records `recovery-required` without live authority entering Query;",
        "9. reconstruction resolves C into a fresh current successor;",
        "10. terminal close releases the managed Query resource.",
    ] {
        assert!(
            specification.contains(event),
            "transition contract omits {event}"
        );
    }
    assert!(specification.contains("`presentation-transitions=10`"));

    let python = repository_document("scripts/ci/worth_ui_3141_p5_contracts.py");
    assert!(
        python.contains("\"P5-TEXT-ASYNC-PRESENTATION-01\": (\"presentation-transitions\", 10)")
    );
    let rust = repository_document(
        "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger/execution_posture.rs",
    );
    assert!(rust.contains("\"P5-TEXT-ASYNC-PRESENTATION-01\" => 10"));
}

#[test]
fn phase_five_text_cost_freezes_performed_m13_ui_locality_matrix() {
    let specification = repository_document("_docs/worth-ui/milestone-3.14.1-phase-5.md");
    for counter in [
        "source_output_deltas_consumed",
        "direct_subscriber_edges_examined",
        "candidates_rejected_by_aspect_contract",
        "candidates_rejected_by_scope",
        "work_items_admitted",
        "work_items_merged",
        "ready_items_enqueued",
        "ready_items_popped",
        "nodes_evaluated",
        "produced_deltas_emitted",
        "propagation_stops",
        "non_semantic_node_visits",
        "maximum_ready_frontier_width",
    ] {
        assert!(
            specification.contains(counter),
            "P5-TEXT-COST omits performed M13 counter {counter}"
        );
    }
    for scenario in [
        "content-only paragraph edit",
        "width-only paragraph change",
        "paint-value-only span change",
        "paint-span-boundary change",
        "pure-DPI change",
        "atlas miss among many hits",
        "upload completion among many mounted presentations",
        "layout removal with both shared and exclusive raster-key pins",
    ] {
        assert!(
            specification.contains(scenario),
            "P5-TEXT-COST omits UI locality scenario {scenario}"
        );
    }
    assert!(specification.contains("*realized* Milestone 13"));
    assert!(specification.contains("Predicted work"));

    let python = repository_document("scripts/ci/worth_ui_3141_p5_contracts.py");
    assert!(python.contains("\"P5-TEXT-COST-01\": (\"ui-locality-worlds\", 32)"));
}

fn assert_complete_future_binding_or_absent() {
    let paths = future_binding_paths();
    let present = future_binding_file_count();
    assert!(
        present == 0 || present == paths.len(),
        "presentation_async must appear as one complete named responsibility set"
    );
    if present == paths.len() {
        let inventory = workspace_source_inventory();
        let operational = inventory
            .source(&format!("{QUERY_BINDING_ROOT}/runtime_bridge.rs"))
            .expect("complete presentation_async runtime bridge owner");
        assert!(!operational
            .text()
            .contains("WorthQueryAsyncRequestIdentityPart::text"));
        let terminal = inventory
            .source(&format!("{QUERY_BINDING_ROOT}/terminal_projection.rs"))
            .expect("complete presentation_async terminal projection owner");
        assert!(!terminal.text().contains("pub fn prepare"));
        assert!(!terminal.text().contains("pub fn settle"));
    }
}

fn future_binding_file_count() -> usize {
    let inventory = workspace_source_inventory();
    future_binding_paths()
        .iter()
        .filter(|path| inventory.source(path).is_some())
        .count()
}

fn future_binding_paths() -> [String; QUERY_BINDING_FILES.len()] {
    QUERY_BINDING_FILES.map(|name| format!("{QUERY_BINDING_ROOT}/{name}"))
}

fn is_test_source(path: &str) -> bool {
    path.contains("/tests/")
        || path.contains("\\tests\\")
        || path.ends_with("_tests.rs")
        || path.ends_with("tests.rs")
        || path.ends_with("_test_support.rs")
}
