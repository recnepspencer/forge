use super::decision::SubscriptionSupportPortabilityDecisionKind;
use super::import_not_resumable::ImportedSupportNotResumableReport;
use super::imported_semantic_access::ImportedSupportSemanticAccess;
use super::partial_omission::PartialSupportOmissionReport;
use super::rejection::SupportPortabilityRejection;
use super::replicated_bundle::ReplicatedSupportBundle;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportPortabilityOutcome {
    FullScopeReplicated(ReplicatedSupportBundle),
    PartialScopeOmitted(PartialSupportOmissionReport),
    Imported(ImportedSupportSemanticAccess),
    ImportedNotResumable(ImportedSupportNotResumableReport),
    Rejected(SupportPortabilityRejection),
}

impl SubscriptionSupportPortabilityOutcome {
    pub fn outcome_kind(&self) -> SubscriptionSupportPortabilityDecisionKind {
        match self {
            Self::FullScopeReplicated(_) => {
                SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
            }
            Self::PartialScopeOmitted(_) => {
                SubscriptionSupportPortabilityDecisionKind::PartialScopeOmission
            }
            Self::Imported(_) => SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted,
            Self::ImportedNotResumable(_) => {
                SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable
            }
            Self::Rejected(rejection) => rejection.rejection_kind(),
        }
    }
}
