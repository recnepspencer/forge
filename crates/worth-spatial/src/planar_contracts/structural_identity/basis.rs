use crate::planar_contracts::contract_bundle::PlanarContractBundleValidationReceipt;
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;

use super::validation::validate_planar_structural_identity_basis;
use super::{CanonicalPlanarTransformBasis, PlanarStructuralIdentityDenial};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarStructuralIdentityBasis {
    boolean_readiness_receipt: PlanarContractBundleValidationReceipt,
    canonical_transform_basis: CanonicalPlanarTransformBasis,
    topology_identity: String,
    persistent_name: String,
    binding_identity: String,
    lineage_identity: String,
    motion_posture_receipt: Option<PlanarMotionPostureReceipt>,
    final_coordinate_digest: Option<String>,
}

impl PlanarStructuralIdentityBasis {
    pub fn builder() -> PlanarStructuralIdentityBuilder {
        PlanarStructuralIdentityBuilder::default()
    }

    pub(crate) fn from_builder(
        builder: PlanarStructuralIdentityBuilder,
    ) -> Result<Self, PlanarStructuralIdentityDenial> {
        let basis = Self {
            boolean_readiness_receipt: builder.boolean_readiness_receipt.ok_or_else(|| {
                super::PlanarStructuralIdentityDenial::new(
                    super::PlanarStructuralIdentityDenialKind::MissingBooleanReadinessReceipt,
                    "planar structural identity requires a boolean-readiness receipt",
                )
            })?,
            canonical_transform_basis: builder.canonical_transform_basis.ok_or_else(|| {
                super::PlanarStructuralIdentityDenial::new(
                    super::PlanarStructuralIdentityDenialKind::MissingCanonicalTransformBasis,
                    "planar structural identity requires a canonical transform basis",
                )
            })?,
            topology_identity: builder.topology_identity.unwrap_or_default(),
            persistent_name: builder.persistent_name.unwrap_or_default(),
            binding_identity: builder.binding_identity.unwrap_or_default(),
            lineage_identity: builder.lineage_identity.unwrap_or_default(),
            motion_posture_receipt: builder.motion_posture_receipt,
            final_coordinate_digest: builder.final_coordinate_digest,
        };
        validate_planar_structural_identity_basis(&basis)?;
        Ok(basis)
    }

    pub fn boolean_readiness_receipt(&self) -> &PlanarContractBundleValidationReceipt {
        &self.boolean_readiness_receipt
    }

    pub fn canonical_transform_basis(&self) -> &CanonicalPlanarTransformBasis {
        &self.canonical_transform_basis
    }

    pub fn topology_identity(&self) -> &str {
        &self.topology_identity
    }

    pub fn persistent_name(&self) -> &str {
        &self.persistent_name
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn lineage_identity(&self) -> &str {
        &self.lineage_identity
    }

    pub fn motion_posture_receipt(&self) -> Option<&PlanarMotionPostureReceipt> {
        self.motion_posture_receipt.as_ref()
    }

    pub fn final_coordinate_digest(&self) -> Option<&str> {
        self.final_coordinate_digest.as_deref()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlanarStructuralIdentityBuilder {
    boolean_readiness_receipt: Option<PlanarContractBundleValidationReceipt>,
    canonical_transform_basis: Option<CanonicalPlanarTransformBasis>,
    topology_identity: Option<String>,
    persistent_name: Option<String>,
    binding_identity: Option<String>,
    lineage_identity: Option<String>,
    motion_posture_receipt: Option<PlanarMotionPostureReceipt>,
    final_coordinate_digest: Option<String>,
}

impl PlanarStructuralIdentityBuilder {
    pub fn boolean_readiness_receipt(
        mut self,
        receipt: PlanarContractBundleValidationReceipt,
    ) -> Self {
        self.boolean_readiness_receipt = Some(receipt);
        self
    }

    pub fn canonical_transform_basis(mut self, basis: CanonicalPlanarTransformBasis) -> Self {
        self.canonical_transform_basis = Some(basis);
        self
    }

    pub fn topology_identity(mut self, identity: impl Into<String>) -> Self {
        self.topology_identity = Some(identity.into());
        self
    }

    pub fn persistent_name(mut self, identity: impl Into<String>) -> Self {
        self.persistent_name = Some(identity.into());
        self
    }

    pub fn binding_identity(mut self, identity: impl Into<String>) -> Self {
        self.binding_identity = Some(identity.into());
        self
    }

    pub fn lineage_identity(mut self, identity: impl Into<String>) -> Self {
        self.lineage_identity = Some(identity.into());
        self
    }

    pub fn motion_posture_receipt(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.motion_posture_receipt = Some(receipt);
        self
    }

    pub fn final_coordinate_digest_only(mut self, digest: impl Into<String>) -> Self {
        self.final_coordinate_digest = Some(digest.into());
        self
    }

    pub fn build(self) -> Result<PlanarStructuralIdentityBasis, PlanarStructuralIdentityDenial> {
        PlanarStructuralIdentityBasis::from_builder(self)
    }
}
