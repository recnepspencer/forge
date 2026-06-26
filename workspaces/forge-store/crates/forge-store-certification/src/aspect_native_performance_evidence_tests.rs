use forge_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceContractName, FoundationalPerformanceCounterName,
    FoundationalPerformanceCounterRow, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use forge_store_aspect_native::{StorePerformanceReceiptEvidence, StorePhysicalBoundaryWitness};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};

#[test]
fn store_performance_evidence_wraps_foundational_counter_backed_receipts() {
    let receipt = foundational_counter_backed_receipt();
    let evidence = StorePerformanceReceiptEvidence::new(receipt, physical_witness());

    assert_eq!(evidence.receipt().counter_rows().len(), 1);
    assert_eq!(
        evidence.receipt().bundle().contract_names()[0].as_str(),
        "store.aspect_native.boundary_authority"
    );
    assert_eq!(
        evidence.receipt().counter_rows()[0].name().as_str(),
        "store.aspect_native.boundary_fact.admit"
    );
}

fn foundational_counter_backed_receipt(
) -> FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim> {
    let counter_name = performance_counter_name("store.aspect_native.boundary_fact.admit");
    let bundle = performance_api::lower_lane::basis::performance_bundle(authoritative_claim())
        .attach_contract_name(performance_contract_name(
            "store.aspect_native.boundary_authority",
        ))
        .attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            counter_name.clone(),
            FoundationalPerformanceWorkClass::AuthoritativeMutation,
            1,
        ))
        .finish()
        .unwrap();

    performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle)
        .attach_counter_row(FoundationalPerformanceCounterRow::new(counter_name, 1))
        .finish()
        .unwrap()
}

fn authoritative_claim() -> FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .unwrap()
}

fn performance_contract_name(name: &'static str) -> FoundationalPerformanceContractName {
    FoundationalPerformanceContractName::new(name).unwrap()
}

fn performance_counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}
