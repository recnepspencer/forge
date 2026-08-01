use crate::facade::intent::{
    UiIntentAffinityPosture, UiIntentConfirmationPosture, UiIntentMutabilityPosture,
    UiIntentOccupancyPosture, UiIntentOperabilityDecision, UiIntentPolicyPosture,
    UiIntentReadinessPosture, UiIntentSupportPosture,
};

/// Typed classifier input for exhaustive certification of the closed
/// operability lattice. It grants no prepared candidate, proof, or reservation.
pub struct UiIntentOperabilityDecisionCertificationInput {
    pub support: UiIntentSupportPosture,
    pub mutability: UiIntentMutabilityPosture,
    pub readiness: UiIntentReadinessPosture,
    pub occupancy: UiIntentOccupancyPosture,
    pub policy: UiIntentPolicyPosture,
    pub affinity: UiIntentAffinityPosture,
    pub confirmation: UiIntentConfirmationPosture,
}

pub fn classify_intent_operability_for_certification(
    input: UiIntentOperabilityDecisionCertificationInput,
) -> UiIntentOperabilityDecision {
    UiIntentOperabilityDecision::new(crate::runtime::intent::UiIntentOperabilityDecisionInput {
        contract_identity: "certification.intent-operability-lattice".into(),
        support: input.support,
        mutability: input.mutability,
        readiness: input.readiness,
        occupancy: input.occupancy,
        policy: input.policy,
        affinity: input.affinity,
        confirmation: input.confirmation,
        selected_dependencies_visited: 0,
    })
}
