#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarStructuralIdentityCounters {
    structural_basis_rows_inspected: usize,
    contrast_identity_rows_inspected: usize,
    transform_basis_rows_inspected: usize,
    rejected_coordinate_only_rows: usize,
    rejected_identity_substitution_rows: usize,
}

impl PlanarStructuralIdentityCounters {
    pub(crate) const fn certified(
        structural_basis_rows_inspected: usize,
        contrast_identity_rows_inspected: usize,
        transform_basis_rows_inspected: usize,
    ) -> Self {
        Self {
            structural_basis_rows_inspected,
            contrast_identity_rows_inspected,
            transform_basis_rows_inspected,
            rejected_coordinate_only_rows: 0,
            rejected_identity_substitution_rows: 0,
        }
    }

    pub(crate) const fn rejected_coordinate_only() -> Self {
        Self {
            structural_basis_rows_inspected: 0,
            contrast_identity_rows_inspected: 0,
            transform_basis_rows_inspected: 0,
            rejected_coordinate_only_rows: 1,
            rejected_identity_substitution_rows: 0,
        }
    }

    pub(crate) const fn rejected_identity_substitution() -> Self {
        Self {
            structural_basis_rows_inspected: 0,
            contrast_identity_rows_inspected: 0,
            transform_basis_rows_inspected: 0,
            rejected_coordinate_only_rows: 0,
            rejected_identity_substitution_rows: 1,
        }
    }

    pub fn structural_basis_rows_inspected(self) -> usize {
        self.structural_basis_rows_inspected
    }

    pub fn contrast_identity_rows_inspected(self) -> usize {
        self.contrast_identity_rows_inspected
    }

    pub fn transform_basis_rows_inspected(self) -> usize {
        self.transform_basis_rows_inspected
    }

    pub fn rejected_coordinate_only_rows(self) -> usize {
        self.rejected_coordinate_only_rows
    }

    pub fn rejected_identity_substitution_rows(self) -> usize {
        self.rejected_identity_substitution_rows
    }
}
