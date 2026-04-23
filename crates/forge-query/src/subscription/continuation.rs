use crate::identity::hash_parts;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::continuation_error::{
    SubscriptionContinuationDenialKind, SubscriptionContinuationError,
};
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::delivery_dimensions::{ContinuationRemapWidth, MaintenanceDeltaWidth};
use super::delivery_window::QueryDeliveryWindow;
use super::maintenance_delta::{
    QuerySubscriptionMaintenanceDelta, QuerySubscriptionMaintenanceDeltaKind,
};
use super::performance_receipt::SubscriptionPerformanceReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubscriptionContinuationClass {
    IdentityRemap,
    CorrespondenceAdvisory,
    IdentityBreak,
    CollectionMembershipRemap,
    GroupedMembershipRemap,
    PreviewPromotionRemap,
    UnsupportedContinuation,
}

impl SubscriptionContinuationClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IdentityRemap => "identity_remap",
            Self::CorrespondenceAdvisory => "correspondence_advisory",
            Self::IdentityBreak => "identity_break",
            Self::CollectionMembershipRemap => "collection_membership_remap",
            Self::GroupedMembershipRemap => "grouped_membership_remap",
            Self::PreviewPromotionRemap => "preview_promotion_remap",
            Self::UnsupportedContinuation => "unsupported_continuation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionContinuationEvidence {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    continuation_class: SubscriptionContinuationClass,
    source_identity_digest: String,
    target_identity_digest: String,
    basis_digest: String,
    authority_digest: String,
    remap_width: ContinuationRemapWidth,
    continuation_digest: String,
}

impl SubscriptionContinuationEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        active_lane_digest: ActiveSubscriptionLaneDigest,
        continuation_class: SubscriptionContinuationClass,
        source_identity: impl Into<String>,
        target_identity: impl Into<String>,
        basis_digest: impl Into<String>,
        authority_digest: impl Into<String>,
        remap_width: ContinuationRemapWidth,
    ) -> Self {
        let source_identity_digest = hash_parts(&[
            "subscription_continuation_source_identity_v1".to_string(),
            format!("source:{}", source_identity.into()),
        ]);
        let target_identity_digest = hash_parts(&[
            "subscription_continuation_target_identity_v1".to_string(),
            format!("target:{}", target_identity.into()),
        ]);
        let basis_digest = basis_digest.into();
        let authority_digest = authority_digest.into();
        let continuation_digest = hash_parts(&[
            "subscription_continuation_evidence_v1".to_string(),
            format!("lane:{}", active_lane_digest.as_str()),
            format!("class:{}", continuation_class.as_str()),
            format!("source:{}", source_identity_digest),
            format!("target:{}", target_identity_digest),
            format!("basis:{}", basis_digest),
            format!("authority:{}", authority_digest),
            format!("remap_width:{}", remap_width.get()),
        ]);
        Self {
            active_lane_digest,
            continuation_class,
            source_identity_digest,
            target_identity_digest,
            basis_digest,
            authority_digest,
            remap_width,
            continuation_digest,
        }
    }

    pub fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn continuation_class(&self) -> SubscriptionContinuationClass {
        self.continuation_class
    }

    pub fn source_identity_digest(&self) -> &str {
        &self.source_identity_digest
    }

    pub fn target_identity_digest(&self) -> &str {
        &self.target_identity_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub fn remap_width(&self) -> u64 {
        self.remap_width.get()
    }

    pub fn continuation_digest(&self) -> &str {
        &self.continuation_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionContinuationReport {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    continuation_class: SubscriptionContinuationClass,
    continuation_digest: String,
    remap_width: u64,
    performance_receipt: SubscriptionPerformanceReceipt,
    report_digest: String,
}

impl SubscriptionContinuationReport {
    pub(super) fn new(evidence: &SubscriptionContinuationEvidence) -> Self {
        let performance_receipt = SubscriptionPerformanceReceipt::new(
            evidence.remap_width(),
            evidence.remap_width(),
            ActiveDeliveryDensityPosture::SparseDelta,
            super::active_budget::ActiveSubscriptionAllocationPosture::PatchScratch,
            evidence.continuation_digest(),
        );
        let report_digest = hash_parts(&[
            "subscription_continuation_report_v1".to_string(),
            format!("lane:{}", evidence.active_lane_digest().as_str()),
            format!("continuation:{}", evidence.continuation_digest()),
            format!("class:{}", evidence.continuation_class().as_str()),
            format!("remap_width:{}", evidence.remap_width()),
            format!(
                "performance:{}",
                performance_receipt.performance_receipt_digest()
            ),
        ]);
        Self {
            active_lane_digest: evidence.active_lane_digest().clone(),
            continuation_class: evidence.continuation_class(),
            continuation_digest: evidence.continuation_digest().to_string(),
            remap_width: evidence.remap_width(),
            performance_receipt,
            report_digest,
        }
    }

    pub fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn continuation_class(&self) -> SubscriptionContinuationClass {
        self.continuation_class
    }

    pub fn continuation_digest(&self) -> &str {
        &self.continuation_digest
    }

    pub fn remap_width(&self) -> u64 {
        self.remap_width
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[allow(clippy::too_many_arguments)]
pub fn admit_subscription_continuation_evidence(
    active_lane_digest: ActiveSubscriptionLaneDigest,
    continuation_class: SubscriptionContinuationClass,
    source_identity: impl Into<String>,
    target_identity: impl Into<String>,
    basis_digest: impl Into<String>,
    authority_digest: impl Into<String>,
    remap_width: ContinuationRemapWidth,
) -> Result<SubscriptionContinuationEvidence, SubscriptionContinuationError> {
    let mut counters = ActiveSubscriptionCounters::default();
    if matches!(
        continuation_class,
        SubscriptionContinuationClass::UnsupportedContinuation
            | SubscriptionContinuationClass::PreviewPromotionRemap
    ) {
        counters.continuation_remap_denial_count = 1;
        return Err(SubscriptionContinuationError::new(
            SubscriptionContinuationDenialKind::UnsupportedContinuationClass,
            "unsupported or later-phase continuation class cannot produce active subscription evidence",
            active_lane_digest.as_str(),
            counters,
        ));
    }
    if remap_width.get() == 0 {
        counters.continuation_remap_denial_count = 1;
        return Err(SubscriptionContinuationError::new(
            SubscriptionContinuationDenialKind::ContinuationRemapBudgetExceeded,
            "continuation remap evidence requires an explicit nonzero remap width",
            active_lane_digest.as_str(),
            counters,
        ));
    }

    Ok(SubscriptionContinuationEvidence::new(
        active_lane_digest,
        continuation_class,
        source_identity,
        target_identity,
        basis_digest,
        authority_digest,
        remap_width,
    ))
}

pub fn apply_subscription_continuation(
    window: QueryDeliveryWindow,
    evidence: SubscriptionContinuationEvidence,
) -> Result<(QueryDeliveryWindow, SubscriptionContinuationReport), SubscriptionContinuationError> {
    let mut counters = ActiveSubscriptionCounters::default();
    if window.active_lane_digest() != evidence.active_lane_digest() {
        counters.continuation_remap_denial_count = 1;
        return Err(SubscriptionContinuationError::new(
            SubscriptionContinuationDenialKind::ContinuationEvidenceMismatch,
            "continuation evidence must target the delivery window lane",
            evidence.continuation_digest(),
            counters,
        ));
    }
    let report = SubscriptionContinuationReport::new(&evidence);
    let window = window.apply_continuation(&report);
    Ok((window, report))
}

pub fn lower_subscription_continuation_report(
    report: &SubscriptionContinuationReport,
) -> (
    QuerySubscriptionMaintenanceDelta,
    ActiveSubscriptionCounters,
) {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.continuation_remap_width = report.remap_width();
    match report.continuation_class() {
        SubscriptionContinuationClass::CorrespondenceAdvisory => {
            counters.continuation_advisory_count = 1;
        }
        SubscriptionContinuationClass::IdentityBreak => {
            counters.continuation_identity_break_count = 1;
        }
        SubscriptionContinuationClass::IdentityRemap
        | SubscriptionContinuationClass::CollectionMembershipRemap
        | SubscriptionContinuationClass::GroupedMembershipRemap
        | SubscriptionContinuationClass::PreviewPromotionRemap => {
            counters.continuation_remap_count = 1;
        }
        SubscriptionContinuationClass::UnsupportedContinuation => {
            counters.continuation_remap_denial_count = 1;
        }
    }
    let delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::ContinuationDelta,
        report.active_lane_digest().clone(),
        report.report_digest(),
        MaintenanceDeltaWidth::measured(report.remap_width()),
    );
    (delta, counters)
}
