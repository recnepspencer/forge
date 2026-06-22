use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_structural_identity::authoring::{
    planar_structural_identity_entry, PlanarStructuralIdentityCase, PlanarStructuralIdentityEntry,
};
use crate::bindings::query_native_planar_structural_identity::domain::PlanarStructuralIdentityQueryDomain;
use crate::bindings::query_native_planar_structural_identity::facts::{
    planar_structural_identity_facts, PlanarStructuralIdentityFactError,
};
use crate::bindings::query_native_planar_structural_identity::inspection::PlanarStructuralIdentityInspectionRow;
use crate::planar_contracts::contract_bundle::PlanarContractBundleValidationReceipt;
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;
use crate::planar_contracts::structural_identity::{
    CanonicalPlanarTransformBasis, PlanarOrientationPolicy, PlanarStructuralIdentityBasis,
    PlanarStructuralIdentityDenial, PlanarStructuralIdentityReceipt,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlanarStructuralIdentity {
    builder: crate::planar_contracts::structural_identity::PlanarStructuralIdentityBuilder,
}

impl PlanarStructuralIdentity {
    pub fn from_boolean_readiness(receipt: PlanarContractBundleValidationReceipt) -> Self {
        Self {
            builder: PlanarStructuralIdentityBasis::builder().boolean_readiness_receipt(receipt),
        }
    }

    pub fn with_canonical_transform_basis(mut self, basis: CanonicalPlanarTransformBasis) -> Self {
        self.builder = self.builder.canonical_transform_basis(basis);
        self
    }

    pub fn with_topology_identity(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.topology_identity(identity);
        self
    }

    pub fn with_persistent_name(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.persistent_name(identity);
        self
    }

    pub fn with_binding_identity(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.binding_identity(identity);
        self
    }

    pub fn with_motion_posture(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        let frame = receipt
            .basis()
            .boolean_readiness_receipt()
            .basis()
            .local_frame_receipt()
            .basis();
        let transform = CanonicalPlanarTransformBasis::builder()
            .local_frame(frame.frame_identity())
            .movement_rotation_posture(receipt.retained_motion_digest())
            .transform_chain_digest(frame.transform_chain_digest())
            .orientation_policy(PlanarOrientationPolicy::Preserve)
            .build()
            .expect("motion receipt must carry canonical transform-compatible basis");
        self.builder = self
            .builder
            .canonical_transform_basis(transform)
            .motion_posture_receipt(receipt);
        self
    }

    pub fn with_lineage_identity(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.lineage_identity(identity);
        self
    }

    pub fn with_final_coordinate_digest_only(mut self, digest: impl Into<String>) -> Self {
        self.builder = self.builder.final_coordinate_digest_only(digest);
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarStructuralIdentityContracts<WC>,
    ) -> Result<PlanarStructuralIdentityPlan<'a, WC>, PlanarStructuralIdentityDenial>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarStructuralIdentityQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry =
            planar_structural_identity_entry(PlanarStructuralIdentityCase::from_basis(basis));
        Ok(PlanarStructuralIdentityPlan { entry, contracts })
    }
}

pub struct PlanarStructuralIdentityContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarStructuralIdentityQueryDomain>,
{
    identity_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarStructuralIdentityQueryDomain, WC>,
}

impl<WC> PlanarStructuralIdentityContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarStructuralIdentityQueryDomain>,
{
    pub fn new(
        identity_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarStructuralIdentityQueryDomain,
            WC,
        >,
    ) -> Self {
        Self { identity_handle }
    }
}

pub struct PlanarStructuralIdentityPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarStructuralIdentityQueryDomain>,
{
    entry: PlanarStructuralIdentityEntry,
    contracts: &'a PlanarStructuralIdentityContracts<WC>,
}

impl<WC> PlanarStructuralIdentityPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarStructuralIdentityQueryDomain>,
{
    pub fn inspected_identity_rows(&self) -> usize {
        PlanarStructuralIdentityInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn certify(
        self,
    ) -> Result<PlanarStructuralIdentityReceipt, PlanarStructuralIdentityFactError> {
        planar_structural_identity_facts(&self.entry, &self.contracts.identity_handle)
    }
}
