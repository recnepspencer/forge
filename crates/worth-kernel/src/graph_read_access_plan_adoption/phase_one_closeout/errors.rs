use super::super::execution_folklore_inventory::WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventoryError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind {
    SeedClaimedGraphReadExecution,
    SeedClaimedAccessPlanConsumption,
    MissingReadFamilyIdentity,
    MissingRequirementRowEvidence,
    MissingExecutionFolkloreInventory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPhaseOneError {
    kind: WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind,
}

impl WorthGraphReadAccessPlanAdoptionPhaseOneError {
    pub(crate) const fn new(kind: WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind {
        self.kind
    }
}

impl From<WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventoryError>
    for WorthGraphReadAccessPlanAdoptionPhaseOneError
{
    fn from(error: WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventoryError) -> Self {
        match error {
            WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventoryError::MissingInventoryRows => {
                Self::new(
                    WorthGraphReadAccessPlanAdoptionPhaseOneErrorKind::MissingExecutionFolkloreInventory,
                )
            }
        }
    }
}
