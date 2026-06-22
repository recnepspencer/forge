use worth_spatial::facade::blocker_provenance::{
    WorkloadBlockerBoundaryKind, WorkloadBlockerSourceKind,
};
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitDecisionKind, PlanarBooleanSplitFailureLocalization,
};

use crate::workload_composition::{
    PlanarBooleanDeclarationReceipt, PlanarBooleanEntryError, PlanarBooleanOutcomeReceipt,
    PlanarBooleanSupportReceipt,
};

impl PlanarBooleanOutcomeReceipt {
    pub fn from_edge_split_failure_localization(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        localization: &PlanarBooleanSplitFailureLocalization,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let human_reason = format!(
            "edge split decision {} localized {} at {}",
            localization.decision_identity(),
            localization
                .policy_or_denial_kind()
                .unwrap_or(localization.kind().as_str()),
            localization.phase().as_str()
        );
        if localization.kind() == PlanarBooleanSplitDecisionKind::SplitPhaseDenied {
            Self::denied(
                declaration,
                support,
                human_reason,
                WorkloadBlockerSourceKind::PlanarBooleanEdgeSplitting,
                WorkloadBlockerBoundaryKind::BooleanExecutionBoundary,
                localization.decision_identity(),
                localization.affected_artifact_identity(),
            )
        } else {
            Self::blocked(
                declaration,
                support,
                human_reason,
                WorkloadBlockerSourceKind::PlanarBooleanEdgeSplitting,
                WorkloadBlockerBoundaryKind::BooleanExecutionBoundary,
                localization.decision_identity(),
                localization.affected_artifact_identity(),
            )
        }
    }
}
