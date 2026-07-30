use super::*;
use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    BoundedResidencyMediaRole, BoundedResidencySchedulerEvidenceClass,
    BoundedResidencySignalAspectRole, BoundedResidencySignalFamily,
    BoundedResidencySignalSettlement, BoundedResidencyWorkTerminalFate,
};

mod fixture;

use fixture::{fixture, FRAME, GENERATION, ROOT, RUNTIME, STORE};

#[test]
fn exact_trace_is_accepted_and_accounting_bypasses_are_rejected() {
    assert!(verify(&fixture(), STORE, RUNTIME, GENERATION).is_ok());
    assert_hostile(
        "physical work reconciliation causal evidence overflowed",
        |hostile| hostile.causal_overflow = 1,
    );
    assert_hostile(
        "physical work reconciliation terminal evidence overflowed",
        |hostile| hostile.terminal_overflow = 1,
    );
    assert_hostile(
        "physical work reconciliation terminal evidence was elided",
        |hostile| hostile.safe_evidence_elided = 1,
    );
}

#[test]
fn runtime_and_operation_identity_bypasses_are_rejected() {
    assert_hostile(
        "physical work reconciliation admitted a foreign runtime identity",
        |hostile| hostile.records[0].store = [8; 16],
    );
    assert_hostile(
        "physical work reconciliation admitted a foreign runtime identity",
        |hostile| hostile.records[0].runtime = RUNTIME + 1,
    );
    assert_hostile(
        "physical work reconciliation admitted a foreign runtime identity",
        |hostile| hostile.records[0].generation = GENERATION + 1,
    );
    assert_hostile(
        "physical work reconciliation duplicated a work identity",
        |hostile| hostile.records[1].operation = hostile.records[0].operation,
    );
    assert_hostile(
        "physical work reconciliation duplicated a backend receipt identity",
        |hostile| {
            hostile.records[1].backend_operation = hostile.records[0].backend_operation;
        },
    );
    assert_hostile(
        "physical work reconciliation duplicated a Signal attempt identity",
        |hostile| {
            hostile.records[1].route.signal_attempt = hostile.records[0].route.signal_attempt;
            hostile.records[1].route.signal = hostile.records[0].route.signal;
        },
    );
}

#[test]
fn route_and_backend_role_bypasses_are_rejected() {
    assert_hostile(
        "physical work reconciliation admitted an inexact causal route",
        |hostile| {
            hostile.records[0].route.signal_family = BoundedResidencySignalFamily::Publication;
        },
    );
    assert_hostile(
        "physical work reconciliation admitted an inexact backend media role",
        |hostile| hostile.records[2].backend_role = BoundedResidencyMediaRole::ReadMetadata,
    );
    assert_hostile(
        "physical work reconciliation admitted an inexact causal route",
        |hostile| hostile.records[0].route.signal_binding = [0; 32],
    );
    assert_hostile(
        "physical work reconciliation admitted an inexact causal route",
        |hostile| {
            hostile.records[0].route.signal_settlement =
                BoundedResidencySignalSettlement::DerivedStateUnavailable;
        },
    );
    assert_hostile(
        "physical work reconciliation admitted an inexact causal route",
        |hostile| {
            hostile.records[0].route.scheduler_evidence_class =
                BoundedResidencySchedulerEvidenceClass::UnverifiableAssumption;
        },
    );
    assert_hostile(
        "physical work reconciliation selected an uninstalled Signal binding",
        |hostile| hostile.records[1].route.signal_binding = [8; 32],
    );
    assert_hostile(
        "physical work reconciliation selected the wrong native Signal basis",
        |hostile| hostile.records[0].route.signal_binding = FRAME,
    );
}

#[test]
fn installed_signal_inventory_bypasses_are_rejected() {
    assert_hostile(
        "physical work reconciliation duplicated an installed Signal binding digest",
        |hostile| hostile.signal_bindings[1].digest = ROOT,
    );
    assert_hostile(
        "physical work reconciliation changed native Signal basis `store.physical.record.root-read-basis`",
        |hostile| hostile.signal_bindings[0].role = BoundedResidencySignalAspectRole::Output,
    );
    assert_hostile(
        "physical work reconciliation changed native Signal basis `store.physical.record.root-read-basis`",
        |hostile| hostile.signal_bindings[0].families.lifecycle = true,
    );
    assert_hostile(
        "physical work reconciliation changed native Signal basis `store.physical.record.root-read-basis`",
        |hostile| hostile.signal_bindings[0].partition = None,
    );
    assert_hostile(
        "physical work reconciliation duplicated an installed Signal aspect identity",
        |hostile| {
            hostile.signal_bindings[1].aspect_key = hostile.signal_bindings[0].aspect_key.clone();
        },
    );
    assert_hostile(
        "physical work reconciliation omitted native Signal basis `store.physical.record.root-read-basis`",
        |hostile| {
            hostile.signal_bindings = hostile.signal_bindings[1..].to_vec().into_boxed_slice();
        },
    );
}

#[test]
fn effect_fate_and_receipt_count_bypasses_are_rejected() {
    assert_hostile(
        "physical work reconciliation admitted an inexact effect fate or recovery",
        |hostile| {
            hostile.records[1].effect_fate = BoundedResidencyWorkEffectFate::WriteCompleted;
        },
    );
    assert_hostile(
        "physical work reconciliation admitted an inexact effect fate or recovery",
        |hostile| hostile.records[4].recovery = BoundedResidencyWorkRecovery::NoEffect,
    );
    assert_hostile(
        "physical work reconciliation fault/source-load count drifted",
        |hostile| hostile.faults += 1,
    );
    assert_hostile(
        "physical work reconciliation fault/source-load count drifted",
        |hostile| hostile.source_loads += 1,
    );
    assert_hostile(
        "physical work reconciliation exact writeback receipt count drifted",
        |hostile| hostile.exact_writebacks += 1,
    );
    assert_hostile(
        "physical work reconciliation identified metadata-read topology drifted",
        |hostile| hostile.identified_metadata_reads += 1,
    );
    assert_hostile(
        "physical work reconciliation identified positioned-read topology drifted",
        |hostile| hostile.identified_positioned_reads += 1,
    );
    assert_hostile(
        "physical work reconciliation identified positioned-write topology drifted",
        |hostile| hostile.identified_positioned_writes += 1,
    );
}

#[test]
fn terminal_and_native_basis_coverage_bypasses_are_rejected() {
    assert_hostile(
        "physical work reconciliation settled terminal fate count drifted",
        |hostile| hostile.settled_terminal_fates += 1,
    );
    assert_hostile(
        "physical work reconciliation settled terminal fate count drifted",
        |hostile| {
            hostile.records[0].terminal =
                BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation;
        },
    );
    assert_hostile(
        "physical work reconciliation did not exercise every native Signal basis",
        |hostile| hostile.records = hostile.records[..5].to_vec().into_boxed_slice(),
    );
    assert_hostile(
        "physical work reconciliation did not exercise every native Signal basis",
        |hostile| hostile.records = hostile.records[..4].to_vec().into_boxed_slice(),
    );
    assert_hostile(
        "physical work reconciliation emitted no media-reaching records",
        |hostile| hostile.records = Vec::new().into_boxed_slice(),
    );
}

#[test]
fn distinct_native_read_bases_are_not_collapsed_to_one_family_binding() {
    if verify(&fixture(), STORE, RUNTIME, GENERATION).is_err() {
        panic!("MUTANT_PREDICATE:physical-work-read-basis-collapsed");
    }
}

#[test]
fn digest_retains_every_installed_signal_binding_field() {
    let exact = fixture();
    let expected = digest(&exact);

    let mut changed = exact.clone();
    changed.signal_bindings[0].digest = [9; 32];
    assert_ne!(digest(&changed), expected);

    let mut changed = exact.clone();
    changed.signal_bindings[0].aspect_key.push_str(".changed");
    assert_ne!(digest(&changed), expected);

    let mut changed = exact.clone();
    changed.signal_bindings[0].role = BoundedResidencySignalAspectRole::Output;
    assert_ne!(digest(&changed), expected);

    let mut changed = exact.clone();
    changed.signal_bindings[0].families.lifecycle = true;
    assert_ne!(digest(&changed), expected);

    let mut changed = exact;
    changed.signal_bindings[0].partition = None;
    assert_ne!(digest(&changed), expected);
}

fn assert_denied(evidence: &BoundedResidencyWorkReconciliationObservation, expected: &str) {
    assert_eq!(
        verify(evidence, STORE, RUNTIME, GENERATION).unwrap_err(),
        expected
    );
}

fn assert_hostile(
    expected: &str,
    mutate: impl FnOnce(&mut BoundedResidencyWorkReconciliationObservation),
) {
    let mut hostile = fixture();
    mutate(&mut hostile);
    assert_denied(&hostile, expected);
}
