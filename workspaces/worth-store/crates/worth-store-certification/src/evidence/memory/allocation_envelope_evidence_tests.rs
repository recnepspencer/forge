use crate::{AllocationEnvelopeEvidenceReport, AllocationEnvelopeEvidenceRow};
use worth_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationDenial, AllocationEnvelopeDeclaration,
    AllocationRequest, AllocationRequestKind, AllocationScope, FixedMetadataReservation,
};

fn budget(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}

fn fixed(bytes: u64) -> FixedMetadataReservation {
    FixedMetadataReservation::constant_bytes(bytes).unwrap()
}

fn admission() -> AllocationAdmission {
    let envelopes = AllocationEnvelopeDeclaration::declare()
        .foreground(budget(32))
        .maintenance(budget(32))
        .recovery(budget(32))
        .scrub(budget(32))
        .import_export(budget(32))
        .streaming(budget(32))
        .fixed_metadata(fixed(8))
        .seal()
        .unwrap();
    AllocationAdmission::from_declaration(envelopes)
}

#[test]
fn allocation_envelope_evidence_requires_all_scope_counters() {
    let mut admission = admission();
    for scope in AllocationScope::ALL {
        let grant = admission
            .admit(AllocationRequest::background_work_memory(scope, 4).unwrap())
            .unwrap();
        admission.record_allocation(grant).unwrap();
    }

    let report = AllocationEnvelopeEvidenceReport::from_admission(
        AllocationEnvelopeEvidenceRow::SeparateScopesAdmittedAndCounted,
        &admission,
    )
    .unwrap();
    assert_eq!(
        report.row(),
        AllocationEnvelopeEvidenceRow::SeparateScopesAdmittedAndCounted
    );
}

#[test]
fn allocation_denial_evidence_requires_no_materialization() {
    let mut admission = admission();
    let request = AllocationRequest::rich_diagnostics(AllocationScope::Maintenance, 33).unwrap();
    let denial = admission.admit(request).unwrap_err();

    let report = AllocationEnvelopeEvidenceReport::from_denial(
        AllocationEnvelopeEvidenceRow::AllocationDeniedBeforeMaterialization,
        denial,
        admission.counters(),
    )
    .unwrap();
    assert_eq!(
        report
            .counters()
            .scope(AllocationScope::Maintenance)
            .allocated_bytes(),
        0
    );
}

#[test]
fn unbounded_allocation_denial_evidence_uses_attempt_count_not_denied_bytes() {
    let mut admission = admission();
    let request = AllocationRequest::unbounded_buffer(AllocationScope::Foreground);
    let denial = admission.admit(request).unwrap_err();

    let report = AllocationEnvelopeEvidenceReport::from_denial(
        AllocationEnvelopeEvidenceRow::AllocationDeniedBeforeMaterialization,
        denial,
        admission.counters(),
    )
    .unwrap();

    let counters = report.counters().scope(AllocationScope::Foreground);
    assert_eq!(counters.denial_count(), 1);
    assert_eq!(counters.denied_bytes(), 0);
    assert_eq!(counters.allocated_bytes(), 0);
}

#[test]
fn background_denial_evidence_proves_foreground_envelope_was_not_spent() {
    let mut admission = admission();
    let request = AllocationRequest::materialized_record_set(AllocationScope::Scrub, 33).unwrap();
    let denial = admission.admit(request).unwrap_err();

    let report = AllocationEnvelopeEvidenceReport::from_denial(
        AllocationEnvelopeEvidenceRow::ForegroundEnvelopeNotStolenByBackground,
        denial,
        admission.counters(),
    )
    .unwrap();
    assert_eq!(
        report
            .counters()
            .scope(AllocationScope::Foreground)
            .admitted_bytes(),
        0
    );
}

#[test]
fn fixed_metadata_evidence_requires_constant_counted_grant() {
    let mut admission = admission();
    let grant = admission.admit_fixed_metadata(fixed(4)).unwrap();

    let report = AllocationEnvelopeEvidenceReport::from_fixed_metadata(&admission, &grant).unwrap();
    assert_eq!(
        report.row(),
        AllocationEnvelopeEvidenceRow::FixedMetadataExemptionConstantAndCounted
    );
}

#[test]
fn fixed_metadata_evidence_rejects_grant_from_unrelated_admission() {
    let mut owner = admission();
    let unrelated = admission();
    let grant = owner.admit_fixed_metadata(fixed(4)).unwrap();

    let denial =
        AllocationEnvelopeEvidenceReport::from_fixed_metadata(&unrelated, &grant).unwrap_err();

    assert_eq!(
        denial,
        crate::AllocationEnvelopeEvidenceDenial::UnprovenAllocationRow
    );
}

#[test]
fn certification_rejects_variable_allocation_as_fixed_metadata() {
    let mut admission = admission();
    let request = AllocationRequest::copied_payload(AllocationScope::Foreground, 4).unwrap();
    let denial = admission.reject_fixed_metadata_for_variable_request(request);

    assert_eq!(
        denial,
        AllocationDenial::VariableAllocationCannotUseFixedMetadata {
            scope: AllocationScope::Foreground,
            kind: AllocationRequestKind::CopiedPayload,
        }
    );
}
