use super::*;

#[test]
fn every_publication_stage_selects_its_distinct_admitted_native_basis() {
    let witness = physical_witness();
    let (contract, identity, admission) = contract(PUBLICATION_ASPECT_KEY, 1_305, witness);
    let bases = RecordPublicationSemanticBases::new(&contract, identity, witness, &admission);
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
        let selected = bases.for_stage(*stage);
        assert_eq!(
            selected, *basis,
            "publication stage must select its own admitted native patch"
        );
        assert!(
            selected.shares_admitted_state_with(basis),
            "stage selection must carry admitted proof without reconstructive cloning"
        );
    }
    for (index, (_, basis)) in expected.iter().enumerate() {
        for (_, other) in &expected[index + 1..] {
            assert_ne!(
                basis, other,
                "publication stages must not collapse onto one proxy semantic basis"
            );
        }
    }
}
