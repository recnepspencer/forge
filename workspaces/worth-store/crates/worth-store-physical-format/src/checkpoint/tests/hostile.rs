use crate::record_framing::crc32c;
use crate::{RecordArtifactFile, RecordFrameCoordinate};

use super::super::{
    CheckpointBindingCompactionHeader, CheckpointDirtyFrameBasis, CheckpointStreamDecodeDenial,
    CheckpointStreamDecoder, CheckpointStreamEncoder,
};
use super::source;

fn basis(block: u64) -> CheckpointDirtyFrameBasis {
    CheckpointDirtyFrameBasis::new(
        RecordFrameCoordinate::new(
            RecordArtifactFile::RootRoutingBlock {
                generation: 5,
                block,
            },
            0,
            64,
        )
        .unwrap(),
        block,
    )
}

fn reseal(record: &mut [u8]) {
    let payload_end = record.len() - 4;
    let checksum = crc32c::checksum(&[&record[..16], &record[16..payload_end]]);
    record[payload_end..].copy_from_slice(&checksum.to_le_bytes());
}

fn begin_compaction(
    encoder: CheckpointStreamEncoder,
    source: crate::PhysicalCheckpointSource,
) -> (super::super::CheckpointBindingCompactionEncoder, Vec<u8>) {
    encoder.begin_binding_compaction(
        CheckpointBindingCompactionHeader::new(1, source.wal().covered_end_lsn_exclusive())
            .unwrap(),
    )
}

#[test]
fn decoder_rejects_structural_corruption_before_claiming_source() {
    let (_, header) = CheckpointStreamEncoder::begin(source(1));
    let mut wrong_magic = header.clone();
    wrong_magic[0] ^= 1;
    assert_eq!(
        CheckpointStreamDecoder::begin(&wrong_magic).unwrap_err(),
        CheckpointStreamDecodeDenial::WrongMagic
    );
    let mut reserved = header.clone();
    reserved[10] = 1;
    assert_eq!(
        CheckpointStreamDecoder::begin(&reserved).unwrap_err(),
        CheckpointStreamDecodeDenial::ReservedFieldNonZero
    );
    let mut trailing = header.clone();
    trailing.push(0);
    assert_eq!(
        CheckpointStreamDecoder::begin(&trailing).unwrap_err(),
        CheckpointStreamDecodeDenial::LengthMismatch
    );
    assert_eq!(
        CheckpointStreamDecoder::begin(&header[..header.len() - 1]).unwrap_err(),
        CheckpointStreamDecodeDenial::LengthMismatch
    );
}

#[test]
fn decoder_rejects_unknown_artifact_even_with_valid_record_checksum() {
    let checkpoint_source = source(1);
    let (mut encoder, header) = CheckpointStreamEncoder::begin(checkpoint_source);
    let mut record = encoder.encode_dirty_basis(basis(1));
    record[16] = 255;
    reseal(&mut record);
    let mut decoder = CheckpointStreamDecoder::begin(&header).unwrap();
    assert_eq!(
        decoder.decode_dirty_basis(&record).unwrap_err(),
        CheckpointStreamDecodeDenial::InvalidArtifactKind(255)
    );
}

#[test]
fn footer_binds_identity_count_and_exact_ordered_record_digest() {
    let checkpoint_source = source(1);
    let (mut encoder, header) = CheckpointStreamEncoder::begin(checkpoint_source);
    let first = encoder.encode_dirty_basis(basis(1));
    let second = encoder.encode_dirty_basis(basis(2));
    let (compaction, compaction_header) = begin_compaction(encoder, checkpoint_source);
    let (_, footer) = compaction.finish();

    let mut reordered = CheckpointStreamDecoder::begin(&header).unwrap();
    reordered.decode_dirty_basis(&second).unwrap();
    reordered.decode_dirty_basis(&first).unwrap();
    let reordered = reordered
        .begin_binding_compaction(&compaction_header)
        .unwrap();
    assert_eq!(
        reordered.finish(&footer).unwrap_err(),
        CheckpointStreamDecodeDenial::AggregateDigestMismatch
    );

    let foreign_source = source(2);
    let (foreign, _) = CheckpointStreamEncoder::begin(foreign_source);
    let (foreign, foreign_compaction_header) = begin_compaction(foreign, foreign_source);
    let (_, foreign_footer) = foreign.finish();
    let decoder = CheckpointStreamDecoder::begin(&header)
        .unwrap()
        .begin_binding_compaction(&foreign_compaction_header)
        .unwrap();
    assert_eq!(
        decoder.finish(&foreign_footer).unwrap_err(),
        CheckpointStreamDecodeDenial::SourceIdentityMismatch
    );

    let decoder = CheckpointStreamDecoder::begin(&header)
        .unwrap()
        .begin_binding_compaction(&compaction_header)
        .unwrap();
    assert_eq!(
        decoder.finish(&footer).unwrap_err(),
        CheckpointStreamDecodeDenial::RecordCountMismatch
    );
}

#[test]
fn binding_compaction_rejects_omission_reorder_and_oversized_payloads() {
    let source = source(3);
    let (encoder, header) = CheckpointStreamEncoder::begin(source);
    let (mut compaction, compaction_header) = begin_compaction(encoder, source);
    let first = compaction.encode_binding_record(b"first").unwrap();
    let second = compaction.encode_binding_record(b"second").unwrap();
    let (_, footer) = compaction.finish();

    let decoder = CheckpointStreamDecoder::begin(&header).unwrap();
    let mut omitted = decoder
        .begin_binding_compaction(&compaction_header)
        .unwrap();
    omitted.decode_binding_record(&first).unwrap();
    assert_eq!(
        omitted.finish(&footer).unwrap_err(),
        CheckpointStreamDecodeDenial::RecordCountMismatch
    );

    let decoder = CheckpointStreamDecoder::begin(&header).unwrap();
    let mut reordered = decoder
        .begin_binding_compaction(&compaction_header)
        .unwrap();
    reordered.decode_binding_record(&second).unwrap();
    reordered.decode_binding_record(&first).unwrap();
    assert_eq!(
        reordered.finish(&footer).unwrap_err(),
        CheckpointStreamDecodeDenial::AggregateDigestMismatch
    );

    let (encoder, _) = CheckpointStreamEncoder::begin(source);
    let (mut compaction, _) = begin_compaction(encoder, source);
    assert_eq!(
        compaction
            .encode_binding_record(&vec![
                0;
                super::super::MAX_CHECKPOINT_BINDING_RECORD_BYTES + 1
            ])
            .unwrap_err(),
        CheckpointStreamDecodeDenial::BindingRecordTooLarge
    );
}
