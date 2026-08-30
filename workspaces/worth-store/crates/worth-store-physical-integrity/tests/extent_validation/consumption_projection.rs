use worth_store_physical_format::{
    encode_data_frame_page_lsn, DurableFrameKind, ExtentChunkCoordinate, PhysicalExtentId,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageLsn,
};
use worth_store_physical_integrity::{
    validate_extent_chunk, ExtentChunkIntegrityValidation, ExtentChunkProjectionDenial,
    UntrustedPhysicalArtifact,
};

use super::support::{record, validated_manifest, ExtentFixture};

#[test]
fn sealed_extent_chunk_projects_exact_payload_and_page_lsn() {
    let fixture = ExtentFixture::new();
    let manifest_bytes = fixture.manifest_bytes();
    let manifest = validated_manifest(&manifest_bytes, fixture.manifest_scope());
    let mut bytes = fixture.tail_chunk_bytes();
    encode_data_frame_page_lsn(
        &mut bytes,
        DurableFrameKind::Extent,
        PhysicalPageLsn::new(144),
    )
    .unwrap();
    let input = UntrustedPhysicalArtifact::from_bounded_bytes(&bytes);
    let (validation, _) = validate_extent_chunk(input, fixture.tail_chunk_scope(), &manifest);
    let ExtentChunkIntegrityValidation::Intact(validated) = validation else {
        panic!("clean extent chunk rejected");
    };

    let projection = validated
        .project_chunk(input, fixture.chunk_coordinate(2))
        .unwrap();

    assert_eq!(projection.coordinate(), fixture.chunk_coordinate(2));
    assert_eq!(projection.page_lsn(), PhysicalPageLsn::new(144));
    assert_eq!(&bytes[projection.payload_range()], b"tail!");
}

#[test]
fn extent_projection_denies_foreign_incarnation_generation_and_ordinal() {
    let fixture = ExtentFixture::new();
    let manifest_bytes = fixture.manifest_bytes();
    let manifest = validated_manifest(&manifest_bytes, fixture.manifest_scope());
    let bytes = fixture.tail_chunk_bytes();
    let input = UntrustedPhysicalArtifact::from_bounded_bytes(&bytes);
    let (validation, _) = validate_extent_chunk(input, fixture.tail_chunk_scope(), &manifest);
    let ExtentChunkIntegrityValidation::Intact(validated) = validation else {
        panic!("clean extent chunk rejected");
    };
    let exact = fixture.chunk_coordinate(2);
    let equal_copy = bytes.clone();
    assert_eq!(
        validated
            .project_chunk(
                UntrustedPhysicalArtifact::from_bounded_bytes(&equal_copy),
                exact,
            )
            .unwrap_err(),
        ExtentChunkProjectionDenial::InputIncarnationMismatch
    );
    assert_eq!(
        validated
            .project_chunk(
                input,
                coordinate(
                    fixture,
                    extent_cell(fixture.extent.extent_id().get(), 6),
                    exact.logical_offset(),
                    exact.ordinal(),
                ),
            )
            .unwrap_err(),
        ExtentChunkProjectionDenial::ExtentGenerationMismatch
    );
    assert_eq!(
        validated
            .project_chunk(
                input,
                ExtentChunkCoordinate::new(
                    record(0x23, 7),
                    fixture.extent,
                    fixture.logical_bytes,
                    exact.logical_offset(),
                    exact.ordinal(),
                )
                .unwrap(),
            )
            .unwrap_err(),
        ExtentChunkProjectionDenial::RecordIdentityMismatch
    );
    assert_eq!(
        validated
            .project_chunk(
                input,
                coordinate(
                    fixture,
                    extent_cell(fixture.extent.extent_id().get() + 1, 5),
                    exact.logical_offset(),
                    exact.ordinal(),
                ),
            )
            .unwrap_err(),
        ExtentChunkProjectionDenial::ExtentIdentityMismatch
    );
    assert_eq!(
        validated
            .project_chunk(
                input,
                ExtentChunkCoordinate::new(
                    fixture.record,
                    fixture.extent,
                    fixture.logical_bytes + 1,
                    exact.logical_offset(),
                    exact.ordinal(),
                )
                .unwrap(),
            )
            .unwrap_err(),
        ExtentChunkProjectionDenial::LogicalLengthMismatch
    );
    assert_eq!(
        validated
            .project_chunk(
                input,
                coordinate(fixture, fixture.extent, 0, exact.ordinal() + 1),
            )
            .unwrap_err(),
        ExtentChunkProjectionDenial::LogicalOffsetMismatch
    );
    assert_eq!(
        validated
            .project_chunk(
                input,
                coordinate(
                    fixture,
                    fixture.extent,
                    exact.logical_offset(),
                    exact.ordinal() + 1,
                ),
            )
            .unwrap_err(),
        ExtentChunkProjectionDenial::ChunkOrdinalMismatch
    );
}

fn coordinate(
    fixture: ExtentFixture,
    extent: worth_store_physical_format::RecordExtentGenerationCell,
    logical_offset: u64,
    ordinal: u32,
) -> ExtentChunkCoordinate {
    ExtentChunkCoordinate::new(
        fixture.record,
        extent,
        fixture.logical_bytes,
        logical_offset,
        ordinal,
    )
    .unwrap()
}

fn extent_cell(
    extent: u64,
    generation: u64,
) -> worth_store_physical_format::RecordExtentGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .record_extent_cell(PhysicalExtentId::from_raw(extent).unwrap())
        .with_extent_generation(PhysicalGeneration::from_raw(generation).unwrap())
}
