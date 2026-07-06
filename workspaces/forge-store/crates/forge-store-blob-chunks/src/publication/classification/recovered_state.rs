use super::crash_point::BlobPublicationCrashPoint;
use super::super::types::BlobPublicationRecoveredState;
use super::super::BlobPublicationCounterSnapshot;

pub(crate) fn classify_recovered_state(
    crash_point: BlobPublicationCrashPoint,
    counters: BlobPublicationCounterSnapshot,
) -> BlobPublicationRecoveredState {
    match crash_point {
        BlobPublicationCrashPoint::AfterChunkWrite => {
            BlobPublicationRecoveredState::DurableChunkNotVisible { counters }
        }
        BlobPublicationCrashPoint::AfterChecksumAdmission => {
            BlobPublicationRecoveredState::ChecksumAdmittedNotVisible { counters }
        }
        BlobPublicationCrashPoint::AfterChunkTreeNodeDurability => {
            BlobPublicationRecoveredState::ChunkTreeNodeDurableNotVisible { counters }
        }
        BlobPublicationCrashPoint::AfterRootCandidateFormation => {
            BlobPublicationRecoveredState::RootCandidateNotVisible { counters }
        }
        BlobPublicationCrashPoint::AfterReachabilityStaging => {
            BlobPublicationRecoveredState::ReachabilityStagedNotVisible { counters }
        }
        BlobPublicationCrashPoint::AfterPublicationRecordWrite => {
            BlobPublicationRecoveredState::PublicationRecordReplayableNotVisible { counters }
        }
        BlobPublicationCrashPoint::AfterSessionClose => {
            BlobPublicationRecoveredState::SessionClosedAwaitingVisibilityCommit { counters }
        }
    }
}