use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, PhysicalRecordInitialization, PhysicalRecordOpen,
    RecordAppendBatch, RecordByteLimit, RecordReadLimits,
};
use worth_store_physical_backend::MediaOperationRole;

use super::scale_support::{access, assert_canonical_parity, complete_scan, format, placement};
use super::scenario_evidence::ScenarioPredicate;
use super::{media, success};

#[test]
fn bounded_scale_identity_format_and_policy_courtroom() {
    let observations = [1_u16, 9, 65].map(observe_scale_world);
    assert_eq!(observations.map(|value| value.routing_level), [0, 3, 6]);
    assert_eq!(
        observations.map(|value| value.point_blocks),
        [2, 5, 8],
        "C5_PREDICATE:locate-open-scale"
    );
    assert!(observations.iter().all(|value| {
        value.point_allocations == 16_384
            && value.scan_allocations >= value.point_allocations
            && value.scan_allocations < 65_536
    }));
    assert!(observations
        .windows(2)
        .all(|pair| pair[0].open_reads == pair[1].open_reads
            && pair[0].open_bytes == pair[1].open_bytes));
    assert!(observations
        .windows(2)
        .all(|pair| pair[1].whole_blocks > pair[0].whole_blocks
            && pair[1].point_blocks - pair[0].point_blocks
                < pair[1].whole_blocks - pair[0].whole_blocks));
    assert!(observations.iter().all(|value| {
        value.point_comparisons >= value.point_blocks
            && value.point_comparisons <= value.point_blocks.saturating_mul(2)
            && value.scan_records == u64::from(value.record_count)
            && value.scan_payload_bytes == u64::from(value.record_count) * 100
    }));

    assert!(observations.iter().all(|value| value.invalid_worlds == 5));
    super::scale_policy_evolution::prove();
}

#[derive(Clone, Copy)]
struct ScaleObservation {
    record_count: u16,
    routing_level: u16,
    whole_blocks: u64,
    point_blocks: u64,
    point_comparisons: u64,
    open_reads: u64,
    open_bytes: u64,
    scan_records: u64,
    scan_payload_bytes: u64,
    point_allocations: usize,
    scan_allocations: usize,
    invalid_worlds: u8,
}

fn observe_scale_world(record_count: u16) -> ScaleObservation {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let format = format();
    let placement = placement(format, 2, 2, 50);
    let initial_access = access(format, 17);
    let mut serving = success(media(&root).initialize_record_store(
        PhysicalRecordInitialization::new(format, placement, initial_access),
    ));
    let payloads = (0..record_count)
        .map(|ordinal| vec![(ordinal % 251) as u8; 100])
        .collect::<Vec<_>>();
    let published = serving
        .records_mut()
        .append_batch(
            RecordAppendBatch::try_from_iter(payloads.iter()).unwrap(),
            placement,
        )
        .unwrap();
    let last = published.record_id(usize::from(record_count - 1)).unwrap();
    let locator = ExternalPhysicalRecordLocator::new(serving.store_identity(), last);
    serving.close();

    let changed_access = access(format, 7);
    let opening = media(&root);
    let before = opening.media_counters();
    let serving =
        success(opening.open_record_store(PhysicalRecordOpen::new(format, changed_access)));
    let after = serving.media_counters();
    assert_eq!(
        serving
            .records()
            .readmit_locator(locator)
            .into_result()
            .unwrap(),
        last
    );
    let point = serving
        .records()
        .open(
            last,
            RecordReadLimits::new(RecordByteLimit::new(100).unwrap()),
        )
        .unwrap()
        .observation();
    let scan = complete_scan(&serving, 7, 16_384);
    let offline = worth_store_offline_verifier::walk_current_durable_record_manifest(
        &root,
        format.declaration(),
    )
    .unwrap();
    assert_eq!(offline.placements().len(), usize::from(record_count));
    assert_canonical_parity(&serving, &offline);
    let runtime_process = super::scenario_evidence::ScenarioProcessEvidence::current_runtime(
        "scale-reopener",
        &serving,
    );
    let runtime_root_generation = serving
        .observer()
        .acquisition_snapshot()
        .unwrap()
        .root_generation();
    serving.close();
    let allocation_stdout = super::child_process::run_child(
        "scale_allocation_reader",
        &root,
        Some(&super::child_process::hex(&locator.encode())),
    );
    let allocation_process = super::scenario_evidence::ScenarioProcessEvidence::parse_child(
        &allocation_stdout,
        "scale-allocation-probe",
    );
    let (point_allocations, scan_allocations) = scale_allocations(&allocation_stdout);
    let observation = ScaleObservation {
        record_count,
        routing_level: offline.routing_level().unwrap(),
        whole_blocks: offline.manifest_blocks(),
        point_blocks: point.manifest_blocks(),
        point_comparisons: point.manifest_comparisons(),
        open_reads: after
            .completed_operations_for(MediaOperationRole::PositionedRead)
            .saturating_sub(before.completed_operations_for(MediaOperationRole::PositionedRead)),
        open_bytes: after
            .completed_bytes_for(MediaOperationRole::PositionedRead)
            .saturating_sub(before.completed_bytes_for(MediaOperationRole::PositionedRead)),
        scan_records: scan.records(),
        scan_payload_bytes: scan.payload_bytes(),
        point_allocations,
        scan_allocations,
        invalid_worlds: 0,
    };
    let world = format!("records-{record_count}");
    let processes = [runtime_process, allocation_process];
    let invalid = super::scale_invalid_worlds::exercise(&root, format, placement, changed_access);
    assert!(
        invalid.missing_catalog_refused,
        "C5_PREDICATE:current-truth missing catalog was guessed"
    );
    assert!(
        invalid.checksum_damage_refused,
        "checksum damage was admitted"
    );
    assert!(
        invalid.stale_manifest_refused,
        "stale manifest was admitted"
    );
    assert!(invalid.format_drift_refused, "format drift was admitted");
    assert!(
        invalid.residue_excluded,
        "unpublished residue was treated as current"
    );
    let invalid_worlds = [
        invalid.missing_catalog_refused,
        invalid.checksum_damage_refused,
        invalid.stale_manifest_refused,
        invalid.format_drift_refused,
        invalid.residue_excluded,
    ]
    .into_iter()
    .filter(|passed| *passed)
    .count() as u8;
    assert_eq!(invalid_worlds, 5);
    let observation = ScaleObservation {
        invalid_worlds,
        ..observation
    };
    let predicates = [
        ScenarioPredicate::equality(
            "locator_readmitted",
            last.ordinal(),
            locator_record_ordinal(locator),
        ),
        ScenarioPredicate::equality(
            "runtime_offline_record_count",
            u64::from(record_count),
            offline.placements().len() as u64,
        ),
        ScenarioPredicate::equality(
            "scan_record_count",
            u64::from(record_count),
            observation.scan_records,
        ),
        ScenarioPredicate::equality(
            "bounded_point_path",
            true,
            observation.point_blocks <= observation.whole_blocks,
        ),
        ScenarioPredicate::equality(
            "point_allocation_contract",
            true,
            observation.point_allocations == 16_384,
        ),
        ScenarioPredicate::equality(
            "scan_allocation_contract",
            true,
            observation.scan_allocations >= observation.point_allocations
                && observation.scan_allocations < 65_536,
        ),
        ScenarioPredicate::equality(
            "invalid_world_localization",
            5_u64,
            u64::from(observation.invalid_worlds),
        ),
        ScenarioPredicate::equality(
            "missing_catalog_refused",
            true,
            invalid.missing_catalog_refused,
        ),
        ScenarioPredicate::equality(
            "checksum_damage_refused",
            true,
            invalid.checksum_damage_refused,
        ),
        ScenarioPredicate::equality(
            "stale_manifest_refused",
            true,
            invalid.stale_manifest_refused,
        ),
        ScenarioPredicate::equality("format_drift_refused", true, invalid.format_drift_refused),
        ScenarioPredicate::equality(
            "unpublished_residue_excluded",
            true,
            invalid.residue_excluded,
        ),
    ];
    super::scenario_evidence::emit(super::scenario_evidence::ScenarioEvidence {
        courtroom: "bounded_scale_identity_format_and_policy_courtroom",
        world: &world,
        root: &root,
        seed: 0xC5C5_0000_0000_0001,
        action_trace: &[
            "initialize",
            "append",
            "close",
            "reopen",
            "locate",
            "scan",
            "offline-walk",
        ],
        authority_transitions: &[
            "absent-to-initialized",
            "batch-to-published-root",
            "fresh-process-readmission",
            "locator-readmission",
            "bounded-read-and-scan",
        ],
        walk: &offline,
        placement,
        publication_identity: Some(published.publication_identity()),
        processes: &processes,
        counters: serde_json::json!({
            "open_reads": observation.open_reads,
            "open_bytes": observation.open_bytes,
            "point_blocks": observation.point_blocks,
            "point_comparisons": observation.point_comparisons,
            "whole_blocks": observation.whole_blocks,
            "point_allocations": observation.point_allocations,
            "scan_allocations": observation.scan_allocations,
            "invalid_worlds": observation.invalid_worlds,
        }),
        runtime_result: serde_json::json!({
            "root_generation": runtime_root_generation,
            "records": observation.scan_records,
            "point_manifest_blocks": observation.point_blocks,
        }),
        oracle_result: serde_json::json!({
            "records": record_count,
            "payload_bytes": u64::from(record_count) * 100,
            "maximum_point_blocks": observation.whole_blocks,
            "invalid_worlds": {
                "missing_catalog_refused": true,
                "checksum_damage_refused": true,
                "stale_manifest_refused": true,
                "format_drift_refused": true,
                "unpublished_residue_excluded": true,
            },
        }),
        mutant_posture: "production-control",
        predicates: &predicates,
    });
    observation
}

fn locator_record_ordinal(locator: ExternalPhysicalRecordLocator) -> u64 {
    u64::from_le_bytes(locator.encode()[32..40].try_into().unwrap())
}

fn scale_allocations(stdout: &str) -> (usize, usize) {
    let fields = stdout
        .lines()
        .find_map(|line| line.strip_prefix("C5_SCALE_ALLOC "))
        .expect("the isolated allocation probe must emit its exact observation")
        .split_whitespace()
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 2);
    (fields[0].parse().unwrap(), fields[1].parse().unwrap())
}
