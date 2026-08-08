use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationCommitReceipt, WorthQueryRecoveryInspectionView,
};

use super::external_effect::publish_external_effect;
use super::outcome::publish_posture;
use super::{WorthQueryPublishedApplicationAftermath, WorthQueryPublishedRecoverySupport};

/// Publishes only a sealed execution commit.
///
/// ```compile_fail
/// use worth_query_installation::facade::PublishedAftermathPosture;
/// use worth_query_publication::facade::application_aftermath::publish_application_aftermath;
///
/// let copied = PublishedAftermathPosture::Reconcilable;
/// let _ = publish_application_aftermath(&copied);
/// ```
pub const fn publish_application_aftermath(
    receipt: &WorthQueryApplicationCommitReceipt,
) -> WorthQueryPublishedApplicationAftermath {
    WorthQueryPublishedApplicationAftermath::new(
        match receipt.published_aftermath_posture() {
            Some(posture) => Some(publish_posture(posture)),
            None => None,
        },
        publish_external_effect(receipt),
    )
}

/// Publishes only a disclosure-admitted runtime inspection view.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryOpaqueRecoveryWireIdentity;
/// use worth_query_publication::facade::application_aftermath::publish_recovery_support;
///
/// let wire: WorthQueryOpaqueRecoveryWireIdentity = todo!();
/// let _ = publish_recovery_support(&wire);
/// ```
pub const fn publish_recovery_support(
    inspection: &WorthQueryRecoveryInspectionView,
) -> WorthQueryPublishedRecoverySupport {
    WorthQueryPublishedRecoverySupport::new(publish_posture(inspection.published_posture()))
}
