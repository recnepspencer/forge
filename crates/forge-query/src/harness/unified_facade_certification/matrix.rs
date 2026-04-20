use super::lane::{
    UnifiedFacadeCertificationMatrix, UnifiedFacadeLane, UnifiedFacadePerturbationClass,
    UnifiedFacadeRejection,
};
use super::row_catalog::{
    UnifiedFacadeCanonicalRowSpec, UnifiedFacadeRejectionRowSpec,
    UNIFIED_FACADE_CANONICAL_ROW_SPECS, UNIFIED_FACADE_REJECTION_ROW_SPECS,
};
use crate::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus,
    ForgeQueryConfig, ForgeQueryConfigSectionFamily, ForgeQueryQueryConfig, ForgeQuerySignalConfig,
    PreviewEvaluationClass, PreviewSessionQueryContext, QueryBasisContextRequest,
    QueryContextBindingSource, WorkflowAuthorityTargetFamily, WorkflowBindingSource,
    WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
    WorkflowFreshnessPolicy,
};
use crate::harness::certification::{
    CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};

pub struct MilestoneFivePointSixUnifiedFacadeCertificationAdapter;

impl MilestoneFivePointSixUnifiedFacadeCertificationAdapter {
    pub fn unified_facade_and_configuration_boundary_test() -> UnifiedFacadeCertificationMatrix {
        let runtime_query = query_read_lane();
        let query_context = query_context_lane();
        let runtime_live = live_lane();
        let preview = preview_lane();
        let workflow = workflow_lane();
        let historical = historical_lane();
        let config_section = workflow_section_lane();
        let support_sync = support_sync_lane();

        UnifiedFacadeCertificationMatrix {
            suite_name: "Unified Facade And Configuration Boundary Test",
            rows: UNIFIED_FACADE_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &runtime_query,
                        &query_context,
                        &runtime_live,
                        &preview,
                        &workflow,
                        &historical,
                        &config_section,
                        &support_sync,
                    )
                })
                .collect(),
            rejection_rows: UNIFIED_FACADE_REJECTION_ROW_SPECS
                .iter()
                .map(rejection_row)
                .collect(),
        }
    }
}

fn query_read_lane() -> UnifiedFacadeLane {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let query_reads = facade
        .query_read_capability()
        .expect("query reads should admit");
    let _execution = query_reads
        .capability()
        .execute_preflight(&preflight)
        .expect("query reads should execute");
    UnifiedFacadeLane::new(
        preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        preflight.plan().query().plan_digest().as_str().to_string(),
        support.support_matrix_digest().to_string(),
        support.capability_registry().registry_digest().to_string(),
        query_reads.counters(),
        ForgeQueryCapabilityFamily::QueryRead,
        query_reads.descriptor().status(),
        query_reads.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn live_lane() -> UnifiedFacadeLane {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let live = facade
        .live_query_capability()
        .expect("live queries should admit");
    let _plan = live
        .capability()
        .promote_preflight(&preflight)
        .expect("live capability should promote admitted preflight");
    UnifiedFacadeLane::new(
        preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        preflight.plan().query().plan_digest().as_str().to_string(),
        support.support_matrix_digest().to_string(),
        support.capability_registry().registry_digest().to_string(),
        live.counters(),
        ForgeQueryCapabilityFamily::LiveQuery,
        live.descriptor().status(),
        live.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn query_context_lane() -> UnifiedFacadeLane {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::alternate_basis_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let contexts = facade
        .query_context_capability()
        .expect("query context capability should admit");
    let left = contexts
        .capability()
        .admit_basis_context(
            contexts
                .capability()
                .bind_basis_context(
                    QueryBasisContextRequest::current_branch_head(),
                    QueryContextBindingSource::RuntimeCurrent(&left_preflight),
                )
                .expect("current context should bind"),
        )
        .expect("current context should admit");
    let right = contexts
        .capability()
        .admit_basis_context(
            contexts
                .capability()
                .bind_basis_context(
                    QueryBasisContextRequest::branch_head("branch:snapshot-2"),
                    QueryContextBindingSource::RuntimeBranch(&right_preflight),
                )
                .expect("branch context should bind"),
        )
        .expect("branch context should admit");
    let _diff = contexts
        .capability()
        .bind_diff_context(&left, &right)
        .expect("query context capability should bind diff context");
    UnifiedFacadeLane::new(
        left_preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        left_preflight
            .plan()
            .query()
            .plan_digest()
            .as_str()
            .to_string(),
        support.support_matrix_digest().to_string(),
        support.capability_registry().registry_digest().to_string(),
        contexts.counters(),
        ForgeQueryCapabilityFamily::QueryContext,
        contexts.descriptor().status(),
        contexts.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn preview_lane() -> UnifiedFacadeLane {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let preview = facade
        .preview_query_capability()
        .expect("preview sessions should admit");
    let (_runtime, active, execution_record) = active_preview_artifacts("unified-facade");
    let _preview_binding = preview
        .capability()
        .bind_preflight(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("preview capability should bind");
    UnifiedFacadeLane::new(
        preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        preflight.plan().query().plan_digest().as_str().to_string(),
        support.support_matrix_digest().to_string(),
        support.capability_registry().registry_digest().to_string(),
        preview.counters(),
        ForgeQueryCapabilityFamily::PreviewSession,
        preview.descriptor().status(),
        preview.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn workflow_lane() -> UnifiedFacadeLane {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let preview = facade
        .preview_query_capability()
        .expect("preview sessions should admit");
    let workflow = facade
        .workflow_query_capability()
        .expect("workflow orchestration should admit");
    let (_runtime, active, execution_record) = active_preview_artifacts("unified-facade");
    let preview_binding = preview
        .capability()
        .bind_preflight(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("preview capability should bind");
    let foundation = crate::preview::admit_preview_workflow_foundation(&preview_binding)
        .expect("workflow foundation should admit");
    let workflow_binding = workflow
        .capability()
        .bind_context(WorkflowBindingSource::PreviewFoundation(&foundation))
        .expect("workflow binding should admit");
    let _declaration = workflow
        .capability()
        .admit_declaration(
            &workflow_binding,
            WorkflowDeclarationRequest::new(
                WorkflowDeclarationFamily::ConflictInspectionNarrow,
                WorkflowAuthorityTargetFamily::QueryInspection,
                WorkflowCostClass::InspectionNarrow,
                WorkflowBudgetClass::InspectionBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
        )
        .expect("workflow declaration should admit");
    UnifiedFacadeLane::new(
        preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        preflight.plan().query().plan_digest().as_str().to_string(),
        support.support_matrix_digest().to_string(),
        support.capability_registry().registry_digest().to_string(),
        workflow.counters(),
        ForgeQueryCapabilityFamily::WorkflowOrchestration,
        workflow.descriptor().status(),
        workflow.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn historical_lane() -> UnifiedFacadeLane {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let historical = facade
        .historical_query_capability()
        .expect("historical capability should be admitted in the unified facade");
    UnifiedFacadeLane::new(
        preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        preflight.plan().query().plan_digest().as_str().to_string(),
        support.support_matrix_digest().to_string(),
        support.capability_registry().registry_digest().to_string(),
        historical.counters(),
        ForgeQueryCapabilityFamily::HistoricalEvaluation,
        historical.descriptor().status(),
        historical.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn workflow_section_lane() -> UnifiedFacadeLane {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let (section, counters) =
        facade.resolve_config_section(ForgeQueryConfigSectionFamily::Relational);
    UnifiedFacadeLane::new(
        preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        preflight.plan().query().plan_digest().as_str().to_string(),
        support.support_matrix_digest().to_string(),
        support.capability_registry().registry_digest().to_string(),
        &counters,
        ForgeQueryCapabilityFamily::WorkflowOrchestration,
        if section.enabled() {
            ForgeQueryCapabilityStatus::Admitted
        } else {
            ForgeQueryCapabilityStatus::Unsupported
        },
        section.section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn support_sync_lane() -> UnifiedFacadeLane {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let historical = facade
        .historical_query_capability()
        .expect("historical capability should be admitted in the unified facade");
    UnifiedFacadeLane::new(
        preflight
            .plan()
            .query()
            .validated_query_digest()
            .as_str()
            .to_string(),
        preflight.plan().query().plan_digest().as_str().to_string(),
        support.support_matrix_digest().to_string(),
        support.capability_registry().registry_digest().to_string(),
        historical.counters(),
        ForgeQueryCapabilityFamily::HistoricalEvaluation,
        historical.descriptor().status(),
        historical.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn canonical_row(
    spec: &UnifiedFacadeCanonicalRowSpec,
    runtime_query: &UnifiedFacadeLane,
    query_context: &UnifiedFacadeLane,
    runtime_live: &UnifiedFacadeLane,
    preview: &UnifiedFacadeLane,
    workflow: &UnifiedFacadeLane,
    historical: &UnifiedFacadeLane,
    config_section: &UnifiedFacadeLane,
    support_sync: &UnifiedFacadeLane,
) -> CanonicalCertificationRow<UnifiedFacadePerturbationClass, UnifiedFacadeLane> {
    let (control_lane, hostile_lane) = match spec.row_name {
        "unified-query-read-capability" => (runtime_query.clone(), runtime_query.clone()),
        "unified-query-context-capability" => (query_context.clone(), query_context.clone()),
        "unified-live-capability" => (runtime_live.clone(), runtime_live.clone()),
        "unified-preview-capability" => (preview.clone(), preview.clone()),
        "unified-workflow-capability" => (workflow.clone(), workflow.clone()),
        "unified-historical-capability" => (historical.clone(), historical.clone()),
        "unified-config-section-explicitness" => (runtime_query.clone(), config_section.clone()),
        "capability-support-metadata-sync" => (support_sync.clone(), support_sync.clone()),
        other => panic!("unexpected unified facade canonical row {other}"),
    };

    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: control_lane.clone(),
        hostile_lane,
        parity_lane: control_lane,
    }
}

fn rejection_row(
    spec: &UnifiedFacadeRejectionRowSpec,
) -> RejectionCertificationRow<
    UnifiedFacadePerturbationClass,
    UnifiedFacadeLane,
    UnifiedFacadeRejection,
> {
    let control_lane = query_read_lane();
    let parity_lane = live_lane();
    let hostile_lane = match spec.row_name {
        "missing-owning-live-section" => {
            let facade = ForgeQueryApplicationFacade::new(
                ForgeQueryConfig::runtime_backed_default()
                    .with_signal(ForgeQuerySignalConfig::disabled()),
            )
            .expect("disabling live should retain a valid facade config");
            let error = facade
                .live_query_capability()
                .expect_err("disabled live capability should deny");
            UnifiedFacadeRejection::from_error(&error)
        }
        "invalid-workflow-support-posture" => {
            let facade = ForgeQueryApplicationFacade::new(
                ForgeQueryConfig::runtime_backed_default().with_relational(
                    crate::facade::ForgeQueryRelationalConfig::enabled()
                        .with_workflow_orchestration(false)
                        .with_historical_evaluation(true),
                ),
            )
            .expect("disabling workflow inside an enabled relational section should preserve a valid facade config");
            let error = facade
                .workflow_query_capability()
                .expect_err("disabled workflow capability should deny");
            UnifiedFacadeRejection::from_error(&error)
        }
        "deferred-durable-artifacts" => {
            let facade = ForgeQueryApplicationFacade::runtime_backed_default();
            let error = facade
                .durable_artifact_capability()
                .expect_err("durable artifacts should remain deferred debt");
            UnifiedFacadeRejection::from_error(&error)
        }
        "invalid-unified-configuration" => {
            let error = ForgeQueryApplicationFacade::new(
                ForgeQueryConfig::runtime_backed_default()
                    .with_query(ForgeQueryQueryConfig::disabled())
                    .with_signal(ForgeQuerySignalConfig::enabled()),
            )
            .expect_err("invalid unified config should deny before facade construction");
            UnifiedFacadeRejection::from_config_error(&error)
        }
        other => panic!("unexpected unified facade rejection row {other}"),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}
