use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, PhysicalPageSizeClass, PhysicalRecordAccessPolicy,
    PhysicalRecordFormatDeclaration, PhysicalRecordOpen, RecordBootstrapDenial,
    RecordServingRebindReason, RecordServingStaleReason,
};
use worth_store_physical_backend::MediaOperationRole;
use worth_store_physical_format::PhysicalRecordFormatDenial;

use super::{configuration, media, serving_from_initialization};

#[test]
fn current_version_reopens_and_every_unimplemented_version_fails_typed() {
    let parent = tempfile::tempdir().unwrap();
    let (format, _, access) = configuration();
    let current = parent.path().join("current");
    serving_from_initialization(&current).close();
    super::success(open_record_store!(media(&current), |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }))
    .close();

    for (name, relative_path, expected_read_bytes) in [
        ("catalog", "families/records/bootstrap.catalog", 74),
        (
            "root",
            "families/records/roots/root-0000000000000001.manifest",
            434,
        ),
        (
            "free-space",
            "families/records/free-space/free-space-0000000000000001.manifest",
            602,
        ),
    ] {
        let root = parent.path().join(name);
        serving_from_initialization(&root).close();
        let artifact = root.join(relative_path);
        let mut bytes = std::fs::read(&artifact).unwrap();
        bytes[10..12].copy_from_slice(&2_u16.to_le_bytes());
        reseal(&mut bytes);
        std::fs::write(&artifact, bytes).unwrap();

        let media = media(&root);
        let before = media.media_counters();
        let outcome = open_record_store!(media, |durability| PhysicalRecordOpen::new(
            format, access, durability
        ))
        .into_raw();
        let TransitionOutcome::Denied(denial) = outcome else {
            panic!("{name} with an unimplemented version cannot open")
        };
        assert!(matches!(
            denial.reason(),
            RecordBootstrapDenial::UnsupportedPhysicalRecordFormat(unsupported)
                if unsupported.reason() == PhysicalRecordFormatDenial::UnsupportedVersion(2)
        ));
        let media = denial.into_runtime();
        let after = media.media_counters();
        assert_eq!(
            after.completed_bytes_for(MediaOperationRole::PositionedRead)
                - before.completed_bytes_for(MediaOperationRole::PositionedRead),
            expected_read_bytes,
            "{name}",
        );
        media.close();
    }
}

#[test]
fn unsupported_catalog_format_dimensions_localize_before_root_traversal() {
    let parent = tempfile::tempdir().unwrap();
    let (format, _, access) = configuration();
    for (name, offset, encoded, expected) in [
        (
            "old-version",
            10,
            &[0, 0][..],
            PhysicalRecordFormatDenial::UnsupportedVersion(0),
        ),
        (
            "page-size",
            12,
            &[1, 0, 0, 0][..],
            PhysicalRecordFormatDenial::UnsupportedPageBytes(1),
        ),
        (
            "byte-order",
            16,
            &[2][..],
            PhysicalRecordFormatDenial::UnsupportedByteOrder(2),
        ),
        (
            "root-protocol",
            17,
            &[2][..],
            PhysicalRecordFormatDenial::UnsupportedRootProtocol(2),
        ),
        (
            "integrity",
            18,
            &[2][..],
            PhysicalRecordFormatDenial::UnsupportedIntegrity(2),
        ),
        (
            "record-identity-width",
            19,
            &[25][..],
            PhysicalRecordFormatDenial::UnsupportedRecordIdentityBytes(25),
        ),
    ] {
        let root = parent.path().join(name);
        serving_from_initialization(&root).close();
        let catalog = root.join("families/records/bootstrap.catalog");
        let mut bytes = std::fs::read(&catalog).unwrap();
        bytes[offset..offset + encoded.len()].copy_from_slice(encoded);
        reseal(&mut bytes);
        std::fs::write(&catalog, bytes).unwrap();
        let media = media(&root);
        let before = media.media_counters();
        let outcome = open_record_store!(media, |durability| PhysicalRecordOpen::new(
            format, access, durability
        ))
        .into_raw();
        let TransitionOutcome::Denied(denial) = outcome else {
            panic!("{name} must be denied")
        };
        assert!(matches!(
            denial.reason(),
            RecordBootstrapDenial::UnsupportedPhysicalRecordFormat(unsupported)
                if unsupported.reason() == expected
        ));
        let media = denial.into_runtime();
        assert_eq!(
            media
                .media_counters()
                .completed_bytes_for(MediaOperationRole::PositionedRead)
                - before.completed_bytes_for(MediaOperationRole::PositionedRead),
            74,
            "{name}",
        );
        media.close();
    }
}

#[test]
fn selected_stale_root_and_foreign_store_use_distinct_proof_outcomes() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("stale");
    serving_from_initialization(&root).close();
    let roots = root.join("families/records/roots");
    std::fs::copy(
        roots.join("root-0000000000000001.manifest"),
        roots.join("root-0000000000000002.manifest"),
    )
    .unwrap();
    let catalog = root.join("families/records/bootstrap.catalog");
    let mut bytes = std::fs::read(&catalog).unwrap();
    bytes[28..36].copy_from_slice(&2_u64.to_le_bytes());
    bytes[56..64].copy_from_slice(&2_u64.to_le_bytes());
    reseal(&mut bytes);
    std::fs::write(&catalog, bytes).unwrap();
    let (format, _, access) = configuration();
    let outcome = open_record_store!(media(&root), |durability| PhysicalRecordOpen::new(
        format, access, durability
    ))
    .into_raw();
    let TransitionOutcome::Stale(stale) = outcome else {
        panic!("selected stale root must be stale")
    };
    assert_eq!(
        stale.reason(),
        RecordServingStaleReason::CatalogSelectedRootGenerationMismatch
    );
    stale.into_runtime().close();

    let foreign_root = parent.path().join("foreign");
    serving_from_initialization(&foreign_root).close();
    let catalog = foreign_root.join("families/records/bootstrap.catalog");
    let mut bytes = std::fs::read(&catalog).unwrap();
    bytes[40..56].copy_from_slice(&[0x77; 16]);
    reseal(&mut bytes);
    std::fs::write(&catalog, bytes).unwrap();
    let outcome = open_record_store!(media(&foreign_root), |durability| PhysicalRecordOpen::new(
        format, access, durability
    ))
    .into_raw();
    let TransitionOutcome::RebindRequired(rebind) = outcome else {
        panic!("foreign persisted Store identity must require rebind")
    };
    assert_eq!(
        rebind.reason(),
        RecordServingRebindReason::StoreIdentityMismatch
    );
    rebind.into_runtime().close();
}

#[test]
fn caller_format_narrows_acceptance_without_authorizing_migration() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    serving_from_initialization(&root).close();
    let expected = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder()
            .page_size(PhysicalPageSizeClass::KiB64)
            .admit()
            .unwrap(),
    );
    let access = PhysicalRecordAccessPolicy::builder()
        .admit(expected)
        .unwrap();
    let outcome = open_record_store!(media(&root), |durability| PhysicalRecordOpen::new(
        expected, access, durability
    ))
    .into_raw();
    let TransitionOutcome::Denied(denial) = outcome else {
        panic!("format drift cannot migrate on open")
    };
    assert!(matches!(
        denial.reason(),
        RecordBootstrapDenial::PhysicalRecordFormatMismatch(_)
    ));
    denial.into_runtime().close();
}

fn reseal(bytes: &mut [u8]) {
    let checksum = super::page_packing_oracle::independent_crc32c(&[&bytes[..36], &bytes[40..]]);
    bytes[36..40].copy_from_slice(&checksum.to_le_bytes());
}
