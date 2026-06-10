use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_precision::domain::{
    PlanarPrecisionCertificationDeclarationFamily, PlanarPrecisionCertificationQueryDomain,
};
use crate::planar_contracts::precision_basis::PlanarPrecisionBasis;
use crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPrecisionCertificationCase {
    predicate_receipt: PlanarPredicateFactReceipt,
    basis: PlanarPrecisionBasis,
}

impl PlanarPrecisionCertificationCase {
    pub fn from_predicate_receipt(
        predicate_receipt: PlanarPredicateFactReceipt,
        basis: PlanarPrecisionBasis,
    ) -> Self {
        Self {
            predicate_receipt,
            basis,
        }
    }

    pub fn predicate_receipt(&self) -> &PlanarPredicateFactReceipt {
        &self.predicate_receipt
    }

    pub fn basis(&self) -> &PlanarPrecisionBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPrecisionCertificationEntry {
    case: PlanarPrecisionCertificationCase,
}

impl PlanarPrecisionCertificationEntry {
    pub fn case(&self) -> &PlanarPrecisionCertificationCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarPrecisionCertificationQueryDomain>
    for PlanarPrecisionCertificationEntry
{
    type Family = PlanarPrecisionCertificationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let basis = self.case.basis();
        let precision = self.case.predicate_receipt().precision_escalation();
        vec![
            entry(
                "geometry.planar_precision.predicate_fact",
                basis.predicate_fact_digest(),
            ),
            entry(
                "geometry.planar_precision.local_frame",
                basis.local_frame_identity(),
            ),
            entry(
                "geometry.planar_precision.topology_basis",
                basis.topology_basis_identity(),
            ),
            entry(
                "geometry.planar_precision.movement_rotation",
                basis.movement_rotation_posture_identity(),
            ),
            entry(
                "geometry.planar_precision.tolerance_policy",
                basis.tolerance_policy_identity(),
            ),
            entry(
                "geometry.planar_precision.local_feature_scale",
                basis.local_feature_scale_order().to_string(),
            ),
            entry(
                "geometry.planar_precision.world_magnitude",
                basis.world_magnitude_order().to_string(),
            ),
            entry(
                "geometry.planar_precision.normalization_scale",
                basis.normalization_scale().to_bits().to_string(),
            ),
            entry(
                "geometry.planar_precision.resolved_at",
                format!("{:?}", precision.get_resolved_at()),
            ),
            entry(
                "geometry.planar_precision.float_agreed",
                precision.get_float_agreed().to_string(),
            ),
            entry(
                "geometry.planar_precision.expansion_length",
                format!("{:?}", precision.get_expansion_length()),
            ),
            entry(
                "geometry.planar_precision.target",
                precision.get_target_triple(),
            ),
        ]
    }
}

pub fn planar_precision_certification_entry(
    case: PlanarPrecisionCertificationCase,
) -> PlanarPrecisionCertificationEntry {
    PlanarPrecisionCertificationEntry { case }
}

fn entry(key: impl Into<String>, value: impl Into<String>) -> ForgeQueryDeclarationCanonicalEntry {
    ForgeQueryDeclarationCanonicalEntry::text(key, value)
}
