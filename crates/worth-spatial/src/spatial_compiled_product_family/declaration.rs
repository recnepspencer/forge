use worth_primitives::{truth_digest_parts, TruthDigestScope};

use schema::facade::platform::authority::compiled_product_semantic_graph::CompiledProductPriorProofRole;

use super::consumer::SpatialCompiledProductConsumer;
#[cfg(test)]
use super::error::{SpatialCompiledProductFamilyError, SpatialCompiledProductFamilyErrorKind};
use super::family_identity::SpatialCompiledProductFamilyIdentity;
use super::posture::{
    SpatialEquivalencePolicyPosture, SpatialEvidenceSupportRolePosture,
    SpatialLocalityFootprintBasisPosture, SpatialPriorProofRolePosture,
    SpatialSourceAuthorityDigestBasisPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialCompiledProductFamilyDeclaration {
    identity: SpatialCompiledProductFamilyIdentity,
    supported_consumers: Vec<SpatialCompiledProductConsumer>,
    source_authority_digest_basis: SpatialSourceAuthorityDigestBasisPosture,
    locality_footprint_basis: SpatialLocalityFootprintBasisPosture,
    prior_proof_role: SpatialPriorProofRolePosture,
    evidence_support_role: SpatialEvidenceSupportRolePosture,
    equivalence_policy: SpatialEquivalencePolicyPosture,
    equivalence_policy_name: &'static str,
    equivalence_dimensions: &'static [&'static str],
    family_digest: String,
}

impl SpatialCompiledProductFamilyDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        identity: SpatialCompiledProductFamilyIdentity,
        mut supported_consumers: Vec<SpatialCompiledProductConsumer>,
        source_authority_digest_basis: SpatialSourceAuthorityDigestBasisPosture,
        locality_footprint_basis: SpatialLocalityFootprintBasisPosture,
        prior_proof_role: SpatialPriorProofRolePosture,
        evidence_support_role: SpatialEvidenceSupportRolePosture,
        equivalence_policy: SpatialEquivalencePolicyPosture,
        equivalence_policy_name: &'static str,
        equivalence_dimensions: &'static [&'static str],
    ) -> Self {
        supported_consumers.sort_by_key(|consumer| consumer.as_str());
        let family_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:compiled-product-family-declaration:v1".to_string(),
                format!("identity:{}", identity.as_str()),
                format!(
                    "consumers:{}",
                    supported_consumers
                        .iter()
                        .map(|consumer| consumer.as_str())
                        .collect::<Vec<_>>()
                        .join("|")
                ),
                format!("source-authority:{source_authority_digest_basis:?}"),
                format!("locality:{locality_footprint_basis:?}"),
                format!("prior-proof:{prior_proof_role:?}"),
                format!("evidence-support:{evidence_support_role:?}"),
                format!("equivalence-posture:{equivalence_policy:?}"),
                format!("equivalence-policy-name:{equivalence_policy_name}"),
                format!(
                    "equivalence-dimensions:{}",
                    equivalence_dimensions.join("|")
                ),
            ],
        );
        Self {
            identity,
            supported_consumers,
            source_authority_digest_basis,
            locality_footprint_basis,
            prior_proof_role,
            evidence_support_role,
            equivalence_policy,
            equivalence_policy_name,
            equivalence_dimensions,
            family_digest,
        }
    }

    pub const fn identity(&self) -> SpatialCompiledProductFamilyIdentity {
        self.identity
    }

    pub fn supported_consumers(&self) -> &[SpatialCompiledProductConsumer] {
        &self.supported_consumers
    }

    pub const fn source_authority_digest_basis(&self) -> SpatialSourceAuthorityDigestBasisPosture {
        self.source_authority_digest_basis
    }

    pub const fn locality_footprint_basis(&self) -> SpatialLocalityFootprintBasisPosture {
        self.locality_footprint_basis
    }

    pub const fn prior_proof_role(&self) -> SpatialPriorProofRolePosture {
        self.prior_proof_role
    }

    pub const fn compiled_product_prior_proof_role(&self) -> Option<CompiledProductPriorProofRole> {
        match self.prior_proof_role {
            SpatialPriorProofRolePosture::NotRequired => None,
            SpatialPriorProofRolePosture::RetainedCancellationCheckpointHistoryBasis => {
                Some(CompiledProductPriorProofRole::EquivalenceDimension)
            }
            SpatialPriorProofRolePosture::SelectedPlanTopologyAndQuerySupportBasis => {
                Some(CompiledProductPriorProofRole::ProductShapingBasis)
            }
        }
    }

    pub const fn evidence_support_role(&self) -> SpatialEvidenceSupportRolePosture {
        self.evidence_support_role
    }

    pub const fn equivalence_policy(&self) -> SpatialEquivalencePolicyPosture {
        self.equivalence_policy
    }

    pub const fn equivalence_policy_name(&self) -> &'static str {
        self.equivalence_policy_name
    }

    pub const fn equivalence_dimensions(&self) -> &'static [&'static str] {
        self.equivalence_dimensions
    }

    pub fn supports(&self, consumer: SpatialCompiledProductConsumer) -> bool {
        self.supported_consumers.contains(&consumer)
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpatialCompiledProductFamilyDeclarationBuilder {
    identity: Option<SpatialCompiledProductFamilyIdentity>,
    supported_consumers: Option<Vec<SpatialCompiledProductConsumer>>,
    source_authority_digest_basis: Option<SpatialSourceAuthorityDigestBasisPosture>,
    locality_footprint_basis: Option<SpatialLocalityFootprintBasisPosture>,
    prior_proof_role: Option<SpatialPriorProofRolePosture>,
    evidence_support_role: Option<SpatialEvidenceSupportRolePosture>,
    equivalence_policy: Option<SpatialEquivalencePolicyPosture>,
    equivalence_policy_name: Option<&'static str>,
    equivalence_dimensions: Option<&'static [&'static str]>,
}

#[cfg(test)]
impl SpatialCompiledProductFamilyDeclarationBuilder {
    pub fn identity(mut self, identity: SpatialCompiledProductFamilyIdentity) -> Self {
        self.identity = Some(identity);
        self
    }

    pub fn supported_consumers(
        mut self,
        supported_consumers: Vec<SpatialCompiledProductConsumer>,
    ) -> Self {
        self.supported_consumers = Some(supported_consumers);
        self
    }

    pub fn source_authority_digest_basis(
        mut self,
        posture: SpatialSourceAuthorityDigestBasisPosture,
    ) -> Self {
        self.source_authority_digest_basis = Some(posture);
        self
    }

    pub fn locality_footprint_basis(
        mut self,
        posture: SpatialLocalityFootprintBasisPosture,
    ) -> Self {
        self.locality_footprint_basis = Some(posture);
        self
    }

    pub fn prior_proof_role(mut self, posture: SpatialPriorProofRolePosture) -> Self {
        self.prior_proof_role = Some(posture);
        self
    }

    pub fn evidence_support_role(mut self, posture: SpatialEvidenceSupportRolePosture) -> Self {
        self.evidence_support_role = Some(posture);
        self
    }

    pub fn equivalence_policy(mut self, posture: SpatialEquivalencePolicyPosture) -> Self {
        self.equivalence_policy = Some(posture);
        self
    }

    pub fn equivalence_policy_name(mut self, name: &'static str) -> Self {
        self.equivalence_policy_name = Some(name);
        self
    }

    pub fn equivalence_dimensions(mut self, dimensions: &'static [&'static str]) -> Self {
        self.equivalence_dimensions = Some(dimensions);
        self
    }

    pub fn build(
        self,
    ) -> Result<SpatialCompiledProductFamilyDeclaration, SpatialCompiledProductFamilyError> {
        let identity = self.identity.ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingFamilyIdentity,
                "spatial compiled-product family declaration requires an explicit family identity",
            )
        })?;
        let supported_consumers = self.supported_consumers.filter(|consumers| !consumers.is_empty()).ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingConsumerForDeclaration,
                "spatial compiled-product family declaration requires at least one supported consumer",
            )
        })?;
        let source_authority_digest_basis = self.source_authority_digest_basis.ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingAuthorityBasis,
                "spatial compiled-product family declaration requires an explicit source authority digest basis",
            )
        })?;
        let locality_footprint_basis = self.locality_footprint_basis.ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingLocalityBasis,
                "spatial compiled-product family declaration requires an explicit locality footprint basis",
            )
        })?;
        let prior_proof_role = self.prior_proof_role.ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingPriorProofRole,
                "spatial compiled-product family declaration requires an explicit prior-proof role",
            )
        })?;
        let evidence_support_role = self.evidence_support_role.ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingEvidenceSupportRole,
                "spatial compiled-product family declaration requires an explicit evidence-support role",
            )
        })?;
        let equivalence_policy = self.equivalence_policy.ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingEquivalencePolicy,
                "spatial compiled-product family declaration requires an explicit equivalence-policy posture",
            )
        })?;
        let equivalence_policy_name = self.equivalence_policy_name.ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingEquivalencePolicy,
                "spatial compiled-product family declaration requires an explicit equivalence-policy name",
            )
        })?;
        let equivalence_dimensions = self.equivalence_dimensions.ok_or_else(|| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::MissingEquivalencePolicy,
                "spatial compiled-product family declaration requires explicit equivalence dimensions",
            )
        })?;
        Ok(SpatialCompiledProductFamilyDeclaration::new(
            identity,
            supported_consumers,
            source_authority_digest_basis,
            locality_footprint_basis,
            prior_proof_role,
            evidence_support_role,
            equivalence_policy,
            equivalence_policy_name,
            equivalence_dimensions,
        ))
    }
}
