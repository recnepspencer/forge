use worth_store_aspect_native::StorePhysicalBoundaryWitness;

use crate::physical_runtime::work::{
    PhysicalSignalAspectDeclaration, PhysicalWorkSemanticBasis, PhysicalWorkSignalFamily,
};

use super::{contract, dependency_and_output_declaration, patch_fact};

pub(super) const FRAME_WRITEBACK_ASPECT_KEY: &str = "store.physical.record.frame-writeback-basis";

pub(super) struct InstalledFrameWritebackSemantics {
    pub(super) basis: PhysicalWorkSemanticBasis,
    pub(super) declaration: PhysicalSignalAspectDeclaration,
}

pub(super) fn install(witness: StorePhysicalBoundaryWitness) -> InstalledFrameWritebackSemantics {
    let (contract, identity, admission) = contract(FRAME_WRITEBACK_ASPECT_KEY, 1_306, witness);
    let basis = PhysicalWorkSemanticBasis::mutation(
        patch_fact(
            &contract,
            identity,
            witness,
            "frame-writeback-work-admitted",
        ),
        admission.clone(),
    )
    .expect("frame-writeback patch and contract are constructed together");
    InstalledFrameWritebackSemantics {
        basis,
        declaration: dependency_and_output_declaration(
            admission,
            PhysicalWorkSignalFamily::ExactWriteback,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_writeback_uses_the_dedicated_mutation_basis() {
        let installed = install(super::super::security_admission::physical_witness());
        assert_eq!(
            installed.basis.aspect_identity().aspect_key().as_str(),
            FRAME_WRITEBACK_ASPECT_KEY
        );
    }
}
