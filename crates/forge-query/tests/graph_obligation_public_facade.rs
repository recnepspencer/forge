use forge_query::facade::runtime::{
    ForgeQueryAuthoritativeMutationObligationDispatchProjection,
    ForgeQueryAuthoritativeMutationObligationDispatchProjectionRow,
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationDispatchContext,
    ForgeQueryGraphObligationDispatchEnvelope, ForgeQueryGraphObligationDispatchPlan,
    ForgeQueryGraphObligationExecutionBudget, ForgeQueryGraphObligationExecutionCostClass,
    ForgeQueryGraphObligationExecutionResultEnvelope, ForgeQueryGraphObligationExecutionScope,
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationIndex,
    ForgeQueryGraphObligationIndexComplexityContractStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationMaterializedDispatch,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportMatrix,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphObligationSupportStatus,
    ForgeQueryGraphObligationVerdict, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorKind, ForgeQueryGraphTouchReadVerb,
    FORGE_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME,
};

#[test]
fn public_facade_graph_obligation_replay_and_row_order_are_canonical() {
    let first = envelope_with_rows(vec![blocking_plan(), advisory_plan(), allow_plan()]);
    let replay = envelope_with_rows(vec![blocking_plan(), advisory_plan(), allow_plan()]);
    let reordered = envelope_with_rows(vec![allow_plan(), advisory_plan(), blocking_plan()]);

    assert_eq!(first.envelope_digest(), replay.envelope_digest());
    assert_eq!(first.envelope_digest(), reordered.envelope_digest());
    assert_eq!(
        first.scheme(),
        FORGE_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME
    );
    assert_eq!(first.rows().len(), 3);
    assert_eq!(first.blocking_count(), 1);
    assert_eq!(first.advisory_count(), 1);
    assert_eq!(first.allow_count(), 1);
    assert_eq!(
        first.kind_count(ForgeQueryGraphObligationKind::BlockingInvariant),
        1
    );
    assert_eq!(row_inventory(&first), row_inventory(&reordered));
}

#[test]
fn public_facade_exposes_inspectable_graph_obligation_index() {
    let index = ForgeQueryGraphObligationIndex::from_catalog(
        &ForgeQueryGraphObligationRegistrationCatalog::empty(),
    );
    let world = ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();

    assert_eq!(index.registration_count(), 0);
    assert_eq!(index.bucket_count(), 0);
    assert_eq!(index.support_rows().len(), 6);
    assert_eq!(index.build_counters().registration_count(), 0);
    assert_eq!(index.build_counters().entry_count(), 0);
    assert_eq!(index.build_counters().bucket_count(), 0);
    assert_eq!(index.build_counters().support_row_count(), 6);
    assert_eq!(index.build_counters().complexity_contract_count(), 2);
    assert_eq!(index.complexity_contracts().len(), 2);
    assert!(index.complexity_contracts().iter().all(|contract| {
        contract.status() == ForgeQueryGraphObligationIndexComplexityContractStatus::Verified
    }));
    assert_eq!(world.kind().as_str(), "any-committed-authority");
}

#[test]
fn public_facade_exposes_graph_obligation_budget_and_support_posture_types() {
    let budget = ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
        ForgeQueryGraphObligationExecutionScope::TouchedRelationKind,
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
    );
    let posture = ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    )
    .with_execution_budget(budget.clone());

    assert_eq!(
        posture.lane(),
        ForgeQueryGraphObligationSupportLane::GraphComposition
    );
    assert_eq!(posture.lane_label(), "graph-composition");
    assert_eq!(
        posture.status(),
        ForgeQueryGraphObligationSupportStatus::Supported
    );
    assert_eq!(
        posture.execution_budget().budget_digest(),
        budget.budget_digest()
    );
    assert!(ForgeQueryGraphObligationExecutionStatus::BudgetExceeded.is_budget_denial());
    assert_eq!(
        ForgeQueryGraphObligationExecutionCostClass::SparseTopology.as_str(),
        "sparse-topology"
    );
}

#[test]
fn public_facade_exposes_graph_obligation_consumer_proof_surfaces() {
    let read_descriptor = ForgeQueryGraphTouchDescriptor::read_family(
        "TaskEdge",
        [ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology],
    )
    .expect("public read descriptor");
    let selection = ForgeQueryGraphObligationIndex::from_catalog(
        &ForgeQueryGraphObligationRegistrationCatalog::empty(),
    )
    .select_for_touch(
        &read_descriptor,
        &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    );
    let dispatch = ForgeQueryGraphObligationMaterializedDispatch::from_selection(selection);
    let result_envelope: ForgeQueryGraphObligationExecutionResultEnvelope =
        dispatch.selected_result_envelope();
    let support_matrix = ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation();

    assert_eq!(
        read_descriptor.kind(),
        ForgeQueryGraphTouchDescriptorKind::ReadFamily
    );
    assert_eq!(dispatch.inputs().len(), 0);
    assert_eq!(result_envelope.rows().len(), 0);
    assert!(
        support_matrix
            .supported_lane_count_for_kind(ForgeQueryGraphObligationKind::BlockingInvariant)
            > 0
    );
}

#[test]
fn public_facade_exposes_authoritative_dispatch_projection_types() {
    fn accept_projection(
        _projection: Option<&ForgeQueryAuthoritativeMutationObligationDispatchProjection>,
        _row: Option<&ForgeQueryAuthoritativeMutationObligationDispatchProjectionRow>,
    ) {
    }

    accept_projection(None, None);
}

fn envelope_with_rows(
    rows: Vec<ForgeQueryGraphObligationDispatchPlan>,
) -> ForgeQueryGraphObligationDispatchEnvelope {
    rows.into_iter()
        .fold(
            ForgeQueryGraphObligationDispatchEnvelope::builder(
                ForgeQueryGraphObligationDispatchContext::graph_composition(
                    "touch.digest",
                    "world.digest",
                )
                .expect("public facade dispatch context"),
            ),
            |builder, row| builder.record(row),
        )
        .seal()
        .expect("public facade dispatch envelope")
}

fn row_inventory(
    envelope: &ForgeQueryGraphObligationDispatchEnvelope,
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

fn blocking_plan() -> ForgeQueryGraphObligationDispatchPlan {
    ForgeQueryGraphObligationDispatchPlan::blocking_invariant("topology.loop-wiring")
        .with_rule_identity(rule("topology", "loop-wiring", "v1"))
        .verdict(
            ForgeQueryGraphObligationVerdict::block(
                "loop successor would break closed-loop continuity",
            )
            .expect("blocking verdict"),
        )
        .expect("blocking plan")
}

fn advisory_plan() -> ForgeQueryGraphObligationDispatchPlan {
    ForgeQueryGraphObligationDispatchPlan::advisory("topology.near-boundary")
        .with_rule_identity(rule("topology", "near-boundary", "v1"))
        .verdict(
            ForgeQueryGraphObligationVerdict::advise(
                "operation is legal but close to a topology boundary",
            )
            .expect("advisory verdict"),
        )
        .expect("advisory plan")
}

fn allow_plan() -> ForgeQueryGraphObligationDispatchPlan {
    ForgeQueryGraphObligationDispatchPlan::schema_contract_validator("schema.closed-loop")
        .with_rule_identity(rule("schema", "closed-loop", "v1"))
        .verdict(ForgeQueryGraphObligationVerdict::allow())
        .expect("allow plan")
}

fn rule(namespace: &str, name: &str, version: &str) -> ForgeQueryGraphObligationRuleIdentity {
    ForgeQueryGraphObligationRuleIdentity::new(namespace, name, version).expect("rule identity")
}
