use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_admission_posture_closeout,
    WorthGraphReadAccessAdmissionPostureOutcome, WorthGraphReadAdmissionCapabilityGapKind,
    WorthGraphReadAdmissionExpectedDenial, WorthGraphReadAdmissionSuggestedPosture,
};
use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};

use super::common::{
    production_admission_posture_closeout, production_requirement_derivation_closeout,
};

#[test]
fn requirement_derivation_gap_is_carried_without_admission_fabrication() {
    let closeout = production_admission_posture_closeout();

    assert!(closeout.posture_records().iter().all(|record| matches!(
        record.posture_outcome(),
        WorthGraphReadAccessAdmissionPostureOutcome::RequirementDerivationGapCarriedForward { .. }
    )));
    assert!(closeout.posture_records().iter().all(|record| {
        let gap = record
            .posture_outcome()
            .admission_gap()
            .expect("carried derivation gaps should expose an admission gap");
        gap.source_requirement_record_digest() == record.source_requirement_record_digest()
            && gap.query_family_anchor_digest() == record.query_family_digest_seed()
            && gap.owner() == "worth_graph_read_declarations"
            && gap.expected_denial()
                == &WorthGraphReadAdmissionExpectedDenial::RequirementDerivationGap
            && gap.suggested_posture()
                == &WorthGraphReadAdmissionSuggestedPosture::RequirementDerivationMustSucceed
            && !gap.blocker().is_empty()
            && !gap.removal_trigger().is_empty()
    }));
}

#[test]
fn missing_query_support_becomes_typed_capability_gap() {
    let closeout = production_admission_posture_closeout();

    assert_eq!(
        closeout.gap_cap_report().gap_count(),
        closeout.posture_records().len()
    );
    assert!(closeout
        .posture_records()
        .iter()
        .all(|record| record.posture_outcome().admission_gap().is_some()));
    assert!(closeout.posture_records().iter().all(|record| record
        .posture_outcome()
        .requirement_derivation_gap()
        .is_some()));
}

#[test]
fn current_requirement_records_do_not_hide_missing_query_read_family_artifacts() {
    let phase_four = production_requirement_derivation_closeout();
    let closeout =
        current_worth_graph_read_access_admission_posture_closeout(phase_four.phase_five_seed())
            .expect("Phase 5 should classify production Phase 4 seed");

    assert!(closeout.posture_records().iter().all(|record| {
        record
            .posture_outcome()
            .requirement_derivation_gap()
            .is_some()
    }));
    assert!(
        phase_four
            .requirement_records()
            .iter()
            .all(|record| record.query_read_family_artifact().is_none()),
        "current Phase 5 input must explicitly expose that Phase 4 has no real ForgeQueryReadFamily artifacts yet"
    );
}

#[test]
fn query_denials_and_required_postures_have_typed_gap_targets() {
    assert_eq!(
        WorthGraphReadAdmissionCapabilityGapKind::from_query_denial_kind(
            &ForgeQueryGraphReadAccessDenialKind::BudgetExceeded
        ),
        WorthGraphReadAdmissionCapabilityGapKind::AsyncMaterializationRequired
    );
    assert_eq!(
        WorthGraphReadAdmissionCapabilityGapKind::from_query_denial_kind(
            &ForgeQueryGraphReadAccessDenialKind::RequiredPersistentIndex
        ),
        WorthGraphReadAdmissionCapabilityGapKind::PersistentIndexRequired
    );
    assert_eq!(
        WorthGraphReadAdmissionCapabilityGapKind::from_query_denial_kind(
            &ForgeQueryGraphReadAccessDenialKind::RequiredAsyncMaterialization
        ),
        WorthGraphReadAdmissionCapabilityGapKind::AsyncMaterializationRequired
    );
    assert_eq!(
        WorthGraphReadAdmissionCapabilityGapKind::from_query_denial_kind(
            &ForgeQueryGraphReadAccessDenialKind::UnsupportedGraphIndexSupport
        ),
        WorthGraphReadAdmissionCapabilityGapKind::StoreBackedCapabilityRequired
    );
    assert_eq!(
        WorthGraphReadAdmissionCapabilityGapKind::from_query_denial_kind(
            &ForgeQueryGraphReadAccessDenialKind::RequiredAccessCapabilityRegistration
        ),
        WorthGraphReadAdmissionCapabilityGapKind::AccessCapabilityRegistrationRequired
    );
    assert_eq!(
        WorthGraphReadAdmissionCapabilityGapKind::from_required_query_posture(
            &ForgeQueryGraphReadAccessAdmissionPosture::PagedStreamingRequired
        ),
        Some(WorthGraphReadAdmissionCapabilityGapKind::PagedStreamingRequired)
    );
    assert_eq!(
        WorthGraphReadAdmissionCapabilityGapKind::from_required_query_posture(
            &ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed
        ),
        None
    );
}
