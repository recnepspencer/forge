use worth_store_wal::{
    inspect_bounded_wal_active_tail_with_evidence, inspect_interrupted_wal_segment_start,
    VerifiedWalActiveTail, WalActiveTailInspectionDenial, WalArtifactStoreDenial,
    WalSegmentArtifactIdentity,
};

use super::{PhysicalRecoveryResidue, PhysicalRecoveryResidueKind, PhysicalWalSegmentCandidate};

mod aggregation;

use aggregation::WalInspectionAggregate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWalArtifactInspectionDenial {
    CounterOverflow,
    FrameLimitExceeded { observed: u64, admitted: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWalArtifactCorruption {
    artifact: String,
    identity: WalSegmentArtifactIdentity,
    denial: WalArtifactStoreDenial,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectedPhysicalWalArtifacts {
    candidates: Vec<PhysicalWalSegmentCandidate>,
    residue: Vec<PhysicalRecoveryResidue>,
    corruptions: Vec<PhysicalWalArtifactCorruption>,
    canonical_segments_scanned: u64,
    frames_scanned: u64,
    valid_frame_count: u64,
    valid_byte_count: u64,
    observed_byte_count: u64,
    torn_suffix_frames: u64,
    torn_suffix_bytes: u64,
}

pub fn inspect_physical_wal_artifacts(
    artifacts: Vec<(String, Vec<u8>)>,
    maximum_frames: u64,
) -> Result<InspectedPhysicalWalArtifacts, PhysicalWalArtifactInspectionDenial> {
    let observed_byte_count = artifacts.iter().try_fold(0_u64, |total, (_, bytes)| {
        total.checked_add(bytes.len() as u64)
    });
    let observed_byte_count =
        observed_byte_count.ok_or(PhysicalWalArtifactInspectionDenial::CounterOverflow)?;
    let mut canonical = Vec::new();
    let mut residue = Vec::new();
    for (name, bytes) in artifacts {
        let Some(identity) = WalSegmentArtifactIdentity::parse(&name) else {
            residue.push(PhysicalRecoveryResidue::new(
                name,
                PhysicalRecoveryResidueKind::NonCanonicalWalArtifact,
            ));
            continue;
        };
        canonical.push((identity, name, bytes));
    }
    canonical.sort_unstable_by_key(|(identity, ..)| *identity);
    inspect_canonical_artifacts(canonical, residue, observed_byte_count, maximum_frames)
}

fn inspect_canonical_artifacts(
    canonical: Vec<(WalSegmentArtifactIdentity, String, Vec<u8>)>,
    residue: Vec<PhysicalRecoveryResidue>,
    observed_byte_count: u64,
    maximum_frames: u64,
) -> Result<InspectedPhysicalWalArtifacts, PhysicalWalArtifactInspectionDenial> {
    let canonical_count = canonical.len();
    let mut aggregate = WalInspectionAggregate::new(residue);
    for (index, (identity, name, bytes)) in canonical.into_iter().enumerate() {
        let terminal = index + 1 == canonical_count;
        let remaining_frames = maximum_frames.saturating_sub(aggregate.frames_scanned());
        let disposition = inspect_canonical_artifact(
            identity,
            &bytes,
            terminal,
            remaining_frames,
            maximum_frames,
            aggregate.frames_scanned(),
        )?;
        aggregate.record(identity, name, disposition)?;
    }
    aggregate.finish(canonical_count as u64, observed_byte_count)
}

enum CanonicalArtifactDisposition {
    Verified {
        candidate: PhysicalWalSegmentCandidate,
        frames_scanned: u64,
        torn_suffix_bytes: u64,
    },
    InterruptedStart {
        observed_bytes: u64,
    },
    TrailingEmpty,
    Corrupt {
        denial: WalArtifactStoreDenial,
        frames_scanned: u64,
    },
}

fn inspect_canonical_artifact(
    identity: WalSegmentArtifactIdentity,
    bytes: &[u8],
    terminal: bool,
    remaining_frames: u64,
    maximum_frames: u64,
    prior_frames: u64,
) -> Result<CanonicalArtifactDisposition, PhysicalWalArtifactInspectionDenial> {
    if bytes.is_empty() && terminal {
        return Ok(CanonicalArtifactDisposition::TrailingEmpty);
    }
    match inspect_bounded_wal_active_tail_with_evidence(identity, bytes, remaining_frames) {
        Ok(active) => classify_verified_tail(active, terminal),
        Err(failure)
            if matches!(
                failure.denial(),
                WalActiveTailInspectionDenial::FrameLimitExceeded { .. }
            ) =>
        {
            let WalActiveTailInspectionDenial::FrameLimitExceeded { observed, .. } =
                failure.denial()
            else {
                unreachable!("the guarded denial is a frame limit")
            };
            Err(PhysicalWalArtifactInspectionDenial::FrameLimitExceeded {
                observed: prior_frames.saturating_add(observed),
                admitted: maximum_frames,
            })
        }
        Err(failure) if terminal => match inspect_interrupted_wal_segment_start(identity, bytes) {
            Ok(interrupted) => Ok(CanonicalArtifactDisposition::InterruptedStart {
                observed_bytes: interrupted.observed_bytes(),
            }),
            Err(_) => Ok(CanonicalArtifactDisposition::Corrupt {
                denial: artifact_denial(failure.denial()),
                frames_scanned: failure.frames_scanned(),
            }),
        },
        Err(failure) => Ok(CanonicalArtifactDisposition::Corrupt {
            denial: artifact_denial(failure.denial()),
            frames_scanned: failure.frames_scanned(),
        }),
    }
}

fn artifact_denial(denial: WalActiveTailInspectionDenial) -> WalArtifactStoreDenial {
    match denial {
        WalActiveTailInspectionDenial::Artifact(denial) => denial,
        WalActiveTailInspectionDenial::FrameLimitExceeded { .. } => {
            unreachable!("frame limits are returned before corruption classification")
        }
    }
}

fn classify_verified_tail(
    active: VerifiedWalActiveTail<'_>,
    terminal: bool,
) -> Result<CanonicalArtifactDisposition, PhysicalWalArtifactInspectionDenial> {
    let interruption = active.interrupted_tail();
    let verified = active.into_verified_prefix();
    let frames = verified
        .frames()
        .iter()
        .map(|frame| frame.to_owned_verified())
        .collect();
    let frames_scanned = verified
        .inspection()
        .frame_count()
        .checked_add(u64::from(interruption.is_some()))
        .ok_or(PhysicalWalArtifactInspectionDenial::CounterOverflow)?;
    if interruption.is_some() && !terminal {
        return Ok(CanonicalArtifactDisposition::Corrupt {
            denial: WalArtifactStoreDenial::InvalidFrame,
            frames_scanned,
        });
    }
    let torn_suffix_bytes = interruption.map_or(0, |tail| {
        tail.observed_bytes()
            .saturating_sub(tail.valid_prefix_bytes())
    });
    Ok(CanonicalArtifactDisposition::Verified {
        candidate: PhysicalWalSegmentCandidate::verified_with_frames(
            verified.inspection(),
            interruption,
            frames,
        ),
        frames_scanned,
        torn_suffix_bytes,
    })
}

impl InspectedPhysicalWalArtifacts {
    pub fn candidates(&self) -> &[PhysicalWalSegmentCandidate] {
        &self.candidates
    }

    pub fn residue(&self) -> &[PhysicalRecoveryResidue] {
        &self.residue
    }

    pub fn rejected(&self) -> bool {
        !self.corruptions.is_empty()
    }

    pub fn frame_count(&self) -> u64 {
        self.valid_frame_count
    }

    pub fn frames_scanned(&self) -> u64 {
        self.frames_scanned
    }

    pub fn byte_count(&self) -> u64 {
        self.observed_byte_count
    }

    pub fn valid_byte_count(&self) -> u64 {
        self.valid_byte_count
    }

    pub fn torn_suffix_frames(&self) -> u64 {
        self.torn_suffix_frames
    }

    pub fn torn_suffix_bytes(&self) -> u64 {
        self.torn_suffix_bytes
    }

    pub fn corruption_denials(&self) -> u64 {
        self.corruptions.len() as u64
    }

    pub fn canonical_segment_count(&self) -> u64 {
        self.canonical_segments_scanned
    }

    pub fn valid_segment_count(&self) -> u64 {
        self.candidates.len() as u64
    }

    pub fn corruptions(&self) -> &[PhysicalWalArtifactCorruption] {
        &self.corruptions
    }
}

impl PhysicalWalArtifactCorruption {
    pub fn artifact(&self) -> &str {
        &self.artifact
    }

    pub const fn identity(&self) -> WalSegmentArtifactIdentity {
        self.identity
    }

    pub const fn denial(&self) -> WalArtifactStoreDenial {
        self.denial
    }
}

#[cfg(test)]
mod tests {
    use worth_store_wal::{
        prepare_wal_frame_append, WalSegmentArtifactIdentity, WalSegmentGeneration, WalSegmentId,
    };

    use super::*;

    #[test]
    fn noncanonical_and_trailing_empty_artifacts_remain_distinct_residue() {
        let trailing = identity(2).file_name();
        let inspected = inspect_physical_wal_artifacts(
            vec![
                ("plausible-newest.wal".into(), b"not authoritative".to_vec()),
                (trailing, Vec::new()),
            ],
            u64::MAX,
        )
        .unwrap();
        assert!(inspected.candidates().is_empty());
        assert!(!inspected.rejected());
        assert_eq!(inspected.frame_count(), 0);
        assert_eq!(inspected.byte_count(), b"not authoritative".len() as u64);
        assert_eq!(inspected.residue().len(), 2);
        assert!(inspected
            .residue()
            .iter()
            .any(|item| item.kind() == PhysicalRecoveryResidueKind::NonCanonicalWalArtifact));
        assert!(inspected
            .residue()
            .iter()
            .any(|item| item.kind() == PhysicalRecoveryResidueKind::TrailingEmptyWalSegment));
    }

    #[test]
    fn corrupt_canonical_artifact_is_rejected_instead_of_becoming_residue() {
        let mut bytes = encoded_frame(1, 10, 20);
        let last = bytes.last_mut().unwrap();
        *last ^= 0xff;
        let inspected = inspect_physical_wal_artifacts(
            vec![(identity(1).file_name(), bytes.clone())],
            u64::MAX,
        )
        .unwrap();
        assert!(inspected.candidates().is_empty());
        assert!(inspected.residue().is_empty());
        assert!(inspected.rejected());
        assert_eq!(inspected.frame_count(), 0);
        assert_eq!(inspected.byte_count(), bytes.len() as u64);
    }

    #[test]
    fn terminal_interrupted_first_frame_preserves_the_prior_valid_prefix() {
        let first = encoded_frame(1, 10, 20);
        let second = encoded_frame(2, 20, 30);
        let interrupted = second[..37].to_vec();
        let inspected = inspect_physical_wal_artifacts(
            vec![
                (identity(1).file_name(), first.clone()),
                (identity(2).file_name(), interrupted.clone()),
            ],
            u64::MAX,
        )
        .unwrap();

        assert!(!inspected.rejected());
        assert_eq!(inspected.candidates().len(), 1);
        assert_eq!(inspected.candidates()[0].identity(), identity(1));
        assert_eq!(inspected.frame_count(), 1);
        assert_eq!(inspected.valid_byte_count(), first.len() as u64);
        assert_eq!(inspected.torn_suffix_frames(), 1);
        assert_eq!(inspected.torn_suffix_bytes(), interrupted.len() as u64);
        assert_eq!(inspected.residue().len(), 1);
        assert_eq!(
            inspected.residue()[0].kind(),
            PhysicalRecoveryResidueKind::InterruptedWalSegmentStart
        );
    }

    #[test]
    fn interrupted_first_frame_is_rejected_when_it_is_not_terminal() {
        let interrupted = encoded_frame(1, 10, 20)[..37].to_vec();
        let complete = encoded_frame(2, 20, 30);
        let inspected = inspect_physical_wal_artifacts(
            vec![
                (identity(1).file_name(), interrupted),
                (identity(2).file_name(), complete),
            ],
            u64::MAX,
        )
        .unwrap();
        assert!(inspected.rejected());
        assert!(inspected.residue().is_empty());
    }

    fn encoded_frame(segment: u64, start: u64, end: u64) -> Vec<u8> {
        let directory = tempfile::tempdir().unwrap();
        prepare_wal_frame_append(
            directory.path(),
            segment,
            1,
            start,
            end,
            "phase-three-wal-artifact",
            b"payload",
        )
        .unwrap()
        .encoded_frame()
        .to_vec()
    }

    fn identity(segment: u64) -> WalSegmentArtifactIdentity {
        WalSegmentArtifactIdentity::new(
            WalSegmentId::new(segment).unwrap(),
            WalSegmentGeneration::new(1).unwrap(),
        )
    }
}
