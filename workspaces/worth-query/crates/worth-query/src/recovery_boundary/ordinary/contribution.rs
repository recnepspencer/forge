use crate::contribution_composed_orchestration::WorthQueryContributionComposedClassification;
use crate::ordinary_outcome::WorthQueryOrdinaryPosture;

use super::mapping::kind_for_contribution_posture;
use crate::recovery_boundary::{WorthQueryRecoveryConflictPosture, WorthQueryRecoveryExplanation};

pub(crate) fn enrich_contribution_explanation(
    explanation: WorthQueryRecoveryExplanation,
    posture: &WorthQueryOrdinaryPosture,
) -> WorthQueryRecoveryExplanation {
    let Some(kind) = posture.checked_topology().contribution_composed_kind() else {
        return explanation;
    };
    let conflict_posture = match kind_for_contribution_posture(kind) {
        WorthQueryContributionComposedClassification::PartiallyAdmitted
        | WorthQueryContributionComposedClassification::MaterializationFailedAfterAdmission => {
            WorthQueryRecoveryConflictPosture::MixedContributionFailure
        }
        WorthQueryContributionComposedClassification::FullyAdmitted
        | WorthQueryContributionComposedClassification::NoContributionAdmitted => {
            WorthQueryRecoveryConflictPosture::None
        }
    };
    explanation.with_conflict_posture(conflict_posture)
}
