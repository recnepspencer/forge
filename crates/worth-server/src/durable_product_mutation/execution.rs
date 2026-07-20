use super::WorthServerDurableProductMutationConclusion;

#[derive(Clone, Debug)]
pub struct WorthServerDurableProductMutationExecution {
    conclusion: WorthServerDurableProductMutationConclusion,
    basis_comparison_count: u64,
}

impl WorthServerDurableProductMutationExecution {
    pub fn before_basis_comparison(
        conclusion: WorthServerDurableProductMutationConclusion,
    ) -> Self {
        Self {
            conclusion,
            basis_comparison_count: 0,
        }
    }

    pub fn after_basis_comparison(conclusion: WorthServerDurableProductMutationConclusion) -> Self {
        Self {
            conclusion,
            basis_comparison_count: 1,
        }
    }

    pub fn conclusion(&self) -> &WorthServerDurableProductMutationConclusion {
        &self.conclusion
    }

    pub fn basis_comparison_count(&self) -> u64 {
        self.basis_comparison_count
    }

    pub(crate) fn into_conclusion(self) -> WorthServerDurableProductMutationConclusion {
        self.conclusion
    }
}
