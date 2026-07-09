use crate::{
    CheckpointAdjacentDamageDenial, CheckpointRecordIntegrityReport, ChecksumAlgorithmClaim,
    ChecksumAlgorithmId, ChecksumAlgorithmMismatchDenial, WalFrameDamageDenial,
    WalFrameDamageDenialKind, WalFrameIntegrityCounters, WalFrameIntegrityInspectionRequest,
    WalFrameIntegrityReport, WalTailIntegrityPosture,
};
use worth_store_physical_format::{CheckpointAdjacencyPosture, PhysicalHeaderKind};

const WAL_FRAME_MAGIC: &[u8] = b"WALF|";
const WAL_STATUS_OK: &str = "ok";
const WAL_STATUS_CHECKSUM_FAILURE: &str = "checksum-fail";
const WAL_STATUS_UNKNOWN: &str = "unknown";
const WAL_STATUS_CHECKPOINT_DAMAGE: &str = "checkpoint-damage";
const WAL_STATUS_RECOVERY_PRECEDENCE_REQUIRED: &str = "recovery-precedence-required";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalFrameIntegrityAuthority;

impl WalFrameIntegrityAuthority {
    pub const fn s3() -> Self {
        Self
    }

    pub fn inspect(
        self,
        request: WalFrameIntegrityInspectionRequest<'_>,
    ) -> Result<WalFrameIntegrityReport, WalFrameDamageDenial> {
        let input = request.input();
        let basis = input.admission().basis().clone();
        let counters = WalFrameIntegrityCounters::start()
            .with_frame_header_check()
            .with_payload_boundary_check()
            .with_checkpoint_adjacency_check()
            .with_checksum_posture_check()
            .with_tail_posture_check()
            .with_skipped_replay_attempt();
        let Some(frame) = input.admission().checked_frame() else {
            return Err(wal_denial(
                WalFrameDamageDenialKind::WrongPhysicalFamily,
                WalTailIntegrityPosture::UnknownTailIntegrity,
                counters,
                basis,
            ));
        };
        if !matches!(
            frame.physical_witness().kind(),
            PhysicalHeaderKind::Frame(_)
        ) {
            return Err(wal_denial(
                WalFrameDamageDenialKind::HeaderWitnessMismatch,
                WalTailIntegrityPosture::UnknownTailIntegrity,
                counters,
                basis,
            ));
        }
        reject_payload_length_mismatch(
            frame.physical_witness().payload_length() as usize,
            frame.checked_bytes().len_bytes(),
            counters,
            &basis,
        )?;
        inspect_tail_evidence(frame.checked_bytes().as_bytes(), counters, basis)
    }

    pub fn inspect_checkpoint_adjacent(
        self,
        request: WalFrameIntegrityInspectionRequest<'_>,
    ) -> Result<CheckpointRecordIntegrityReport, WalFrameDamageDenial> {
        let report = self.inspect(request)?;
        reject_non_checkpoint_adjacent_report(
            report.basis(),
            report.counters(),
            report.tail_posture(),
        )?;
        Ok(CheckpointRecordIntegrityReport::new(
            report.basis().clone(),
            report.tail_posture(),
            report.counters(),
        ))
    }
}

fn inspect_tail_evidence(
    bytes: &[u8],
    counters: WalFrameIntegrityCounters,
    basis: crate::PhysicalScopeBasis,
) -> Result<WalFrameIntegrityReport, WalFrameDamageDenial> {
    let evidence = match parse_wal_tail_evidence(bytes) {
        Some(evidence) => evidence,
        None => {
            return Err(wal_denial(
                WalFrameDamageDenialKind::UnknownTailIntegrity,
                WalTailIntegrityPosture::UnknownTailIntegrity,
                counters,
                basis,
            ));
        }
    };
    if let Err(denial) = evidence.admit_algorithm() {
        return Err(wal_denial(
            WalFrameDamageDenialKind::UnsupportedAlgorithm,
            WalTailIntegrityPosture::UnsupportedTailIntegrity,
            counters,
            basis,
        )
        .with_checksum_denial(denial));
    }
    if evidence.actual_length < evidence.declared_length {
        return Err(wal_denial(
            WalFrameDamageDenialKind::TornWalFrame,
            WalTailIntegrityPosture::TornTail,
            counters,
            basis,
        )
        .with_lengths(evidence.declared_length, evidence.actual_length));
    }
    if evidence.actual_length > evidence.declared_length {
        return Err(wal_denial(
            WalFrameDamageDenialKind::MismatchedLength,
            WalTailIntegrityPosture::UnknownTailIntegrity,
            counters,
            basis,
        )
        .with_lengths(evidence.declared_length, evidence.actual_length));
    }
    match evidence.status {
        WAL_STATUS_OK => Ok(WalFrameIntegrityReport::new(
            basis,
            WalTailIntegrityPosture::IntactTail,
            counters,
        )),
        WAL_STATUS_CHECKSUM_FAILURE => Err(wal_denial(
            WalFrameDamageDenialKind::ChecksumFailure,
            WalTailIntegrityPosture::UnknownTailIntegrity,
            counters,
            basis,
        )),
        WAL_STATUS_UNKNOWN => Err(wal_denial(
            WalFrameDamageDenialKind::UnknownTailIntegrity,
            WalTailIntegrityPosture::UnknownTailIntegrity,
            counters,
            basis,
        )),
        WAL_STATUS_CHECKPOINT_DAMAGE => {
            reject_non_checkpoint_adjacent_damage(&basis, counters)?;
            let damage = CheckpointAdjacentDamageDenial::new(
                basis.scope(),
                CheckpointAdjacencyPosture::CheckpointAdjacent,
            );
            Err(wal_denial(
                WalFrameDamageDenialKind::CheckpointAdjacentCorruption,
                WalTailIntegrityPosture::CheckpointAdjacentDamage,
                counters,
                basis,
            )
            .with_checkpoint_adjacent_damage(damage))
        }
        WAL_STATUS_RECOVERY_PRECEDENCE_REQUIRED => Err(wal_denial(
            WalFrameDamageDenialKind::RecoveryPrecedenceRequired,
            WalTailIntegrityPosture::RecoveryPrecedenceRequired,
            counters,
            basis,
        )),
        _ => Err(wal_denial(
            WalFrameDamageDenialKind::UnknownTailIntegrity,
            WalTailIntegrityPosture::UnknownTailIntegrity,
            counters,
            basis,
        )),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WalTailPhysicalEvidence<'bytes> {
    algorithm: &'bytes str,
    declared_length: usize,
    actual_length: usize,
    status: &'bytes str,
}

impl WalTailPhysicalEvidence<'_> {
    fn admit_algorithm(self) -> Result<ChecksumAlgorithmId, ChecksumAlgorithmMismatchDenial> {
        ChecksumAlgorithmId::admit_claim(ChecksumAlgorithmClaim::declared_text(self.algorithm))
    }
}

fn parse_wal_tail_evidence(bytes: &[u8]) -> Option<WalTailPhysicalEvidence<'_>> {
    let tail = bytes.strip_prefix(WAL_FRAME_MAGIC)?;
    let (algorithm, after_algorithm) = split_once(tail, b'|')?;
    let (declared_length, after_length) = split_once(after_algorithm, b'|')?;
    let (status, body) = split_once(after_length, b'|')?;
    Some(WalTailPhysicalEvidence {
        algorithm: as_utf8(algorithm)?,
        declared_length: parse_nonempty_ascii_usize(declared_length)?,
        actual_length: body.len(),
        status: as_utf8(status)?,
    })
}

fn split_once(bytes: &[u8], delimiter: u8) -> Option<(&[u8], &[u8])> {
    let index = bytes.iter().position(|byte| *byte == delimiter)?;
    Some((&bytes[..index], &bytes[index + 1..]))
}

fn as_utf8(bytes: &[u8]) -> Option<&str> {
    std::str::from_utf8(bytes).ok()
}

fn parse_nonempty_ascii_usize(bytes: &[u8]) -> Option<usize> {
    if bytes.is_empty() {
        return None;
    }
    let mut value = 0usize;
    for byte in bytes {
        let digit = byte.checked_sub(b'0')?;
        if digit > 9 {
            return None;
        }
        value = value.checked_mul(10)?.checked_add(digit as usize)?;
    }
    Some(value)
}

fn reject_non_checkpoint_adjacent_damage(
    basis: &crate::PhysicalScopeBasis,
    counters: WalFrameIntegrityCounters,
) -> Result<(), WalFrameDamageDenial> {
    if basis.checkpoint_adjacency() == CheckpointAdjacencyPosture::CheckpointAdjacent {
        return Ok(());
    }
    Err(wal_denial(
        WalFrameDamageDenialKind::UnknownTailIntegrity,
        WalTailIntegrityPosture::UnknownTailIntegrity,
        counters,
        basis.clone(),
    ))
}

fn reject_non_checkpoint_adjacent_report(
    basis: &crate::PhysicalScopeBasis,
    counters: WalFrameIntegrityCounters,
    posture: WalTailIntegrityPosture,
) -> Result<(), WalFrameDamageDenial> {
    if basis.checkpoint_adjacency() == CheckpointAdjacencyPosture::CheckpointAdjacent {
        return Ok(());
    }
    Err(wal_denial(
        WalFrameDamageDenialKind::WrongCheckpointAdjacency,
        posture,
        counters,
        basis.clone(),
    ))
}

fn reject_payload_length_mismatch(
    expected: usize,
    actual: usize,
    counters: WalFrameIntegrityCounters,
    basis: &crate::PhysicalScopeBasis,
) -> Result<(), WalFrameDamageDenial> {
    if actual == expected {
        return Ok(());
    }
    let kind = if actual < expected {
        WalFrameDamageDenialKind::TornWalFrame
    } else {
        WalFrameDamageDenialKind::MismatchedLength
    };
    let posture = if actual < expected {
        WalTailIntegrityPosture::TornTail
    } else {
        WalTailIntegrityPosture::UnknownTailIntegrity
    };
    Err(wal_denial(kind, posture, counters, basis.clone()).with_lengths(expected, actual))
}

fn wal_denial(
    kind: WalFrameDamageDenialKind,
    posture: WalTailIntegrityPosture,
    counters: WalFrameIntegrityCounters,
    basis: crate::PhysicalScopeBasis,
) -> WalFrameDamageDenial {
    WalFrameDamageDenial::new(kind, posture, counters).with_basis(basis)
}
