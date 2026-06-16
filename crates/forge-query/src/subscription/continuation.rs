use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

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
    lifecycle_continuation_ordinary_checkpoint_identity,
};
use super::evidence_projection::subscription_evidence_projection;
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
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    authority_identity: ForgeQueryEvidenceIdentity,
    remap_width: ContinuationRemapWidth,
    continuation_identity: ForgeQueryEvidenceIdentity,
}

impl SubscriptionContinuationEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        active_lane_digest: ActiveSubscriptionLaneDigest,
        continuation_class: SubscriptionContinuationClass,
        source_identity: ForgeQueryEvidenceIdentity,
        target_identity: ForgeQueryEvidenceIdentity,
        future_selection: QuerySubscriptionFutureSelection,
        basis_identity: ForgeQueryEvidenceIdentity,
        checkpoint_identity: ForgeQueryEvidenceIdentity,
        authority_identity: ForgeQueryEvidenceIdentity,
        remap_width: ContinuationRemapWidth,
    ) -> Self {
        let source_identity = lifecycle_continuation_endpoint_identity("source", &source_identity);
        let target_identity = lifecycle_continuation_endpoint_identity("target", &target_identity);
        let basis_identity = lifecycle_continuation_endpoint_identity("basis", &basis_identity);
        let checkpoint_endpoint_identity =
            lifecycle_continuation_endpoint_identity("checkpoint", &checkpoint_identity);
        let authority_identity =
            lifecycle_continuation_endpoint_identity("authority", &authority_identity);
        let continuation_identity = lifecycle_continuation_identity(
            active_lane_digest.evidence_identity(),
            continuation_class.as_str(),
            &source_identity,
            &target_identity,
            future_selection.projection_identity(),
            &basis_identity,
            &checkpoint_endpoint_identity,
            &authority_identity,
            remap_width.get(),
        );
        Self {
            active_lane_digest,
            continuation_class,
            source_identity,
            target_identity,
            future_selection,
            basis_identity,
            checkpoint_identity,
            authority_identity,
            remap_width,
            continuation_identity,
        }
    }

    pub(crate) fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn continuation_class(&self) -> SubscriptionContinuationClass {
        self.continuation_class
    }

    pub fn source_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.source_identity)
    }

    pub fn source_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.source_identity
    }

    pub fn target_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.target_identity)
    }

    pub fn target_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.target_identity
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.basis_identity)
    }

    pub fn basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn checkpoint_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.checkpoint_identity)
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn authority_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.authority_identity)
    }

    pub fn authority_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.authority_identity
    }

    pub fn remap_width(&self) -> u64 {
        self.remap_width.get()
    }

    pub fn continuation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.continuation_identity)
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.continuation_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionContinuationReport {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    continuation_class: SubscriptionContinuationClass,
    continuation_identity: ForgeQueryEvidenceIdentity,
    future_selection: QuerySubscriptionFutureSelection,
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    remap_width: u64,
    performance_receipt: SubscriptionPerformanceReceipt,
    report_identity: ForgeQueryEvidenceIdentity,
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
        Self {
            active_lane_digest: evidence.active_lane_digest().clone(),
            continuation_class: evidence.continuation_class(),
            continuation_identity: evidence.evidence_identity().clone(),
            future_selection: evidence.future_selection().clone(),
            checkpoint_identity: evidence.checkpoint_identity().clone(),
            remap_width: evidence.remap_width(),
            performance_receipt,
            report_identity,
        }
    }

    pub(crate) fn active_lane_digest(&self) -> &ActiveSubscriptionLaneDigest {
        &self.active_lane_digest
    }

    pub fn continuation_class(&self) -> SubscriptionContinuationClass {
        self.continuation_class
    }

    pub fn continuation_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.continuation_identity)
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn checkpoint_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.checkpoint_identity)
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn remap_width(&self) -> u64 {
        self.remap_width
    }

    pub fn performance_receipt(&self) -> &SubscriptionPerformanceReceipt {
        &self.performance_receipt
    }

    pub fn report_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.report_identity)
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.report_identity
    }
}

#[allow(clippy::too_many_arguments)]
pub fn admit_subscription_continuation_evidence(
    active_lane_digest: ActiveSubscriptionLaneDigest,
    continuation_class: SubscriptionContinuationClass,
    source_identity: ForgeQueryEvidenceIdentity,
    target_identity: ForgeQueryEvidenceIdentity,
    basis_identity: ForgeQueryEvidenceIdentity,
    authority_identity: ForgeQueryEvidenceIdentity,
    remap_width: ContinuationRemapWidth,
) -> Result<SubscriptionContinuationEvidence, SubscriptionContinuationError> {
    let checkpoint_identity =
        lifecycle_continuation_ordinary_checkpoint_identity(active_lane_digest.evidence_identity());
    admit_subscription_continuation_evidence_with_active_identity(
        active_lane_digest,
        continuation_class,
        source_identity,
        target_identity,
        QuerySubscriptionFutureSelection::ordinary(),
        basis_identity,
        checkpoint_identity,
        authority_identity,
        remap_width,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn admit_subscription_continuation_evidence_with_active_identity(
    active_lane_digest: ActiveSubscriptionLaneDigest,
    continuation_class: SubscriptionContinuationClass,
    source_identity: ForgeQueryEvidenceIdentity,
    target_identity: ForgeQueryEvidenceIdentity,
    future_selection: QuerySubscriptionFutureSelection,
    basis_identity: ForgeQueryEvidenceIdentity,
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    authority_identity: ForgeQueryEvidenceIdentity,
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
            active_lane_digest.evidence_identity().clone(),
            counters,
        ));
    }
    if remap_width.get() == 0 {
        counters.continuation_remap_denial_count = 1;
        return Err(SubscriptionContinuationError::new(
            SubscriptionContinuationDenialKind::ContinuationRemapBudgetExceeded,
            "continuation remap evidence requires an explicit nonzero remap width",
            active_lane_digest.evidence_identity().clone(),
            counters,
        ));
    }

    Ok(SubscriptionContinuationEvidence::new(
        active_lane_digest,
        continuation_class,
        source_identity,
        target_identity,
        future_selection,
        basis_identity,
        checkpoint_identity,
        authority_identity,
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
            evidence.evidence_identity().clone(),
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
        report.evidence_identity(),
        MaintenanceDeltaWidth::measured(report.remap_width()),
    );
    (delta, counters)
}
