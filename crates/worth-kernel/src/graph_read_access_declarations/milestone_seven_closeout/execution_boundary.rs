use super::errors::{
    WorthGraphReadAccessDeclarationCloseoutError, WorthGraphReadAccessDeclarationCloseoutErrorKind,
};
use crate::graph_read_access_declarations::WorthGraphReadAccessDeclarationPhaseSevenSeed;

pub(crate) fn reject_execution_shaped_seed(
    seed: &WorthGraphReadAccessDeclarationPhaseSevenSeed,
) -> Result<(), WorthGraphReadAccessDeclarationCloseoutError> {
    if seed.claims_graph_read_execution() {
        return Err(error(
            WorthGraphReadAccessDeclarationCloseoutErrorKind::SeedClaimedExecutionAuthority,
        ));
    }
    if seed.claims_access_plan_consumption() {
        return Err(error(
            WorthGraphReadAccessDeclarationCloseoutErrorKind::SeedClaimedAccessPlanConsumption,
        ));
    }
    Ok(())
}

pub(crate) const fn claims_graph_read_execution() -> bool {
    false
}

pub(crate) const fn claims_access_plan_consumption() -> bool {
    false
}

pub(crate) const fn claims_graph_read_receipts_complete() -> bool {
    false
}

pub(crate) const fn claims_milestone_eight_access_plan_adoption() -> bool {
    false
}

const fn error(
    kind: WorthGraphReadAccessDeclarationCloseoutErrorKind,
) -> WorthGraphReadAccessDeclarationCloseoutError {
    WorthGraphReadAccessDeclarationCloseoutError::new(kind)
}
