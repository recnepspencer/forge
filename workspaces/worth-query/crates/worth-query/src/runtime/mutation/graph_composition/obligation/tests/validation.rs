use super::{
    context, rule, WorthQueryGraphObligationDispatchEnvelope,
    WorthQueryGraphObligationDispatchError, WorthQueryGraphObligationDispatchPlan,
    WorthQueryGraphObligationVerdict,
};

#[test]
fn sealed_envelope_rejects_empty_dispatch_rows() {
    let error =
        WorthQueryGraphObligationDispatchEnvelope::builder(context("touch.digest", "world.digest"))
            .seal()
            .expect_err("empty envelopes are not phase-one proof");

    assert_eq!(error, WorthQueryGraphObligationDispatchError::EmptyEnvelope);
}

#[test]
fn typed_validation_rejects_empty_verdict_context() {
    let error =
        WorthQueryGraphObligationVerdict::block(" ").expect_err("empty block context rejected");

    assert_eq!(
        error,
        WorthQueryGraphObligationDispatchError::EmptyVerdictContext
    );
}

#[test]
fn typed_validation_preserves_empty_rule_name_denial() {
    let error = WorthQueryGraphObligationDispatchPlan::blocking_invariant(" ")
        .verdict(WorthQueryGraphObligationVerdict::allow())
        .expect_err("empty default rule name rejected");

    assert_eq!(error, WorthQueryGraphObligationDispatchError::EmptyRuleName);
}

#[test]
fn constructors_normalize_whitespace_before_canonical_identity() {
    let compact_context = context("touch.digest", "world.digest");
    let padded_context = context(" touch.digest ", " world.digest ");
    let compact_rule = rule("topology", "loop-wiring", "v1");
    let padded_rule = rule(" topology ", " loop-wiring ", " v1 ");
    let compact_verdict =
        WorthQueryGraphObligationVerdict::block("loop successor would break continuity")
            .expect("compact verdict");
    let padded_verdict =
        WorthQueryGraphObligationVerdict::block(" loop successor would break continuity ")
            .expect("padded verdict");

    assert_eq!(
        compact_context.context_digest(),
        padded_context.context_digest()
    );
    assert_eq!(
        compact_rule.identity_digest(),
        padded_rule.identity_digest()
    );
    assert_eq!(compact_verdict.context(), padded_verdict.context());
}
