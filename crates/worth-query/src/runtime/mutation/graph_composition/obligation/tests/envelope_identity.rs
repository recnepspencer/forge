use super::{
    advisory_plan, allow_plan, blocking_plan, context, envelope_with_rows, row_inventory,
    WorthQueryGraphObligationDispatchEnvelope, WorthQueryGraphObligationDispatchPlan,
    WorthQueryGraphObligationKind, WORTH_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME,
};

#[test]
fn dispatch_envelope_digest_is_stable_under_replay_and_row_order() {
    let first = envelope_with_rows(vec![blocking_plan(), advisory_plan()]);
    let replay = envelope_with_rows(vec![blocking_plan(), advisory_plan()]);
    let reordered = envelope_with_rows(vec![advisory_plan(), blocking_plan()]);

    assert_eq!(first.envelope_digest(), replay.envelope_digest());
    assert_eq!(first.envelope_digest(), reordered.envelope_digest());
    assert_eq!(first.rows().len(), 2);
    assert_eq!(
        first.scheme(),
        WORTH_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME
    );
    assert_eq!(
        first.rows()[0].plan_digest(),
        reordered.rows()[0].plan_digest()
    );
    assert!(
        first.rows()[0].plan_digest() <= first.rows()[1].plan_digest(),
        "sealed envelope rows must expose canonical digest order"
    );
}

#[test]
fn multi_obligation_touch_records_every_fired_rule() {
    let envelope = envelope_with_rows(vec![blocking_plan(), advisory_plan(), allow_plan()]);

    assert_eq!(envelope.rows().len(), 3);
    assert_eq!(envelope.blocking_count(), 1);
    assert_eq!(envelope.advisory_count(), 1);
    assert_eq!(envelope.allow_count(), 1);
    assert_eq!(
        envelope.kind_count(WorthQueryGraphObligationKind::BlockingInvariant),
        1
    );
    assert_eq!(
        row_inventory(&envelope),
        vec![
            (
                "schema".to_string(),
                "closed-loop".to_string(),
                "v1".to_string(),
                "schema-contract-validator".to_string(),
                "allow".to_string(),
                None,
            ),
            (
                "topology".to_string(),
                "loop-wiring".to_string(),
                "v1".to_string(),
                "blocking-invariant".to_string(),
                "block".to_string(),
                Some("loop successor would break closed-loop continuity".to_string()),
            ),
            (
                "topology".to_string(),
                "near-boundary".to_string(),
                "v1".to_string(),
                "advisory-obligation".to_string(),
                "advise".to_string(),
                Some("operation is legal but close to a topology boundary".to_string()),
            ),
        ]
    );
    assert_eq!(
        envelope
            .rows()
            .iter()
            .map(WorthQueryGraphObligationDispatchPlan::plan_digest)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        envelope.rows().len(),
        "every fired rule row must carry a unique plan digest"
    );
}

#[test]
fn verdicts_are_distinct_evidence_not_booleans() {
    let allow = allow_plan();
    let advise = advisory_plan();
    let block = blocking_plan();

    assert_ne!(allow.plan_digest(), advise.plan_digest());
    assert_ne!(allow.plan_digest(), block.plan_digest());
    assert_ne!(advise.plan_digest(), block.plan_digest());
}

#[test]
fn context_digest_participates_in_envelope_identity() {
    let world_a = context("touch.digest", "world.a");
    let world_b = context("touch.digest", "world.b");
    let envelope_a = WorthQueryGraphObligationDispatchEnvelope::builder(world_a)
        .record(blocking_plan())
        .seal()
        .expect("world-a envelope");
    let envelope_b = WorthQueryGraphObligationDispatchEnvelope::builder(world_b)
        .record(blocking_plan())
        .seal()
        .expect("world-b envelope");

    assert_ne!(envelope_a.envelope_digest(), envelope_b.envelope_digest());
}

#[test]
fn touch_digest_participates_in_envelope_identity() {
    let touch_a = context("touch.a", "world.digest");
    let touch_b = context("touch.b", "world.digest");
    let envelope_a = WorthQueryGraphObligationDispatchEnvelope::builder(touch_a)
        .record(blocking_plan())
        .seal()
        .expect("touch-a envelope");
    let envelope_b = WorthQueryGraphObligationDispatchEnvelope::builder(touch_b)
        .record(blocking_plan())
        .seal()
        .expect("touch-b envelope");

    assert_ne!(envelope_a.envelope_digest(), envelope_b.envelope_digest());
}
