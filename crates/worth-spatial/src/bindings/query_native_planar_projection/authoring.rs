use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_projection::domain::{
    ProjectPointToCertifiedPlane2DDeclarationFamily, ProjectPointToCertifiedPlane2DQueryDomain,
};
use crate::planar_contracts::projection_2d::{
    project_point_to_certified_plane_2d_identity_entries, ProjectPointToCertifiedPlane2DBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DCase {
    basis: ProjectPointToCertifiedPlane2DBasis,
}

impl ProjectPointToCertifiedPlane2DCase {
    pub fn from_local_frame(basis: ProjectPointToCertifiedPlane2DBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &ProjectPointToCertifiedPlane2DBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DEntry {
    case: ProjectPointToCertifiedPlane2DCase,
}

impl ProjectPointToCertifiedPlane2DEntry {
    pub fn case(&self) -> &ProjectPointToCertifiedPlane2DCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<ProjectPointToCertifiedPlane2DQueryDomain>
    for ProjectPointToCertifiedPlane2DEntry
{
    type Family = ProjectPointToCertifiedPlane2DDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        project_point_to_certified_plane_2d_identity_entries(self.case.basis())
            .into_iter()
            .map(|entry| canonical_entry(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn project_point_to_certified_plane_2d_entry(
    case: ProjectPointToCertifiedPlane2DCase,
) -> ProjectPointToCertifiedPlane2DEntry {
    ProjectPointToCertifiedPlane2DEntry { case }
}

fn canonical_entry(
    key: impl Into<String>,
    value: impl Into<String>,
) -> ForgeQueryDeclarationCanonicalEntry {
    ForgeQueryDeclarationCanonicalEntry::text(key, value)
}
