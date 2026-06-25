use crate::graph_read_access_declarations::{
    admission_gap_cap_ledger_row, WorthGraphReadAdmissionCapabilityGapKind,
};

use super::common::production_admission_posture_closeout;

#[test]
fn capability_gap_counts_are_capped() {
    let closeout = production_admission_posture_closeout();

    assert!(closeout.gap_cap_report().gap_count() > 0);
    assert!(closeout.gap_cap_report().capped_gap_family_count() > 0);
    assert!(!closeout.gap_cap_report().report_digest().is_empty());
    assert!(closeout.posture_records().iter().all(|record| record
        .posture_outcome()
        .admission_gap()
        .is_some_and(|gap| gap.must_not_exceed_count() > 0)));

    let family_cap = closeout
        .gap_cap_report()
        .gap_family_caps()
        .first()
        .expect("cap report should expose the capped gap family");
    assert_eq!(
        family_cap.kind(),
        WorthGraphReadAdmissionCapabilityGapKind::RequirementDerivationBlocked
    );
    let ledger_row = admission_gap_cap_ledger_row(family_cap.kind())
        .expect("cap report family should be backed by the fixed cap ledger");
    assert_eq!(family_cap.current_count(), closeout.posture_records().len());
    assert_eq!(
        family_cap.must_not_exceed_count(),
        ledger_row.must_not_exceed_count()
    );
    assert!(family_cap.current_count() <= ledger_row.must_not_exceed_count());
    assert!(family_cap
        .cap_ledger_digest_part()
        .contains(ledger_row.owner()));
}
