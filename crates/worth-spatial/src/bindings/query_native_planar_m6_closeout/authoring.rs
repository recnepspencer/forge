use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_m6_closeout::domain::{
    M6PlanarCloseoutDeclarationFamily, M6PlanarCloseoutQueryDomain,
};
use crate::planar_contracts::m6_closeout::M6PlanarCloseoutBasis;

#[derive(Clone, Debug, PartialEq)]
pub struct M6PlanarCloseoutCase {
    basis: M6PlanarCloseoutBasis,
}

impl M6PlanarCloseoutCase {
    pub fn from_basis(basis: M6PlanarCloseoutBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &M6PlanarCloseoutBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct M6PlanarCloseoutEntry {
    case: M6PlanarCloseoutCase,
}

impl M6PlanarCloseoutEntry {
    pub fn case(&self) -> &M6PlanarCloseoutCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<M6PlanarCloseoutQueryDomain> for M6PlanarCloseoutEntry {
    type Family = M6PlanarCloseoutDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let basis = self.case.basis();
        let mut entries = vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.planar_m6_closeout.m7_readiness",
                basis.readiness().readiness_digest(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.planar_m6_closeout.query.declaration",
                basis.query_boundary().declaration_digest(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.planar_m6_closeout.query.envelope",
                basis.query_boundary().envelope_digest(),
            ),
        ];
        entries.extend(basis.premetaboss_rows().iter().map(|row| {
            ForgeQueryDeclarationCanonicalEntry::text(
                format!(
                    "geometry.planar_m6_closeout.premetaboss.{}",
                    row.family().as_str()
                ),
                row.evidence_digest(),
            )
        }));
        entries.extend(basis.legacy_deletion_rows().iter().map(|row| {
            ForgeQueryDeclarationCanonicalEntry::text(
                format!(
                    "geometry.planar_m6_closeout.legacy_deletion.{}",
                    row.family().as_str()
                ),
                row.evidence_digest(),
            )
        }));
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            "geometry.planar_m6_closeout.legacy_fixture_fence",
            basis.legacy_fixture_fence().fence_digest(),
        ));
        entries
    }
}

pub fn m6_planar_closeout_entry(case: M6PlanarCloseoutCase) -> M6PlanarCloseoutEntry {
    M6PlanarCloseoutEntry { case }
}
