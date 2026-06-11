use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_contract_bundle::domain::{
    PlanarContractBundleValidationDeclarationFamily, PlanarContractBundleValidationQueryDomain,
};
use crate::planar_contracts::contract_bundle::{
    planar_contract_bundle_identity_entries, PlanarContractBundleValidationBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarContractBundleValidationCase {
    basis: PlanarContractBundleValidationBasis,
    planar_neighborhood_identity: String,
}

impl PlanarContractBundleValidationCase {
    pub fn from_boolean_readiness_bundle(
        basis: PlanarContractBundleValidationBasis,
        planar_neighborhood_identity: impl Into<String>,
    ) -> Self {
        Self {
            basis,
            planar_neighborhood_identity: planar_neighborhood_identity.into(),
        }
    }

    pub fn basis(&self) -> &PlanarContractBundleValidationBasis {
        &self.basis
    }

    pub fn planar_neighborhood_identity(&self) -> &str {
        &self.planar_neighborhood_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarContractBundleValidationEntry {
    case: PlanarContractBundleValidationCase,
}

impl PlanarContractBundleValidationEntry {
    pub fn case(&self) -> &PlanarContractBundleValidationCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarContractBundleValidationQueryDomain>
    for PlanarContractBundleValidationEntry
{
    type Family = PlanarContractBundleValidationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = planar_contract_bundle_identity_entries(self.case.basis());
        entries.push(
            crate::planar_contracts::contract_bundle::PlanarContractBundleIdentityEntry::new(
                "planar_neighborhood",
                self.case.planar_neighborhood_identity(),
            ),
        );
        entries.sort_by(|left, right| {
            left.locus()
                .cmp(right.locus())
                .then_with(|| left.value().cmp(right.value()))
        });
        entries
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn planar_contract_bundle_validation_entry(
    case: PlanarContractBundleValidationCase,
) -> PlanarContractBundleValidationEntry {
    PlanarContractBundleValidationEntry { case }
}
