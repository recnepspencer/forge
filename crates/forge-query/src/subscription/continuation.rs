use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::continuation_error::{
    SubscriptionContinuationDenialKind, SubscriptionContinuationError,
};
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::delivery_dimensions::{ContinuationRemapWidth, MaintenanceDeltaWidth};
use super::delivery_window::QueryDeliveryWindow;
use super::evidence_identities::{
    lifecycle_continuation_endpoint_identity, lifecycle_continuation_identity,
};
use super::future_selection::QuerySubscriptionFutureSelection;
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
    source_identity: ForgeQueryEvidenceIdentity,
    target_identity: ForgeQueryEvidenceIdentity,
    future_selection: QuerySubscriptionFutureSelection,
    basis_identity: ForgeQueryEvidenceIdentity,
    basis_for_reporting: String,
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    checkpoint_for_reporting: String,
    authority_identity: ForgeQueryEvidenceIdentity,
    authority_for_reporting: String,
    remap_width: ContinuationRemapWidth,
    continuation_identity: ForgeQueryEvidenceIdentity,
    continuation_digest: String,
}

impl SubscriptionContinuationEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        active_lane_digest: ActiveSubscriptionLaneDigest,
        continuation_class: SubscriptionContinuationClass,
        source_identity: impl Into<String>,
        target_identity: impl Into<String>,
        future_selection: QuerySubscriptionFutureSelection,
        basis_digest: impl Into<String>,
        checkpoint_identity_digest: impl Into<String>,
        authority_digest: impl Into<String>,
        remap_width: ContinuationRemapWidth,
    ) -> Self {
        let basis_label = basis_digest.into();
        let checkpoint_label = checkpoint_identity_digest.into();
        let authority_label = authority_digest.into();
        let source_identity = lifecycle_continuation_endpoint_identity("source", &source_identity.into());
        let target_identity = lifecycle_continuation_endpoint_identity("target", &target_identity.into());
        let basis_identity =
            lifecycle_continuation_endpoint_identity("basis", &basis_label);
        let checkpoint_identity =
            lifecycle_continuation_endpoint_identity("checkpoint", &checkpoint_label);
        let authority_identity =
            lifecycle_continuation_endpoint_identity("authority", &authority_label);
        let continuation_identity = lifecycle_continuation_identity(
            active_lane_digest.evidence_identity(),
            continuation_class.as_str(),
            &source_identity,
            &target_identity,
            future_selection.projection_identity(),
            &basis_identity,
            &checkpoint_identity,
            &authority_identity,
            remap_width.get(),
        );
        let continuation_digest = continuation_identity.as_str().to_string();
        Self {
            active_lane_digest,
            continuation_class,
            source_identity,
            target_identity,
            future_selection,
            basis_identity,
            basis_for_reporting: basis_label,
            checkpoint_identity,
            checkpoint_for_reporting: checkpoint_label,
            authority_identity,
            authority_for_reporting: authority_label,
            remap_width,
            continuation_identity,
            continuation_digest,
        }
    }

    pub fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn continuation_class(&self) -> SubscriptionContinuationClass {
        self.continuation_class
    }

    pub fn source_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.source_identity
    }

    pub fn source_for_reporting(&self) -> &str {
        self.source_identity.as_str()
    }

    pub fn target_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.target_identity
    }

    pub fn target_for_reporting(&self) -> &str {
        self.target_identity.as_str()
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_for_reporting
    }

    pub fn checkpoint_identity_digest(&self) -> &str {
        &self.checkpoint_for_reporting
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_for_reporting
    }

    pub fn remap_width(&self) -> u64 {
        self.remap_width.get()
    }

    pub fn continuation_digest(&self) -> &str {
        &self.continuation_digest
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.continuation_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionContinuationReport {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    continuation_class: SubscriptionContinuationClass,
    continuation_digest: String,
    future_selection: QuerySubscriptionFutureSelection,
    checkpoint_identity_digest: String,
    remap_width: u64,
    performance_receipt: SubscriptionPerformanceReceipt,
    report_identity: ForgeQueryEvidenceIdentity,
    report_digest: String,
}

impl SubscriptionContinuationReport {
    pub(super) fn new(evidence: &SubscriptionContinuationEvidence) -> Self {
        let performance_receipt = SubscriptionPerformanceReceipt::new(
            evidence.remap_width(),
            evidence.remap_width(),
            ActiveDeliveryDensityPosture::SparseDelta,
            super::active_budget::ActiveSubscriptionAllocationPosture::PatchScratch,
            evidence.evidence_identity(),
        );
        let report_identity = ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
            "subscription_continuation_report_v1",
        )
        .field_evidence_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("lane"),
            evidence.active_lane_digest().evidence_identity(),
        )
        .field_evidence_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("continuation"),
            evidence.evidence_identity(),
        )
        .field_evidence_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("performance"),
            performance_receipt.performance_receipt_identity(),
        )
        .seal();
        let report_digest = report_identity.as_str().to_string();
        Self {
            active_lane_digest: evidence.active_lane_digest().clone(),
            continuation_class: evidence.continuation_class(),
            continuation_digest: evidence.continuation_digest().to_string(),
            future_selection: evidence.future_selection().clone(),
            checkpoint_identity_digest: evidence.checkpoint_identity_digest().to_string(),
            remap_width: evidence.remap_width(),
            performance_receipt,
            report_identity,
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

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn checkpoint_identity_digest(&self) -> &str {
        &self.checkpoint_identity_digest
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

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.report_identity
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
    admit_subscription_continuation_evidence_with_active_identity(
        active_lane_digest,
        continuation_class,
        source_identity,
        target_identity,
        QuerySubscriptionFutureSelection::ordinary(),
        basis_digest,
        "active-checkpoint-ordinary",
        authority_digest,
        remap_width,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn admit_subscription_continuation_evidence_with_active_identity(
    active_lane_digest: ActiveSubscriptionLaneDigest,
    continuation_class: SubscriptionContinuationClass,
    source_identity: impl Into<String>,
    target_identity: impl Into<String>,
    future_selection: QuerySubscriptionFutureSelection,
    basis_digest: impl Into<String>,
    checkpoint_identity_digest: impl Into<String>,
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
        future_selection,
        basis_digest,
        checkpoint_identity_digest,
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
