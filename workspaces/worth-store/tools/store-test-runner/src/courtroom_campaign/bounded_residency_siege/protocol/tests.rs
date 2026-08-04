use std::num::NonZeroU32;

use super::decoding::{
    parse_allocation, parse_dirty, parse_pinned_eviction, parse_process_allocation, parse_reads,
    parse_speculation, parse_work_reconciliation,
};
use super::{
    BoundedResidencyMediaRole, BoundedResidencySignalAspectRole, BoundedResidencyWorkEffectFate,
    BoundedResidencyWorkFamily, BoundedResidencyWorkRecovery, BoundedResidencyWorkTerminalFate,
};

const VALID_DIRTY_MARKER: &str = "\
BOUNDED_RESIDENCY_DIRTY 501 502 1 1 3 3 1 601 602 1 1 1 2 1 0 0 1 1 1 0 1 1 1 true true true \
1 true 3 2 1 2 3 2 0 0 0 7 2 2";
const VALID_READ_MARKER: &str = "\
BOUNDED_RESIDENCY_READS 2 0 2 3 0 3 3 0 3 10 4 10 0 0 6 0 0 4 11 20 true \
65536 128000 4 4 1 1 10 1000 10 1000 64 64 128 1024";
const VALID_PINNED_EVICTION_MARKER: &str = "BOUNDED_RESIDENCY_PINNED_EVICTION 9 3 3 3 3 true";
const VALID_SPECULATION_MARKERS: [&str; 3] = [
    "BOUNDED_RESIDENCY_PREFETCH 5 4 1 4 2 0 1 3 0 0 3 true true",
    "BOUNDED_RESIDENCY_READ_AHEAD 3 2 1 2 2 0 1 3 0 0 3 true true",
    "BOUNDED_RESIDENCY_WRITE_BEHIND 3 2 1 2 1 0 0 2 0 0 2 true true",
];
const VALID_WORK_RECONCILIATION_MARKERS: [&str; 16] = [
    "BOUNDED_RESIDENCY_WORK_RECONCILIATION 0 0 0 1 1 1 1 1 1 3 1 7 4",
    "BOUNDED_RESIDENCY_SIGNAL_BINDING 0101010101010101010101010101010101010101010101010101010101010101 store.physical.record.root-read-basis dependency true false false false false false false false false store.physical.record.root",
    "BOUNDED_RESIDENCY_SIGNAL_BINDING 0202020202020202020202020202020202020202020202020202020202020202 store.physical.record.artifact-read-basis dependency true false false false false false false false false store.physical.record.artifact",
    "BOUNDED_RESIDENCY_SIGNAL_BINDING 0303030303030303030303030303030303030303030303030303030303030303 store.physical.record.frame-read-basis dependency true false false false false false false false false store.physical.record.frame",
    "BOUNDED_RESIDENCY_SIGNAL_BINDING 0404040404040404040404040404040404040404040404040404040404040404 store.physical.record.scan-read-basis dependency true false false false false false false false false store.physical.record.scan",
    "BOUNDED_RESIDENCY_SIGNAL_BINDING 0505050505050505050505050505050505050505050505050505050505050505 store.physical.record.frame-writeback-basis dependency-and-output false true false false false false false false false none",
    "BOUNDED_RESIDENCY_SIGNAL_BINDING 0606060606060606060606060606060606060606060606060606060606060606 store.physical.record.publication-basis dependency-and-output false false true false false false false false false none",
    "BOUNDED_RESIDENCY_SIGNAL_BINDING 0707070707070707070707070707070707070707070707070707070707070707 store.physical.durability.policy-binding-basis dependency false false false false true true true true true physical-durability-policy/0707",
    "BOUNDED_RESIDENCY_WORK_RECORD 09090909090909090909090909090909 11 13 1 artifact-metadata-read 101 read-metadata read-completed no-effect settled",
    "BOUNDED_RESIDENCY_WORK_ROUTE 1 0:0:0:0 none 0 read-fault 0101010101010101010101010101010101010101010101010101010101010101 posix-file-fsync-dir-sync established-by-filesystem-admission 1 1 false committed",
    "BOUNDED_RESIDENCY_WORK_RECORD 09090909090909090909090909090909 11 13 2 artifact-range-read 102 positioned-read read-completed no-effect settled",
    "BOUNDED_RESIDENCY_WORK_ROUTE 2 102:3:1:0 none 202 read-fault 0303030303030303030303030303030303030303030303030303030303030303 posix-file-fsync-dir-sync established-by-filesystem-admission 1 1 false committed",
    "BOUNDED_RESIDENCY_WORK_RECORD 09090909090909090909090909090909 11 13 3 artifact-range-write 103 positioned-write write-completed continue-settlement continued-after-consumer-cancellation",
    "BOUNDED_RESIDENCY_WORK_ROUTE 3 103:3:1:0 none 203 exact-writeback 0505050505050505050505050505050505050505050505050505050505050505 posix-file-fsync-dir-sync established-by-filesystem-admission 1 1 false committed",
    "BOUNDED_RESIDENCY_WORK_RECORD 09090909090909090909090909090909 11 13 4 artifact-publication 104 synchronize-file-state publication-completed continue-settlement settled",
    "BOUNDED_RESIDENCY_WORK_ROUTE 4 104:3:1:0 none 204 publication 0606060606060606060606060606060606060606060606060606060606060606 posix-file-fsync-dir-sync established-by-filesystem-admission 1 1 false committed",
];

#[test]
fn process_allocation_marker_preserves_request_semantics_and_process_identity() {
    let process = NonZeroU32::new(41).unwrap();
    let marker = "BOUNDED_RESIDENCY_PROCESS_ALLOCATION 41 8388608".to_owned();
    let parsed = parse_process_allocation(std::slice::from_ref(&marker), process).unwrap();

    assert_eq!(parsed.process, process);
    assert_eq!(parsed.largest_successful_request_bytes, 8_388_608);
    assert!(parse_process_allocation(&[format!("{marker} 1")], process).is_err());
    assert!(parse_process_allocation(&[marker.replace(" 41 ", " 42 ")], process).is_err());
}

#[test]
fn dirty_marker_preserves_ordinary_append_pressure_coordinates() {
    let parsed = parse_dirty(&[VALID_DIRTY_MARKER.to_owned()]).unwrap();

    assert_eq!(parsed.primary_publication, 501);
    assert_eq!(parsed.retry_publication, 502);
    assert_eq!(parsed.primary_last_candidate_operation, 601);
    assert_eq!(parsed.retry_last_candidate_operation, 602);
    assert_eq!(parsed.primary_candidate_publications, 3);
    assert_eq!(parsed.retry_candidate_publications, 3);
    assert_eq!(parsed.denied_candidate_publications, 1);
    assert_eq!(parsed.dirty_peak, 2);
    assert_eq!(parsed.pressure_limit, 1);
    assert!(parsed.pressure_basis_exact);
    assert!(parsed.pressure_retry_after_settlement);
    assert!(parsed.pressure_effect_free);
    assert_eq!(parsed.cleanup_deletions, 1);
    assert!(parsed.cleanup_complete);
    assert_eq!(parsed.candidate_publications, 7);

    assert!(parse_dirty(&[format!("{VALID_DIRTY_MARKER} 1")]).is_err());
    let missing = VALID_DIRTY_MARKER
        .split_whitespace()
        .take(40)
        .collect::<Vec<_>>()
        .join(" ");
    assert!(parse_dirty(&[missing]).is_err());
    assert!(parse_dirty(&[VALID_DIRTY_MARKER.replacen("true", "unknown", 1)]).is_err());
}

#[test]
fn read_marker_preserves_the_complete_bounded_copy_contract() {
    let parsed = parse_reads(&[VALID_READ_MARKER.to_owned()]).unwrap();
    assert_eq!(parsed.caller_copy_operations, 10);
    assert_eq!(parsed.caller_copied_bytes, 1_000);
    assert_eq!(parsed.store_copy_operations, 10);
    assert_eq!(parsed.store_copied_bytes, 1_000);
    assert_eq!(parsed.peak_copy_width, 64);
    assert_eq!(parsed.store_maximum_copy_width, 64);
    assert_eq!(parsed.streaming_scratch_bytes, 128);
    assert_eq!(parsed.largest_record_bytes, 1_024);

    assert!(parse_reads(&[format!("{VALID_READ_MARKER} 1")]).is_err());
    let missing = VALID_READ_MARKER
        .split_whitespace()
        .take(35)
        .collect::<Vec<_>>()
        .join(" ");
    assert!(parse_reads(&[missing]).is_err());
}

#[test]
fn pinned_eviction_marker_preserves_every_protected_authority_fact() {
    let parsed = parse_pinned_eviction(&[VALID_PINNED_EVICTION_MARKER.to_owned()]).unwrap();
    assert_eq!(parsed.forced_evictions, 9);
    assert_eq!(parsed.pinned_frames_before, 3);
    assert_eq!(parsed.pinned_frames_after, 3);
    assert_eq!(parsed.pin_leases_before, 3);
    assert_eq!(parsed.pin_leases_after, 3);
    assert!(parsed.bases_preserved);

    assert!(parse_pinned_eviction(&[format!("{VALID_PINNED_EVICTION_MARKER} 1")]).is_err());
    assert!(
        parse_pinned_eviction(&["BOUNDED_RESIDENCY_PINNED_EVICTION 9 3 3 3 true".to_owned()])
            .is_err()
    );
    assert!(
        parse_pinned_eviction(&[VALID_PINNED_EVICTION_MARKER.replace("true", "unknown")]).is_err()
    );
}

#[test]
fn speculative_markers_preserve_kind_counters_signal_absence_and_exact_basis() {
    let markers = VALID_SPECULATION_MARKERS.map(str::to_owned);
    let parsed = parse_speculation(&markers).unwrap();

    assert_eq!(
        (parsed.prefetch.attempts, parsed.prefetch.peak_frames),
        (5, 2)
    );
    assert_eq!(
        (
            parsed.read_ahead.hits,
            parsed.read_ahead.effectful_signal_requests,
        ),
        (1, 3)
    );
    assert_eq!(
        (
            parsed.write_behind.denial_signal_requests,
            parsed.write_behind.effectful_signal_requests,
        ),
        (0, 2)
    );
    assert!(parsed.prefetch.signal_family_exact);
    assert!(parsed.read_ahead.foundational_basis_exact);

    let mut malformed = markers.clone();
    malformed[0].push_str(" 1");
    assert!(parse_speculation(&malformed).is_err());
    let missing = [markers[0].clone(), markers[1].clone()];
    assert!(parse_speculation(&missing).is_err());
}

#[test]
fn work_reconciliation_protocol_preserves_each_raw_causal_and_terminal_field() {
    let markers = VALID_WORK_RECONCILIATION_MARKERS.map(str::to_owned);
    let parsed = parse_work_reconciliation(&markers).unwrap();

    assert_eq!((parsed.faults, parsed.source_loads), (1, 1));
    assert_eq!(parsed.exact_writebacks, 1);
    assert_eq!(parsed.identified_metadata_reads, 1);
    assert_eq!(parsed.identified_positioned_reads, 1);
    assert_eq!(parsed.identified_positioned_writes, 1);
    assert_eq!(parsed.settled_terminal_fates, 3);
    assert_eq!(parsed.continued_terminal_fates, 1);
    assert_eq!(parsed.signal_bindings.len(), 7);
    assert_eq!(
        parsed.signal_bindings[0].aspect_key,
        "store.physical.record.root-read-basis"
    );
    assert_eq!(parsed.signal_bindings[0].digest, [1; 32]);
    assert_eq!(
        parsed.signal_bindings[0].role,
        BoundedResidencySignalAspectRole::Dependency
    );
    assert_eq!(
        parsed.signal_bindings[0].partition.as_deref(),
        Some("store.physical.record.root")
    );
    assert!(parsed.signal_bindings[0].families.read_fault);
    assert!(!parsed.signal_bindings[0].families.exact_writeback);
    assert!(!parsed.signal_bindings[0].families.publication);
    assert!(!parsed.signal_bindings[0].families.lifecycle);
    assert!(!parsed.signal_bindings[0].families.wal_append);
    assert!(!parsed.signal_bindings[0].families.durability_barrier);
    assert!(!parsed.signal_bindings[0].families.checkpoint_capture);
    assert!(!parsed.signal_bindings[0].families.root_publication);
    assert!(!parsed.signal_bindings[0].families.wal_reclamation);
    assert_eq!(
        parsed.signal_bindings[4].role,
        BoundedResidencySignalAspectRole::DependencyAndOutput
    );
    assert!(parsed.signal_bindings[4].partition.is_none());
    assert!(parsed.signal_bindings[4].families.exact_writeback);
    assert!(parsed.signal_bindings[5].families.publication);
    assert!(parsed.signal_bindings[6].families.wal_append);
    assert!(parsed.signal_bindings[6].families.durability_barrier);
    assert!(parsed.signal_bindings[6].families.checkpoint_capture);
    assert!(parsed.signal_bindings[6].families.root_publication);
    assert!(parsed.signal_bindings[6].families.wal_reclamation);
    assert_eq!(parsed.records.len(), 4);
    assert_eq!(
        parsed.records[0].family,
        BoundedResidencyWorkFamily::ArtifactMetadataRead
    );
    assert_eq!(parsed.records[0].route.signal.request, 0);
    assert_eq!(parsed.records[0].route.signal.generation, 0);
    assert_eq!(parsed.records[0].route.signal_attempt, 0);
    assert_eq!(parsed.records[1].backend_operation, 102);
    assert_eq!(
        parsed.records[1].backend_role,
        BoundedResidencyMediaRole::PositionedRead
    );
    assert_eq!(
        parsed.records[2].effect_fate,
        BoundedResidencyWorkEffectFate::WriteCompleted
    );
    assert_eq!(
        parsed.records[2].recovery,
        BoundedResidencyWorkRecovery::ContinueSettlement
    );
    assert_eq!(
        parsed.records[2].terminal,
        BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation
    );

    let missing = markers[..14].to_vec();
    assert!(parse_work_reconciliation(&missing).is_err());
    let missing_binding = markers
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != 1)
        .map(|(_, marker)| marker.clone())
        .collect::<Vec<_>>();
    assert!(parse_work_reconciliation(&missing_binding)
        .unwrap_err()
        .contains("declared 7 Signal bindings but emitted 6"));
    let mut malformed = markers.clone();
    malformed[1].push_str(" extra");
    assert!(parse_work_reconciliation(&malformed).is_err());
    let foreign_family = markers.map(|marker| marker.replace("artifact-range-read", "record-read"));
    assert!(parse_work_reconciliation(&foreign_family).is_err());
}

#[test]
fn allocation_protocol_requires_one_scope_and_each_named_dimension() {
    let mut markers =
        vec!["BOUNDED_RESIDENCY_SCOPES 7 7 true 1 4194304 4194304 4194304 0 true".to_owned()];
    for name in [
        "total-bytes",
        "resident-bytes",
        "metadata-bytes",
        "frame-entries",
        "pinned-frames",
        "pin-leases",
        "dirty-frames",
        "dirty-replacement-bytes",
        "operation-bytes",
        "scope-foreground-read",
        "scope-foreground-write",
        "scope-recovery",
        "scope-scrub",
        "scope-maintenance",
        "scope-verification",
        "scope-blob",
        "speculative-read-ahead",
        "speculative-prefetch",
        "speculative-write-behind",
    ] {
        markers.push(format!(
            "BOUNDED_RESIDENCY_ALLOCATION {name} 1 1 1 0 0 1 1 0 0 0 1 1"
        ));
    }
    markers.push(
        "BOUNDED_RESIDENCY_ALLOCATION_TRACE 11111111111111111111111111111111 7 1 41 1".to_owned(),
    );
    markers.push(
        "BOUNDED_RESIDENCY_ALLOCATION_EVENT 1 actualization resident-bytes \
         foreground-read 8 7 41 3"
            .to_owned(),
    );
    assert!(parse_allocation(&markers).is_ok());

    let missing = markers
        .iter()
        .filter(|marker| {
            !marker.starts_with("BOUNDED_RESIDENCY_ALLOCATION speculative-write-behind")
        })
        .cloned()
        .collect::<Vec<_>>();
    assert!(parse_allocation(&missing).is_err());
    markers.push("BOUNDED_RESIDENCY_ALLOCATION foreign 0 0 0 0 0 0 0 0 0 0 0 1".to_owned());
    assert!(parse_allocation(&markers[1..]).is_err());
}
