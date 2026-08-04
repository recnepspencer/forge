use super::{CourtroomInvocation, WriteScenario};

#[test]
fn reopen_cannot_receive_oracle_or_scenario_state() {
    for extra in [["--oracle", "oracle"], ["--scenario", "seed-prior-truth"]] {
        let arguments = [
            "reopen",
            "--root",
            "root",
            "--configuration",
            "configuration",
            extra[0],
            extra[1],
        ]
        .into_iter()
        .map(Into::into);
        assert!(CourtroomInvocation::parse(arguments).is_err());
    }
}

#[test]
fn write_scenario_is_typed_at_the_process_boundary() {
    let parsed = CourtroomInvocation::parse(
        [
            "write",
            "--root",
            "root",
            "--configuration",
            "configuration",
            "--oracle",
            "oracle",
            "--scenario",
            "before-backend-dispatch",
        ]
        .into_iter()
        .map(Into::into),
    )
    .unwrap();
    let CourtroomInvocation::Write(invocation) = parsed else {
        panic!("write invocation must retain its mode");
    };
    assert_eq!(invocation.scenario, WriteScenario::BeforeBackendDispatch);
}

#[test]
fn bounded_residency_producer_accepts_only_its_required_paths() {
    let producer = CourtroomInvocation::parse(
        [
            "bounded-residency-producer",
            "--root",
            "root",
            "--configuration",
            "configuration",
        ]
        .into_iter()
        .map(Into::into),
    )
    .unwrap();
    assert!(matches!(
        producer,
        CourtroomInvocation::BoundedResidencyProducer(_)
    ));
}

#[test]
fn bounded_residency_serving_requires_and_accepts_its_schedule() {
    let serving = CourtroomInvocation::parse(
        [
            "bounded-residency-serving",
            "--root",
            "root",
            "--configuration",
            "configuration",
            "--schedule-plan",
            "worker-start-order=first-then-second;\
             equivalent-contender-identity=first-owner;\
             gate-release-order=owner-then-waiter;\
             independent-ready-work-selection=first-worker-then-second",
        ]
        .into_iter()
        .map(Into::into),
    )
    .unwrap();
    assert!(matches!(
        serving,
        CourtroomInvocation::BoundedResidencyServing(_)
    ));

    let missing_schedule = CourtroomInvocation::parse(
        [
            "bounded-residency-serving",
            "--root",
            "root",
            "--configuration",
            "configuration",
        ]
        .into_iter()
        .map(Into::into),
    );
    assert!(missing_schedule.is_err());
}

#[test]
fn bounded_residency_serving_rejects_oracle_sidecars() {
    let denied = CourtroomInvocation::parse(
        [
            "bounded-residency-serving",
            "--root",
            "root",
            "--configuration",
            "configuration",
            "--oracle",
            "oracle",
        ]
        .into_iter()
        .map(Into::into),
    );
    assert!(denied.is_err());
}
