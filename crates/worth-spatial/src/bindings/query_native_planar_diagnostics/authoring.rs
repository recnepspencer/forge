use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_diagnostics::domain::{
    PlanarDiagnosticBundleDeclarationFamily, PlanarDiagnosticBundleQueryDomain,
};
use crate::planar_contracts::planar_diagnostics::{
    planar_diagnostic_authority_entries, PlanarDiagnosticBundleBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarDiagnosticBundleCase {
    basis: PlanarDiagnosticBundleBasis,
}

impl PlanarDiagnosticBundleCase {
    pub fn from_basis(basis: PlanarDiagnosticBundleBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PlanarDiagnosticBundleBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarDiagnosticBundleEntry {
    case: PlanarDiagnosticBundleCase,
}

impl PlanarDiagnosticBundleEntry {
    pub fn case(&self) -> &PlanarDiagnosticBundleCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarDiagnosticBundleQueryDomain> for PlanarDiagnosticBundleEntry {
    type Family = PlanarDiagnosticBundleDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        planar_diagnostic_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn planar_diagnostic_bundle_entry(
    case: PlanarDiagnosticBundleCase,
) -> PlanarDiagnosticBundleEntry {
    PlanarDiagnosticBundleEntry { case }
}
