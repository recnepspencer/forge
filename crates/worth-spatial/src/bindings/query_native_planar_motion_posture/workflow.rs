use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_motion_posture::authoring::{
    planar_motion_posture_entry, PlanarMotionPostureCase, PlanarMotionPostureEntry,
};
use crate::bindings::query_native_planar_motion_posture::continuation::PlanarMotionContinuation;
use crate::bindings::query_native_planar_motion_posture::domain::PlanarMotionPostureQueryDomain;
use crate::bindings::query_native_planar_motion_posture::facts::{
    planar_motion_posture_facts, PlanarMotionPostureFactError,
};
use crate::bindings::query_native_planar_motion_posture::inspection::PlanarMotionPostureInspectionRow;
use crate::planar_contracts::contract_bundle::PlanarContractBundleValidationReceipt;
use crate::planar_contracts::motion_posture::{
    PlanarMotionCancellation, PlanarMotionPostureBasis, PlanarMotionPostureDenial,
    PlanarMotionPostureReceipt, PlanarReorientation,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlanarMotionPosture {
    builder: crate::planar_contracts::motion_posture::PlanarMotionPostureBuilder,
}

impl PlanarMotionPosture {
    pub fn from_boolean_readiness(receipt: PlanarContractBundleValidationReceipt) -> Self {
        Self {
            builder: PlanarMotionPostureBasis::builder().boolean_readiness_receipt(receipt),
        }
    }

    pub fn after_exact_translation(mut self, step_identity: impl Into<String>) -> Self {
        self.builder = self.builder.exact_translation(step_identity);
        self
    }

    pub fn after_exact_rotation(mut self, step_identity: impl Into<String>) -> Self {
        self.builder = self.builder.exact_rotation(step_identity);
        self
    }

    pub fn after_reorientation(mut self, posture: PlanarReorientation) -> Self {
        self.builder = self.builder.reorientation(posture);
        self
    }

    pub fn with_cancellation_policy(mut self, cancellation: PlanarMotionCancellation) -> Self {
        self.builder = self.builder.cancellation_policy(cancellation);
        self
    }

    pub fn with_final_coordinate_digest_only(mut self, digest: impl Into<String>) -> Self {
        self.builder = self.builder.final_coordinate_digest_only(digest);
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarMotionPostureContracts<WC>,
    ) -> Result<PlanarMotionPosturePlan<'a, WC>, PlanarMotionPostureDenial>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarMotionPostureQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry = planar_motion_posture_entry(PlanarMotionPostureCase::from_basis(basis));
        Ok(PlanarMotionPosturePlan { entry, contracts })
    }
}

pub struct PlanarMotionPostureContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarMotionPostureQueryDomain>,
{
    motion_handle: ForgeQueryAdmittedConfiguredDomainHandle<PlanarMotionPostureQueryDomain, WC>,
}

impl<WC> PlanarMotionPostureContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarMotionPostureQueryDomain>,
{
    pub fn new(
        motion_handle: ForgeQueryAdmittedConfiguredDomainHandle<PlanarMotionPostureQueryDomain, WC>,
    ) -> Self {
        Self { motion_handle }
    }
}

pub struct PlanarMotionPosturePlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarMotionPostureQueryDomain>,
{
    entry: PlanarMotionPostureEntry,
    contracts: &'a PlanarMotionPostureContracts<WC>,
}

impl<WC> PlanarMotionPosturePlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarMotionPostureQueryDomain>,
{
    pub fn inspected_motion_rows(&self) -> usize {
        PlanarMotionPostureInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn certify(self) -> Result<PlanarMotionPostureReceipt, PlanarMotionPostureFactError> {
        planar_motion_posture_facts(&self.entry, &self.contracts.motion_handle)
    }
}

impl PlanarMotionPostureReceipt {
    pub fn continuation(&self) -> PlanarMotionContinuation {
        PlanarMotionContinuation::from_receipt(self)
    }
}
