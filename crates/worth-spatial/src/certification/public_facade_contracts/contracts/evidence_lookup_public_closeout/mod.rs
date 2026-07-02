use worth_spatial::facade::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout, EvidenceLookupPublicCloseout,
    EvidenceLookupPublicCloseoutCounters, EvidenceLookupPublicCloseoutDisposition,
    EvidenceLookupPublicCloseoutError, EvidenceLookupPublicCloseoutErrorKind,
    EvidenceLookupPublicCloseoutFamilyStageRow,
};
use worth_spatial::facade::evidence_lookup_workload_cutover::{
    EvidenceLookupMilestoneTwelveReplayReadinessPosture, EvidenceLookupMilestoneTwelveSeed,
};

#[test]
fn spatial_public_api_exports_lookup_public_closeout_contract() {
    let _: fn(&EvidenceLookupPublicCloseout) -> &[EvidenceLookupPublicCloseoutFamilyStageRow] =
        EvidenceLookupPublicCloseout::family_stage_rows;
    let _: fn(&EvidenceLookupPublicCloseout) -> &EvidenceLookupPublicCloseoutCounters =
        EvidenceLookupPublicCloseout::counters;
    let _: fn(&EvidenceLookupPublicCloseout) -> &str =
        EvidenceLookupPublicCloseout::family_coverage_digest;
    let _: fn(&EvidenceLookupPublicCloseout) -> &str =
        EvidenceLookupPublicCloseout::spatial_deletion_ledger_digest;
    let _: fn(&EvidenceLookupPublicCloseout) -> &str =
        EvidenceLookupPublicCloseout::residue_audit_digest;
    let _: fn(&EvidenceLookupPublicCloseout) -> &EvidenceLookupMilestoneTwelveSeed =
        EvidenceLookupPublicCloseout::milestone_twelve_seed;
    let _: fn(&EvidenceLookupPublicCloseout) -> &str =
        EvidenceLookupPublicCloseout::closeout_digest;
}

#[test]
fn spatial_public_api_exports_lookup_public_closeout_support_types() {
    let _: fn(&EvidenceLookupPublicCloseoutFamilyStageRow) -> &str =
        EvidenceLookupPublicCloseoutFamilyStageRow::family_identity;
    let _: fn(&EvidenceLookupPublicCloseoutFamilyStageRow) -> &str =
        EvidenceLookupPublicCloseoutFamilyStageRow::family_declaration_digest;
    let _: fn(&EvidenceLookupPublicCloseoutFamilyStageRow) -> &str =
        EvidenceLookupPublicCloseoutFamilyStageRow::stage_receipt_family_identity;
    let _: fn(&EvidenceLookupPublicCloseoutFamilyStageRow) -> Option<&str> =
        EvidenceLookupPublicCloseoutFamilyStageRow::spatial_touch_digest;
    let _: fn(&EvidenceLookupPublicCloseoutFamilyStageRow) -> &str =
        EvidenceLookupPublicCloseoutFamilyStageRow::topology_input_summary;
    let _: fn(&EvidenceLookupPublicCloseoutFamilyStageRow) -> Option<&str> =
        EvidenceLookupPublicCloseoutFamilyStageRow::query_import_evidence_digest;
    let _: fn(&EvidenceLookupPublicCloseoutFamilyStageRow) -> &str =
        EvidenceLookupPublicCloseoutFamilyStageRow::row_digest;
    let _: fn(
        &EvidenceLookupPublicCloseoutFamilyStageRow,
    ) -> &EvidenceLookupPublicCloseoutDisposition =
        EvidenceLookupPublicCloseoutFamilyStageRow::disposition;
    let _: fn(&EvidenceLookupPublicCloseoutCounters) -> usize =
        EvidenceLookupPublicCloseoutCounters::receipt_proof_row_count;
    let _: fn(&EvidenceLookupPublicCloseoutError) -> EvidenceLookupPublicCloseoutErrorKind =
        EvidenceLookupPublicCloseoutError::kind;
    let _: fn(&EvidenceLookupPublicCloseoutError) -> &str =
        EvidenceLookupPublicCloseoutError::detail;
    let _: fn(
        &EvidenceLookupMilestoneTwelveSeed,
    ) -> EvidenceLookupMilestoneTwelveReplayReadinessPosture =
        EvidenceLookupMilestoneTwelveSeed::replay_readiness_posture;
    let _: fn(&EvidenceLookupMilestoneTwelveSeed) -> usize =
        EvidenceLookupMilestoneTwelveSeed::family_stage_row_count;
}

#[test]
fn spatial_public_api_reads_current_lookup_public_closeout() {
    let closeout = current_evidence_lookup_public_closeout().expect("current public closeout");
    assert!(!closeout.closeout_digest().is_empty());
}
