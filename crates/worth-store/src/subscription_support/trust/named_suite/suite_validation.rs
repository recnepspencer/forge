use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::certification_row::SubscriptionSupportAccuracyCertificationRow;
use super::counter_snapshot::SubscriptionSupportAccuracyCertificationCounterSnapshot;
use super::row_kind::SubscriptionSupportAccuracyCertificationRowKind;
use std::collections::BTreeSet;

pub(super) fn validate_required_rows(
    rows: &[SubscriptionSupportAccuracyCertificationRow],
) -> Result<(), SupportTrustFailure> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.row_kind()) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite cannot contain duplicate row kinds",
            ));
        }
    }
    for required in SubscriptionSupportAccuracyCertificationRowKind::required() {
        if !seen.contains(required) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite is missing a required certification row",
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_required_row_count(
    counter_snapshot: &SubscriptionSupportAccuracyCertificationCounterSnapshot,
) -> Result<(), SupportTrustFailure> {
    if counter_snapshot.certified_row_count() != counter_snapshot.required_row_count() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "subscription-support accuracy suite counters must match required row coverage",
        ));
    }
    Ok(())
}
