use worth_proof::TransitionOutcome;
use worth_signal::facade::{
    apply_installed_scoped_changes, Aspect, ChangedRegion, InstalledSignalScopedChange, SignalGraph,
};

#[test]
fn wasm_supported_facade_retains_partition_and_detail_locality() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let aspect = Aspect::new(0);
    let TransitionOutcome::Success(capability) = graph.admit_installed_aspect(source, aspect)
    else {
        panic!("the WASM-supported Signal facade must admit its own installed aspect")
    };
    let TransitionOutcome::Success(admitted) = apply_installed_scoped_changes(
        &mut graph,
        [InstalledSignalScopedChange::new(
            capability,
            [ChangedRegion::new("opaque-region").with_detail("opaque-scope")],
        )],
    ) else {
        panic!("the WASM-supported facade must admit exact scoped invalidation")
    };

    let change = admitted.changes().next().unwrap();
    assert_eq!(change.aspect(), aspect);
    assert_eq!(change.changed_regions()[0].partition.0, "opaque-region");
    assert_eq!(
        change.changed_regions()[0].detail.as_deref(),
        Some("opaque-scope")
    );
}
