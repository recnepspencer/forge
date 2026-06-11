use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_predicate_consumption::domain::{
    PredicateCertificateConsumptionDeclarationFamily, PredicateCertificateConsumptionQueryDomain,
};
use crate::planar_contracts::predicate_consumption::{
    predicate_certificate_consumption_identity_entries, PredicateCertificateConsumptionBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PredicateCertificateConsumptionCase {
    basis: PredicateCertificateConsumptionBasis,
}

impl PredicateCertificateConsumptionCase {
    pub fn from_basis(basis: PredicateCertificateConsumptionBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PredicateCertificateConsumptionBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PredicateCertificateConsumptionEntry {
    case: PredicateCertificateConsumptionCase,
}

impl PredicateCertificateConsumptionEntry {
    pub fn case(&self) -> &PredicateCertificateConsumptionCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PredicateCertificateConsumptionQueryDomain>
    for PredicateCertificateConsumptionEntry
{
    type Family = PredicateCertificateConsumptionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        predicate_certificate_consumption_identity_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn predicate_certificate_consumption_entry(
    case: PredicateCertificateConsumptionCase,
) -> PredicateCertificateConsumptionEntry {
    PredicateCertificateConsumptionEntry { case }
}
