use forge_store_test_support::harness::physical_isolation::epoch_scope as support;

use forge_store_physical_isolation::{
    latch_counter_backed_performance_receipt, lower_latch_acquisition_plan,
    pre_wait_denial_for_execution_time_latch_discovery, pre_wait_denial_for_hierarchy_inversion,
    pre_wait_denial_for_unauthorized_latch_upgrade, pre_wait_denial_for_unordered_latch_set,
    DeadlockPreventionDenial, LatchAcquisitionDenial, LatchAcquisitionRequest,
    LatchAcquisitionStep, LatchUpgradeAuthority, LatchWaitCounterSnapshot, LatchWaitForGraph,
    PhysicalLatchClass, PhysicalLatchKey, PhysicalLatchWaitEdge,
};
use support::{
    current_generation_extent_reference, current_generation_page_reference,
    current_generation_segment_reference, current_root_from_authority,
    physical_authority_from_complete_closeout,
};

#[test]
fn declared_footprints_lower_to_same_canonical_latch_order() {
    let keys = representative_latch_keys();
    let requested_a = vec![
        LatchAcquisitionStep::shared(keys.page),
        LatchAcquisitionStep::shared(keys.root),
        LatchAcquisitionStep::shared(keys.extent),
        LatchAcquisitionStep::shared(keys.segment),
        LatchAcquisitionStep::shared(keys.manifest),
        LatchAcquisitionStep::shared(keys.chunk),
    ];
    let requested_b = vec![
        LatchAcquisitionStep::shared(keys.chunk),
        LatchAcquisitionStep::shared(keys.manifest),
        LatchAcquisitionStep::shared(keys.segment),
        LatchAcquisitionStep::shared(keys.root),
        LatchAcquisitionStep::shared(keys.page),
        LatchAcquisitionStep::shared(keys.extent),
    ];

    let plan_a =
        lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(requested_a))
            .unwrap();
    let plan_b =
        lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(requested_b))
            .unwrap();

    assert_eq!(plan_a.steps(), plan_b.steps());
    assert_eq!(
        latch_classes(plan_a.steps()),
        vec![
            PhysicalLatchClass::Root,
            PhysicalLatchClass::Manifest,
            PhysicalLatchClass::Segment,
            PhysicalLatchClass::Extent,
            PhysicalLatchClass::Page,
            PhysicalLatchClass::FutureChunk,
        ]
    );
    let _proof = plan_a.order_proof();
}

#[test]
fn upgrade_steps_require_physical_authority_witness() {
    let authority = physical_authority_from_complete_closeout();
    let keys = representative_latch_keys();
    let upgrade_authority =
        LatchUpgradeAuthority::from_physical_read_stability_authority(&authority);

    let upgraded =
        lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(vec![
            LatchAcquisitionStep::upgrade(keys.page, upgrade_authority),
        ]))
        .unwrap();

    assert_eq!(upgraded.steps()[0].key(), keys.page);
    let denied_upgrade = pre_wait_denial_for_unauthorized_latch_upgrade(keys.page).unwrap();

    assert_denial_evidence(
        &denied_upgrade,
        DeadlockPreventionDenial::UnauthorizedUpgrade(keys.page),
        &[
            ("s5.latch.attempts", 1),
            ("s5.latch.denied-upgrades", 1),
            ("s5.latch.detected-cycles", 0),
            ("s5.latch.execution-time-discovery-denials", 0),
            ("s5.latch.waits", 0),
        ],
    );
}

#[test]
fn duplicate_and_conflicting_latch_requests_deny_before_waiting() {
    let keys = representative_latch_keys();
    let duplicate =
        lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(vec![
            LatchAcquisitionStep::shared(keys.page),
            LatchAcquisitionStep::shared(keys.page),
        ]));
    let conflicting =
        lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(vec![
            LatchAcquisitionStep::shared(keys.extent),
            LatchAcquisitionStep::exclusive(keys.extent),
        ]));

    assert_eq!(
        duplicate.unwrap_err(),
        LatchAcquisitionDenial::DuplicateLatchKey(keys.page)
    );
    assert!(matches!(
        conflicting.unwrap_err(),
        LatchAcquisitionDenial::ConflictingLatchMode { key, .. } if key == keys.extent
    ));
}

#[test]
fn unordered_execution_inputs_and_hierarchy_inversions_deny_before_waiting() {
    let keys = representative_latch_keys();
    let inverted = vec![
        LatchAcquisitionStep::shared(keys.page),
        LatchAcquisitionStep::shared(keys.root),
    ];

    assert!(pre_wait_denial_for_unordered_latch_set(&[
        LatchAcquisitionStep::shared(keys.root),
        LatchAcquisitionStep::shared(keys.page),
    ])
    .unwrap()
    .is_none());
    let unordered_denial = pre_wait_denial_for_unordered_latch_set(&inverted)
        .unwrap()
        .unwrap();
    assert_denial_evidence(
        &unordered_denial,
        DeadlockPreventionDenial::UnorderedLockSet,
        &[
            ("s5.latch.attempts", 2),
            ("s5.latch.denied-upgrades", 0),
            ("s5.latch.detected-cycles", 0),
            ("s5.latch.execution-time-discovery-denials", 0),
            ("s5.latch.waits", 0),
        ],
    );
    let hierarchy_denial = pre_wait_denial_for_hierarchy_inversion(&inverted)
        .unwrap()
        .unwrap();
    assert_denial_evidence(
        &hierarchy_denial,
        DeadlockPreventionDenial::HierarchyInversion,
        &[
            ("s5.latch.attempts", 2),
            ("s5.latch.denied-upgrades", 0),
            ("s5.latch.detected-cycles", 0),
            ("s5.latch.execution-time-discovery-denials", 0),
            ("s5.latch.waits", 0),
        ],
    );
    let discovery_denial = pre_wait_denial_for_execution_time_latch_discovery(keys.page).unwrap();

    assert_denial_evidence(
        &discovery_denial,
        DeadlockPreventionDenial::ExecutionTimeLatchDiscovery(keys.page),
        &[
            ("s5.latch.attempts", 1),
            ("s5.latch.denied-upgrades", 0),
            ("s5.latch.detected-cycles", 0),
            ("s5.latch.execution-time-discovery-denials", 1),
            ("s5.latch.waits", 0),
        ],
    );
}

#[test]
fn wait_for_graph_detects_multi_actor_cycles_with_exact_counter_evidence() {
    let keys = representative_latch_keys();
    let graph = LatchWaitForGraph::bounded(
        vec![
            PhysicalLatchWaitEdge::new(1, 2, keys.page),
            PhysicalLatchWaitEdge::new(2, 3, keys.extent),
            PhysicalLatchWaitEdge::new(3, 1, keys.segment),
        ],
        4,
    )
    .unwrap();

    let report = graph.detect_cycle().unwrap();

    assert_eq!(report.cycle_edges().len(), 3);
    assert_eq!(report.counters().attempt_count(), 3);
    assert_eq!(report.counters().wait_count(), 3);
    assert_eq!(report.counters().detected_cycle_count(), 1);
    assert_counter_rows(
        report.counter_receipt(),
        &[
            ("s5.latch.attempts", 3),
            ("s5.latch.denied-upgrades", 0),
            ("s5.latch.detected-cycles", 1),
            ("s5.latch.execution-time-discovery-denials", 0),
            ("s5.latch.waits", 3),
        ],
    );
    let cycle_denial = graph.pre_wait_cycle_denial().unwrap().unwrap();

    assert_latch_denial_evidence(
        &cycle_denial,
        LatchAcquisitionDenial::CyclicPlan,
        &[
            ("s5.latch.attempts", 3),
            ("s5.latch.denied-upgrades", 0),
            ("s5.latch.detected-cycles", 1),
            ("s5.latch.execution-time-discovery-denials", 0),
            ("s5.latch.waits", 3),
        ],
    );
}

#[test]
fn latch_counter_evidence_is_exact_and_counter_backed() {
    let counters = LatchWaitCounterSnapshot::from_exact_counts(6, 2, 1, 1, 1);
    let receipt = latch_counter_backed_performance_receipt(counters).unwrap();
    let observed: Vec<(&str, u64)> = receipt
        .counter_rows()
        .iter()
        .map(|row| (row.name().as_str(), row.observed_count()))
        .collect();

    assert_eq!(
        observed,
        vec![
            ("s5.latch.attempts", 6),
            ("s5.latch.denied-upgrades", 1),
            ("s5.latch.detected-cycles", 1),
            ("s5.latch.execution-time-discovery-denials", 1),
            ("s5.latch.waits", 2),
        ]
    );
}

fn assert_counter_rows(
    receipt: &forge_store_physical_isolation::LatchCounterPerformanceReceipt,
    expected: &[(&str, u64)],
) {
    let observed: Vec<(&str, u64)> = receipt
        .counter_rows()
        .iter()
        .map(|row| (row.name().as_str(), row.observed_count()))
        .collect();

    assert_eq!(observed, expected);
}

fn assert_denial_evidence(
    evidence: &forge_store_physical_isolation::LatchDeniedBeforeWaitEvidence,
    expected: DeadlockPreventionDenial,
    expected_rows: &[(&str, u64)],
) {
    assert_latch_denial_evidence(evidence, expected, expected_rows);
}

fn assert_latch_denial_evidence(
    evidence: &forge_store_physical_isolation::LatchDeniedBeforeWaitEvidence,
    expected: LatchAcquisitionDenial,
    expected_rows: &[(&str, u64)],
) {
    assert_eq!(evidence.denial(), expected);
    assert_counter_rows(evidence.counter_receipt(), expected_rows);
}

#[derive(Debug, Clone, Copy)]
struct RepresentativeLatchKeys {
    root: PhysicalLatchKey,
    manifest: PhysicalLatchKey,
    segment: PhysicalLatchKey,
    extent: PhysicalLatchKey,
    page: PhysicalLatchKey,
    chunk: PhysicalLatchKey,
}

fn representative_latch_keys() -> RepresentativeLatchKeys {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let root_epoch = root.epoch();
    let segment = root
        .admit_segment_publication_epoch(current_generation_segment_reference(3))
        .unwrap()
        .epoch();
    let extent = root
        .admit_extent_publication_epoch(current_generation_extent_reference(5))
        .unwrap()
        .epoch();
    let page = root
        .admit_page_publication_epoch(current_generation_page_reference(7))
        .unwrap()
        .epoch();
    let chunk = root.future_chunk_publication_epoch_placeholder().epoch();

    RepresentativeLatchKeys {
        root: PhysicalLatchKey::root(root_epoch),
        manifest: PhysicalLatchKey::manifest(root_epoch, root.manifest_epoch()),
        segment: PhysicalLatchKey::segment(root_epoch, segment),
        extent: PhysicalLatchKey::extent(root_epoch, extent),
        page: PhysicalLatchKey::page(root_epoch, page),
        chunk: PhysicalLatchKey::future_chunk(root_epoch, chunk),
    }
}

fn latch_classes(steps: &[LatchAcquisitionStep]) -> Vec<PhysicalLatchClass> {
    steps.iter().map(|step| step.key().class()).collect()
}
