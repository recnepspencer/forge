use super::super::classification_error;
use super::decision::{
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportCompatibilityDecisionKind,
};
use super::receipt_witness::SupportCompatibilityReceiptWitness;
use crate::failure::StoreError;
use crate::CompatibilityRelation;

pub(super) fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support compatibility {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}

pub(super) fn validate_decision_against_receipt(
    decision: &SubscriptionSupportCompatibilityDecision,
    receipt: &SupportCompatibilityReceiptWitness,
) -> Result<(), StoreError> {
    match decision.kind() {
        SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration => {
            match receipt.relation() {
                Some(
                    CompatibilityRelation::Native
                    | CompatibilityRelation::BackwardRead
                    | CompatibilityRelation::ForwardRead,
                ) if receipt.rejection_kind().is_none() => Ok(()),
                _ => Err(classification_error(
                    "exact support compatibility migration requires a native/forward/backward Milestone 12 read receipt",
                )),
            }
        }
        SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility => {
            match receipt.relation() {
                Some(
                    CompatibilityRelation::AdapterRequired
                    | CompatibilityRelation::DerivedRebuildRequired,
                ) if receipt.rejection_kind().is_none() => Ok(()),
                _ => Err(classification_error(
                    "degraded support compatibility requires an admitted adapter or rebuild-required Milestone 12 relation",
                )),
            }
        }
        SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
        | SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
        | SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected => {
            if receipt.rejection_kind().is_some() && receipt.relation().is_none() {
                Ok(())
            } else {
                Err(classification_error(
                    "support compatibility rejection requires a rejected Milestone 12 read admission outcome",
                ))
            }
        }
    }
}
