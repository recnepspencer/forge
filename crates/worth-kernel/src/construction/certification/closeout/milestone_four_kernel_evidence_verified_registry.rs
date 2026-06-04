use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

use crate::construction::certification::arbitration::PrimitiveConstructionIntentArbitrationBundleCase;
use crate::construction::certification::arbitration::PrimitiveConstructionIntentArbitrationPolicyCase;
use crate::construction::certification::closeout::milestone_four_kernel_requirements::{
    required_arbitration_cases, required_continuity_cases, required_motion_cases,
    required_policy_profile_cases, required_preview_cases, required_realization_witness_kinds,
};
use crate::construction::certification::continuity::PrimitiveConstructionContinuityCase;
use crate::construction::certification::motion::PrimitiveConstructionMotionResolutionPolicyCase;
use crate::construction::certification::preview::PrimitiveConstructionPreviewCase;
use crate::construction::certification::profile::PrimitiveConstructionPolicyProfileCase;
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::proof_grade::PrimitiveConstructionProofSubject;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct PrimitiveConstructionMilestoneFourKernelCloseoutRegistry {
    pub(super) required_motion_cases: &'static [PrimitiveConstructionMotionResolutionPolicyCase],
    pub(super) required_arbitration_cases:
        &'static [PrimitiveConstructionIntentArbitrationPolicyCase],
    pub(super) required_preview_cases: &'static [PrimitiveConstructionPreviewCase],
    pub(super) required_continuity_cases: &'static [PrimitiveConstructionContinuityCase],
    pub(super) required_policy_profile_cases: &'static [PrimitiveConstructionPolicyProfileCase],
    pub(super) required_realization_witness_kinds:
        &'static [PrimitiveRealizationExhaustionWitnessKind],
    pub(super) required_intent_representative_case:
        PrimitiveConstructionIntentArbitrationBundleCase,
    pub(super) required_preview_representative_case: PrimitiveConstructionPreviewCase,
    pub(super) required_continuity_representative_case: PrimitiveConstructionContinuityCase,
    pub(super) required_policy_profile_representative_case: PrimitiveConstructionPolicyProfileCase,
    pub(super) required_proof_substrate_subject: PrimitiveConstructionProofSubject,
    pub(super) registry_digest: String,
}

impl PrimitiveConstructionMilestoneFourKernelCloseoutRegistry {
    pub(super) fn new() -> Self {
        let required_motion_cases = required_motion_cases();
        let required_arbitration_cases = required_arbitration_cases();
        let required_preview_cases = required_preview_cases();
        let required_continuity_cases = required_continuity_cases();
        let required_policy_profile_cases = required_policy_profile_cases();
        let required_realization_witness_kinds = required_realization_witness_kinds();
        let required_intent_representative_case =
            PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice;
        let required_preview_representative_case =
            PrimitiveConstructionPreviewCase::OverlapHighFidelity;
        let required_continuity_representative_case =
            PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged;
        let required_policy_profile_representative_case =
            PrimitiveConstructionPolicyProfileCase::HighFidelityPreview;
        let required_proof_substrate_subject =
            PrimitiveConstructionProofSubject::ProofSubstrateCloseout;
        let registry_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ArtifactIdentity,
            &required_motion_cases
                .iter()
                .map(|case| format!("{case:?}"))
                .chain(
                    required_arbitration_cases
                        .iter()
                        .map(|case| format!("{case:?}")),
                )
                .chain(
                    required_preview_cases
                        .iter()
                        .map(|case| format!("{case:?}")),
                )
                .chain(
                    required_continuity_cases
                        .iter()
                        .map(|case| format!("{case:?}")),
                )
                .chain(
                    required_policy_profile_cases
                        .iter()
                        .map(|case| format!("{case:?}")),
                )
                .chain(
                    required_realization_witness_kinds
                        .iter()
                        .map(|kind| format!("{kind:?}")),
                )
                .chain([
                    format!("{required_intent_representative_case:?}"),
                    format!("{required_preview_representative_case:?}"),
                    format!("{required_continuity_representative_case:?}"),
                    format!("{required_policy_profile_representative_case:?}"),
                    required_proof_substrate_subject.as_str().to_string(),
                ])
                .collect::<Vec<_>>(),
        );
        Self {
            required_motion_cases,
            required_arbitration_cases,
            required_preview_cases,
            required_continuity_cases,
            required_policy_profile_cases,
            required_realization_witness_kinds,
            required_intent_representative_case,
            required_preview_representative_case,
            required_continuity_representative_case,
            required_policy_profile_representative_case,
            required_proof_substrate_subject,
            registry_digest,
        }
    }
}
