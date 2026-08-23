use std::collections::BTreeSet;
use std::sync::Arc;

use worth_foundational::facade::{
    AspectBinding, AspectContract, CanonicalBasisReadyArtifact, FieldKey,
};

use super::WorthQueryInstalledApplicationAspectLocus;

/// Installed, immutable native contract for one application entity/aspect locus.
#[derive(Clone)]
pub struct WorthQueryInstalledApplicationAspectContract {
    locus: WorthQueryInstalledApplicationAspectLocus,
    contract: AspectContract,
    fields: BTreeSet<FieldKey>,
    binding: AspectBinding,
    canonical_contract_basis: Arc<CanonicalBasisReadyArtifact>,
    canonical_contract_material: Arc<str>,
}

impl WorthQueryInstalledApplicationAspectContract {
    pub(crate) fn new(
        locus: WorthQueryInstalledApplicationAspectLocus,
        contract: AspectContract,
        fields: BTreeSet<FieldKey>,
        binding: AspectBinding,
        canonical_contract_basis: CanonicalBasisReadyArtifact,
        canonical_contract_material: String,
    ) -> Self {
        Self {
            locus,
            contract,
            fields,
            binding,
            canonical_contract_basis: Arc::new(canonical_contract_basis),
            canonical_contract_material: Arc::from(canonical_contract_material),
        }
    }

    pub fn locus(&self) -> &WorthQueryInstalledApplicationAspectLocus {
        &self.locus
    }

    pub fn contract(&self) -> &AspectContract {
        &self.contract
    }

    pub fn fields(&self) -> impl ExactSizeIterator<Item = &FieldKey> {
        self.fields.iter()
    }

    pub fn contains_field(&self, field: &FieldKey) -> bool {
        self.fields.contains(field)
    }

    pub fn binding(&self) -> &AspectBinding {
        &self.binding
    }

    pub fn canonical_contract_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.canonical_contract_basis.as_ref()
    }

    pub(crate) fn retain_canonical_contract_basis(&self) -> Arc<CanonicalBasisReadyArtifact> {
        Arc::clone(&self.canonical_contract_basis)
    }

    pub fn canonical_contract_material(&self) -> &str {
        self.canonical_contract_material.as_ref()
    }

    pub(crate) fn retain_canonical_contract_material(&self) -> Arc<str> {
        Arc::clone(&self.canonical_contract_material)
    }
}

impl std::fmt::Debug for WorthQueryInstalledApplicationAspectContract {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryInstalledApplicationAspectContract")
            .field("locus", &self.locus)
            .field("contract", &self.contract)
            .field("fields", &self.fields)
            .field("binding", &self.binding)
            .finish_non_exhaustive()
    }
}
