use crate::graph_read_access_declarations::WorthGraphReadAccessDeclarationMilestoneEightSeed;

use super::super::phase_one_closeout::{
    WorthGraphReadAccessPlanAdoptionPhaseOneError,
    WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionAdmittedSeed {
    seed: WorthGraphReadAccessDeclarationMilestoneEightSeed,
}

pub fn admit_milestone_eight_seed(
    seed: &WorthGraphReadAccessDeclarationMilestoneEightSeed,
) -> Result<
    WorthGraphReadAccessPlanAdoptionAdmittedSeed,
    WorthGraphReadAccessPlanAdoptionPhaseOneError,
> {
    if seed.claims_graph_read_execution() {
        return Err(error(
            WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::SeedClaimedGraphReadExecution,
        ));
    }
    if seed.claims_access_plan_consumption() {
        return Err(error(
            WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::SeedClaimedAccessPlanConsumption,
        ));
    }
    if seed.read_family_identities().is_empty() {
        return Err(error(
            WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::MissingReadFamilyIdentity,
        ));
    }
    if seed.requirement_row_evidence().is_empty() {
        return Err(error(
            WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::MissingRequirementRowEvidence,
        ));
    }
    Ok(WorthGraphReadAccessPlanAdoptionAdmittedSeed { seed: seed.clone() })
}

impl WorthGraphReadAccessPlanAdoptionAdmittedSeed {
    pub const fn seed(&self) -> &WorthGraphReadAccessDeclarationMilestoneEightSeed {
        &self.seed
    }
}

const fn error(
    kind: WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind,
) -> WorthGraphReadAccessPlanAdoptionPhaseOneError {
    WorthGraphReadAccessPlanAdoptionPhaseOneError::new(kind)
}
