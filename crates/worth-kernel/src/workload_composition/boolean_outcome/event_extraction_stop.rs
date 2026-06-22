use worth_spatial::facade::blocker_provenance::{
    WorkloadBlockerBoundaryKind, WorkloadBlockerSourceKind,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventExtractionDenialKind, PlanarBooleanEventExtractionPhaseStop,
};

use crate::workload_composition::{
    PlanarBooleanDeclarationReceipt, PlanarBooleanEntryError, PlanarBooleanOutcomeReceipt,
    PlanarBooleanSupportReceipt,
};

impl PlanarBooleanOutcomeReceipt {
    pub fn from_event_extraction_stop(
        declaration: PlanarBooleanDeclarationReceipt,
        support: PlanarBooleanSupportReceipt,
        stop: &PlanarBooleanEventExtractionPhaseStop,
    ) -> Result<Self, PlanarBooleanEntryError> {
        match stop {
            PlanarBooleanEventExtractionPhaseStop::Denied(denial)
                if denial_maps_to_integrity_mismatch(denial.kind()) =>
            {
                Self::integrity_mismatch(
                    declaration,
                    support,
                    denial.human_reason(),
                    WorkloadBlockerSourceKind::PlanarBooleanEventExtraction,
                    WorkloadBlockerBoundaryKind::BooleanEventExtractionBoundary,
                    denial.denial_identity(),
                    denial.reduced_pair_identity(),
                )
            }
            PlanarBooleanEventExtractionPhaseStop::Denied(denial) => Self::denied(
                declaration,
                support,
                denial.human_reason(),
                WorkloadBlockerSourceKind::PlanarBooleanEventExtraction,
                WorkloadBlockerBoundaryKind::BooleanEventExtractionBoundary,
                denial.denial_identity(),
                denial.reduced_pair_identity(),
            ),
            PlanarBooleanEventExtractionPhaseStop::PolicyExit(policy_exit) => {
                Self::policy_required(
                    declaration,
                    support,
                    policy_exit.human_reason(),
                    WorkloadBlockerSourceKind::PlanarBooleanEventExtraction,
                    WorkloadBlockerBoundaryKind::BooleanEventExtractionBoundary,
                    policy_exit.policy_exit_identity(),
                    policy_exit.reduced_pair_identity(),
                )
            }
        }
    }
}

fn denial_maps_to_integrity_mismatch(kind: PlanarBooleanEventExtractionDenialKind) -> bool {
    matches!(
        kind,
        PlanarBooleanEventExtractionDenialKind::MissingTopologyProvenance
            | PlanarBooleanEventExtractionDenialKind::MixedReducedPairIdentity
            | PlanarBooleanEventExtractionDenialKind::MixedFrameIdentity
    )
}
