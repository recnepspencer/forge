use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_predicate::domain::{
    PlanarPredicateAuthorityDeclarationFamily, PlanarPredicateAuthorityQueryDomain,
};
use crate::planar_contracts::predicate_authority::{
    canonical_cyclic_orient2d_points, canonical_planar_coordinate_bits, PlanarPredicateInputBasis,
    PlanarPredicateKind,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPredicateAuthorityCase {
    predicate_kind: PlanarPredicateKind,
    input_basis: PlanarPredicateInputBasis,
}

impl PlanarPredicateAuthorityCase {
    pub fn orient2d(input_basis: PlanarPredicateInputBasis) -> Self {
        Self {
            predicate_kind: PlanarPredicateKind::Orient2d,
            input_basis,
        }
    }

    pub fn predicate_kind(&self) -> PlanarPredicateKind {
        self.predicate_kind
    }

    pub fn input_basis(&self) -> &PlanarPredicateInputBasis {
        &self.input_basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPredicateAuthorityEntry {
    case: PlanarPredicateAuthorityCase,
}

impl PlanarPredicateAuthorityEntry {
    pub fn case(&self) -> &PlanarPredicateAuthorityCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarPredicateAuthorityQueryDomain>
    for PlanarPredicateAuthorityEntry
{
    type Family = PlanarPredicateAuthorityDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let basis = self.case.input_basis();
        let canonical_points = match self.case.predicate_kind() {
            PlanarPredicateKind::Orient2d => {
                canonical_cyclic_orient2d_points(basis.projected_points())
            }
        };
        canonical_entries_from_points(self.case.predicate_kind(), basis, canonical_points)
    }
}

pub fn planar_predicate_authority_entry(
    case: PlanarPredicateAuthorityCase,
) -> PlanarPredicateAuthorityEntry {
    PlanarPredicateAuthorityEntry { case }
}

fn canonical_entries_from_points(
    kind: PlanarPredicateKind,
    basis: &PlanarPredicateInputBasis,
    canonical_points: [[f64; 2]; 3],
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    let mut entries = Vec::new();
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "geometry.planar_predicate.kind",
        kind.as_str(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "geometry.planar_predicate.local_frame",
        basis.local_frame_identity(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "geometry.planar_predicate.topology_basis",
        basis.topology_basis_identity(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "geometry.planar_predicate.movement_rotation",
        basis.movement_rotation_posture_identity(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "geometry.planar_predicate.tolerance_policy",
        basis.tolerance_policy_identity(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "geometry.planar_predicate.coincidence_policy",
        basis.coincidence_policy().as_str(),
    ));
    for (point_index, point) in canonical_points.iter().enumerate() {
        for (axis_index, coordinate) in point.iter().enumerate() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                format!("geometry.planar_predicate.point.{point_index}.{axis_index}"),
                canonical_planar_coordinate_bits(*coordinate).to_string(),
            ));
        }
    }
    entries
}
