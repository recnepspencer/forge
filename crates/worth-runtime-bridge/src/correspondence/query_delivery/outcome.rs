use super::{
    bind_performed_signal_invalidation, BridgeGranularInvalidationDelivery,
    BridgePerformedSignalInvalidationDenial,
};
use crate::correspondence::BridgeCorrespondenceDeliveryReceipt;

/// Assembles the Query-readable delivery without conflating authoritative
/// truth with an optional performed Signal consequence.
pub fn assemble_granular_invalidation_delivery(
    truth: &BridgeCorrespondenceDeliveryReceipt,
    performed_signal: Option<&mut crate::conditional_execution::BridgeConditionalDecisionEvidence>,
) -> Result<BridgeGranularInvalidationDelivery, BridgePerformedSignalInvalidationDenial> {
    let delivery = BridgeGranularInvalidationDelivery::direct(truth);
    match performed_signal {
        Some(decision) => Ok(
            delivery.with_performed_signal(bind_performed_signal_invalidation(truth, decision)?)
        ),
        None => Ok(delivery),
    }
}
