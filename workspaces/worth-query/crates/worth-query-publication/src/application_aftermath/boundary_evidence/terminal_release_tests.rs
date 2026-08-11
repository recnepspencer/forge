use super::{
    publish_attempt_release_posture, WorthQueryPublishedApplicationCommitAttemptReleasePosture,
};

#[test]
fn authoritative_attempt_release_postures_are_exhaustively_published() {
    assert_eq!(
        publish_attempt_release_posture(None),
        WorthQueryPublishedApplicationCommitAttemptReleasePosture::NotAttempted
    );
    assert_eq!(
        publish_attempt_release_posture(Some(true)),
        WorthQueryPublishedApplicationCommitAttemptReleasePosture::Released
    );
    assert_eq!(
        publish_attempt_release_posture(Some(false)),
        WorthQueryPublishedApplicationCommitAttemptReleasePosture::ReleaseFailed
    );
}
