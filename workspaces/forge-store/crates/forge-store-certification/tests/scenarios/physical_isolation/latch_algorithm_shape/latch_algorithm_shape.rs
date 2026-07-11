#[path = "../../../support/recovery/closeout/fixture.rs"]
mod closeout_fixture;
#[path = "../../../support/physical_isolation/epoch_scope_and_root_kind/support.rs"]
mod support;

use std::collections::HashSet;
use std::hash::{BuildHasher, Hasher};

use forge_store_physical_isolation::{
    lower_latch_acquisition_plan, CanonicalLatchAcquisitionOrder, LatchAcquisitionDenial,
    LatchAcquisitionRequest, LatchAcquisitionStep, LatchUpgradeAuthority, LatchWaitForGraph,
    LatchWaitForGraphAdmissionDenial, PhysicalLatchClass, PhysicalLatchDeadlockPolicy,
    PhysicalLatchFamilyDeadlockPolicy, PhysicalLatchKey, PhysicalLatchWaitEdge,
};
use support::{
    current_generation_extent_reference, current_generation_page_reference,
    current_generation_segment_reference, current_root_from_authority,
    physical_authority_from_complete_closeout,
};

#[test]
fn canonical_latch_order_survives_permutation_hash_seed_and_rebuild() {
    let expected_steps = mixed_mode_expected_steps();
    let expected_plan_steps = lower_steps(expected_steps.clone());
    let lanes = vec![
        expected_steps.clone(),
        reversed(expected_steps.clone()),
        rotated_left(expected_steps.clone(), 3),
        salted_hash_steps(expected_steps.clone(), 0xA5),
        salted_hash_steps(expected_steps, 0x5A),
        mixed_mode_expected_steps(),
    ];

    for lane in lanes {
        assert_eq!(lower_steps(lane), expected_plan_steps);
    }
}

#[test]
fn canonical_comparator_preserves_mode_and_action_tiebreakers() {
    let keys = algorithm_latch_keys();
    let exclusive = LatchAcquisitionStep::exclusive(keys.page_a);
    let upgrade = LatchAcquisitionStep::upgrade(keys.page_a, upgrade_authority());
    let shared = LatchAcquisitionStep::shared(keys.page_a);

    assert!(CanonicalLatchAcquisitionOrder::compare_steps(&shared, &exclusive).is_lt());
    assert!(CanonicalLatchAcquisitionOrder::compare_steps(&exclusive, &upgrade).is_lt());
}

#[test]
fn latch_family_policy_is_explicit_for_every_class() {
    for class in all_latch_classes() {
        let prevention = PhysicalLatchFamilyDeadlockPolicy::for_class(class);
        let detection =
            PhysicalLatchFamilyDeadlockPolicy::detect_with_bounded_wait_for_graph(class);

        assert_eq!(prevention.class(), class);
        assert_eq!(
            prevention.policy(),
            PhysicalLatchDeadlockPolicy::PreventByCanonicalOrder
        );
        assert_eq!(detection.class(), class);
        assert_eq!(
            detection.policy(),
            PhysicalLatchDeadlockPolicy::DetectWithBoundedWaitForGraph
        );
    }
}

#[test]
fn bounded_detection_policy_denies_with_exact_counter_evidence() {
    let keys = algorithm_latch_keys();
    let denial = LatchWaitForGraph::bounded_with_evidence(
        vec![
            PhysicalLatchWaitEdge::new(1, 2, keys.page_a),
            PhysicalLatchWaitEdge::new(2, 3, keys.extent_a),
        ],
        1,
    )
    .unwrap_err();

    let LatchWaitForGraphAdmissionDenial::BoundExceeded(evidence) = denial else {
        panic!("bound excess must deny with pre-wait evidence");
    };
    assert_eq!(
        evidence.denial(),
        LatchAcquisitionDenial::WaitForGraphBoundExceeded
    );
    assert_counter_rows(
        evidence.counter_receipt(),
        &[
            ("s5.latch.attempts", 2),
            ("s5.latch.denied-upgrades", 0),
            ("s5.latch.detected-cycles", 0),
            ("s5.latch.execution-time-discovery-denials", 0),
            ("s5.latch.waits", 2),
        ],
    );
}

fn mixed_mode_expected_steps() -> Vec<LatchAcquisitionStep> {
    let keys = algorithm_latch_keys();
    let upgrade = upgrade_authority();
    vec![
        LatchAcquisitionStep::shared(keys.root),
        LatchAcquisitionStep::exclusive(keys.manifest),
        LatchAcquisitionStep::shared(keys.segment_a),
        LatchAcquisitionStep::exclusive(keys.segment_b),
        LatchAcquisitionStep::upgrade(keys.extent_a, upgrade),
        LatchAcquisitionStep::exclusive(keys.extent_b),
        LatchAcquisitionStep::shared(keys.page_a),
        LatchAcquisitionStep::upgrade(keys.page_b, upgrade),
        LatchAcquisitionStep::shared(keys.chunk),
    ]
}

fn lower_steps(steps: Vec<LatchAcquisitionStep>) -> Vec<LatchAcquisitionStep> {
    lower_latch_acquisition_plan(LatchAcquisitionRequest::for_declared_footprint(steps))
        .unwrap()
        .steps()
        .to_vec()
}

fn reversed(mut steps: Vec<LatchAcquisitionStep>) -> Vec<LatchAcquisitionStep> {
    steps.reverse();
    steps
}

fn rotated_left(mut steps: Vec<LatchAcquisitionStep>, by: usize) -> Vec<LatchAcquisitionStep> {
    steps.rotate_left(by);
    steps
}

fn salted_hash_steps(steps: Vec<LatchAcquisitionStep>, salt: u64) -> Vec<LatchAcquisitionStep> {
    let mut rebuilt = HashSet::with_hasher(SaltedBuildHasher { salt });
    rebuilt.extend(steps);
    rebuilt.into_iter().collect()
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

#[derive(Debug, Clone, Copy)]
struct AlgorithmLatchKeys {
    root: PhysicalLatchKey,
    manifest: PhysicalLatchKey,
    segment_a: PhysicalLatchKey,
    segment_b: PhysicalLatchKey,
    extent_a: PhysicalLatchKey,
    extent_b: PhysicalLatchKey,
    page_a: PhysicalLatchKey,
    page_b: PhysicalLatchKey,
    chunk: PhysicalLatchKey,
}

fn algorithm_latch_keys() -> AlgorithmLatchKeys {
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let root_epoch = root.epoch();
    let segment_a = root
        .admit_segment_publication_epoch(current_generation_segment_reference(3))
        .unwrap()
        .epoch();
    let segment_b = root
        .admit_segment_publication_epoch(current_generation_segment_reference(4))
        .unwrap()
        .epoch();
    let extent_a = root
        .admit_extent_publication_epoch(current_generation_extent_reference(5))
        .unwrap()
        .epoch();
    let extent_b = root
        .admit_extent_publication_epoch(current_generation_extent_reference(6))
        .unwrap()
        .epoch();
    let page_a = root
        .admit_page_publication_epoch(current_generation_page_reference(7))
        .unwrap()
        .epoch();
    let page_b = root
        .admit_page_publication_epoch(current_generation_page_reference(8))
        .unwrap()
        .epoch();
    let chunk = root.future_chunk_publication_epoch_placeholder().epoch();

    AlgorithmLatchKeys {
        root: PhysicalLatchKey::root(root_epoch),
        manifest: PhysicalLatchKey::manifest(root_epoch, root.manifest_epoch()),
        segment_a: PhysicalLatchKey::segment(root_epoch, segment_a),
        segment_b: PhysicalLatchKey::segment(root_epoch, segment_b),
        extent_a: PhysicalLatchKey::extent(root_epoch, extent_a),
        extent_b: PhysicalLatchKey::extent(root_epoch, extent_b),
        page_a: PhysicalLatchKey::page(root_epoch, page_a),
        page_b: PhysicalLatchKey::page(root_epoch, page_b),
        chunk: PhysicalLatchKey::future_chunk(root_epoch, chunk),
    }
}

fn upgrade_authority() -> LatchUpgradeAuthority {
    LatchUpgradeAuthority::from_physical_read_stability_authority(
        &physical_authority_from_complete_closeout(),
    )
}

fn all_latch_classes() -> [PhysicalLatchClass; 6] {
    [
        PhysicalLatchClass::Root,
        PhysicalLatchClass::Manifest,
        PhysicalLatchClass::Segment,
        PhysicalLatchClass::Extent,
        PhysicalLatchClass::Page,
        PhysicalLatchClass::FutureChunk,
    ]
}

#[derive(Debug, Clone, Copy)]
struct SaltedBuildHasher {
    salt: u64,
}

impl BuildHasher for SaltedBuildHasher {
    type Hasher = SaltedHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SaltedHasher { state: self.salt }
    }
}

struct SaltedHasher {
    state: u64,
}

impl Hasher for SaltedHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state = self.state.rotate_left(5) ^ u64::from(*byte);
        }
    }
}
