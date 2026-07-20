use super::aspect::WorthQueryOrchestrationAspectPosture;
use super::authority::{
    WorthQueryOrchestrationBasisPosture, WorthQueryOrchestrationCollaborativeExtensionPosture,
    WorthQueryOrchestrationLowerAuthorityAttachment, WorthQueryOrchestrationPolicyTenantPosture,
};
use super::certification::WorthQueryOrchestrationSurfaceCertificationReference;
use super::contribution::WorthQueryOrchestrationContributionCompatibility;
use super::docs::WorthQueryOrchestrationSurfaceDocReference;
use super::family::{
    WorthQueryOrchestrationBindingProjection, WorthQueryOrchestrationSurfaceFamily,
    WorthQueryOrchestrationSurfaceVisibility,
};
use super::strategy::WorthQueryOrchestrationStrategyAttachment;
use super::transcript::WorthQueryOrchestrationProofContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOrchestrationSemanticProfile {
    aspect_posture: WorthQueryOrchestrationAspectPosture,
    basis_posture: WorthQueryOrchestrationBasisPosture,
    policy_tenant_posture: WorthQueryOrchestrationPolicyTenantPosture,
    lower_authority_attachment: WorthQueryOrchestrationLowerAuthorityAttachment,
    strategy_attachment: WorthQueryOrchestrationStrategyAttachment,
    contribution_compatibility: WorthQueryOrchestrationContributionCompatibility,
    collaborative_extension_posture: WorthQueryOrchestrationCollaborativeExtensionPosture,
}

impl WorthQueryOrchestrationSemanticProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        aspect_posture: WorthQueryOrchestrationAspectPosture,
        basis_posture: WorthQueryOrchestrationBasisPosture,
        policy_tenant_posture: WorthQueryOrchestrationPolicyTenantPosture,
        lower_authority_attachment: WorthQueryOrchestrationLowerAuthorityAttachment,
        strategy_attachment: WorthQueryOrchestrationStrategyAttachment,
        contribution_compatibility: WorthQueryOrchestrationContributionCompatibility,
        collaborative_extension_posture: WorthQueryOrchestrationCollaborativeExtensionPosture,
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

    pub fn aspect_posture(&self) -> WorthQueryOrchestrationAspectPosture {
        self.aspect_posture
    }

    pub fn basis_posture(&self) -> WorthQueryOrchestrationBasisPosture {
        self.basis_posture
    }

    pub fn policy_tenant_posture(&self) -> WorthQueryOrchestrationPolicyTenantPosture {
        self.policy_tenant_posture
    }

    pub fn lower_authority_attachment(&self) -> WorthQueryOrchestrationLowerAuthorityAttachment {
        self.lower_authority_attachment
    }

    pub fn strategy_attachment(&self) -> WorthQueryOrchestrationStrategyAttachment {
        self.strategy_attachment
    }

    pub fn contribution_compatibility(&self) -> &WorthQueryOrchestrationContributionCompatibility {
        &self.contribution_compatibility
    }

    pub fn collaborative_extension_posture(
        &self,
    ) -> WorthQueryOrchestrationCollaborativeExtensionPosture {
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
pub struct WorthQueryOrchestrationSurfaceRow {
    public_name: &'static str,
    canonical_base_name: &'static str,
    family: WorthQueryOrchestrationSurfaceFamily,
    visibility: WorthQueryOrchestrationSurfaceVisibility,
    ordinary_outcome_supported: bool,
    binding_projection: WorthQueryOrchestrationBindingProjection,
    proof_contract: WorthQueryOrchestrationProofContract,
    semantic_profile: WorthQueryOrchestrationSemanticProfile,
    doc_reference: WorthQueryOrchestrationSurfaceDocReference,
    certification_reference: WorthQueryOrchestrationSurfaceCertificationReference,
    row_digest: String,
}

impl WorthQueryOrchestrationSurfaceRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        public_name: &'static str,
        canonical_base_name: &'static str,
        family: WorthQueryOrchestrationSurfaceFamily,
        visibility: WorthQueryOrchestrationSurfaceVisibility,
        ordinary_outcome_supported: bool,
        binding_projection: WorthQueryOrchestrationBindingProjection,
        proof_contract: WorthQueryOrchestrationProofContract,
        semantic_profile: WorthQueryOrchestrationSemanticProfile,
        doc_reference: WorthQueryOrchestrationSurfaceDocReference,
        certification_reference: WorthQueryOrchestrationSurfaceCertificationReference,
    ) -> Self {
        let mut digest_parts = vec![
            "worth_query_orchestration_surface_row_v2".to_string(),
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

    pub fn family(&self) -> WorthQueryOrchestrationSurfaceFamily {
        self.family
    }

    pub fn visibility(&self) -> WorthQueryOrchestrationSurfaceVisibility {
        self.visibility
    }

    pub fn ordinary_outcome_supported(&self) -> bool {
        self.ordinary_outcome_supported
    }

    pub fn binding_projection(&self) -> WorthQueryOrchestrationBindingProjection {
        self.binding_projection
    }

    pub fn proof_contract(&self) -> &WorthQueryOrchestrationProofContract {
        &self.proof_contract
    }

    pub fn semantic_profile(&self) -> &WorthQueryOrchestrationSemanticProfile {
        &self.semantic_profile
    }

    pub fn aspect_posture(&self) -> WorthQueryOrchestrationAspectPosture {
        self.semantic_profile.aspect_posture()
    }

    pub fn basis_posture(&self) -> WorthQueryOrchestrationBasisPosture {
        self.semantic_profile.basis_posture()
    }

    pub fn policy_tenant_posture(&self) -> WorthQueryOrchestrationPolicyTenantPosture {
        self.semantic_profile.policy_tenant_posture()
    }

    pub fn lower_authority_attachment(&self) -> WorthQueryOrchestrationLowerAuthorityAttachment {
        self.semantic_profile.lower_authority_attachment()
    }

    pub fn strategy_attachment(&self) -> WorthQueryOrchestrationStrategyAttachment {
        self.semantic_profile.strategy_attachment()
    }

    pub fn contribution_compatibility(&self) -> &WorthQueryOrchestrationContributionCompatibility {
        self.semantic_profile.contribution_compatibility()
    }

    pub fn collaborative_extension_posture(
        &self,
    ) -> WorthQueryOrchestrationCollaborativeExtensionPosture {
        self.semantic_profile.collaborative_extension_posture()
    }

    pub fn doc_reference(&self) -> WorthQueryOrchestrationSurfaceDocReference {
        self.doc_reference
    }

    pub fn certification_reference(&self) -> WorthQueryOrchestrationSurfaceCertificationReference {
        self.certification_reference
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOrchestrationSurfaceInventory {
    rows: Vec<WorthQueryOrchestrationSurfaceRow>,
    inventory_digest: String,
}

impl WorthQueryOrchestrationSurfaceInventory {
    pub(crate) fn new(rows: Vec<WorthQueryOrchestrationSurfaceRow>) -> Self {
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
        super::current::worth_query_current_orchestration_surface_inventory()
    }

    pub fn rows(&self) -> &[WorthQueryOrchestrationSurfaceRow] {
        &self.rows
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }

    pub fn row_for_public_name(
        &self,
        public_name: &str,
    ) -> Option<&WorthQueryOrchestrationSurfaceRow> {
        self.rows
            .iter()
            .find(|row| row.public_name() == public_name)
    }

    pub fn rows_for_family(
        &self,
        family: WorthQueryOrchestrationSurfaceFamily,
    ) -> Vec<&WorthQueryOrchestrationSurfaceRow> {
        self.rows
            .iter()
            .filter(|row| row.family() == family)
            .collect()
    }
}
