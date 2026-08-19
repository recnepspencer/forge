use worth_foundational::{
    performance_api::{
        common_path,
        lower_lane::{basis, receipts, reports},
        performance_public_surface_inventory,
        stronger_lane::{certified, readiness as stronger_readiness},
        FoundationalPerformancePublicLane,
    },
    profiles, CertificationPostureProfile, DiagnosticRichnessProfile,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceAttachmentTargetKind,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceLayoutIntent,
    FoundationalPerformanceWorkClass, RetentionDeliveryProfile, SupportPostureProfile,
};

fn standard_profile() -> worth_foundational::FoundationalProfileSet {
    profiles()
        .set()
        .diagnostic_richness(DiagnosticRichnessProfile::Standard)
        .support_posture(SupportPostureProfile::SupportReady)
        .compatibility_posture(worth_foundational::CompatibilityPostureProfile::NativeOnly)
        .admission_readiness(worth_foundational::AdmissionReadinessProfile::Admitted)
        .retention_delivery(RetentionDeliveryProfile::Retained)
        .certification_posture(CertificationPostureProfile::Uncertified)
        .execution_objective(worth_foundational::ExecutionObjectiveProfile::Balanced)
        .observation_activation(worth_foundational::ObservationActivationProfile::Continuous)
        .compose()
        .expect("standard profile should compose")
}

#[test]
fn grouped_performance_surface_exposes_common_lower_and_stronger_lanes() {
    let common_front_door = common_path::performance();
    let claim = common_front_door
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
        .expect("common-path claim should build");

    let layout = common_front_door.define_layout_intent(
        FoundationalPerformanceLayoutIntent::AoS,
        FoundationalPerformanceAccessPatternPosture::PointLookup,
        worth_foundational::FoundationalPerformanceAllocationPosture::ActionLocal,
    );

    let bundle = basis::performance_bundle(claim)
        .attach_layout_intent_claim(layout)
        .attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            FoundationalPerformanceCounterName::new("authoritative.rows")
                .expect("valid counter name"),
            FoundationalPerformanceWorkClass::AuthoritativeMutation,
            3,
        ))
        .finish()
        .expect("lower-lane bundle should build");
    let receipt = receipts::counter_backed_performance_receipt(bundle)
        .attach_counter_row(FoundationalPerformanceCounterRow::new(
            FoundationalPerformanceCounterName::new("authoritative.rows")
                .expect("valid counter name"),
            3,
        ))
        .finish()
        .expect("counter-backed receipt should build");
    let attached = reports::attach_counter_backed_performance_receipt(
        FoundationalPerformanceAttachmentTargetKind::BoundaryReport,
        receipt,
    )
    .expect("receipt attachment should build");
    let plan = reports::plan_performance_report(
        worth_foundational::FoundationalPerformanceReportRequest {
            source: attached,
            profile: standard_profile(),
            include_layout_intent: true,
            include_contract_names: false,
            include_counter_specs: true,
            include_counter_rows: true,
            include_supporting_evidence_rows: false,
            include_budget_decisions: false,
            include_denied_work: false,
            include_widened_work: false,
        },
    );
    let report = plan.materialize();
    let readiness = stronger_readiness::foundational_performance_milestone8_readiness_report();
    let readiness_certified =
        stronger_readiness::certify_foundational_performance_milestone8_production_test_readiness();

    assert_eq!(report.counter_rows()[0].observed_count(), 3);
    assert!(report.layout_intent_claim().is_some());
    assert!(readiness.passes_readiness_checklist());
    assert!(std::ptr::eq(
        stronger_readiness::require_foundational_performance_milestone8_production_test_readiness(
            &readiness_certified
        ),
        readiness_certified.payload()
    ));
    let _ = certified::foundational_performance_certified_attachment_authority();
}

#[test]
fn grouped_performance_surface_exposes_certified_and_readiness_stronger_lanes() {
    let public_surface_inventory = performance_public_surface_inventory();
    let stronger_lane_entries: Vec<_> = public_surface_inventory
        .iter()
        .filter(|entry| entry.lane() == FoundationalPerformancePublicLane::StrongerLane)
        .collect();

    assert_eq!(stronger_lane_entries.len(), 3);
    assert!(stronger_lane_entries.iter().any(|entry| {
        entry.path() == "worth_foundational::performance_api::stronger_lane"
            && entry.teaches()
                == "grouped stronger lane for certified performance bundles, trust-boundary readmission, and readiness certification"
            && entry.does_not_hide() == "common-path authoring or lower-lane inspection"
    }));
    assert!(stronger_lane_entries.iter().any(|entry| {
        entry.path() == "worth_foundational::performance_api::stronger_lane::certified"
            && entry.teaches()
                == "proof-bearing certified performance bundles and trust-boundary readmission over current-basis hot-path receipts and support-expansion reports"
            && entry.does_not_hide()
                == "plain lower-lane receipt/report inspection or readiness-only certification"
    }));
    assert!(stronger_lane_entries.iter().any(|entry| {
        entry.path() == "worth_foundational::performance_api::stronger_lane::readiness"
            && entry.teaches()
                == "production-readiness certification and proof-bearing readiness requirement"
            && entry.does_not_hide()
                == "plain readiness report or certified bundle proof progression"
    }));
}
