use super::declaration::PlanarBooleanSplitEdgeChainLedgerDeclaration;
use super::denial::PlanarBooleanSplitEdgeChainLedgerDenialKind;
use super::ledger::{reject_incomplete_chain_authority_bindings, ChainAuthorityBindings};
use super::query_domain::{
    PlanarBooleanSplitEdgeChainLedgerQueryDomain, PlanarBooleanSplitEdgeChainLedgerQueryInput,
    PlanarBooleanSplitEdgeChainLedgerQueryResult,
};
use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::tests_support::{
    raw_interval_entry, raw_point_entry, raw_schedule, raw_set_from_schedules,
};
use crate::workload_platform::planar_boolean_edge_splitting::source_edge_carrier_recovery::test_support::{
    source_carriers, subject_with_carriers,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanMicroIntervalPolicy, PlanarBooleanSplitDecisionLogQueryDomain,
    PlanarBooleanSplitDecisionLogQueryInput, PlanarBooleanSplitDecisionLogQueryResult,
    PlanarBooleanSplitPersistentNamingCounters, PlanarBooleanSplitPersistentNamingInput,
    PlanarBooleanSplitPersistentNamingQueryBasis, PlanarBooleanSplitPersistentNamingReceipt,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitChainValidationCounters, PlanarBooleanSplitChainValidationReceipt,
    PlanarBooleanSplitEdgeFragmentSet, PlanarBooleanSplitVertexIdentitySet,
};

#[test]
fn split_edge_chain_ledger_declaration_rejects_missing_product_identity() {
    let denial = PlanarBooleanSplitEdgeChainLedgerDeclaration::from_product_identities(
        "split-request",
        "chain-validation",
        "",
        "decision-log",
    )
    .expect_err("empty product identity must deny ledger declaration");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEdgeChainLedgerDenialKind::EmptyQueryDeclarationIdentity
    );
}

#[test]
fn split_edge_chain_ledger_declaration_is_canonical_for_bound_products() {
    let first = PlanarBooleanSplitEdgeChainLedgerDeclaration::from_product_identities(
        "split-request",
        "chain-validation",
        "persistent-naming",
        "decision-log",
    )
    .expect("bound query products should declare a ledger");
    let second = PlanarBooleanSplitEdgeChainLedgerDeclaration::from_product_identities(
        "split-request",
        "chain-validation",
        "persistent-naming",
        "decision-log",
    )
    .expect("same query products should redeclare the same ledger");

    assert_eq!(first, second);
    assert!(!first.declaration_identity().is_empty());
    assert_eq!(
        first.lowered_plan_identity(),
        format!("lowered:{}", first.declaration_identity())
    );
}

#[test]
fn split_edge_chain_ledger_rejects_missing_schedule_binding() {
    let mut counters = Default::default();
    let denial = reject_incomplete_chain_authority_bindings(
        &("edge".to_string(), "carrier".to_string()),
        &ChainAuthorityBindings {
            endpoint_boundary_schedule_identity: "endpoint-schedule".to_string(),
            interval_subdivision_schedule_identity: String::new(),
            vertex_schedule_identity: "vertex-schedule".to_string(),
            fragment_schedule_identity: "fragment-schedule".to_string(),
            fragment_coverage_identities: vec!["fragment-coverage".to_string()],
            overlap_coverage_identities: Vec::new(),
        },
        &mut counters,
    )
    .expect_err("ledger chains must reject missing schedule authority");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingScheduleBinding
    );
    assert_eq!(denial.counters().missing_validation_denials(), 1);
}

#[test]
fn split_edge_chain_ledger_rejects_missing_fragment_validation_coverage() {
    let mut counters = Default::default();
    let denial = reject_incomplete_chain_authority_bindings(
        &("edge".to_string(), "carrier".to_string()),
        &ChainAuthorityBindings {
            endpoint_boundary_schedule_identity: "endpoint-schedule".to_string(),
            interval_subdivision_schedule_identity: "interval-schedule".to_string(),
            vertex_schedule_identity: "vertex-schedule".to_string(),
            fragment_schedule_identity: "fragment-schedule".to_string(),
            fragment_coverage_identities: Vec::new(),
            overlap_coverage_identities: Vec::new(),
        },
        &mut counters,
    )
    .expect_err("ledger chains must reject missing validation coverage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingFragmentValidationCoverage
    );
    assert_eq!(denial.counters().missing_validation_denials(), 1);
}

#[test]
fn split_edge_chain_ledger_orders_all_products_canonically_across_replay() {
    let first = prepared_products(ordered_entries());
    let replayed = prepared_products(replayed_entries());

    let first_result = ledger_result_for(&first).expect("first ledger should assemble");
    let replayed_result = ledger_result_for(&replayed).expect("replayed ledger should assemble");

    assert_eq!(
        first_result.ledger().ledger_identity(),
        replayed_result.ledger().ledger_identity()
    );
    assert_eq!(
        first_result.receipt().receipt_identity(),
        replayed_result.receipt().receipt_identity()
    );
    assert_eq!(
        first_result.receipt().chain_identities(),
        replayed_result.receipt().chain_identities()
    );
    let mut sorted_chain_identities = first_result.receipt().chain_identities().to_vec();
    sorted_chain_identities.sort();
    assert_eq!(
        first_result.receipt().chain_identities(),
        sorted_chain_identities
    );
}

#[test]
fn split_edge_chain_ledger_rejects_missing_validation_or_name_propagation_receipts() {
    let products = prepared_products(ordered_entries());
    let missing_validation = validation_without_fragment_coverage(&products.validation);
    let validation_denial = ledger_result_for_with_validation(&products, &missing_validation)
        .expect_err("ledger must reject a validation receipt that proves no fragment coverage");

    assert_eq!(
        validation_denial.kind(),
        PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingFragmentValidationCoverage
    );
    assert_eq!(validation_denial.counters().missing_validation_denials(), 1);

    let missing_names = products.naming.with_rows_for_tests(
        Vec::new(),
        PlanarBooleanSplitPersistentNamingCounters::default(),
    );
    let naming_denial = ledger_result_for_with_naming(&products, &missing_names)
        .expect_err("ledger must reject a naming receipt that propagates no names");

    assert_eq!(
        naming_denial.kind(),
        PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingPersistentNameBinding
    );
    assert_eq!(
        naming_denial.counters().missing_persistent_name_denials(),
        1
    );
}

struct PreparedLedgerProducts {
    request: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest,
    endpoint_boundary: PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision: PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    vertices: PlanarBooleanSplitVertexIdentitySet,
    fragments: PlanarBooleanSplitEdgeFragmentSet,
    chains: PlanarBooleanOverlapEdgeChainSet,
    validation: PlanarBooleanSplitChainValidationReceipt,
    naming: PlanarBooleanSplitPersistentNamingReceipt,
    decision_log: PlanarBooleanSplitDecisionLogQueryResult,
}

fn prepared_products(
    entries: Vec<
        crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleEntry,
    >,
) -> PreparedLedgerProducts {
    let request_subject = subject_with_carriers(source_carriers());
    let endpoint_boundary = raw_set_from_schedules(vec![raw_schedule(
        "raw schedule",
        "source edge",
        "carrier",
        entries,
    )])
    .canonicalize_split_schedule_order()
    .expect("raw schedule should order")
    .collapse_duplicate_split_points()
    .expect("duplicates should normalize")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint boundary should normalize");
    let interval_subdivision = endpoint_boundary
        .normalize_overlap_interval_subdivisions(
            PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance,
        )
        .expect("interval subdivisions should normalize");
    let vertices = interval_subdivision
        .mint_split_vertex_identities()
        .expect("split vertices should mint");
    let fragments = interval_subdivision
        .build_split_edge_fragments(&vertices)
        .expect("split fragments should build");
    let chains = interval_subdivision
        .build_overlap_edge_chains(&fragments)
        .expect("overlap chains should build");
    let validation = fragments
        .validate_split_edge_chains(&chains)
        .expect("split chains should validate");
    let naming = PlanarBooleanSplitPersistentNamingReceipt::admit(
        PlanarBooleanSplitPersistentNamingInput::new(
            &validation,
            &fragments,
            &vertices,
            &chains,
            PlanarBooleanSplitPersistentNamingQueryBasis::from_query_runtime(
                "worth.topology/current_head_authoritative",
                "persistent-name-live-view",
                "naming-attachment-report",
            ),
        ),
    )
    .expect("persistent naming should admit");
    let decision_log = decision_log_for(
        &request_subject.request,
        &endpoint_boundary,
        &interval_subdivision,
        &vertices,
        &fragments,
        &validation,
        &naming,
    );
    PreparedLedgerProducts {
        request: request_subject.request,
        endpoint_boundary,
        interval_subdivision,
        vertices,
        fragments,
        chains,
        validation,
        naming,
        decision_log,
    }
}

fn ordered_entries() -> Vec<
    crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleEntry,
>{
    vec![
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.25),
        raw_interval_entry("interval", "source edge", "carrier", "event:interval", 0.5),
        raw_point_entry("point b", "source edge", "carrier", "event:b", 0.75),
    ]
}

fn replayed_entries() -> Vec<
    crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleEntry,
>{
    vec![
        raw_point_entry("point b", "source edge", "carrier", "event:b", 0.75),
        raw_interval_entry("interval", "source edge", "carrier", "event:interval", 0.5),
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.25),
    ]
}

fn decision_log_for(
    request: &crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest,
    endpoint_boundary: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    vertices: &PlanarBooleanSplitVertexIdentitySet,
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    validation: &PlanarBooleanSplitChainValidationReceipt,
    naming: &PlanarBooleanSplitPersistentNamingReceipt,
) -> PlanarBooleanSplitDecisionLogQueryResult {
    PlanarBooleanSplitDecisionLogQueryDomain::declare(PlanarBooleanSplitDecisionLogQueryInput::new(
        request,
        endpoint_boundary,
        interval_subdivision,
        vertices,
        fragments,
        validation,
        naming,
    ))
    .expect("decision log declaration should lower")
    .execute()
    .expect("decision log should execute")
}

fn ledger_result_for(
    products: &PreparedLedgerProducts,
) -> Result<
    PlanarBooleanSplitEdgeChainLedgerQueryResult,
    super::denial::PlanarBooleanSplitEdgeChainLedgerDenial,
> {
    ledger_result_for_with_validation(products, &products.validation)
}

fn ledger_result_for_with_validation(
    products: &PreparedLedgerProducts,
    validation: &PlanarBooleanSplitChainValidationReceipt,
) -> Result<
    PlanarBooleanSplitEdgeChainLedgerQueryResult,
    super::denial::PlanarBooleanSplitEdgeChainLedgerDenial,
> {
    ledger_result_for_with_validation_and_naming(products, validation, &products.naming)
}

fn ledger_result_for_with_naming(
    products: &PreparedLedgerProducts,
    naming: &PlanarBooleanSplitPersistentNamingReceipt,
) -> Result<
    PlanarBooleanSplitEdgeChainLedgerQueryResult,
    super::denial::PlanarBooleanSplitEdgeChainLedgerDenial,
> {
    ledger_result_for_with_validation_and_naming(products, &products.validation, naming)
}

fn ledger_result_for_with_validation_and_naming(
    products: &PreparedLedgerProducts,
    validation: &PlanarBooleanSplitChainValidationReceipt,
    naming: &PlanarBooleanSplitPersistentNamingReceipt,
) -> Result<
    PlanarBooleanSplitEdgeChainLedgerQueryResult,
    super::denial::PlanarBooleanSplitEdgeChainLedgerDenial,
> {
    PlanarBooleanSplitEdgeChainLedgerQueryDomain::declare(
        PlanarBooleanSplitEdgeChainLedgerQueryInput::new(
            &products.request,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.chains,
            validation,
            naming,
            &products.decision_log,
        ),
    )?
    .execute()
}

fn validation_without_fragment_coverage(
    validation: &PlanarBooleanSplitChainValidationReceipt,
) -> PlanarBooleanSplitChainValidationReceipt {
    PlanarBooleanSplitChainValidationReceipt::new(
        validation.receipt_identity().to_string(),
        validation.split_edge_fragment_set_identity().to_string(),
        validation.overlap_edge_chain_set_identity().to_string(),
        validation
            .interval_subdivision_schedule_set_identity()
            .to_string(),
        Vec::new(),
        validation.overlap_coverage_rows().to_vec(),
        PlanarBooleanSplitChainValidationCounters::new(1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
    )
}
