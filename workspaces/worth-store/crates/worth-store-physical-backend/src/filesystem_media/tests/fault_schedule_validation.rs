use super::super::*;

#[test]
fn schedules_reject_ambiguous_or_semantically_impossible_matches() {
    let zero = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::PositionedWrite,
        0,
        MediaFaultDirective::AllowPrefix { bytes: 1 },
    )]);
    assert!(matches!(zero, Err(MediaFaultScheduleDenial::ZeroOrdinal)));

    let mismatch = MediaFaultSchedule::for_certification(vec![MediaFaultRule::for_certification(
        MediaOperationRole::ReadMetadata,
        1,
        MediaFaultDirective::AllowPrefix { bytes: 1 },
    )]);
    assert!(matches!(
        mismatch,
        Err(MediaFaultScheduleDenial::DirectiveRoleMismatch)
    ));

    let rule = || {
        MediaFaultRule::for_certification(
            MediaOperationRole::PositionedRead,
            1,
            MediaFaultDirective::FailBefore {
                kind: std::io::ErrorKind::Other,
                raw_os_error: None,
            },
        )
    };
    let duplicate = MediaFaultSchedule::for_certification(vec![rule(), rule()]);
    assert!(matches!(
        duplicate,
        Err(MediaFaultScheduleDenial::DuplicateSemanticMatch)
    ));

    let activation = CertificationMediaFaultActivation::for_certification();
    let activated_rule = |ordinal| {
        MediaFaultRule::for_certification(
            MediaOperationRole::PositionedRead,
            ordinal,
            MediaFaultDirective::PauseBefore(MediaPauseGate::for_certification()),
        )
        .for_next_identified_operation_after_activation(activation.clone())
    };
    let duplicate_activated =
        MediaFaultSchedule::for_certification(vec![activated_rule(1), activated_rule(99)]);
    assert!(matches!(
        duplicate_activated,
        Err(MediaFaultScheduleDenial::DuplicateSemanticMatch)
    ));
}
