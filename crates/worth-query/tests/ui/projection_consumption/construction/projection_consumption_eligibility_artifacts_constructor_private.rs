use worth_query::facade::foundation::{AdmittedProjectionConsumption, DeferredProjectionConsumption, DeniedProjectionConsumption, ProjectionConsumptionEligibilityCounters, ProjectionConsumptionEligibilityTrace, ProjectionConsumptionWarningKind, ProjectionConsumptionWarnings, SourceMismatchedProjectionConsumption};

fn impossible<T>() -> T {
    panic!("fixture should fail before construction")
}

fn main() {
    let counters = ProjectionConsumptionEligibilityCounters::default();
    let trace = ProjectionConsumptionEligibilityTrace {
        rule_label: "illegal",
        explanation: "illegal",
    };

    let _ = AdmittedProjectionConsumption {
        declaration_digest: String::new(),
        query_digest: String::new(),
        basis_digest: String::new(),
        result_shape_digest: String::new(),
        authorized_projection_identity: String::new(),
        counters: counters.clone(),
        trace: trace.clone(),
        eligibility_digest: String::new(),
    };
    let _ = DeniedProjectionConsumption {
        declaration_digest: String::new(),
        reason: impossible(),
        counters: counters.clone(),
        trace: trace.clone(),
        failure_digest: String::new(),
    };
    let _ = DeferredProjectionConsumption {
        declaration_digest: String::new(),
        reason: impossible(),
        counters: counters.clone(),
        trace: trace.clone(),
        failure_digest: String::new(),
    };
    let _ = SourceMismatchedProjectionConsumption {
        declaration_digest: String::new(),
        source_family: impossible(),
        requested_fact_kind: impossible(),
        counters,
        trace,
        failure_digest: String::new(),
    };
    let _ = ProjectionConsumptionWarnings {
        warning_kinds: vec![ProjectionConsumptionWarningKind::QueryContextRowBound],
        warning_digest: String::new(),
    };
}
