#[derive(Debug)]
pub struct WorthUiQueryBindingSubsystem {
    allocation_source_authority: crate::prerequisites::WorthUiQueryAllocationSourceAuthority,
}

/// Allocation admission consumes a Query authority outcome; decomposed
/// projection attempts are not an accepted boundary shape.
///
/// ```compile_fail
/// fn decomposed_authority_cannot_admit(
///     admission: &mut worth_ui_query_binding::WorthUiQueryAllocationAdmission<'_>,
///     prerequisites: worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
///     authority: &worth_query::facade::foundation::WorthQueryConsumedProjectionAuthority,
/// ) {
///     let _ = admission.admit(prerequisites, authority);
/// }
/// ```
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
        outcome: worth_query::facade::read::WorthQueryProjectionOutcome,
    ) -> Result<
        crate::WorthUiQueryMeasurementFactSettlement,
        crate::WorthUiQueryMeasurementFactSettlementDenial,
    > {
        self.authority.admit(prerequisites, outcome)
    }
}
