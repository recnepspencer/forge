use crate::bindings::query_native_planar_clean_fail_boundary::authoring::{
    planar_clean_fail_boundary_entry, PlanarCleanFailBoundaryCase, PlanarCleanFailBoundaryEntry,
};
use crate::bindings::query_native_planar_clean_fail_boundary::domain::PlanarCleanFailBoundaryQueryDomain;
use crate::bindings::query_native_planar_clean_fail_boundary::facts::{
    planar_clean_fail_boundary, PlanarCleanFailBoundaryFactError,
};
use crate::bindings::query_native_planar_clean_fail_boundary::inspection::PlanarCleanFailBoundaryInspectionRow;
use crate::planar_contracts::clean_fail_boundary::{
    PlanarCleanFailBoundaryBasis, PlanarCleanFailBoundaryDenial, PlanarCleanFailBoundaryReceipt,
    PlanarCleanFailInput,
};
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::planar_contracts::planar_recovery::PlanarRecoveryPostureReceipt;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarCleanFailBoundary {
    builder: crate::planar_contracts::clean_fail_boundary::PlanarCleanFailBoundaryBuilder,
}

impl PlanarCleanFailBoundary {
    pub fn from_planar_input(input: PlanarCleanFailInput) -> Self {
        Self {
            builder: PlanarCleanFailBoundaryBasis::builder(input),
        }
    }

    pub fn recovery_posture(mut self, receipt: PlanarRecoveryPostureReceipt) -> Self {
        self.builder = self.builder.recovery_posture(receipt);
        self
    }

    pub fn diagnostics(mut self, receipt: PlanarDiagnosticBundleReceipt) -> Self {
        self.builder = self.builder.diagnostics(receipt);
        self
    }

    pub fn with_heuristic_repair_attempt(mut self) -> Self {
        self.builder = self.builder.repair_was_attempted();
        self
    }

    pub fn with_bounded_conversion_attempt(mut self) -> Self {
        self.builder = self.builder.bounded_conversion_was_attempted();
        self
    }

    pub fn certify_clean_fail_boundary(self) -> Self {
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarCleanFailBoundaryContracts<WC>,
    ) -> Result<PlanarCleanFailBoundaryPlan<'a, WC>, PlanarCleanFailBoundaryDenial>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarCleanFailBoundaryQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry =
            planar_clean_fail_boundary_entry(PlanarCleanFailBoundaryCase::from_basis(basis));
        Ok(PlanarCleanFailBoundaryPlan { entry, contracts })
    }
}

pub struct PlanarCleanFailBoundaryContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarCleanFailBoundaryQueryDomain>,
{
    handle: ForgeQueryAdmittedConfiguredDomainHandle<PlanarCleanFailBoundaryQueryDomain, WC>,
}

impl<WC> PlanarCleanFailBoundaryContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarCleanFailBoundaryQueryDomain>,
{
    pub fn new(
        handle: ForgeQueryAdmittedConfiguredDomainHandle<PlanarCleanFailBoundaryQueryDomain, WC>,
    ) -> Self {
        Self { handle }
    }
}

pub struct PlanarCleanFailBoundaryPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarCleanFailBoundaryQueryDomain>,
{
    entry: PlanarCleanFailBoundaryEntry,
    contracts: &'a PlanarCleanFailBoundaryContracts<WC>,
}

impl<WC> PlanarCleanFailBoundaryPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarCleanFailBoundaryQueryDomain>,
{
    pub fn inspected_clean_fail_rows(&self) -> usize {
        PlanarCleanFailBoundaryInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn certify(
        self,
    ) -> Result<PlanarCleanFailBoundaryReceipt, PlanarCleanFailBoundaryFactError> {
        planar_clean_fail_boundary(&self.entry, &self.contracts.handle)
    }
}
