use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::tests_support::{
    raw_interval_entry, raw_point_entry, raw_schedule, raw_set_from_schedules,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanMicroIntervalPolicy, PlanarBooleanPointSplitPosture,
    PlanarBooleanSplitPersistentNamingInput, PlanarBooleanSplitPersistentNamingQueryBasis,
    PlanarBooleanSplitPersistentNamingReceipt,
};
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::{
    PlanarBooleanRawEdgeSplitScheduleEntry, PlanarBooleanRawEdgeSplitScheduleEntryKind,
    PlanarBooleanRawPointEndpointAuthority,
};

use super::*;

#[test]
fn edge_split_decision_log_covers_every_split_collapse_coalescence_and_denial() {
    let products = prepared_products();
    let mut receipt = decision_log_for(&products)
        .with_phase_stop(typed_stop())
        .record();

    assert!(receipt.certifies_query_native_split_decision_log());
    assert!(receipt.counters().endpoint_decisions_recorded() >= 1);
    assert!(receipt.counters().interval_subdivision_decisions_recorded() >= 1);
    assert!(receipt.counters().coalescence_decisions_recorded() >= 1);
    assert!(receipt.counters().fragment_decisions_recorded() >= 1);
    assert!(receipt.counters().coverage_decisions_recorded() >= 1);
    assert!(receipt.counters().persistent_name_decisions_recorded() >= 1);
    assert_eq!(receipt.counters().phase_stop_decisions_recorded(), 1);
    assert_eq!(
        receipt.counters().lookup_index_entries(),
        receipt.decision_rows().len()
    );
    assert!(receipt.counters().affected_artifact_index_entries() >= 1);
    let stop_row = receipt
        .decision_rows()
        .iter()
        .find(|row| row.kind() == PlanarBooleanSplitDecisionKind::SplitPhaseDenied)
        .expect("typed phase stop should become a decision row");
    let stop_identity = stop_row.decision_identity().to_string();
    let stop_artifact_identity = stop_row.affected_artifact_identity().to_string();
    assert!(receipt.decision_by_identity(&stop_identity).is_some());
    assert_eq!(receipt.counters().lookup_hits(), 1);
    let artifact_rows = receipt.decisions_for_artifact(&stop_artifact_identity);
    assert_eq!(artifact_rows.len(), 1);
    assert_eq!(
        artifact_rows.collect::<Vec<_>>()[0].decision_identity(),
        stop_identity
    );
    assert_eq!(receipt.counters().lookup_hits(), 2);
}

#[test]
fn edge_split_failure_localization_identifies_phase_source_edge_and_event() {
    let products = prepared_products();
    let mut receipt = decision_log_for(&products)
        .with_phase_stop(typed_stop())
        .record();
    let denial_identity = receipt
        .decision_rows()
        .iter()
        .find(|row| row.kind() == PlanarBooleanSplitDecisionKind::SplitPhaseDenied)
        .expect("denial row")
        .decision_identity()
        .to_string();

    let localization = receipt
        .localize_failure(&denial_identity)
        .expect("indexed decision should localize");
    assert_eq!(
        localization.phase(),
        PlanarBooleanSplitDecisionPhase::SplitVertexIdentity
    );
    assert_eq!(localization.source_edge_identity(), "source edge");
    assert_eq!(localization.carrier_identity(), "carrier");
    assert_eq!(
        localization.event_identities(),
        &["event:denied".to_string()]
    );
    assert_eq!(
        localization.policy_or_denial_kind(),
        Some("CoordinateOnlySplitVertexIdentity")
    );
    let report = receipt.structured_failure_report(&localization);
    assert_eq!(report.decision_identity(), denial_identity);
    assert_eq!(report.phase_name(), "split_vertex_identity");
    assert_eq!(report.decision_kind_name(), "split_phase_denied");
    assert_eq!(report.source_edge_identity(), "source edge");
    assert_eq!(report.carrier_identity(), "carrier");
    assert_eq!(report.event_identities(), &["event:denied".to_string()]);
    assert_eq!(
        report.event_group_identities(),
        &["event-group:event:denied".to_string()]
    );
    assert_eq!(report.machine_reason(), "CoordinateOnlySplitVertexIdentity");
    assert_eq!(receipt.counters().diagnostic_reports_emitted(), 1);
}

#[test]
fn edge_split_diagnostics_do_not_change_operational_split_digest() {
    let products = prepared_products();
    let mut receipt = decision_log_for(&products)
        .with_phase_stop(typed_stop())
        .record();
    let truth_before = PlanarBooleanSplitOperationalTruthDigest::from_split_products(
        &products.fragments,
        &products.validation,
        &products.naming,
    );
    let decision_identity = receipt
        .decision_rows()
        .iter()
        .find(|row| row.kind() == PlanarBooleanSplitDecisionKind::SplitPhaseDenied)
        .expect("denial row")
        .decision_identity()
        .to_string();
    let localization = receipt
        .localize_failure(&decision_identity)
        .expect("decision should localize");
    let _report = receipt.structured_failure_report(&localization);
    let truth_after = PlanarBooleanSplitOperationalTruthDigest::from_split_products(
        &products.fragments,
        &products.validation,
        &products.naming,
    );

    assert_eq!(
        truth_after.digest_identity(),
        truth_before.digest_identity()
    );
    assert_eq!(
        truth_after.split_edge_fragment_set_identity(),
        products.fragments.fragment_set_identity()
    );
    assert_eq!(
        truth_after.split_chain_validation_receipt_identity(),
        receipt.split_chain_validation_receipt_identity()
    );
}

#[test]
fn edge_split_failure_localization_rejects_non_failure_decision_rows() {
    let products = prepared_products();
    let mut receipt = decision_log_for(&products).record();
    let declaration_identity = receipt
        .decision_rows()
        .iter()
        .find(|row| row.kind() == PlanarBooleanSplitDecisionKind::QueryDecisionLogDeclared)
        .expect("query declaration row")
        .decision_identity()
        .to_string();

    assert!(receipt.localize_failure(&declaration_identity).is_none());
    assert_eq!(receipt.counters().lookup_hits(), 1);
    assert_eq!(receipt.counters().non_failure_localizations_rejected(), 1);
}

#[test]
fn split_decision_log_rejects_foreign_persistent_naming_receipt() {
    let products = prepared_products();
    let foreign = prepared_products_with_label("foreign");
    let declaration = PlanarBooleanSplitDecisionLogDeclaration::from_product_identities(
        "split request",
        products.validation.receipt_identity(),
        foreign.naming.receipt_identity(),
    )
    .expect("declaration shape should build");

    let denial = PlanarBooleanSplitDecisionLogReceipt::record_decisions(
        PlanarBooleanSplitDecisionLogInput::from_certified_product_identities_for_tests(
            declaration,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.validation,
            &products.naming,
        ),
    )
    .expect_err("foreign persistent naming declaration must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitDecisionLogDenialKind::ForeignPersistentNamingProduct
    );
    assert_eq!(denial.counters().foreign_product_denials(), 1);
}

#[test]
fn query_owned_split_decision_log_certifies_coverage_with_duplicate_artifact_rows() {
    let products = prepared_products();
    let result = PlanarBooleanSplitDecisionLogQueryDomain::declare(
        PlanarBooleanSplitDecisionLogQueryInput::new(
            &products.request,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.validation,
            &products.naming,
        ),
    )
    .expect("query declaration should lower")
    .execute()
    .expect("query-owned decision log should execute");

    assert!(result.certifies_query_owned_decision_log());
    assert_eq!(
        result.coverage().observed_rows(),
        result.receipt().decision_rows().len()
    );
}

struct DecisionLogInputBuilder<'a> {
    input: PlanarBooleanSplitDecisionLogInput<'a>,
}

impl DecisionLogInputBuilder<'_> {
    fn with_phase_stop(mut self, stop: PlanarBooleanEdgeSplitPhaseStop) -> Self {
        self.input = self.input.with_phase_stop(stop);
        self
    }
    fn record(self) -> PlanarBooleanSplitDecisionLogReceipt {
        PlanarBooleanSplitDecisionLogReceipt::record_decisions(self.input)
            .expect("prepared products should record a split decision log")
    }
}

fn decision_log_for(products: &PreparedProducts) -> DecisionLogInputBuilder<'_> {
    let declaration = PlanarBooleanSplitDecisionLogDeclaration::from_product_identities(
        "split request",
        products.validation.receipt_identity(),
        products.naming.receipt_identity(),
    )
    .expect("query declaration should bind split products");
    DecisionLogInputBuilder {
        input: PlanarBooleanSplitDecisionLogInput::from_certified_product_identities_for_tests(
            declaration,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.validation,
            &products.naming,
        ),
    }
}

fn typed_stop() -> PlanarBooleanEdgeSplitPhaseStop {
    PlanarBooleanEdgeSplitPhaseStop::typed_denial(
        PlanarBooleanSplitDecisionPhase::SplitVertexIdentity,
        "source edge",
        "carrier",
        "coordinate-only-vertex",
        vec!["event:denied".to_string()],
        vec!["event-group:event:denied".to_string()],
        "CoordinateOnlySplitVertexIdentity",
        "coordinate-only split vertex identity was rejected",
    )
}

struct PreparedProducts {
    request: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest,
    endpoint_boundary: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    vertices: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentitySet,
    fragments: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet,
    validation: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitChainValidationReceipt,
    naming: PlanarBooleanSplitPersistentNamingReceipt,
}

fn prepared_products() -> PreparedProducts {
    prepared_products_with_label("base")
}

fn prepared_products_with_label(label: &str) -> PreparedProducts {
    let endpoint_boundary = raw_set_from_schedules(vec![raw_schedule(
        &format!("raw schedule {label}"),
        "source edge",
        "carrier",
        vec![
            endpoint_noop_entry(label, "start", 0.0),
            raw_point_entry(
                &format!("a {label}"),
                "source edge",
                "carrier",
                &format!("event:a:{label}"),
                0.25,
            ),
            raw_interval_entry(
                &format!("interval {label}"),
                "source edge",
                "carrier",
                &format!("event:i:{label}"),
                0.5,
            ),
            raw_point_entry(
                &format!("interval endpoint point {label}"),
                "source edge",
                "carrier",
                &format!("event:interval-point:{label}"),
                0.5,
            ),
            raw_point_entry(
                &format!("a dup {label}"),
                "source edge",
                "carrier",
                &format!("event:a-dup:{label}"),
                0.25,
            ),
            endpoint_noop_entry(label, "end", 1.0),
        ],
    )])
    .canonicalize_split_schedule_order()
    .expect("raw schedule should order")
    .collapse_duplicate_split_points()
    .expect("duplicate split points should normalize")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint boundary splits should normalize");
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
    let request = crate::workload_platform::planar_boolean_edge_splitting::source_edge_carrier_recovery::test_support::subject_with_carriers(
        crate::workload_platform::planar_boolean_edge_splitting::source_edge_carrier_recovery::test_support::source_carriers(),
    )
    .request;
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
    PreparedProducts {
        request,
        endpoint_boundary,
        interval_subdivision,
        vertices,
        fragments,
        validation,
        naming,
    }
}

fn endpoint_noop_entry(
    label: &str,
    endpoint: &str,
    parameter: f64,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    let entry_identity = format!("{endpoint} noop {label}");
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        entry_identity.clone(),
        "source edge".to_string(),
        "carrier".to_string(),
        format!("candidate:{entry_identity}"),
        format!("event:{endpoint}:{label}"),
        Some(format!("parameter-fact:{entry_identity}")),
        parameter,
        None,
        "local frame".to_string(),
        "precision basis".to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(
            PlanarBooleanPointSplitPosture::EndpointNoOp,
        ),
        vec![format!("segment-pair:event:{endpoint}:{label}")],
        vec![format!("predicate:event:{endpoint}:{label}")],
        vec![format!("event-group:event:{endpoint}:{label}")],
        PlanarBooleanRawPointEndpointAuthority {
            exact_endpoint_source_identity: Some(format!("endpoint:{endpoint}")),
            exact_projected_endpoint_fact_identity: Some(format!("projection:{endpoint}")),
            shared_endpoint_source_identities: Vec::new(),
            shared_endpoint_projection_fact_digests: Vec::new(),
        },
        None,
    )
}
