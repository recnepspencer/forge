use crate::runtime::{WorthUiHandlePlanGeneration, WorthUiRuntimeHandleAllocationBasis};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeHandleAllocationReceipt {
    basis_digest: u64,
    plan_generation: WorthUiHandlePlanGeneration,
}

impl WorthUiRuntimeHandleAllocationReceipt {
    pub(crate) fn from_basis(basis: &WorthUiRuntimeHandleAllocationBasis) -> Self {
        let basis_digest = basis.digest();
        Self {
            basis_digest,
            plan_generation: WorthUiHandlePlanGeneration::new(
                basis_digest.rotate_left(17) ^ 0x51a7_e10c_a110_cafe,
            ),
        }
    }

    pub fn certifies_basis(self, basis: &WorthUiRuntimeHandleAllocationBasis) -> bool {
        self.basis_digest == basis.digest()
    }

    pub fn basis_digest(self) -> u64 {
        self.basis_digest
    }

    pub fn plan_generation(self) -> WorthUiHandlePlanGeneration {
        self.plan_generation
    }
}
