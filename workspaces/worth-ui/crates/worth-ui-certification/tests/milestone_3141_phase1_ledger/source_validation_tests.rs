use super::result_artifact::SourceValidationPosture;

#[test]
fn every_closed_predecessor_phase_uses_historical_artifact_validation() {
    assert_eq!(
        super::row_evidence::source_validation_posture("1"),
        SourceValidationPosture::HistoricalArtifactOnly
    );
    assert_eq!(
        super::row_evidence::source_validation_posture("2"),
        SourceValidationPosture::HistoricalArtifactOnly
    );
    assert_eq!(
        super::row_evidence::source_validation_posture("3"),
        SourceValidationPosture::HistoricalArtifactOnly
    );
    assert_eq!(
        super::row_evidence::source_validation_posture("4"),
        SourceValidationPosture::HistoricalArtifactOnly
    );
    assert_eq!(
        super::row_evidence::source_validation_posture("5"),
        SourceValidationPosture::HistoricalArtifactOnly
    );
    assert_eq!(
        super::row_evidence::source_validation_posture("6"),
        SourceValidationPosture::CurrentSource
    );
}
