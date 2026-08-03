use crate::repository_document;

const VISUAL_DOC: &str = "workspaces/worth-ui/docs/visual-inspection.md";
const LIFECYCLE_DOC: &str = "workspaces/worth-ui/docs/application-lifecycle.md";
const DIAGNOSTIC_DOC: &str = "_docs/worth-ui/ai-diagnostics.md";
const SPEC: &str = "_docs/worth-ui/milestone-3.11.md";
const PHASE_5D_PLAN: &str = "_docs/worth-ui/milestone-3.11-phase-5d-implementation-plan.md";
const ROADMAP: &str = "_docs/worth-ui/worth_ui_roadmap.md";
const SUCCESSOR_SPEC: &str = "_docs/worth-ui/milestone-3.12.md";
const PASS_SOURCE: &str =
    "workspaces/worth-ui/crates/worth-ui/tests/ui/visual_snapshot/pass/governed_visual_snapshot_lifecycle.rs";
const PASS_ROW: &str = "pass,tests/ui/visual_snapshot/pass/governed_visual_snapshot_lifecycle.rs,visual_snapshot_facade_compile.rs";
const FENCE_MARKER: &str =
    "<!-- compile-pass-source:tests/ui/visual_snapshot/pass/governed_visual_snapshot_lifecycle.rs -->";
const SMALL_EXAMPLE_MARKER: &str = "<!-- compile-pass-fragment:required_pixels_are_total -->";

pub(super) fn assert_phase_5c_documentation() {
    let visual = repository_document(VISUAL_DOC);
    assert_visual_topics(&visual);
    assert_small_compile_fragment(&visual);
    assert_exact_compile_source(&visual);
    assert_compile_inventory();
    assert_human_pulse_workflow();
    assert_diagnostic_successor_boundary();
}

pub(super) fn assert_phase_5d_documentary_closure() {
    let spec = normalized_whitespace(&repository_document(SPEC));
    let plan = normalized_whitespace(&repository_document(PHASE_5D_PLAN));
    let roadmap = normalized_whitespace(&repository_document(ROADMAP));
    let successor = normalized_whitespace(&repository_document(SUCCESSOR_SPEC));
    for (document, required) in [
        (
            spec.as_str(),
            "Status: Complete. Phases 1 through 5 closed on 2026-07-27.",
        ),
        (plan.as_str(), "Status: complete"),
        (
            roadmap.as_str(),
            "Status: Closed on 2026-07-27. Phases 1 through 5 are complete",
        ),
        (
            roadmap.as_str(),
            "The permanent Platform Pulse now renders distinct blue background and yellow target regions",
        ),
        (
            successor.as_str(),
            "Milestone 3.11 closed on 2026-07-27.",
        ),
        (
            successor.as_str(),
            "Identity-aware predecessor/successor comparison is now a shipped Milestone 3.12 capability",
        ),
    ] {
        assert!(
            document.contains(required),
            "Phase 5D documentary closure misses `{required}`"
        );
    }
}

fn assert_visual_topics(visual: &str) {
    for required in [
        "A screenshot is only an image.",
        "## Why You Use It",
        "## Stable Entry Points",
        "## Core Mental Model",
        "## How It Executes",
        "## Small Example",
        "## Real Example",
        "## How It Relates To Other Features",
        "## Inspection And Debugging",
        "## Anti-Patterns",
        "## Current Limits",
        "The runtime owns visible meaning and mounted identity.",
        "UiCurrentPresentedSurfaceTarget",
        "UiMountedNodeVisualTarget",
        "UiClientRegionVisualTarget",
        "Rectangles are half-open",
        "UiVisualVisibleOutcome",
        "UiVisualHitTestOutcome",
        "UiVisualSnapshotOutcome<P>",
        "UiVisualSnapshotAffinity",
        "UiVisualInspectionPolicy",
        "UiGeometryOnly",
        "UiPixelsRequired",
        "UiVisualIdentityTrace",
        "UiPublishedVisualOverlay",
        "UiClearedVisualOverlayReceipt",
        "UiVisualCancellationPosture",
        "dispose_visual_snapshot",
    ] {
        assert!(
            visual.contains(required),
            "visual inspection documentation misses `{required}`"
        );
    }
}

fn assert_small_compile_fragment(visual: &str) {
    let fragment = fenced_rust_after(visual, SMALL_EXAMPLE_MARKER);
    let source = repository_document(PASS_SOURCE);
    assert!(
        normalized_source(&source).contains(&normalized_source(fragment)),
        "small visual inspection example is not compiled by the existing pass source"
    );
}

fn assert_exact_compile_source(visual: &str) {
    assert_eq!(
        visual.matches(FENCE_MARKER).count(),
        1,
        "visual documentation must own one exact compile-source marker"
    );
    let documented = fenced_rust_after(visual, FENCE_MARKER);
    let source = repository_document(PASS_SOURCE);
    assert_eq!(
        normalized_source(documented),
        normalized_source(&source),
        "documented public example drifted from the existing compile-pass source"
    );
}

fn assert_compile_inventory() {
    for inventory in [
        "workspaces/worth-ui/crates/worth-ui/tests/suites/compile_contract_cases.csv",
        "workspaces/worth-ui/crates/worth-ui/tests/suites/compile_contract_execution.csv",
    ] {
        let text = repository_document(inventory);
        assert!(
            text.lines().any(|line| line == PASS_ROW),
            "existing compile inventory `{inventory}` misses the documented pass source"
        );
    }
}

fn assert_human_pulse_workflow() {
    let lifecycle = repository_document(LIFECYCLE_DOC);
    for required in [
        "-p worth-ui-platform-pulse",
        "--source-root $sourceRoot",
        "[48, 24]",
        "[112, 72]",
        "[80, 48]",
        "[16, 16]",
        "VisualSnapshotCaptured",
        "VisualPointTrace",
        "VisualOverlayPublished",
        "VisualOverlayCleared",
        "VisualSnapshotRetired",
        "component:platform.pulse.component.identity_target",
        "component platform.pulse.component.identity_target {}",
        "token theme.platform_pulse.identity_target_fill = \"theme.platform_pulse.yellow\";",
        "disposed_visual_structural_bytes",
        "--features executable-world --test executable_world -- --nocapture",
    ] {
        assert!(
            lifecycle.contains(required),
            "human Platform Pulse workflow misses `{required}`"
        );
    }
}

fn assert_diagnostic_successor_boundary() {
    let diagnostic = repository_document(DIAGNOSTIC_DOC);
    assert!(
        diagnostic_successor_boundary_is_present(&diagnostic),
        "diagnostic architecture loses the implemented 3.11 / committed 3.12 boundary"
    );
}

fn diagnostic_successor_boundary_is_present(diagnostic: &str) -> bool {
    let diagnostic = normalized_whitespace(diagnostic);
    [
        "UiVisualSnapshotReceipt<ArtifactPosture>",
        "Implemented Visual Snapshot Closure: Milestone 3.11",
        "Committed Successor: Milestone 3.12",
        "Identity-aware predecessor/successor snapshot comparison is not a 3.11 capability.",
        "AI and human consumers project the same receipt",
    ]
    .iter()
    .all(|required| diagnostic.contains(required))
}

#[test]
fn removing_the_3_12_comparison_reservation_turns_documentation_proof_red() {
    let diagnostic = repository_document(DIAGNOSTIC_DOC);
    let mutated = diagnostic.replace(
        "Committed Successor: Milestone 3.12",
        "Unowned Future Comparison",
    );
    assert!(!diagnostic_successor_boundary_is_present(&mutated));
}

fn fenced_rust_after<'a>(document: &'a str, marker: &str) -> &'a str {
    let after_marker = document
        .split_once(marker)
        .expect("compile-source marker exists")
        .1;
    let after_open = after_marker
        .split_once("```rust")
        .expect("compile-source marker is followed by a Rust fence")
        .1;
    let after_open = after_open
        .strip_prefix("\r\n")
        .or_else(|| after_open.strip_prefix('\n'))
        .expect("Rust fence body begins on the next line");
    after_open
        .split_once("\r\n```")
        .or_else(|| after_open.split_once("\n```"))
        .expect("compile-source fence closes")
        .0
}

fn normalized_source(text: &str) -> String {
    text.replace("\r\n", "\n").trim_end_matches('\n').to_owned()
}

fn normalized_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
