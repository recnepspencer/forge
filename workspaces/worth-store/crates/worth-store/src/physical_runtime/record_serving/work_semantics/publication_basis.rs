use worth_store_aspect_native::StorePhysicalBoundaryWitness;

use crate::physical_runtime::work::{
    PhysicalSignalAspectDeclaration, PhysicalWorkSemanticBasis, PhysicalWorkSignalFamily,
};

use super::{contract, dependency_and_output_declaration, patch_fact};
use crate::physical_runtime::RecordPublicationStage;

const PUBLICATION_ASPECT_KEY: &str = "store.physical.record.publication-basis";

pub(super) struct InstalledPublicationSemantics {
    pub(super) bases: RecordPublicationSemanticBases,
    pub(super) declaration: PhysicalSignalAspectDeclaration,
}

#[derive(Clone, Debug)]
pub(super) struct RecordPublicationSemanticBases {
    pub(super) candidate_data: PhysicalWorkSemanticBasis,
    data_synchronization: PhysicalWorkSemanticBasis,
    payload_manifest: PhysicalWorkSemanticBasis,
    manifest: PhysicalWorkSemanticBasis,
    catalog_candidate: PhysicalWorkSemanticBasis,
    catalog_replacement: PhysicalWorkSemanticBasis,
    namespace_synchronization: PhysicalWorkSemanticBasis,
}

pub(super) fn install(witness: StorePhysicalBoundaryWitness) -> InstalledPublicationSemantics {
    let (contract, identity, admission) = contract(PUBLICATION_ASPECT_KEY, 1_305, witness);
    let bases = RecordPublicationSemanticBases::new(&contract, identity, witness, &admission);
    InstalledPublicationSemantics {
        bases,
        declaration: dependency_and_output_declaration(
            admission,
            PhysicalWorkSignalFamily::Publication,
        ),
    }
}

impl RecordPublicationSemanticBases {
    fn new(
        contract: &worth_foundational::AspectContract,
        identity: worth_store_aspect_native::StoreAspectIdentity,
        witness: StorePhysicalBoundaryWitness,
        admission: &worth_store_aspect_native::StoreAspectContractAdmission,
    ) -> Self {
        let basis = |identity, value| {
            PhysicalWorkSemanticBasis::mutation(
                patch_fact(contract, identity, witness, value),
                admission.clone(),
            )
            .expect("publication patch and contract are constructed together")
        };
        Self {
            candidate_data: basis(identity.clone(), "candidate-data-work-admitted"),
            data_synchronization: basis(identity.clone(), "data-synchronization-work-admitted"),
            payload_manifest: basis(identity.clone(), "payload-manifest-work-admitted"),
            manifest: basis(identity.clone(), "manifest-work-admitted"),
            catalog_candidate: basis(identity.clone(), "catalog-candidate-work-admitted"),
            catalog_replacement: basis(identity.clone(), "catalog-replacement-work-admitted"),
            namespace_synchronization: basis(identity, "namespace-synchronization-work-admitted"),
        }
    }

    pub(super) fn for_stage(&self, stage: RecordPublicationStage) -> PhysicalWorkSemanticBasis {
        match stage {
            RecordPublicationStage::CandidateDataWrite => self.candidate_data.clone(),
            RecordPublicationStage::DataSynchronization => self.data_synchronization.clone(),
            RecordPublicationStage::PayloadManifestSynchronization => self.payload_manifest.clone(),
            RecordPublicationStage::ManifestSynchronization => self.manifest.clone(),
            RecordPublicationStage::CatalogCandidateSynchronization => {
                self.catalog_candidate.clone()
            }
            RecordPublicationStage::CatalogReplacement => self.catalog_replacement.clone(),
            RecordPublicationStage::NamespaceSynchronization => {
                self.namespace_synchronization.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_publication_stage_selects_its_distinct_admitted_native_basis() {
        let installed = install(super::super::security_admission::physical_witness());
        let bases = installed.bases;
        let expected = [
            (
                RecordPublicationStage::CandidateDataWrite,
                bases.candidate_data.clone(),
            ),
            (
                RecordPublicationStage::DataSynchronization,
                bases.data_synchronization.clone(),
            ),
            (
                RecordPublicationStage::PayloadManifestSynchronization,
                bases.payload_manifest.clone(),
            ),
            (
                RecordPublicationStage::ManifestSynchronization,
                bases.manifest.clone(),
            ),
            (
                RecordPublicationStage::CatalogCandidateSynchronization,
                bases.catalog_candidate.clone(),
            ),
            (
                RecordPublicationStage::CatalogReplacement,
                bases.catalog_replacement.clone(),
            ),
            (
                RecordPublicationStage::NamespaceSynchronization,
                bases.namespace_synchronization.clone(),
            ),
        ];
        for (stage, basis) in &expected {
            assert_eq!(bases.for_stage(*stage), *basis);
        }
        for (index, (_, basis)) in expected.iter().enumerate() {
            for (_, other) in &expected[index + 1..] {
                assert_ne!(basis, other);
            }
        }
    }
}
