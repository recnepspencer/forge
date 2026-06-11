use crate::bindings::query_native_planar_recovery::authoring::{
    planar_recovery_posture_entry, PlanarRecoveryPostureCase, PlanarRecoveryPostureEntry,
};
use crate::bindings::query_native_planar_recovery::domain::PlanarRecoveryPostureQueryDomain;
use crate::bindings::query_native_planar_recovery::facts::{
    planar_recovery_posture, PlanarRecoveryPostureFactError,
};
use crate::bindings::query_native_planar_recovery::inspection::PlanarRecoveryPostureInspectionRow;
use crate::planar_contracts::planar_recovery::{
    PlanarRecoveryPostureBasis, PlanarRecoveryPostureDenial, PlanarRecoveryPostureReceipt,
    PlanarRecoverySource,
};
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarRecoveryPosture {
    builder: crate::planar_contracts::planar_recovery::PlanarRecoveryPostureBuilder,
}

impl PlanarRecoveryPosture {
    pub fn from_blocked_planar_source(source: PlanarRecoverySource) -> Self {
        Self {
            builder: PlanarRecoveryPostureBasis::builder(source),
        }
    }

    pub fn with_retained_planar_facts(mut self, receipt: RetainedPlanarFactsReceipt) -> Self {
        self.builder = self.builder.retained_planar_facts(receipt);
        self
    }

    pub fn with_projection_consumed_facts(
        mut self,
        receipt: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.builder = self.builder.projection_consumed_facts(receipt);
        self
    }

    pub fn prepare_next_step(self) -> Self {
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarRecoveryPostureContracts<WC>,
    ) -> Result<PlanarRecoveryPosturePlan<'a, WC>, PlanarRecoveryPostureDenial>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarRecoveryPostureQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry = planar_recovery_posture_entry(PlanarRecoveryPostureCase::from_basis(basis));
        Ok(PlanarRecoveryPosturePlan { entry, contracts })
    }
}

pub struct PlanarRecoveryPostureContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarRecoveryPostureQueryDomain>,
{
    recovery_handle: ForgeQueryAdmittedConfiguredDomainHandle<PlanarRecoveryPostureQueryDomain, WC>,
}

impl<WC> PlanarRecoveryPostureContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarRecoveryPostureQueryDomain>,
{
    pub fn new(
        recovery_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarRecoveryPostureQueryDomain,
            WC,
        >,
    ) -> Self {
        Self { recovery_handle }
    }
}

pub struct PlanarRecoveryPosturePlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarRecoveryPostureQueryDomain>,
{
    entry: PlanarRecoveryPostureEntry,
    contracts: &'a PlanarRecoveryPostureContracts<WC>,
}

impl<WC> PlanarRecoveryPosturePlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarRecoveryPostureQueryDomain>,
{
    pub fn inspected_recovery_rows(&self) -> usize {
        PlanarRecoveryPostureInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn certify(self) -> Result<PlanarRecoveryPostureReceipt, PlanarRecoveryPostureFactError> {
        planar_recovery_posture(&self.entry, &self.contracts.recovery_handle)
    }
}
