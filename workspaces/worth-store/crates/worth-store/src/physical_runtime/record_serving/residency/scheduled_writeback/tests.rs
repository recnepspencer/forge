use super::*;

#[test]
fn exact_range_postures_are_admitted() {
    for posture in [
        PhysicalWritebackRangePosture::ExistingRange,
        PhysicalWritebackRangePosture::CandidateArtifactTail,
    ] {
        assert_eq!(
            require_matching_range_posture(Some(posture), posture),
            Ok(posture)
        );
    }
}

#[test]
fn stale_or_missing_claimed_range_posture_is_rejected() {
    assert_eq!(
        require_matching_range_posture(
            Some(PhysicalWritebackRangePosture::ExistingRange),
            PhysicalWritebackRangePosture::CandidateArtifactTail,
        ),
        Err(PhysicalScheduledWritebackAdmissionDenial::RangePostureMismatch)
    );
    assert_eq!(
        require_matching_range_posture(
            Some(PhysicalWritebackRangePosture::CandidateArtifactTail),
            PhysicalWritebackRangePosture::ExistingRange,
        ),
        Err(PhysicalScheduledWritebackAdmissionDenial::RangePostureMismatch)
    );
    assert_eq!(
        require_matching_range_posture(None, PhysicalWritebackRangePosture::ExistingRange),
        Err(PhysicalScheduledWritebackAdmissionDenial::RangePostureMismatch)
    );
}
