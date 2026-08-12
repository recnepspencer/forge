#[allow(dead_code)]
mod phase_three_support;

use phase_three_support::*;
use worth_store_physical_format::{
    DurableExtentRecordPlacement, PhysicalPageSizeClass, PhysicalRecordFormatDeclaration,
    PhysicalRootRoutingBlock, RootRoutingBlockDenial,
};
use worth_store_recovery_runtime::{
    PhysicalManifestObservationDenial, PhysicalRecoverySourceDenial,
};

#[test]
fn missing_and_undecodable_manifest_blocks_retain_distinct_references() {
    let missing = manifest_case("missing-routing-block", |path| {
        std::fs::remove_file(path).unwrap();
    });
    assert!(matches!(
        missing.evidence().source_denials.as_slice(),
        [PhysicalRecoverySourceDenial::ManifestObservation(
            PhysicalManifestObservationDenial::MissingArtifact { reference }
        )] if reference.generation() == 1 && reference.block() == 1
    ));

    let undecodable = manifest_case("undecodable-routing-block", |path| {
        std::fs::write(path, b"not-a-durable-routing-block").unwrap();
    });
    assert!(matches!(
        undecodable.evidence().source_denials.as_slice(),
        [PhysicalRecoverySourceDenial::ManifestObservation(
            PhysicalManifestObservationDenial::Decode {
                reference,
                denial: RootRoutingBlockDenial::Frame(_),
            }
        )] if reference.generation() == 1 && reference.block() == 1
    ));
}

#[test]
fn manifest_format_denial_retains_the_expected_and_observed_declarations() {
    let blocked = manifest_case("wrong-routing-format", |path| {
        let bytes = std::fs::read(path).unwrap();
        let (block, _) = PhysicalRootRoutingBlock::decode(&bytes, 4).unwrap();
        let alternate = PhysicalRecordFormatDeclaration::builder()
            .page_size(PhysicalPageSizeClass::KiB32)
            .admit()
            .unwrap();
        std::fs::write(path, block.encode(alternate)).unwrap();
    });
    let [PhysicalRecoverySourceDenial::ManifestObservation(
        PhysicalManifestObservationDenial::FormatIdentity {
            reference,
            expected,
            observed,
        },
    )] = blocked.evidence().source_denials.as_slice()
    else {
        panic!("alternate canonical format must retain format evidence")
    };
    assert_eq!(reference.block(), 1);
    assert_eq!(expected.page_size(), PhysicalPageSizeClass::KiB16);
    assert_eq!(observed.page_size(), PhysicalPageSizeClass::KiB32);
}

#[test]
fn tree_and_reference_integrity_denials_preserve_expected_and_observed_values() {
    let wrong_tree = manifest_case("wrong-routing-tree", |path| {
        let bytes = std::fs::read(path).unwrap();
        let (block, format) = PhysicalRootRoutingBlock::decode(&bytes, 4).unwrap();
        let replacement = PhysicalRootRoutingBlock::leaf(
            8,
            block.generation(),
            block.block(),
            block.entries().unwrap().to_vec(),
            4,
        )
        .unwrap();
        std::fs::write(path, replacement.encode(format)).unwrap();
    });
    assert!(matches!(
        wrong_tree.evidence().source_denials.as_slice(),
        [PhysicalRecoverySourceDenial::ManifestObservation(
            PhysicalManifestObservationDenial::TreeIdentity {
                reference,
                expected: 7,
                observed: 8,
            }
        )] if reference.block() == 1
    ));

    let wrong_reference = manifest_case("wrong-routing-reference", |path| {
        let bytes = std::fs::read(path).unwrap();
        let (block, format) = PhysicalRootRoutingBlock::decode(&bytes, 4).unwrap();
        let mut entries = block.entries().unwrap().to_vec();
        let worth_store_physical_format::CurrentPhysicalRecordPlacement::Extent(first) = entries[0]
        else {
            panic!("synthetic routing block must use extent placement")
        };
        entries[0] = worth_store_physical_format::CurrentPhysicalRecordPlacement::Extent(
            DurableExtentRecordPlacement::new(
                first.record(),
                first.extent_cell(),
                first.payload_bytes() + 1,
            )
            .unwrap(),
        );
        let replacement = PhysicalRootRoutingBlock::leaf(
            block.tree_identity(),
            block.generation(),
            block.block(),
            entries,
            4,
        )
        .unwrap();
        std::fs::write(path, replacement.encode(format)).unwrap();
    });
    let [PhysicalRecoverySourceDenial::ManifestObservation(
        PhysicalManifestObservationDenial::ReferenceIntegrity { expected, observed },
    )] = wrong_reference.evidence().source_denials.as_slice()
    else {
        panic!("changed canonical payload must retain reference-integrity evidence")
    };
    assert_eq!(expected.generation(), observed.generation());
    assert_eq!(expected.block(), observed.block());
    assert_eq!(expected.first(), observed.first());
    assert_eq!(expected.last(), observed.last());
    assert_ne!(expected.checksum(), observed.checksum());
}

fn manifest_case(
    name: &str,
    mutate: impl FnOnce(&std::path::Path),
) -> worth_store_recovery_runtime::PhysicalRecoveryBlock {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join(name);
    let store = initialize_store(&root);
    publish_synthetic_nonempty_genesis(&root, store);
    mutate(
        &root
            .join("families")
            .join("records")
            .join("roots")
            .join("root-0000000000000001-block-0000000000000001.manifest"),
    );
    expect_blocked(
        admitted_recovery(&root)
            .discover()
            .unwrap()
            .select()
            .err()
            .expect("manifest observation denial must block"),
    )
}
