use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::digest::stable_digest;
use super::row_kind::SubscriptionSupportAccuracyCertificationRowKind;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationRow {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    evidence_digest: String,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
    row_digest: String,
}

impl SubscriptionSupportAccuracyCertificationRow {
    pub(crate) fn new(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        evidence_digest: impl Into<String>,
        forbidden_exact_overclaim_count: u64,
        global_scan_debt_count: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if forbidden_exact_overclaim_count != 0 || global_scan_debt_count != 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite rows require zero exact-overclaim and global-scan debt counters",
            ));
        }
        let mut row = Self {
            row_kind,
            evidence_digest: require_non_empty("suite row evidence digest", evidence_digest)?,
            forbidden_exact_overclaim_count,
            global_scan_debt_count,
            row_digest: String::new(),
        };
        row.row_digest = stable_digest(&SubscriptionSupportAccuracyCertificationRowDigestBasis {
            row_kind: row.row_kind,
            evidence_digest: &row.evidence_digest,
            forbidden_exact_overclaim_count: row.forbidden_exact_overclaim_count,
            global_scan_debt_count: row.global_scan_debt_count,
        })?;
        Ok(row)
    }

    pub fn row_kind(&self) -> SubscriptionSupportAccuracyCertificationRowKind {
        self.row_kind
    }

    pub(super) fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyCertificationRowDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    evidence_digest: &'a str,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            format!("subscription-support accuracy suite {label} must be non-empty"),
        ));
    }
    Ok(value)
}
