use serde::Serialize;

use crate::derived_topology::invalidation_plan::inventory::DerivedInvalidationPhaseTwoSeed;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationResidueRow {
    residue_label: String,
    capped_count: usize,
    row_digest: String,
}

impl DerivedInvalidationResidueRow {
    pub(crate) fn from_phase_two_seed(phase_two_seed: &DerivedInvalidationPhaseTwoSeed) -> Self {
        Self::new(
            "phase-two-certification-bootstrap-capped-residue",
            phase_two_seed.capped_residue_count(),
            phase_two_seed.seed_digest(),
        )
    }

    fn new(residue_label: impl Into<String>, capped_count: usize, authority_digest: &str) -> Self {
        let residue_label = residue_label.into();
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-residue-row:v1".to_string(),
            format!("label:{residue_label}"),
            format!("capped-count:{capped_count}"),
            format!("authority:{authority_digest}"),
        ]);
        Self {
            residue_label,
            capped_count,
            row_digest,
        }
    }

    pub fn residue_label(&self) -> &str {
        &self.residue_label
    }

    pub const fn capped_count(&self) -> usize {
        self.capped_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
