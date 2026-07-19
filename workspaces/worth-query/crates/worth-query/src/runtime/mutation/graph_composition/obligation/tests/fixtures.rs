use super::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryGraphCompositionDomainInvariantSummary, WorthQueryGraphObligationDispatchContext,
    WorthQueryGraphObligationDispatchEnvelope, WorthQueryGraphObligationDispatchPlan,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphObligationVerdict,
};

pub(crate) fn row_inventory(
    envelope: &WorthQueryGraphObligationDispatchEnvelope,
) -> Vec<(String, String, String, String, String, Option<String>)> {
    let mut rows = envelope
        .rows()
        .iter()
        .map(|row| {
            (
                row.rule_identity().namespace().to_string(),
                row.rule_identity().name().to_string(),
                row.rule_identity().semantic_version().to_string(),
                row.kind().as_str().to_string(),
                row.verdict().as_str().to_string(),
                row.verdict().context().map(str::to_string),
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

pub(crate) fn envelope_with_rows(
    rows: Vec<WorthQueryGraphObligationDispatchPlan>,
) -> WorthQueryGraphObligationDispatchEnvelope {
    rows.into_iter()
        .fold(
            WorthQueryGraphObligationDispatchEnvelope::builder(context(
                "touch.digest",
                "world.digest",
            )),
            |builder, row| builder.record(row),
        )
        .seal()
        .expect("dispatch envelope")
}

pub(crate) fn context(touch: &str, world: &str) -> WorthQueryGraphObligationDispatchContext {
    WorthQueryGraphObligationDispatchContext::graph_composition(touch, world)
        .expect("dispatch context")
}

pub(crate) fn blocking_plan() -> WorthQueryGraphObligationDispatchPlan {
    WorthQueryGraphObligationDispatchPlan::blocking_invariant("topology.loop-wiring")
        .with_rule_identity(rule("topology", "loop-wiring", "v1"))
        .verdict(
            WorthQueryGraphObligationVerdict::block(
                "loop successor would break closed-loop continuity",
            )
            .expect("blocking verdict"),
        )
        .expect("blocking plan")
}

pub(crate) fn advisory_plan() -> WorthQueryGraphObligationDispatchPlan {
    WorthQueryGraphObligationDispatchPlan::advisory("topology.near-boundary")
        .with_rule_identity(rule("topology", "near-boundary", "v1"))
        .verdict(
            WorthQueryGraphObligationVerdict::advise(
                "operation is legal but close to a topology boundary",
            )
            .expect("advisory verdict"),
        )
        .expect("advisory plan")
}

pub(crate) fn allow_plan() -> WorthQueryGraphObligationDispatchPlan {
    WorthQueryGraphObligationDispatchPlan::schema_contract_validator("schema.closed-loop")
        .with_rule_identity(rule("schema", "closed-loop", "v1"))
        .verdict(WorthQueryGraphObligationVerdict::allow())
        .expect("allow plan")
}

pub(crate) fn schema_contract_block_plan() -> WorthQueryGraphObligationDispatchPlan {
    WorthQueryGraphObligationDispatchPlan::schema_contract_validator("schema.closed-loop")
        .with_rule_identity(rule("schema", "closed-loop", "v1"))
        .verdict(
            WorthQueryGraphObligationVerdict::block(
                "schema contract would be violated by composition",
            )
            .expect("schema contract block verdict"),
        )
        .expect("schema contract block plan")
}

pub(crate) fn preflight_block_plan() -> WorthQueryGraphObligationDispatchPlan {
    WorthQueryGraphObligationDispatchPlan::preflight_sequencing("motion.finish-before-witness")
        .with_rule_identity(rule("motion", "finish-before-witness", "v1"))
        .verdict(
            WorthQueryGraphObligationVerdict::block(
                "finish cannot execute before witness is admitted",
            )
            .expect("preflight block verdict"),
        )
        .expect("preflight block plan")
}

pub(crate) fn capability_gap_block_plan() -> WorthQueryGraphObligationDispatchPlan {
    WorthQueryGraphObligationDispatchPlan::capability_gap_screen("support.store-backed-gap")
        .with_rule_identity(rule("support", "store-backed-gap", "v1"))
        .verdict(
            WorthQueryGraphObligationVerdict::block("required capability is not admitted")
                .expect("capability gap block verdict"),
        )
        .expect("capability gap block plan")
}

pub(crate) fn operating_context_block_plan() -> WorthQueryGraphObligationDispatchPlan {
    WorthQueryGraphObligationDispatchPlan::operating_context_gate("policy.restricted-world")
        .with_rule_identity(rule("policy", "restricted-world", "v1"))
        .verdict(
            WorthQueryGraphObligationVerdict::block(
                "restricted operating world denies this graph touch",
            )
            .expect("operating context block verdict"),
        )
        .expect("operating context block plan")
}

pub(crate) fn rule(
    namespace: &str,
    name: &str,
    version: &str,
) -> WorthQueryGraphObligationRuleIdentity {
    WorthQueryGraphObligationRuleIdentity::new(namespace, name, version).expect("rule identity")
}

pub(crate) fn domain_invariant_summary() -> WorthQueryGraphCompositionDomainInvariantSummary {
    WorthQueryGraphCompositionDomainInvariantSummary::from_parts(
        vec!["topology.half_edge".to_string()],
        vec!["half_edge_a".to_string()],
        vec!["same_batch_entity_relation_identity_edges".to_string()],
        vec!["mixed_existing_target_retarget".to_string()],
        evidence_identity("program"),
        evidence_identity("breadth"),
        "components=2;symbolic_entities=1".to_string(),
    )
}

fn evidence_identity(label: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_value(crate::WorthQueryEvidenceTag::new("test_label"), label)
        .seal()
}
