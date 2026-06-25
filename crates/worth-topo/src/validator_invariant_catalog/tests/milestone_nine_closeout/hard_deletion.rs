use crate::validator_invariant_catalog::WorthTopologyMilestoneNineDeletionDisposition;

use super::fixtures::milestone_nine_closeout;

#[test]
fn deletion_ledger_closes_every_recorded_old_authority_row() {
    let closeout = milestone_nine_closeout();
    let ledger = closeout.deletion_ledger();
    assert!(!ledger.rows().is_empty());
    assert_eq!(ledger.closed_old_authority_count(), ledger.rows().len());
    assert!(ledger.rows().iter().all(|row| {
        matches!(
            row.disposition(),
            WorthTopologyMilestoneNineDeletionDisposition::Deleted
                | WorthTopologyMilestoneNineDeletionDisposition::CappedResidue
                | WorthTopologyMilestoneNineDeletionDisposition::CertificationOnly
        )
    }));
    assert!(ledger.capped_residue_count() > 0);
    assert!(ledger.whole_view_certification_only_count() >= 2);
}
