use super::*;
use crate::geom_facade::WeakSimpleCertificate;
use forge_topo::bitset::EntityBitset;

#[test]
fn weakly_simple_gate_uses_registry_backed_policy_resolution() {
    let mut selected = EntityBitset::with_capacity(4);
    selected.insert(0).expect("bitset capacity");
    let protected = EntityBitset::with_capacity(4);
    let selection =
        MergeRegionSelection::new(selected, protected, FaceId::new(0, 0));

    let mut ctx = ModelingContext::new();
    ctx.config
        .policy
        .fallback_rules
        .insert(PolicyKind::CoincidentGeometry, false);

    let err = apply_boundary_cert_gate_policy(
        &WeakSimpleCertificate::WeaklySimple { touch_count: 2 },
        &selection,
        &mut ctx,
    )
    .expect_err("session override rejecting CoincidentGeometry must fail merge gate");

    assert!(matches!(
        err,
        KernelError::MergeFailure(MergeError::BoundaryCertificationFailed { .. })
    ));
    assert_eq!(ctx.get_trace_adjuncts().records().len(), 1);

    let payload = ctx.get_trace_adjuncts().records()[0]
        .as_policy_payload()
        .expect("policy adjunct kind")
        .expect("decode policy payload");
    assert_eq!(
        payload.source,
        forge_core::PolicyResolutionSource::DefaultPolicy
    );
    assert_eq!(payload.source_scope);
    assert_eq!(payload.operation_scope_id);
    assert_eq!(
        payload.outcome,
        forge_core::PolicyResolutionOutcome::RejectedPotentialValue
    );
    assert_eq!(
        ctx.get_decision_count(),
        1,
        "policy resolution must emit one traced decision"
    );
}
