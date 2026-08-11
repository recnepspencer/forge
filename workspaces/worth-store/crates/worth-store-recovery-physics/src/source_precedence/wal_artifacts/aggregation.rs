use worth_store_wal::WalSegmentArtifactIdentity;

use super::{
    CanonicalArtifactDisposition, InspectedPhysicalWalArtifacts, PhysicalRecoveryResidue,
    PhysicalRecoveryResidueKind, PhysicalWalArtifactCorruption,
    PhysicalWalArtifactInspectionDenial, PhysicalWalSegmentCandidate,
};

pub(super) struct WalInspectionAggregate {
    candidates: Vec<PhysicalWalSegmentCandidate>,
    residue: Vec<PhysicalRecoveryResidue>,
    corruptions: Vec<PhysicalWalArtifactCorruption>,
    frames_scanned: u64,
    torn_suffix_frames: u64,
    torn_suffix_bytes: u64,
}

impl WalInspectionAggregate {
    pub(super) fn new(residue: Vec<PhysicalRecoveryResidue>) -> Self {
        Self {
            candidates: Vec::new(),
            residue,
            corruptions: Vec::new(),
            frames_scanned: 0,
            torn_suffix_frames: 0,
            torn_suffix_bytes: 0,
        }
    }

    pub(super) const fn frames_scanned(&self) -> u64 {
        self.frames_scanned
    }

    pub(super) fn record(
        &mut self,
        identity: WalSegmentArtifactIdentity,
        name: String,
        disposition: CanonicalArtifactDisposition,
    ) -> Result<(), PhysicalWalArtifactInspectionDenial> {
        match disposition {
            CanonicalArtifactDisposition::Verified {
                candidate,
                frames_scanned,
                torn_suffix_bytes,
            } => {
                checked_accumulate(&mut self.frames_scanned, frames_scanned)?;
                if torn_suffix_bytes != 0 {
                    checked_accumulate(&mut self.torn_suffix_frames, 1)?;
                    checked_accumulate(&mut self.torn_suffix_bytes, torn_suffix_bytes)?;
                }
                self.candidates.push(candidate);
            }
            CanonicalArtifactDisposition::InterruptedStart { observed_bytes } => {
                self.record_interrupted_start(name, observed_bytes)?;
            }
            CanonicalArtifactDisposition::TrailingEmpty => {
                self.residue.push(PhysicalRecoveryResidue::new(
                    name,
                    PhysicalRecoveryResidueKind::TrailingEmptyWalSegment,
                ));
            }
            CanonicalArtifactDisposition::Corrupt {
                denial,
                frames_scanned,
            } => {
                checked_accumulate(&mut self.frames_scanned, frames_scanned)?;
                self.corruptions.push(PhysicalWalArtifactCorruption {
                    artifact: name,
                    identity,
                    denial,
                });
            }
        }
        Ok(())
    }

    fn record_interrupted_start(
        &mut self,
        name: String,
        observed_bytes: u64,
    ) -> Result<(), PhysicalWalArtifactInspectionDenial> {
        checked_accumulate(&mut self.frames_scanned, 1)?;
        checked_accumulate(&mut self.torn_suffix_frames, 1)?;
        checked_accumulate(&mut self.torn_suffix_bytes, observed_bytes)?;
        self.residue
            .push(PhysicalRecoveryResidue::with_observed_bytes(
                name,
                PhysicalRecoveryResidueKind::InterruptedWalSegmentStart,
                observed_bytes,
            ));
        Ok(())
    }

    pub(super) fn finish(
        self,
        canonical_segments_scanned: u64,
        observed_byte_count: u64,
    ) -> Result<InspectedPhysicalWalArtifacts, PhysicalWalArtifactInspectionDenial> {
        let valid_frame_count = checked_candidate_total(&self.candidates, |candidate| {
            candidate.inspection().frame_count()
        })?;
        let valid_byte_count = checked_candidate_total(&self.candidates, |candidate| {
            candidate.inspection().byte_count()
        })?;
        Ok(InspectedPhysicalWalArtifacts {
            candidates: self.candidates,
            residue: self.residue,
            corruptions: self.corruptions,
            canonical_segments_scanned,
            frames_scanned: self.frames_scanned,
            valid_frame_count,
            valid_byte_count,
            observed_byte_count,
            torn_suffix_frames: self.torn_suffix_frames,
            torn_suffix_bytes: self.torn_suffix_bytes,
        })
    }
}

fn checked_accumulate(
    total: &mut u64,
    value: u64,
) -> Result<(), PhysicalWalArtifactInspectionDenial> {
    *total = total
        .checked_add(value)
        .ok_or(PhysicalWalArtifactInspectionDenial::CounterOverflow)?;
    Ok(())
}

fn checked_candidate_total(
    candidates: &[PhysicalWalSegmentCandidate],
    value: impl Fn(&PhysicalWalSegmentCandidate) -> u64,
) -> Result<u64, PhysicalWalArtifactInspectionDenial> {
    candidates.iter().try_fold(0_u64, |total, candidate| {
        total
            .checked_add(value(candidate))
            .ok_or(PhysicalWalArtifactInspectionDenial::CounterOverflow)
    })
}
