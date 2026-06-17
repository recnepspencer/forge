use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::counters::QuerySubscriptionDeclarationCounters;
use super::diagnostic::QuerySubscriptionDiagnosticStage;
use super::error::{
    QuerySubscriptionFamilySelectionError, QuerySubscriptionFamilySelectionFailureClass,
};
use super::family::QuerySubscriptionFamily;
use super::future_selection::QuerySubscriptionFutureSelectionClass;
use super::input::LiveQueryAdmissionArtifact;
use super::posture::QuerySubscriptionBasisPosture;

pub(super) fn validate_future_selection(
    live: &LiveQueryAdmissionArtifact,
    family: &QuerySubscriptionFamily,
    source_identity: &ForgeQueryEvidenceIdentity,
    counters: &mut QuerySubscriptionDeclarationCounters,
) -> Result<(), QuerySubscriptionFamilySelectionError> {
    match live.future_selection.class() {
        QuerySubscriptionFutureSelectionClass::Ordinary => Ok(()),
        QuerySubscriptionFutureSelectionClass::Temporal => {
            validate_future_basis(live, source_identity, counters)?;
            if family == &QuerySubscriptionFamily::InspectorDetailExact {
                counters.family_denial_count = 1;
                return Err(QuerySubscriptionFamilySelectionError::new(
                    QuerySubscriptionFamilySelectionFailureClass::UnsupportedTemporalLiveShape,
                    "temporal live meaning does not admit inspector-detail subscription selection",
                    QuerySubscriptionDiagnosticStage::ViewMismatch,
                    source_identity,
                    counters.clone(),
                ));
            }
            Ok(())
        }
        QuerySubscriptionFutureSelectionClass::AsyncResource => {
            validate_future_basis(live, source_identity, counters)?;
            if matches!(
                family,
                QuerySubscriptionFamily::GroupedCollectionMembership
                    | QuerySubscriptionFamily::BoundedMaterialization
            ) {
                counters.family_denial_count = 1;
                return Err(QuerySubscriptionFamilySelectionError::new(
                    QuerySubscriptionFamilySelectionFailureClass::UnsupportedAsyncLiveShape,
                    "async live meaning does not admit grouped or bounded-materialization subscription selection",
                    QuerySubscriptionDiagnosticStage::ViewMismatch,
                    source_identity,
                    counters.clone(),
                ));
            }
            Ok(())
        }
        QuerySubscriptionFutureSelectionClass::TemporalAsync => {
            validate_future_basis(live, source_identity, counters)?;
            if family != &QuerySubscriptionFamily::CollectionMembership {
                counters.family_denial_count = 1;
                return Err(QuerySubscriptionFamilySelectionError::new(
                    QuerySubscriptionFamilySelectionFailureClass::UnsupportedAsyncLiveShape,
                    "temporal+async live meaning currently admits only collection-membership subscription selection",
                    QuerySubscriptionDiagnosticStage::ViewMismatch,
                    source_identity,
                    counters.clone(),
                ));
            }
            Ok(())
        }
    }
}

fn validate_future_basis(
    live: &LiveQueryAdmissionArtifact,
    source_identity: &ForgeQueryEvidenceIdentity,
    counters: &mut QuerySubscriptionDeclarationCounters,
) -> Result<(), QuerySubscriptionFamilySelectionError> {
    if matches!(
        live.basis_posture,
        QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot
            | QuerySubscriptionBasisPosture::DeniedUnsupportedBasis
    ) {
        counters.family_denial_count = 1;
        return Err(QuerySubscriptionFamilySelectionError::new(
            QuerySubscriptionFamilySelectionFailureClass::FutureLiveBasisUnsupported,
            "future-bearing live meaning requires a current, branch, or preview-scoped basis posture",
            QuerySubscriptionDiagnosticStage::FamilySelection,
            source_identity,
            counters.clone(),
        ));
    }
    Ok(())
}
