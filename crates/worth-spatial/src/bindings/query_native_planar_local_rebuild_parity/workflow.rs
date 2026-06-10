use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_local_rebuild_parity::authoring::{
    planar_local_rebuild_parity_entry, PlanarLocalRebuildParityCase, PlanarLocalRebuildParityEntry,
};
use crate::bindings::query_native_planar_local_rebuild_parity::domain::PlanarLocalRebuildParityQueryDomain;
use crate::bindings::query_native_planar_local_rebuild_parity::facts::{
    planar_local_rebuild_parity, PlanarLocalRebuildParityFactError,
};
use crate::bindings::query_native_planar_local_rebuild_parity::inspection::PlanarLocalRebuildParityInspectionRow;
use crate::bindings::query_native_rebinding_neighborhood_replacement::TopologyNeighborhoodReplacementFactReceipt;
use crate::planar_contracts::local_rebuild_parity::{
    PlanarLocalRebuildParityBasis, PlanarLocalRebuildParityDenial, PlanarLocalRebuildParityReceipt,
    PlanarLocalRebuildScope, PlanarRebindingContinuityEvidence,
};
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::planar_contracts::planar_recovery::PlanarRecoveryPostureReceipt;
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::structural_identity::PlanarStructuralIdentityReceipt;
use crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalRebuildParity {
    builder: crate::planar_contracts::local_rebuild_parity::PlanarLocalRebuildParityBuilder,
}

impl PlanarLocalRebuildParity {
    pub fn for_local_rebuild(rebuild_scope: PlanarLocalRebuildScope) -> Self {
        Self {
            builder: PlanarLocalRebuildParityBasis::builder(rebuild_scope),
        }
    }

    pub fn local_neighborhood(
        mut self,
        receipt: TopologyNeighborhoodReplacementFactReceipt,
    ) -> Self {
        self.builder = self.builder.local_neighborhood(receipt);
        self
    }

    pub fn rebinding_continuity(mut self, evidence: PlanarRebindingContinuityEvidence) -> Self {
        self.builder = self.builder.rebinding_continuity(evidence);
        self
    }

    pub fn structural_identity(mut self, receipt: PlanarStructuralIdentityReceipt) -> Self {
        self.builder = self.builder.structural_identity(receipt);
        self
    }

    pub fn retained_planar_facts(mut self, receipt: RetainedPlanarFactsReceipt) -> Self {
        self.builder = self.builder.retained_planar_facts(receipt);
        self
    }

    pub fn projection_consumed_planar_facts(
        mut self,
        receipt: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.builder = self.builder.projection_consumed_planar_facts(receipt);
        self
    }

    pub fn motion_posture(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.builder = self.builder.motion_posture(receipt);
        self
    }

    pub fn topology_contract(mut self, receipt: PlanarTopologyContractCompletenessReceipt) -> Self {
        self.builder = self.builder.topology_contract(receipt);
        self
    }

    pub fn recovery_posture(mut self, receipt: PlanarRecoveryPostureReceipt) -> Self {
        self.builder = self.builder.recovery_posture(receipt);
        self
    }

    pub fn diagnostics(mut self, receipt: PlanarDiagnosticBundleReceipt) -> Self {
        self.builder = self.builder.diagnostics(receipt);
        self
    }

    pub fn certify_same_planar_basis_across_views(self) -> Self {
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarLocalRebuildParityContracts<WC>,
    ) -> Result<PlanarLocalRebuildParityPlan<'a, WC>, PlanarLocalRebuildParityDenial>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarLocalRebuildParityQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry =
            planar_local_rebuild_parity_entry(PlanarLocalRebuildParityCase::from_basis(basis));
        Ok(PlanarLocalRebuildParityPlan { entry, contracts })
    }
}

pub struct PlanarLocalRebuildParityContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarLocalRebuildParityQueryDomain>,
{
    handle: ForgeQueryAdmittedConfiguredDomainHandle<PlanarLocalRebuildParityQueryDomain, WC>,
}

impl<WC> PlanarLocalRebuildParityContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarLocalRebuildParityQueryDomain>,
{
    pub fn new(
        handle: ForgeQueryAdmittedConfiguredDomainHandle<PlanarLocalRebuildParityQueryDomain, WC>,
    ) -> Self {
        Self { handle }
    }
}

pub struct PlanarLocalRebuildParityPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarLocalRebuildParityQueryDomain>,
{
    entry: PlanarLocalRebuildParityEntry,
    contracts: &'a PlanarLocalRebuildParityContracts<WC>,
}

impl<WC> PlanarLocalRebuildParityPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarLocalRebuildParityQueryDomain>,
{
    pub fn inspected_parity_rows(&self) -> usize {
        PlanarLocalRebuildParityInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn certify(
        self,
    ) -> Result<PlanarLocalRebuildParityReceipt, PlanarLocalRebuildParityFactError> {
        planar_local_rebuild_parity(&self.entry, &self.contracts.handle)
    }
}
