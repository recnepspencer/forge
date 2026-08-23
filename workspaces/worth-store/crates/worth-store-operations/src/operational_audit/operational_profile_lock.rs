#[test]
fn s6_operational_evidence_source_names_throughput_on_demand_and_retained() {
    let s6_source =
        include_str!("../../../worth-store-certification/src/evidence/scheduling/profiles.rs");
    assert!(
        s6_source.contains("diagnostic_richness: DiagnosticRichnessProfile::OperationalMinimal"),
        "S6 operational evidence must keep OperationalMinimal richness"
    );
    assert!(
        s6_source.contains("retention_delivery: RetentionDeliveryProfile::Retained"),
        "S6 operational evidence must keep Retained delivery"
    );
    assert!(
        s6_source.contains("execution_objective: ExecutionObjectiveProfile::Throughput"),
        "S6 operational evidence must name Throughput"
    );
    assert!(
        s6_source.contains("observation_activation: ObservationActivationProfile::OnDemand"),
        "S6 operational evidence must name OnDemand"
    );
}
