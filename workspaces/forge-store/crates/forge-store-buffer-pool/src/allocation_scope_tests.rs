use crate::{
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
        .foreground(budget(64))
        .maintenance(budget(32))
        .recovery(budget(48))
        .scrub(budget(16))
        .import_export(budget(96))
        .streaming(budget(128))
        .fixed_metadata(fixed(8))
        .seal()
        .unwrap();
    AllocationAdmission::from_declaration(envelopes)
}

#[test]
fn allocation_envelopes_are_separately_admitted_and_counted() {
    let mut admission = admission();
    let requests = [
        AllocationRequest::copied_payload(AllocationScope::Foreground, 4).unwrap(),
        AllocationRequest::rich_diagnostics(AllocationScope::Maintenance, 5).unwrap(),
        AllocationRequest::background_work_memory(AllocationScope::Recovery, 6).unwrap(),
        AllocationRequest::materialized_record_set(AllocationScope::Scrub, 7).unwrap(),
        AllocationRequest::copied_payload(AllocationScope::ImportExport, 8).unwrap(),
        AllocationRequest::streaming_window(AllocationScope::Streaming, 9).unwrap(),
    ];

    for request in requests {
        let grant = admission.admit(request).unwrap();
        let receipt = admission.record_allocation(grant).unwrap();
        assert_eq!(receipt.scope(), request.scope());
    }

    let counters = admission.counters();
    assert_eq!(
        counters
            .scope(AllocationScope::Foreground)
            .allocated_bytes(),
        4
    );
    assert_eq!(
        counters
            .scope(AllocationScope::Maintenance)
            .allocated_bytes(),
        5
    );
    assert_eq!(
        counters.scope(AllocationScope::Recovery).allocated_bytes(),
        6
    );
    assert_eq!(counters.scope(AllocationScope::Scrub).allocated_bytes(), 7);
    assert_eq!(
        counters
            .scope(AllocationScope::ImportExport)
            .allocated_bytes(),
        8
    );
    assert_eq!(
        counters.scope(AllocationScope::Streaming).allocated_bytes(),
        9
    );
    assert_eq!(
        counters.scope(AllocationScope::Foreground).copied_bytes(),
        4
    );
    assert_eq!(
        counters.scope(AllocationScope::ImportExport).copied_bytes(),
        8
    );
}

#[test]
fn allocation_grant_cannot_be_spent_by_different_admission_authority() {
    let mut admitting_authority = admission();
    let mut unrelated_authority = admission();
    let grant = admitting_authority
        .admit(AllocationRequest::copied_payload(AllocationScope::Foreground, 4).unwrap())
        .unwrap();

    let denial = unrelated_authority.record_allocation(grant).unwrap_err();

    assert_eq!(
        denial,
        AllocationDenial::GrantAuthorityMismatch {
            scope: AllocationScope::Foreground,
            bytes: 4,
        }
    );
    assert_eq!(
        unrelated_authority
            .counters()
            .scope(AllocationScope::Foreground)
            .allocated_bytes(),
        0
    );
    assert_eq!(
        unrelated_authority
            .counters()
            .scope(AllocationScope::Foreground)
            .denied_bytes(),
        4
    );
}

#[test]
fn maintenance_and_scrub_cannot_steal_foreground_envelope() {
    let mut admission = admission();

    let maintenance =
        AllocationRequest::background_work_memory(AllocationScope::Maintenance, 40).unwrap();
    let denied = admission.admit(maintenance).unwrap_err();
    assert!(matches!(
        denied,
        AllocationDenial::EnvelopeExceeded {
            scope: AllocationScope::Maintenance,
            requested_bytes: 40,
            remaining_bytes: 32,
            ..
        }
    ));

    let scrub = AllocationRequest::rich_diagnostics(AllocationScope::Scrub, 17).unwrap();
    let denied = admission.admit(scrub).unwrap_err();
    assert!(matches!(
        denied,
        AllocationDenial::EnvelopeExceeded {
            scope: AllocationScope::Scrub,
            requested_bytes: 17,
            remaining_bytes: 16,
            ..
        }
    ));
    assert_eq!(admission.remaining(AllocationScope::Foreground), 64);
    assert_eq!(
        admission
            .counters()
            .scope(AllocationScope::Maintenance)
            .allocated_bytes(),
        0
    );
}

#[test]
fn unbounded_and_over_budget_requests_deny_before_allocation() {
    let mut admission = admission();

    let unbounded = AllocationRequest::unbounded_buffer(AllocationScope::Foreground);
    assert_eq!(
        admission.admit(unbounded).unwrap_err(),
        AllocationDenial::UnboundedRequest {
            scope: AllocationScope::Foreground
        }
    );

    let diagnostics =
        AllocationRequest::rich_diagnostics(AllocationScope::Maintenance, 33).unwrap();
    let denied = admission.admit(diagnostics).unwrap_err();
    assert!(matches!(
        denied,
        AllocationDenial::EnvelopeExceeded {
            scope: AllocationScope::Maintenance,
            kind: AllocationRequestKind::RichDiagnostics,
            requested_bytes: 33,
            remaining_bytes: 32,
        }
    ));
    let counters = admission.counters();
    assert_eq!(
        counters.scope(AllocationScope::Foreground).denial_count(),
        1
    );
    assert_eq!(
        counters
            .scope(AllocationScope::Maintenance)
            .allocated_bytes(),
        0
    );
    assert_eq!(
        counters.scope(AllocationScope::Maintenance).denied_bytes(),
        33
    );
    assert_eq!(
        counters.scope(AllocationScope::Maintenance).denial_count(),
        1
    );
}

#[test]
fn fixed_metadata_exemption_is_explicit_counted_and_constant() {
    let mut admission = admission();
    let grant = admission.admit_fixed_metadata(fixed(4)).unwrap();

    assert_eq!(grant.bytes(), 4);
    assert_eq!(
        grant.constant_size_at_scale(1_000, 10, 10, 10),
        grant.constant_size_at_scale(1_000_000, 10_000, 100_000, 200_000)
    );
    let counters = admission.counters();
    assert_eq!(counters.fixed_metadata_bytes(), 4);
    assert_eq!(counters.fixed_metadata_exemption_count(), 1);
}

#[test]
fn variable_requests_cannot_use_fixed_metadata_exemption() {
    let mut admission = admission();
    let request = AllocationRequest::rich_diagnostics(AllocationScope::Foreground, 4).unwrap();
    let denial = admission.reject_fixed_metadata_for_variable_request(request);

    assert_eq!(
        denial,
        AllocationDenial::VariableAllocationCannotUseFixedMetadata {
            scope: AllocationScope::Foreground,
            kind: AllocationRequestKind::RichDiagnostics,
        }
    );
    assert_eq!(
        admission
            .counters()
            .scope(AllocationScope::Foreground)
            .denied_bytes(),
        4
    );
}
