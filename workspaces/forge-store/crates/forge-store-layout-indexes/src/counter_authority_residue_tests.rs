fn source(path: &str) -> String {
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
}

#[test]
fn lsm_counter_observations_are_read_only_and_owner_issued() {
    let schema = source("src/strategy/lsm/execution/counter_observation.rs");
    let lookup = source("src/strategy/lsm/execution/lookup_outcome.rs");
    let replay = source("src/strategy/lsm/replay_runtime.rs");
    let publication = source("src/strategy/lsm/execution/witness.rs");
    let compaction = source("src/strategy/lsm/compaction/execution.rs");

    assert!(!schema.contains("PartialEq, Eq, Default"));
    assert!(!schema.contains("pub(crate) const fn new(\n        point_lookups"));
    for owner_issuer in [
        "fn lookup()",
        "fn replay(",
        "fn manifest_publication(",
        "fn compaction(",
    ] {
        assert!(
            schema.contains(owner_issuer),
            "missing owner counter issuer {owner_issuer}"
        );
    }
    assert!(lookup.contains("BaselineLsmCounterObservation::lookup()"));
    assert!(!lookup.contains("tombstone_blocks_older: bool,\n        counters:"));
    assert!(!replay.contains("BaselineLsmCounterObservation"));
    assert!(!publication.contains("BaselineLsmCounterObservation"));
    assert!(compaction.contains("BaselineLsmCounterObservation::compaction("));
}

#[test]
fn degraded_counter_observation_comes_from_the_physical_owner_receipt() {
    let execution = source("src/access/execution/degraded_scan/executed.rs");
    let physical = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../forge-store-physical-format/src/facade/runtime_receipt.rs"),
    )
    .unwrap();

    assert!(execution.contains("PlatformPhysicalDegradedExecutionObservation"));
    assert!(execution.contains("self.physical.scan().observed_rows()"));
    assert!(physical.contains("pub(super) const fn new("));
    assert!(physical.contains("counters: PlatformPhysicalFacadeCounterSnapshot"));
}

#[test]
fn executed_observation_is_an_exhaustive_non_authoring_owner_union() {
    let observation = source("src/access/execution/view.rs");
    for owner in [
        "BTreeLookup(crate::BaselineBTreeLookupExecution)",
        "BTreeReplay(crate::BaselineBTreeReplayRecoveryExecution)",
        "LsmLookup(crate::BaselineLsmLookupExecution)",
        "LsmRunPublication(crate::BaselineLsmManifestPublicationExecution)",
        "LsmReplay(crate::BaselineLsmReplayExecution)",
        "LsmCompaction(crate::BaselineLsmCompactionPublicationReceipt)",
        "DegradedScan(super::DegradedScanExecution)",
    ] {
        assert!(
            observation.contains(owner),
            "missing executed owner {owner}"
        );
    }
    assert!(!observation.contains("pub fn new"));
    assert!(!observation.contains("pub fn issue"));
}

#[test]
fn lsm_replay_execution_is_issued_inside_the_execution_owner() {
    let execution = source("src/strategy/lsm/execution/replay_execution.rs").replace('\r', "");
    let module = source("src/strategy/lsm/execution/mod.rs");
    let lsm = source("src/strategy/lsm/mod.rs");
    let replay = execution
        .split("impl BaselineLsmReplayExecution")
        .nth(1)
        .expect("LSM replay execution impl")
        .split("#[derive")
        .next()
        .unwrap();

    assert!(module.contains("#[path = \"../replay_runtime.rs\"]"));
    assert!(!lsm.contains("mod replay_runtime;"));
    assert!(replay.contains("const fn new("));
    assert!(!replay.contains("pub(crate) const fn new("));
}
