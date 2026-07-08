use super::{assert_pre_decode_denial_counters, crc32c, with_pre_decode_admission};
use forge_store_physical_format::PhysicalFrameKind;
use forge_store_physical_integrity::{
    DeclaredPhysicalChecksum, LogicalDecodeGateIdentity, PhysicalIntegrityAdmissionRequest,
    PreDecodePhysicalDenial, PreDecodePhysicalDenialKind,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CheckedFrameObservation {
    pub(crate) identity: LogicalDecodeGateIdentity,
    pub(crate) checked_bytes: Vec<u8>,
    pub(crate) checked_byte_count: u64,
    pub(crate) checksum_executions: u32,
    pub(crate) skipped_decodes: u32,
}

pub(crate) fn admit_checked_frame(
    expected_payload: &[u8],
    actual_payload: &[u8],
) -> CheckedFrameObservation {
    let expected = DeclaredPhysicalChecksum::new(crc32c(expected_payload));
    let mut output = None;
    with_pre_decode_admission(actual_payload, |admission, validation, witness| {
        let checked = admission
            .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                validation,
                witness,
                PhysicalFrameKind::RecordFrame,
                expected,
            ))
            .unwrap();
        output = Some(CheckedFrameObservation {
            identity: checked.gate_evidence().identity().clone(),
            checked_bytes: checked.checked_bytes().as_bytes().to_vec(),
            checked_byte_count: checked.counters().checked_byte_count(),
            checksum_executions: checked.counters().checksum_execution_count(),
            skipped_decodes: checked.counters().skipped_logical_decode().skipped_count(),
        });
    });
    output.unwrap()
}

pub(crate) fn deny_checked_frame(
    expected_payload: &[u8],
    actual_payload: &[u8],
    expected_kind: PreDecodePhysicalDenialKind,
) -> PreDecodePhysicalDenial {
    let expected = DeclaredPhysicalChecksum::new(crc32c(expected_payload));
    let mut output = None;
    with_pre_decode_admission(actual_payload, |admission, validation, witness| {
        let denial = admission
            .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                validation,
                witness,
                PhysicalFrameKind::RecordFrame,
                expected,
            ))
            .unwrap_err();
        assert_eq!(denial.kind(), expected_kind);
        assert_eq!(denial.locality(), Some(witness.owner()));
        assert_pre_decode_denial_counters(denial.clone(), actual_payload.len() as u64, 1);
        output = Some(denial);
    });
    output.unwrap()
}
