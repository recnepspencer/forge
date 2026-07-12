#[derive(Debug)]
pub struct WorthUiQueryBindingSubsystem {
    allocation_source_authority: crate::prerequisites::WorthUiQueryAllocationSourceAuthority,
}

pub struct WorthUiQueryAllocationAdmission<'a> {
    authority: &'a mut crate::prerequisites::WorthUiQueryAllocationSourceAuthority,
}

impl WorthUiQueryBindingSubsystem {
    pub fn bootstrap() -> Self {
        Self {
            allocation_source_authority: Default::default(),
        }
    }

    pub fn prerequisites(&self) -> crate::WorthUiQueryPrerequisiteBoundary {
        crate::WorthUiQueryPrerequisiteBoundary::new()
    }

    pub fn allocation_admission(&mut self) -> WorthUiQueryAllocationAdmission<'_> {
        WorthUiQueryAllocationAdmission {
            authority: &mut self.allocation_source_authority,
        }
    }
}

impl WorthUiQueryAllocationAdmission<'_> {
    pub fn admit(
        &mut self,
        prerequisites: crate::WorthUiQueryPrerequisiteEvidence,
        consumption: &worth_query::facade::ProjectionFactConsumptionAttempt,
    ) -> Result<
        crate::WorthUiQueryMeasurementFactSettlement,
        crate::WorthUiQueryMeasurementFactSettlementDenial,
    > {
        self.authority.admit(prerequisites, consumption)
    }
}
