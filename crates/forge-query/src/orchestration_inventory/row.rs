use super::certification::ForgeQueryOrchestrationSurfaceCertificationReference;
use super::docs::ForgeQueryOrchestrationSurfaceDocReference;
use super::family::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceVisibility,
};
use super::transcript::ForgeQueryOrchestrationProofContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryOrchestrationSurfaceRow {
    public_name: &'static str,
    canonical_base_name: &'static str,
    family: ForgeQueryOrchestrationSurfaceFamily,
    visibility: ForgeQueryOrchestrationSurfaceVisibility,
    ordinary_outcome_supported: bool,
    binding_projection: ForgeQueryOrchestrationBindingProjection,
    proof_contract: ForgeQueryOrchestrationProofContract,
    doc_reference: ForgeQueryOrchestrationSurfaceDocReference,
    certification_reference: ForgeQueryOrchestrationSurfaceCertificationReference,
    row_digest: String,
}

impl ForgeQueryOrchestrationSurfaceRow {
    pub(crate) fn new(
        public_name: &'static str,
        canonical_base_name: &'static str,
        family: ForgeQueryOrchestrationSurfaceFamily,
        visibility: ForgeQueryOrchestrationSurfaceVisibility,
        ordinary_outcome_supported: bool,
        binding_projection: ForgeQueryOrchestrationBindingProjection,
        proof_contract: ForgeQueryOrchestrationProofContract,
        doc_reference: ForgeQueryOrchestrationSurfaceDocReference,
        certification_reference: ForgeQueryOrchestrationSurfaceCertificationReference,
    ) -> Self {
        let row_digest = crate::identity::hash_parts(&[
            "forge_query_orchestration_surface_row_v1".to_string(),
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
            doc_reference.path().to_string(),
            doc_reference.section().to_string(),
            certification_reference.suite().to_string(),
            certification_reference.command().to_string(),
        ]);
        Self {
            public_name,
            canonical_base_name,
            family,
            visibility,
            ordinary_outcome_supported,
            binding_projection,
            proof_contract,
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
