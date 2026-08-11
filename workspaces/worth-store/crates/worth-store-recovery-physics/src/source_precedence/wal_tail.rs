use worth_store_wal::{
    InterruptedWalTail, VerifiedWalFrame, WalLsnRange, WalSegmentArtifactIdentity,
    WalSegmentInspection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWalSegmentCandidate {
    inspection: WalSegmentInspection,
    interrupted_tail: Option<InterruptedWalTail>,
    frames: Box<[VerifiedWalFrame]>,
    selected_range: Option<WalLsnRange>,
    selected_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedPhysicalWalTail {
    segments: Vec<PhysicalWalSegmentCandidate>,
    checkpoint_covered: Vec<super::CheckpointCoveredWalArtifact>,
    frame_count: u64,
    byte_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedPhysicalWalTailDenial {
    DuplicateArtifact,
    GenerationMismatch,
    SegmentGap,
    LsnGap,
    CheckpointFrontierMismatch,
    InterruptedMiddleSegment,
    CounterOverflow,
}

impl PhysicalWalSegmentCandidate {
    pub fn verified(
        inspection: WalSegmentInspection,
        interrupted_tail: Option<InterruptedWalTail>,
    ) -> Self {
        Self {
            inspection,
            interrupted_tail,
            frames: Box::new([]),
            selected_range: None,
            selected_bytes: None,
        }
    }

    pub(crate) fn verified_with_frames(
        inspection: WalSegmentInspection,
        interrupted_tail: Option<InterruptedWalTail>,
        frames: Vec<VerifiedWalFrame>,
    ) -> Self {
        Self {
            inspection,
            interrupted_tail,
            frames: frames.into_boxed_slice(),
            selected_range: None,
            selected_bytes: None,
        }
    }

    pub const fn identity(&self) -> WalSegmentArtifactIdentity {
        self.inspection.identity()
    }

    pub const fn inspection(&self) -> WalSegmentInspection {
        self.inspection
    }

    pub const fn interrupted_tail(&self) -> Option<InterruptedWalTail> {
        self.interrupted_tail
    }

    pub fn frames(&self) -> &[VerifiedWalFrame] {
        &self.frames
    }

    fn selected_range(&self) -> WalLsnRange {
        self.selected_range
            .unwrap_or_else(|| self.inspection.lsn_range())
    }

    fn selected_frame_count(&self) -> u64 {
        self.selected_range
            .map_or(self.inspection.frame_count(), |_| self.frames.len() as u64)
    }

    fn selected_byte_count(&self) -> u64 {
        self.selected_bytes
            .unwrap_or_else(|| self.inspection.byte_count())
    }

    fn trim_before(mut self, frontier: u64) -> Result<Option<Self>, SelectedPhysicalWalTailDenial> {
        let range = self.inspection.lsn_range();
        if range.end_exclusive().get() <= frontier {
            return Ok(None);
        }
        if range.start().get() >= frontier {
            return Ok(Some(self));
        }
        let first = self
            .frames
            .iter()
            .position(|frame| frame.lsn_range().start().get() >= frontier)
            .ok_or(SelectedPhysicalWalTailDenial::CheckpointFrontierMismatch)?;
        if self.frames[first].lsn_range().start().get() != frontier {
            return Err(SelectedPhysicalWalTailDenial::CheckpointFrontierMismatch);
        }
        self.frames = self.frames.into_vec().split_off(first).into_boxed_slice();
        let start = self.frames.first().unwrap().lsn_range().start();
        let end = self.frames.last().unwrap().lsn_range().end_exclusive();
        self.selected_range = Some(
            WalLsnRange::new(start, end)
                .map_err(|_| SelectedPhysicalWalTailDenial::CheckpointFrontierMismatch)?,
        );
        self.selected_bytes = Some(
            self.frames
                .iter()
                .try_fold(0_u64, |bytes, frame| {
                    bytes.checked_add(frame.encoded_bytes())
                })
                .ok_or(SelectedPhysicalWalTailDenial::CounterOverflow)?,
        );
        Ok(Some(self))
    }
}

pub fn admit_physical_wal_tail(
    checkpoint_frontier: u64,
    mut candidates: Vec<PhysicalWalSegmentCandidate>,
) -> Result<SelectedPhysicalWalTail, SelectedPhysicalWalTailDenial> {
    candidates.sort_unstable_by_key(|candidate| candidate.identity());
    if candidates
        .windows(2)
        .any(|pair| pair[0].identity() == pair[1].identity())
    {
        return Err(SelectedPhysicalWalTailDenial::DuplicateArtifact);
    }
    let mut checkpoint_covered = Vec::new();
    let mut retained = Vec::new();
    for candidate in candidates {
        if candidate.inspection().lsn_range().end_exclusive().get() <= checkpoint_frontier {
            checkpoint_covered.push(super::CheckpointCoveredWalArtifact::from_candidate(
                &candidate,
            ));
            continue;
        }
        if let Some(candidate) = candidate.trim_before(checkpoint_frontier)? {
            retained.push(candidate);
        }
    }
    candidates = retained;
    let mut frame_count = 0_u64;
    let mut byte_count = 0_u64;
    for (index, candidate) in candidates.iter().enumerate() {
        crate::wal_prefix::classify_terminal_interruption(
            index,
            candidates.len(),
            candidate.interrupted_tail(),
        )
        .map_err(map_prefix_denial)?;
        if let Some(previous) = index.checked_sub(1).map(|prior| &candidates[prior]) {
            if previous.identity() == candidate.identity() {
                return Err(SelectedPhysicalWalTailDenial::DuplicateArtifact);
            }
            if previous.identity().generation() != candidate.identity().generation() {
                return Err(SelectedPhysicalWalTailDenial::GenerationMismatch);
            }
            if previous.identity().segment().get().checked_add(1)
                != Some(candidate.identity().segment().get())
            {
                return Err(SelectedPhysicalWalTailDenial::SegmentGap);
            }
        }
        frame_count = frame_count
            .checked_add(candidate.selected_frame_count())
            .ok_or(SelectedPhysicalWalTailDenial::CounterOverflow)?;
        byte_count = byte_count
            .checked_add(candidate.selected_byte_count())
            .ok_or(SelectedPhysicalWalTailDenial::CounterOverflow)?;
    }
    crate::wal_prefix::require_contiguous_prefix(
        checkpoint_frontier,
        candidates
            .iter()
            .map(PhysicalWalSegmentCandidate::selected_range),
    )
    .map_err(map_prefix_denial)?;
    let facts = crate::wal_prefix::WalValidPrefixFacts {
        frame_count,
        byte_count,
    };
    Ok(SelectedPhysicalWalTail {
        segments: candidates,
        checkpoint_covered,
        frame_count: facts.frame_count,
        byte_count: facts.byte_count,
    })
}

fn map_prefix_denial(
    denial: crate::wal_prefix::WalPrefixAdmissionDenial,
) -> SelectedPhysicalWalTailDenial {
    match denial {
        crate::wal_prefix::WalPrefixAdmissionDenial::FrontierMismatch => {
            SelectedPhysicalWalTailDenial::CheckpointFrontierMismatch
        }
        crate::wal_prefix::WalPrefixAdmissionDenial::Gap => SelectedPhysicalWalTailDenial::LsnGap,
        crate::wal_prefix::WalPrefixAdmissionDenial::InterruptedMiddle => {
            SelectedPhysicalWalTailDenial::InterruptedMiddleSegment
        }
    }
}

impl SelectedPhysicalWalTail {
    pub fn segments(&self) -> &[PhysicalWalSegmentCandidate] {
        &self.segments
    }

    pub fn checkpoint_covered(&self) -> &[super::CheckpointCoveredWalArtifact] {
        &self.checkpoint_covered
    }

    pub const fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub const fn byte_count(&self) -> u64 {
        self.byte_count
    }

    pub fn frames(&self) -> impl Iterator<Item = &VerifiedWalFrame> {
        self.segments.iter().flat_map(|segment| segment.frames())
    }
}

#[cfg(test)]
mod tests {
    use worth_store_wal::{
        inspect_verified_wal_active_tail, inspect_verified_wal_segment, prepare_wal_frame_append,
        WalSegmentArtifactIdentity, WalSegmentGeneration, WalSegmentId,
    };

    use super::*;

    #[test]
    fn semantic_segment_order_is_deterministic_and_checkpoint_contiguous() {
        let first = complete_candidate(1, 10, 20);
        let second = complete_candidate(2, 20, 30);
        let selected = admit_physical_wal_tail(10, vec![second, first]).unwrap();
        assert_eq!(selected.segments()[0].identity().segment().get(), 1);
        assert_eq!(selected.segments()[1].identity().segment().get(), 2);
        assert_eq!(selected.frame_count(), 2);
    }

    #[test]
    fn whole_checkpoint_covered_artifacts_are_retained_as_cleanup_facts() {
        let covered = complete_candidate(1, 0, 10);
        let retained = complete_candidate(2, 10, 20);
        let selected = admit_physical_wal_tail(10, vec![retained, covered]).unwrap();

        assert_eq!(selected.segments().len(), 1);
        assert_eq!(selected.segments()[0].identity().segment().get(), 2);
        assert_eq!(selected.checkpoint_covered().len(), 1);
        assert_eq!(
            selected.checkpoint_covered()[0].identity().segment().get(),
            1
        );
        assert_eq!(
            selected.checkpoint_covered()[0].lsn_range().start().get(),
            0
        );
        assert_eq!(
            selected.checkpoint_covered()[0]
                .lsn_range()
                .end_exclusive()
                .get(),
            10
        );
        assert!(selected.checkpoint_covered()[0].byte_count() > 0);
        assert!(selected.checkpoint_covered()[0].cleanup_safe());
    }

    #[test]
    fn interrupted_checkpoint_covered_artifact_remains_an_unsafe_cleanup_fact() {
        let covered = interrupted_candidate(1, 0, 10, 20);
        let observed = covered.interrupted_tail().unwrap().observed_bytes();
        let retained = complete_candidate(2, 10, 20);
        let selected = admit_physical_wal_tail(10, vec![retained, covered]).unwrap();

        assert_eq!(selected.checkpoint_covered().len(), 1);
        assert!(!selected.checkpoint_covered()[0].cleanup_safe());
        assert_eq!(selected.checkpoint_covered()[0].byte_count(), observed);
    }

    #[test]
    fn segment_and_lsn_gaps_are_independently_rejected() {
        assert_eq!(
            admit_physical_wal_tail(
                10,
                vec![complete_candidate(1, 10, 20), complete_candidate(3, 20, 30)],
            ),
            Err(SelectedPhysicalWalTailDenial::SegmentGap)
        );
        assert_eq!(
            admit_physical_wal_tail(
                10,
                vec![complete_candidate(1, 10, 20), complete_candidate(2, 21, 30)],
            ),
            Err(SelectedPhysicalWalTailDenial::LsnGap)
        );
    }

    #[test]
    fn interrupted_suffix_is_legal_only_on_the_terminal_segment() {
        let interrupted = interrupted_candidate(1, 10, 20, 30);
        let terminal = admit_physical_wal_tail(10, vec![interrupted.clone()]).unwrap();
        assert!(terminal.segments()[0].interrupted_tail().is_some());
        assert_eq!(
            admit_physical_wal_tail(10, vec![interrupted, complete_candidate(2, 20, 30)],),
            Err(SelectedPhysicalWalTailDenial::InterruptedMiddleSegment)
        );
    }

    fn complete_candidate(segment: u64, start: u64, end: u64) -> PhysicalWalSegmentCandidate {
        let directory = tempfile::tempdir().unwrap();
        let plan = prepare_wal_frame_append(
            directory.path(),
            segment,
            1,
            start,
            end,
            "test-frame",
            b"payload",
        )
        .unwrap();
        let identity = identity(segment);
        let verified = inspect_verified_wal_segment(identity, plan.encoded_frame()).unwrap();
        PhysicalWalSegmentCandidate::verified(verified.inspection(), None)
    }

    fn interrupted_candidate(
        segment: u64,
        start: u64,
        first_end: u64,
        second_end: u64,
    ) -> PhysicalWalSegmentCandidate {
        let directory = tempfile::tempdir().unwrap();
        let first = prepare_wal_frame_append(
            directory.path(),
            segment,
            1,
            start,
            first_end,
            "first-frame",
            b"first",
        )
        .unwrap();
        let path = directory.path().join(first.relative_path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, first.encoded_frame()).unwrap();
        let second = prepare_wal_frame_append(
            directory.path(),
            segment,
            1,
            first_end,
            second_end,
            "second-frame",
            b"second",
        )
        .unwrap();
        let mut bytes = first.encoded_frame().to_vec();
        bytes.extend_from_slice(&second.encoded_frame()[..20]);
        let active = inspect_verified_wal_active_tail(identity(segment), &bytes).unwrap();
        let interruption = active.interrupted_tail();
        PhysicalWalSegmentCandidate::verified(
            active.into_verified_prefix().inspection(),
            interruption,
        )
    }

    fn identity(segment: u64) -> WalSegmentArtifactIdentity {
        WalSegmentArtifactIdentity::new(
            WalSegmentId::new(segment).unwrap(),
            WalSegmentGeneration::new(1).unwrap(),
        )
    }
}
