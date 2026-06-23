use super::support::*;

#[test]
fn multi_denial_attachment_is_deterministic_across_replay_and_registration_order() {
    let left = blocking_registration("multi-denial-left");
    let right = blocking_registration("multi-denial-right");

    let first = denied_attachment_projection([left.clone(), right.clone()], "multi-denial");
    let replay = denied_attachment_projection([left, right.clone()], "multi-denial");
    let reversed = denied_attachment_projection(
        [right, blocking_registration("multi-denial-left")],
        "multi-denial",
    );

    assert_eq!(first.rows().len(), 2);
    assert_eq!(first.projection_digest(), replay.projection_digest());
    assert_eq!(first.projection_digest(), reversed.projection_digest());
    assert_eq!(
        first
            .rows()
            .iter()
            .map(|row| row.rule_identity_digest())
            .collect::<Vec<_>>(),
        reversed
            .rows()
            .iter()
            .map(|row| row.rule_identity_digest())
            .collect::<Vec<_>>()
    );
    assert!(first
        .rows()
        .windows(2)
        .all(|rows| { rows[0].rule_identity_digest() <= rows[1].rule_identity_digest() }));
}

#[test]
fn denial_wording_does_not_mutate_rule_identity_but_rule_identity_changes_projection() {
    let unsupported = blocking_registration("wording-stable-rule").with_support_posture(
        ForgeQueryGraphObligationSupportPosture::unsupported(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    );
    let capability_gap = task_collection_registration(
        ForgeQueryGraphObligationKind::CapabilityGapScreen,
        "wording-stable-rule",
        supported_command_batch_posture(),
    );
    let changed_identity = blocking_registration("changed-rule-identity");

    let unsupported_denial = denied_attachment_projection([unsupported], "wording-stable");
    let capability_gap_denial = denied_attachment_projection([capability_gap], "wording-stable");
    let changed_identity_denial =
        denied_attachment_projection([changed_identity], "wording-stable");

    let unsupported_row = unsupported_denial.rows().first().unwrap();
    let capability_gap_row = capability_gap_denial.rows().first().unwrap();
    let changed_identity_row = changed_identity_denial.rows().first().unwrap();

    assert_eq!(
        unsupported_row.rule_identity_digest(),
        capability_gap_row.rule_identity_digest()
    );
    assert_ne!(
        unsupported_row.verdict_context(),
        capability_gap_row.verdict_context()
    );
    assert_ne!(
        unsupported_denial.projection_digest(),
        capability_gap_denial.projection_digest()
    );
    assert_ne!(
        capability_gap_row.rule_identity_digest(),
        changed_identity_row.rule_identity_digest()
    );
    assert_ne!(
        capability_gap_denial.projection_digest(),
        changed_identity_denial.projection_digest()
    );
}

fn blocking_registration(name: &str) -> ForgeQueryGraphObligationRegistration {
    task_collection_registration(
        ForgeQueryGraphObligationKind::BlockingInvariant,
        name,
        ForgeQueryGraphObligationSupportPosture::unsupported(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    )
}

fn denied_attachment_projection(
    registrations: impl IntoIterator<Item = ForgeQueryGraphObligationRegistration>,
    task_id: &str,
) -> ForgeQueryGraphObligationDenialAttachmentProjection {
    let mut runtime = runtime_with_registrations(registrations);
    let error = runtime
        .write_batch(vec![task_insert_command(task_id)])
        .expect_err("blocking graph obligations should deny through runtime");
    let ForgeQueryRuntimeError::GraphObligationDenied(denial) = error else {
        panic!("expected graph obligation denial, got {error:?}");
    };
    denial.attachment_projection().clone()
}
