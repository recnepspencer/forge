use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_formal_models::{
    map_quarantine_readmission_outcome, map_quarantine_record, QuarantineReadmissionState,
};
use worth_store_layout_indexes::integrity::{
    layout_readmission, RecoveryLayoutReadmissionOutcomeView,
};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReferenceScope,
    PhysicalSegmentId,
};
use worth_store_physical_integrity::{
    ExecutedQuarantineFinding, PhysicalQuarantineAuthority, QuarantineHandoffPosture,
    QuarantineSealRequest,
};
use worth_store_test_support::harness::layout::{
    authoritative_layout_quarantine_record, layout_integrity_authority,
    unresolved_layout_authority_record,
};

pub(in crate::courtroom::protocol_models) fn execute_ordinary_quarantine_entry(
) -> Vec<QuarantineReadmissionState> {
    execute_ordinary_quarantine_entry_traces()
        .into_iter()
        .flatten()
        .collect()
}

pub(in crate::courtroom::protocol_models) fn execute_ordinary_quarantine_entry_traces(
) -> Vec<Vec<QuarantineReadmissionState>> {
    let fixture = layout_integrity_authority("protocol-quarantine-readmission");
    let record = authoritative_layout_quarantine_record("protocol-quarantine-readmission");
    let readmitted = layout_readmission().admit_quarantine(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &record,
        fixture.current_authority(),
        fixture.security_scope().witnesses(),
    );
    let readmitted_trace = map_quarantine_record(&record)
        .states()
        .chain(map_quarantine_readmission_outcome(readmitted.view()).states())
        .collect();

    let denied_record = unresolved_layout_authority_record("protocol-quarantine-wrong-class");
    let denied = layout_readmission().admit_quarantine(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &denied_record,
        fixture.current_authority(),
        fixture.security_scope().witnesses(),
    );
    let denied_trace = map_quarantine_record(&denied_record)
        .states()
        .chain(map_quarantine_readmission_outcome(denied.view()).states())
        .collect();

    let retained_record = PhysicalQuarantineAuthority::seal(
        QuarantineSealRequest::from_executed_finding(
            ExecutedQuarantineFinding::authoritative_quarantine(reference_scope()),
        )
        .with_handoff_posture(QuarantineHandoffPosture::AuditRetentionOwnerRequired),
    )
    .unwrap();
    let retained = layout_readmission().admit_quarantine(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &retained_record,
        fixture.current_authority(),
        fixture.security_scope().witnesses(),
    );
    let retained_trace = map_quarantine_record(&retained_record)
        .states()
        .chain(map_quarantine_readmission_outcome(retained.view()).states())
        .collect();
    vec![readmitted_trace, denied_trace, retained_trace]
}

pub(in crate::courtroom::protocol_models) fn replay_unverified_readmission_guard(
    seed: u64,
) -> Vec<QuarantineReadmissionState> {
    let label = format!("protocol-quarantine-counterexample-{seed}");
    let fixture = layout_integrity_authority(&label);
    let record = unresolved_layout_authority_record(&label);
    let denied = layout_readmission().admit_quarantine(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &record,
        fixture.current_authority(),
        fixture.security_scope().witnesses(),
    );
    assert!(matches!(
        denied.view(),
        RecoveryLayoutReadmissionOutcomeView::Denied(_)
    ));
    map_quarantine_readmission_outcome(denied.view())
        .states()
        .collect()
}

fn reference_scope() -> PhysicalReferenceScope {
    PhysicalReferenceScope::derived_index(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .page_cell(
                PhysicalSegmentId::from_raw(41).unwrap(),
                PhysicalPageId::from_raw(43).unwrap(),
            )
            .with_page_generation(PhysicalGeneration::from_raw(47).unwrap()),
    )
}
