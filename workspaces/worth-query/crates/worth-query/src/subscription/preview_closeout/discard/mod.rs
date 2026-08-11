use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::super::active_counters::ActiveSubscriptionCounters;
use super::super::active_digest::ActiveSubscriptionLaneDigest;
use super::super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::super::future_selection::QuerySubscriptionFutureSelection;
use super::super::performance_receipt::SubscriptionPerformanceReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionDiscardCloseout {
    active_lane_digest: ActiveSubscriptionLaneDigest,
    attachment_digest: SubscriptionConsumerAttachmentDigest,
    future_selection: QuerySubscriptionFutureSelection,
    basis_binding_identity: WorthQueryEvidenceIdentity,
    checkpoint_identity: WorthQueryEvidenceIdentity,
    preview_epoch_identity: WorthQueryEvidenceIdentity,
    residue_report_identity: WorthQueryEvidenceIdentity,
    performance_receipt: SubscriptionPerformanceReceipt,
    counters: ActiveSubscriptionCounters,
    closeout_identity: WorthQueryEvidenceIdentity,
}

mod assembly;
mod model;

pub use assembly::discard_preview_subscription;
