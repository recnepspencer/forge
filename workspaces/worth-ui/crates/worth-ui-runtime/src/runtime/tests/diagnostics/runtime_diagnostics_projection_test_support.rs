use crate::facade::WorthUi;
use crate::runtime::WorthUiRuntimeLaunch;
use crate::source::{
    WorthUiArtifact, WorthUiArtifactHandle, WorthUiArtifactIdentitySeed,
    WorthUiArtifactImportHandle, WorthUiArtifactImportNode, WorthUiArtifactModule,
    WorthUiArtifactNode, WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
};
use std::collections::BTreeMap;
use std::path::Path;
use worth_foundational::{
    performance, performance_api, profiles, CertificationPostureProfile, DiagnosticRichnessProfile,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceAttachmentTargetKind,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceReportRequest,
    FoundationalPerformanceSupportingEvidenceCode, FoundationalPerformanceSupportingEvidenceRow,
    FoundationalPerformanceWorkClass, RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_ui_dsl::WorthUiSourceModuleId;

pub(super) fn foundational_frame_report(
    observed_frame_rows: u64,
) -> worth_foundational::FoundationalMaterializedPerformanceReport<impl std::fmt::Debug> {
    let claim = performance()
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
        .expect("claim builds");
    let counter_name = FoundationalPerformanceCounterName::new("worth_ui.frame.projected_rows")
        .expect("valid counter");
    let support_code = FoundationalPerformanceSupportingEvidenceCode::new("worth_ui.frame.receipt")
        .expect("valid support code");
    let bundle = performance_api::lower_lane::basis::performance_bundle(claim)
        .attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            counter_name.clone(),
            FoundationalPerformanceWorkClass::AuthoritativeMutation,
            observed_frame_rows,
        ))
        .attach_supporting_evidence_row(FoundationalPerformanceSupportingEvidenceRow::new(
            support_code,
            FoundationalPerformanceWorkClass::SupportReportAssembly,
        ))
        .finish()
        .expect("bundle builds");
    let receipt = performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle)
        .attach_counter_row(FoundationalPerformanceCounterRow::new(
            counter_name,
            observed_frame_rows,
        ))
        .finish()
        .expect("receipt builds");
    let attached = performance_api::lower_lane::reports::attach_counter_backed_performance_receipt(
        FoundationalPerformanceAttachmentTargetKind::BoundaryReport,
        receipt,
    )
    .expect("attached report source builds");

    performance_api::lower_lane::reports::plan_performance_report(
        FoundationalPerformanceReportRequest {
            source: attached,
            profile: profiles()
                .set()
                .diagnostic_richness(DiagnosticRichnessProfile::Standard)
                .support_posture(SupportPostureProfile::SupportReady)
                .compatibility_posture(worth_foundational::CompatibilityPostureProfile::NativeOnly)
                .admission_readiness(worth_foundational::AdmissionReadinessProfile::Admitted)
                .retention_delivery(RetentionDeliveryProfile::Retained)
                .certification_posture(CertificationPostureProfile::Uncertified)
                .compose()
                .expect("profile composes"),
            include_layout_intent: false,
            include_contract_names: false,
            include_counter_specs: true,
            include_counter_rows: true,
            include_supporting_evidence_rows: true,
            include_budget_decisions: false,
            include_denied_work: false,
            include_widened_work: false,
        },
    )
    .materialize()
}

pub(super) fn runtime_from_import_target(
    target: &str,
) -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    WorthUi::app()
        .bind_certification_host()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed")
        .launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(
            artifact_from_import_target(target),
        ))
        .expect("runtime launches")
}

fn artifact_from_import_target(target: &str) -> WorthUiArtifact {
    let module_id = module_id("app/main.wui");
    let node = WorthUiArtifactNode::Import(WorthUiArtifactImportNode::new(
        WorthUiArtifactHandle::Import(WorthUiArtifactImportHandle::new(module_id.clone(), 0)),
        crate::source::test_compilation::semantic_import(target)
            .target()
            .clone(),
        0,
        WorthUiArtifactIdentitySeed::structural_fallback(format!(
            "module:{}|import:{}",
            module_id.as_str(),
            target
        )),
        WorthUiDurableStateEligibility::Ineligible {
            reason: WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
        },
    ));
    let module = WorthUiArtifactModule::new(module_id.clone(), vec![node]);
    let mut modules = BTreeMap::new();
    modules.insert(module_id.clone(), module);
    WorthUiArtifact::new(modules, vec![module_id])
}

fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(path)).expect("valid module id")
}
