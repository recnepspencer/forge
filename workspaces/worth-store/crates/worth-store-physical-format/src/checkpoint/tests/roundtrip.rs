use crate::{RecordArtifactFile, RecordFrameCoordinate};

use super::super::{
    CheckpointBindingCompactionHeader, CheckpointDirtyFrameBasis, CheckpointStreamDecoder,
    CheckpointStreamEncoder,
};
use super::source;

#[test]
fn independently_framed_stream_roundtrips_without_whole_artifact_memory() {
    let source = source(3);
    let bases = [
        CheckpointDirtyFrameBasis::new(
            RecordFrameCoordinate::new(
                RecordArtifactFile::Segment {
                    segment: 2,
                    generation: 9,
                },
                128,
                64,
            )
            .unwrap(),
            17,
        ),
        CheckpointDirtyFrameBasis::new(
            RecordFrameCoordinate::new(
                RecordArtifactFile::RootRoutingBlock {
                    generation: 5,
                    block: 4,
                },
                0,
                96,
            )
            .unwrap(),
            19,
        ),
    ];
    let (mut encoder, header) = CheckpointStreamEncoder::begin(source);
    let records = bases.map(|basis| encoder.encode_dirty_basis(basis));
    let compaction_header =
        CheckpointBindingCompactionHeader::new(1, source.wal().covered_end_lsn_exclusive())
            .unwrap();
    let (mut compaction, compaction_record) = encoder.begin_binding_compaction(compaction_header);
    let binding_payloads = [b"binding-one".as_slice(), b"binding-two".as_slice()];
    let binding_records = binding_payloads.map(|payload| {
        compaction
            .encode_binding_record(payload)
            .expect("bounded binding record")
    });
    let (encoded_footer, footer) = compaction.finish();

    let mut decoder = CheckpointStreamDecoder::begin(&header).unwrap();
    assert_eq!(decoder.source(), source);
    for (record, expected) in records.iter().zip(bases) {
        assert_eq!(decoder.decode_dirty_basis(record).unwrap(), expected);
    }
    let mut compaction = decoder
        .begin_binding_compaction(&compaction_record)
        .unwrap();
    assert_eq!(compaction.header(), compaction_header);
    for (record, expected) in binding_records.iter().zip(binding_payloads) {
        assert_eq!(compaction.decode_binding_record(record).unwrap(), expected);
    }
    let decoded_footer = compaction.finish(&footer).unwrap();
    assert_eq!(decoded_footer, encoded_footer);
    assert_eq!(decoded_footer.dirty_record_count(), 2);
    assert_eq!(decoded_footer.binding_record_count(), 2);
    assert_eq!(decoded_footer.identity(), source.identity());
}
