use super::super::*;

#[test]
fn identified_operation_selector_rejects_unbound_recovery_helpers() {
    let rule = MediaFaultRule::for_certification(
        MediaOperationRole::SynchronizeFileState,
        3,
        MediaFaultDirective::FailBarrier {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    )
    .for_identified_operation();
    let binding = MediaOperationIdentityBinding {
        owner: None,
        runtime_incarnation: None,
        store: None,
    };
    let recovery_helper = MediaOperationContext::new(
        binding,
        MediaOperationRole::SynchronizeFileState,
        0,
        MediaOperationCoordinates::unbound(),
        3,
        None,
    );
    assert!(!rule.matches(recovery_helper));

    let record_artifact = MediaOperationContext::new(
        binding,
        MediaOperationRole::SynchronizeFileState,
        0,
        MediaOperationCoordinates::for_path(
            MediaOperationIdentity::for_test(17),
            MediaPathRole::ArtifactOwned,
            None,
        ),
        3,
        Some(1),
    );
    assert!(rule.matches(record_artifact));
}

#[test]
fn identified_operation_ordinal_ignores_unbound_raw_ordinal_drift() {
    let rule = MediaFaultRule::for_certification(
        MediaOperationRole::SynchronizeFileState,
        2,
        MediaFaultDirective::FailBarrier {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
    )
    .for_identified_operation_ordinal();
    let binding = MediaOperationIdentityBinding {
        owner: None,
        runtime_incarnation: None,
        store: None,
    };
    let coordinates = |operation| {
        MediaOperationCoordinates::for_path(
            MediaOperationIdentity::for_test(operation),
            MediaPathRole::ArtifactOwned,
            None,
        )
    };
    let first_identified = MediaOperationContext::new(
        binding,
        MediaOperationRole::SynchronizeFileState,
        0,
        coordinates(17),
        2,
        Some(1),
    );
    let second_identified = MediaOperationContext::new(
        binding,
        MediaOperationRole::SynchronizeFileState,
        0,
        coordinates(18),
        9,
        Some(2),
    );
    assert!(!rule.matches(first_identified));
    assert!(rule.matches(second_identified));
}
