use super::{export_block, read_repository_document};

const DURABILITY_EXPORTS: &[&str] = &[
    "CompletedPhysicalMutation",
    "CompletedUnobservedPhysicalMutation",
    "IndeterminatePhysicalMutation",
    "IndeterminatePhysicalMutationEvidence",
    "PhysicalMutationAcknowledgment",
    "PhysicalMutationCancellationOutcome",
    "PhysicalMutationCompletedBreadth",
    "PhysicalMutationExecutedBoundaryEvidence",
    "PhysicalMutationHandle",
    "PhysicalMutationIndeterminateStage",
    "PhysicalMutationObservation",
    "PhysicalMutationOutcome",
    "PhysicalMutationPerformanceEvidence",
    "PhysicalMutationPoll",
    "PhysicalMutationProgress",
    "PhysicalMutationProgressPhase",
    "PhysicalMutationShutdown",
    "PhysicalMutationTerminalObservation",
    "ProvenNoEffectPhysicalMutationEvidence",
];

pub(super) fn assert_reachability(durability_exports: &str) {
    for expected in DURABILITY_EXPORTS {
        assert!(
            durability_exports.contains(expected),
            "Phase 8 durability surface `{expected}` is hidden from physical_runtime"
        );
    }

    let prepared = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/durable_preparation/prepared.rs",
    )
    .expect("read prepared mutation owner");
    assert!(prepared.contains("pub fn start(self)"));
    assert!(prepared.contains("pub fn execute(self)"));

    let serving = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/lifecycle/serving_runtime.rs",
    )
    .expect("read serving runtime owner");
    assert!(serving.contains("pub fn physical_mutation_observation("));

    let runtime = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    )
    .expect("read physical runtime facade");
    let exports = export_block(runtime.as_str(), "pub use durability::{");
    for expected in DURABILITY_EXPORTS {
        assert!(exports.contains(expected));
    }
}
