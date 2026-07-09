use crate::assumption::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis};

use super::{Lowered, Recipe};

impl<T, B> Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>> {
    pub fn lowered_payload(&self) -> &T {
        self.payload()
    }

    pub fn lowered_basis(&self) -> &FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>> {
        self.basis()
    }
}
