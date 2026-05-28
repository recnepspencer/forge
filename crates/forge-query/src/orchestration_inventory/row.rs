use super::aspect::ForgeQueryOrchestrationAspectPosture;
use super::authority::{
    ForgeQueryOrchestrationBasisPosture, ForgeQueryOrchestrationCollaborativeExtensionPosture,
    ForgeQueryOrchestrationLowerAuthorityAttachment, ForgeQueryOrchestrationPolicyTenantPosture,
};
use super::certification::ForgeQueryOrchestrationSurfaceCertificationReference;
use super::contribution::ForgeQueryOrchestrationContributionCompatibility;
use super::docs::ForgeQueryOrchestrationSurfaceDocReference;
use super::family::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceVisibility,
};
use super::strategy::ForgeQueryOrchestrationStrategyAttachment;
use super::transcript::ForgeQueryOrchestrationProofContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryOrchestrationSemanticProfile {
    aspect_posture: ForgeQueryOrchestrationAspectPosture,
    basis_posture: ForgeQueryOrchestrationBasisPosture,
    policy_tenant_posture: ForgeQueryOrchestrationPolicyTenantPosture,
    lower_authority_attachment: ForgeQueryOrchestrationLowerAuthorityAttachment,
    strategy_attachment: ForgeQueryOrchestrationStrategyAttachment,
    contribution_compatibility: ForgeQueryOrchestrationContributionCompatibility,
    collaborative_extension_posture: ForgeQueryOrchestrationCollaborativeExtensionPosture,
}

impl ForgeQueryOrchestrationSemanticProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        aspect_posture: ForgeQueryOrchestrationAspectPosture,
        basis_posture: ForgeQueryOrchestrationBasisPosture,
        policy_tenant_posture: ForgeQueryOrchestrationPolicyTenantPosture,
        lower_authority_attachment: ForgeQueryOrchestrationLowerAuthorityAttachment,
        strategy_attachment: ForgeQueryOrchestrationStrategyAttachment,
        contribution_compatibility: ForgeQueryOrchestrationContributionCompatibility,
        collaborative_extension_posture: ForgeQueryOrchestrationCollaborativeExtensionPosture,
    ) -> Self {
        Self {
            aspect_posture,
            basis_posture,
            policy_tenant_posture,
            lower_authority_attachment,
            strategy_attachment,
            contribution_compatibility,
            collaborative_extension_posture,
        }
    }

    pub fn aspect_posture(&self) -> ForgeQueryOrchestrationAspectPosture {
        self.aspect_posture
    }

    pub fn basis_posture(&self) -> ForgeQueryOrchestrationBasisPosture {
        self.basis_posture
    }

    pub fn policy_tenant_posture(&self) -> ForgeQueryOrchestrationPolicyTenantPosture {
        self.policy_tenant_posture
    }

    pub fn lower_authority_attachment(&self) -> ForgeQueryOrchestrationLowerAuthorityAttachment {
        self.lower_authority_attachment
    }

    pub fn strategy_attachment(&self) -> ForgeQueryOrchestrationStrategyAttachment {
        self.strategy_attachment
    }

    pub fn contribution_compatibility(&self) -> &ForgeQueryOrchestrationContributionCompatibility {
        &self.contribution_compatibility
    }

    pub fn collaborative_extension_posture(
        &self,
    ) -> ForgeQueryOrchestrationCollaborativeExtensionPosture {
        self.collaborative_extension_posture
    }

    fn digest_parts(&self) -> Vec<String> {
        vec![
            self.aspect_posture.as_str().to_string(),
            self.basis_posture.as_str().to_string(),
            self.policy_tenant_posture.as_str().to_string(),
            self.lower_authority_attachment.as_str().to_string(),
            self.strategy_attachment.as_str().to_string(),
            self.contribution_compatibility.as_digest_fragment(),
            self.collaborative_extension_posture.as_str().to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryOrchestrationSurfaceRow {
    public_name: &'static str,
    canonical_base_name: &'static str,
    family: ForgeQueryOrchestrationSurfaceFamily,
    visibility: ForgeQueryOrchestrationSurfaceVisibility,
    ordinary_outcome_supported: bool,
    binding_projection: ForgeQueryOrchestrationBindingProjection,
    proof_contract: ForgeQueryOrchestrationProofContract,
    semantic_profile: ForgeQueryOrchestrationSemanticProfile,
    doc_reference: ForgeQueryOrchestrationSurfaceDocReference,
    certification_reference: ForgeQueryOrchestrationSurfaceCertificationReference,
    row_digest: String,
}

impl ForgeQueryOrchestrationSurfaceRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        public_name: &'static str,
        canonical_base_name: &'static str,
        family: ForgeQueryOrchestrationSurfaceFamily,
        visibility: ForgeQueryOrchestrationSurfaceVisibility,
        ordinary_outcome_supported: bool,
        binding_projection: ForgeQueryOrchestrationBindingProjection,
        proof_contract: ForgeQueryOrchestrationProofContract,
        semantic_profile: ForgeQueryOrchestrationSemanticProfile,
        doc_reference: ForgeQueryOrchestrationSurfaceDocReference,
        certification_reference: ForgeQueryOrchestrationSurfaceCertificationReference,
    ) -> Self {
        let mut digest_parts = vec![
            "forge_query_orchestration_surface_row_v2".to_string(),
            public_name.to_string(),
            canonical_base_name.to_string(),
            family.as_str().to_string(),
            visibility.as_str().to_string(),
            ordinary_outcome_supported.to_string(),
            binding_projection.as_str().to_string(),
            proof_contract.checked_type_name().to_string(),
            proof_contract.proof_type_name().to_string(),
            proof_contract.transcript_family().as_str().to_string(),
            proof_contract.checked_topology_kind().as_str().to_string(),
            proof_contract.support_surface().as_str().to_string(),
        ];
        digest_parts.extend(semantic_profile.digest_parts());
        digest_parts.extend([
            doc_reference.path().to_string(),
            doc_reference.section().to_string(),
            certification_reference.suite().to_string(),
            certification_reference.command().to_string(),
        ]);
        let row_digest = crate::identity::hash_parts(&digest_parts);
        Self {
            public_name,
            canonical_base_name,
            family,
            visibility,
            ordinary_outcome_supported,
            binding_projection,
            proof_contract,
            semantic_profile,
            doc_reference,
            certification_reference,
            row_digest,
        }
    }

    pub fn public_name(&self) -> &'static str {
        self.public_name
    }

    pub fn canonical_base_name(&self) -> &'static str {
        self.canonical_base_name
    }

    pub fn family(&self) -> ForgeQueryOrchestrationSurfaceFamily {
        self.family
    }

    pub fn visibility(&self) -> ForgeQueryOrchestrationSurfaceVisibility {
        self.visibility
    }

    pub fn ordinary_outcome_supported(&self) -> bool {
        self.ordinary_outcome_supported
    }

    pub fn binding_projection(&self) -> ForgeQueryOrchestrationBindingProjection {
        self.binding_projection
    }

    pub fn proof_contract(&self) -> &ForgeQueryOrchestrationProofContract {
        &self.proof_contract
    }

    pub fn semantic_profile(&self) -> &ForgeQueryOrchestrationSemanticProfile {
        &self.semantic_profile
    }

    pub fn aspect_posture(&self) -> ForgeQueryOrchestrationAspectPosture {
        self.semantic_profile.aspect_posture()
    }

    pub fn basis_posture(&self) -> ForgeQueryOrchestrationBasisPosture {
        self.semantic_profile.basis_posture()
    }

    pub fn policy_tenant_posture(&self) -> ForgeQueryOrchestrationPolicyTenantPosture {
        self.semantic_profile.policy_tenant_posture()
    }

    pub fn lower_authority_attachment(&self) -> ForgeQueryOrchestrationLowerAuthorityAttachment {
        self.semantic_profile.lower_authority_attachment()
    }

    pub fn strategy_attachment(&self) -> ForgeQueryOrchestrationStrategyAttachment {
        self.semantic_profile.strategy_attachment()
    }

    pub fn contribution_compatibility(&self) -> &ForgeQueryOrchestrationContributionCompatibility {
        self.semantic_profile.contribution_compatibility()
    }

    pub fn collaborative_extension_posture(
        &self,
    ) -> ForgeQueryOrchestrationCollaborativeExtensionPosture {
        self.semantic_profile.collaborative_extension_posture()
    }

    pub fn doc_reference(&self) -> ForgeQueryOrchestrationSurfaceDocReference {
        self.doc_reference
    }

    pub fn certification_reference(&self) -> ForgeQueryOrchestrationSurfaceCertificationReference {
        self.certification_reference
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryOrchestrationSurfaceInventory {
    rows: Vec<ForgeQueryOrchestrationSurfaceRow>,
    inventory_digest: String,
}

impl ForgeQueryOrchestrationSurfaceInventory {
    pub(crate) fn new(rows: Vec<ForgeQueryOrchestrationSurfaceRow>) -> Self {
        let inventory_digest = crate::identity::hash_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            inventory_digest,
        }
    }

    pub fn current() -> Self {
        super::current::forge_query_current_orchestration_surface_inventory()
    }

    pub fn rows(&self) -> &[ForgeQueryOrchestrationSurfaceRow] {
        &self.rows
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }

    pub fn row_for_public_name(
        &self,
        public_name: &str,
    ) -> Option<&ForgeQueryOrchestrationSurfaceRow> {
        self.rows
            .iter()
            .find(|row| row.public_name() == public_name)
    }

    pub fn rows_for_family(
        &self,
        family: ForgeQueryOrchestrationSurfaceFamily,
    ) -> Vec<&ForgeQueryOrchestrationSurfaceRow> {
        self.rows
            .iter()
            .filter(|row| row.family() == family)
            .collect()
    }
}
