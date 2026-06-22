use crate::bindings::query_native_planar_diagnostics::authoring::{
    planar_diagnostic_bundle_entry, PlanarDiagnosticBundleCase, PlanarDiagnosticBundleEntry,
};
use crate::bindings::query_native_planar_diagnostics::domain::PlanarDiagnosticBundleQueryDomain;
use crate::bindings::query_native_planar_diagnostics::facts::{
    planar_diagnostic_bundle, PlanarDiagnosticBundleFactError,
};
use crate::bindings::query_native_planar_diagnostics::inspection::PlanarDiagnosticInspectionRow;
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;
use crate::planar_contracts::planar_diagnostics::{
    PlanarDiagnosticBundleBasis, PlanarDiagnosticBundleReceipt, PlanarDiagnosticCausalEvidence,
    PlanarDiagnosticDenial, PlanarDiagnosticSubject, PlanarDiagnosticTopologyEvidence,
};
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessReceipt;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarDiagnosticBundle {
    builder: crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleBuilder,
}

impl PlanarDiagnosticBundle {
    pub fn explain_planar_failure(subject: PlanarDiagnosticSubject) -> Self {
        Self {
            builder: PlanarDiagnosticBundleBasis::builder(subject),
        }
    }

    pub fn with_topology_declared_surface(
        mut self,
        evidence: PlanarDiagnosticTopologyEvidence,
    ) -> Self {
        self.builder = self.builder.topology_declared_surface(evidence);
        self
    }

    pub fn with_query_causal_inspection(
        mut self,
        evidence: PlanarDiagnosticCausalEvidence,
    ) -> Self {
        self.builder = self.builder.query_causal_inspection(evidence);
        self
    }

    pub fn with_retained_planar_facts(mut self, receipt: RetainedPlanarFactsReceipt) -> Self {
        self.builder = self.builder.retained_planar_facts(receipt);
        self
    }

    pub fn with_projection_consumed_planar_facts(
        mut self,
        receipt: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.builder = self.builder.projection_consumed_planar_facts(receipt);
        self
    }

    pub fn with_topology_contract(
        mut self,
        receipt: PlanarTopologyContractCompletenessReceipt,
    ) -> Self {
        self.builder = self.builder.topology_contract(receipt);
        self
    }

    pub fn with_motion_posture(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.builder = self.builder.motion_posture(receipt);
        self
    }

    pub fn request_materialized_causal_archive(mut self) -> Self {
        self.builder = self.builder.request_materialized_causal_archive();
        self
    }

    pub fn inspect_failure_locality(self) -> Self {
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarDiagnosticBundleContracts<WC>,
    ) -> Result<PlanarDiagnosticBundlePlan<'a, WC>, PlanarDiagnosticDenial>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarDiagnosticBundleQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry = planar_diagnostic_bundle_entry(PlanarDiagnosticBundleCase::from_basis(basis));
        Ok(PlanarDiagnosticBundlePlan { entry, contracts })
    }
}

pub struct PlanarDiagnosticBundleContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarDiagnosticBundleQueryDomain>,
{
    diagnostic_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarDiagnosticBundleQueryDomain, WC>,
}

impl<WC> PlanarDiagnosticBundleContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarDiagnosticBundleQueryDomain>,
{
    pub fn new(
        diagnostic_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarDiagnosticBundleQueryDomain,
            WC,
        >,
    ) -> Self {
        Self { diagnostic_handle }
    }
}

pub struct PlanarDiagnosticBundlePlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarDiagnosticBundleQueryDomain>,
{
    entry: PlanarDiagnosticBundleEntry,
    contracts: &'a PlanarDiagnosticBundleContracts<WC>,
}

impl<WC> PlanarDiagnosticBundlePlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarDiagnosticBundleQueryDomain>,
{
    pub fn inspected_diagnostic_rows(&self) -> usize {
        PlanarDiagnosticInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn certify(self) -> Result<PlanarDiagnosticBundleReceipt, PlanarDiagnosticBundleFactError> {
        planar_diagnostic_bundle(&self.entry, &self.contracts.diagnostic_handle)
    }
}
