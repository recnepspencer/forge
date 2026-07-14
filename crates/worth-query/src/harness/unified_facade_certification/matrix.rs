use super::lane::{
    UnifiedFacadeCertificationMatrix, UnifiedFacadeLane, UnifiedFacadePerturbationClass,
    UnifiedFacadeRejection,
};
use super::row_catalog::{
    UnifiedFacadeCanonicalRowSpec, UnifiedFacadeRejectionRowSpec,
    UNIFIED_FACADE_CANONICAL_ROW_SPECS, UNIFIED_FACADE_REJECTION_ROW_SPECS,
};
use crate::basis_lifecycle::basis_lifecycle;
use crate::facade::foundation::{
    IdentityEvolutionComparisonBasisFamily, IdentityEvolutionQueryContext,
    LineageTraversalDescriptor, WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryCapabilityStatus, WorthQueryConfig, WorthQueryConfigSectionFamily,
    WorthQueryQueryConfig, WorthQuerySignalConfig,
};
use crate::facade::policy::{
    PreviewEvaluationClass, PreviewSessionQueryContext, QueryContextBindingSource,
};
use crate::facade::runtime::{
    WorkflowAuthorityTargetFamily, WorkflowBindingSource, WorkflowBudgetClass, WorkflowCostClass,
    WorkflowDeclarationFamily, WorkflowDeclarationRequest, WorkflowFreshnessPolicy,
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
        let identity_evolution = identity_evolution_lane();
        let runtime_live = live_lane();
        let preview = preview_lane();
        let workflow = workflow_lane();
        let historical = historical_lane();
        let config_section = workflow_section_lane();
        let support_sync = support_sync_lane();
        let identity_support_sync = identity_evolution_support_sync_lane();

        UnifiedFacadeCertificationMatrix {
            suite_name: "Unified Facade And Configuration Boundary Test",
            rows: UNIFIED_FACADE_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &runtime_query,
                        &query_context,
                        &identity_evolution,
                        &runtime_live,
                        &preview,
                        &workflow,
                        &historical,
                        &config_section,
                        &support_sync,
                        &identity_support_sync,
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

fn identity_evolution_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let identity = facade
        .identity_evolution_capability()
        .expect("identity evolution should admit");
    let admitted = identity
        .capability()
        .admit_query(IdentityEvolutionQueryContext::lineage_traversal_for_test(
            crate::identity::CanonicalQueryDigest::from_parts(&[format!(
                "unified-facade-identity-evolution:{}",
                preflight.plan().query().validated_query_digest().as_str()
            )]),
            crate::identity::BasisDigest::from_parts(&[
                "unified-facade-identity-evolution".to_string()
            ]),
            LineageTraversalDescriptor::direct_replacement("entity:replacement"),
        ))
        .expect("identity evolution should admit a direct replacement query");
    let execution = identity
        .capability()
        .execute_query(&admitted)
        .expect("identity evolution should execute");
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
        identity.counters(),
        WorthQueryCapabilityFamily::IdentityEvolution,
        identity.descriptor().status(),
        identity.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
    .with_identity_evolution_result_digests(
        execution.result_digest().to_string(),
        execution
            .result_bundle()
            .metadata()
            .branch_locality_digest()
            .as_str()
            .to_string(),
    )
}

fn query_read_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
        WorthQueryCapabilityFamily::QueryRead,
        query_reads.descriptor().status(),
        query_reads.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn live_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
        WorthQueryCapabilityFamily::LiveQuery,
        live.descriptor().status(),
        live.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn query_context_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
            basis_lifecycle().current_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("current context should admit");
    let right = contexts
        .capability()
        .admit_basis_context(
            basis_lifecycle().branch_head("branch:snapshot-2", true),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("branch context should admit");
    let _diff = contexts
        .capability()
        .bind_diff_context(&left, &right)
        .expect("query context capability should bind diff context");
    let basis_bundle = contexts
        .capability()
        .execute_basis_result_bundle(&left)
        .expect("query context basis bundle should shape");
    let left_execution = basis_bundle.execution().clone();
    let right_execution = contexts
        .capability()
        .execute_basis_context(&right)
        .expect("right branch context should execute");
    let diff = contexts
        .capability()
        .bind_diff_context(&left, &right)
        .expect("query context capability should bind diff context");
    let diff_bundle = contexts
        .capability()
        .shape_diff_result_bundle(&diff, &left_execution, &right_execution)
        .expect("query context diff bundle should shape");
    let report_profile = report
        .query_context_support_profile()
        .expect("query context support profile should be present");

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
        WorthQueryCapabilityFamily::QueryContext,
        contexts.descriptor().status(),
        contexts.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
    .with_query_context_support_profile(
        report_profile.profile_digest().to_string(),
        report_profile
            .admitted_basis_families()
            .iter()
            .map(|family| family.as_str().to_string())
            .collect(),
        report_profile
            .admitted_comparison_families()
            .iter()
            .map(|family| family.as_str().to_string())
            .collect(),
        report_profile
            .deferred_scope_markers()
            .iter()
            .map(|marker| marker.as_str().to_string())
            .collect(),
    )
    .with_query_context_result_digests(
        basis_bundle.metadata().result_digest().to_string(),
        diff_bundle
            .metadata()
            .comparison_result_digest()
            .to_string(),
        diff_bundle.replay_digest().to_string(),
    )
}

fn preview_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
        WorthQueryCapabilityFamily::PreviewSession,
        preview.descriptor().status(),
        preview.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn workflow_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
        WorthQueryCapabilityFamily::WorkflowOrchestration,
        workflow.descriptor().status(),
        workflow.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn historical_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
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
        WorthQueryCapabilityFamily::HistoricalEvaluation,
        historical.descriptor().status(),
        historical.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn workflow_section_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let (section, counters) =
        facade.resolve_config_section(WorthQueryConfigSectionFamily::Relational);
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
        WorthQueryCapabilityFamily::WorkflowOrchestration,
        if section.enabled() {
            WorthQueryCapabilityStatus::Admitted
        } else {
            WorthQueryCapabilityStatus::Unsupported
        },
        section.section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
}

fn support_sync_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let query_context = facade
        .query_context_capability()
        .expect("query context capability should be admitted in the unified facade");
    let profile = report
        .query_context_support_profile()
        .expect("query context support profile should be present");
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
        query_context.counters(),
        WorthQueryCapabilityFamily::QueryContext,
        query_context.descriptor().status(),
        query_context.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
    .with_query_context_support_profile(
        profile.profile_digest().to_string(),
        profile
            .admitted_basis_families()
            .iter()
            .map(|family| family.as_str().to_string())
            .collect(),
        profile
            .admitted_comparison_families()
            .iter()
            .map(|family| family.as_str().to_string())
            .collect(),
        profile
            .deferred_scope_markers()
            .iter()
            .map(|marker| marker.as_str().to_string())
            .collect(),
    )
}

fn identity_evolution_support_sync_lane() -> UnifiedFacadeLane {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let preflight = execution_preflights::direct_runtime_preflight();
    let support = facade.support_matrix();
    let report = facade.support_report();
    let identity = facade
        .identity_evolution_capability()
        .expect("identity evolution capability should be admitted in the unified facade");
    let profile = report
        .identity_evolution_support_profile()
        .expect("identity evolution support profile should be present");
    let admitted = identity
        .capability()
        .admit_query(
            IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
                crate::identity::CanonicalQueryDigest::from_parts(&[format!(
                    "identity-evolution-support-sync:{}",
                    preflight.plan().query().validated_query_digest().as_str()
                )]),
                IdentityEvolutionComparisonBasisFamily::BranchToBranch,
                crate::identity::BasisDigest::from_parts(&["identity-left".to_string()]),
                crate::identity::BasisDigest::from_parts(&["identity-right".to_string()]),
                crate::facade::foundation::CorrespondenceIdentityComparison::advisory_between(
                    "entity:left",
                    "entity:right",
                ),
            ),
        )
        .expect("identity evolution comparison should admit");
    let execution = identity
        .capability()
        .execute_query(&admitted)
        .expect("identity evolution comparison should execute");
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
        identity.counters(),
        WorthQueryCapabilityFamily::IdentityEvolution,
        identity.descriptor().status(),
        identity.descriptor().config_section(),
    )
    .with_report_digest(
        report.report_digest().to_string(),
        report.counters().support_report_generation_count(),
    )
    .with_identity_evolution_support_profile(
        profile.profile_digest().to_string(),
        profile
            .admitted_traversal_families()
            .iter()
            .map(|family| family.as_str().to_string())
            .collect(),
        profile
            .admitted_comparison_basis_families()
            .iter()
            .map(|family| family.as_str().to_string())
            .collect(),
        profile
            .admitted_inspector_consumable_identity_classifications()
            .iter()
            .map(|classification| classification.as_str().to_string())
            .collect(),
        profile
            .deferred_scope_markers()
            .iter()
            .map(|marker| marker.as_str().to_string())
            .collect(),
    )
    .with_identity_evolution_result_digests(
        execution.result_digest().to_string(),
        execution
            .result_bundle()
            .metadata()
            .branch_locality_digest()
            .as_str()
            .to_string(),
    )
}

fn canonical_row(
    spec: &UnifiedFacadeCanonicalRowSpec,
    runtime_query: &UnifiedFacadeLane,
    query_context: &UnifiedFacadeLane,
    identity_evolution: &UnifiedFacadeLane,
    runtime_live: &UnifiedFacadeLane,
    preview: &UnifiedFacadeLane,
    workflow: &UnifiedFacadeLane,
    historical: &UnifiedFacadeLane,
    config_section: &UnifiedFacadeLane,
    support_sync: &UnifiedFacadeLane,
    identity_support_sync: &UnifiedFacadeLane,
) -> CanonicalCertificationRow<UnifiedFacadePerturbationClass, UnifiedFacadeLane> {
    let (control_lane, hostile_lane) = match spec.row_name {
        "unified-query-read-capability" => (runtime_query.clone(), runtime_query.clone()),
        "unified-query-context-capability" => (query_context.clone(), query_context.clone()),
        "unified-identity-evolution-capability" => {
            (identity_evolution.clone(), identity_evolution.clone())
        }
        "unified-query-context-basis-result-bundle" => {
            (query_context.clone(), query_context.clone())
        }
        "unified-query-context-diff-result-bundle" => {
            (query_context.clone(), query_context.clone())
        }
        "unified-live-capability" => (runtime_live.clone(), runtime_live.clone()),
        "unified-preview-capability" => (preview.clone(), preview.clone()),
        "unified-workflow-capability" => (workflow.clone(), workflow.clone()),
        "unified-historical-capability" => (historical.clone(), historical.clone()),
        "unified-config-section-explicitness" => (runtime_query.clone(), config_section.clone()),
        "capability-support-metadata-sync" => (support_sync.clone(), support_sync.clone()),
        "query-context-support-profile-sync" => (support_sync.clone(), support_sync.clone()),
        "identity-evolution-support-profile-sync" => {
            (identity_support_sync.clone(), identity_support_sync.clone())
        }
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
            let facade = WorthQueryApplicationFacade::new(
                WorthQueryConfig::runtime_backed_default()
                    .with_signal(WorthQuerySignalConfig::disabled()),
            )
            .expect("disabling live should retain a valid facade config");
            let error = facade
                .live_query_capability()
                .expect_err("disabled live capability should deny");
            UnifiedFacadeRejection::from_error(&error)
        }
        "invalid-workflow-support-posture" => {
            let facade = WorthQueryApplicationFacade::new(
                WorthQueryConfig::runtime_backed_default().with_relational(
                    crate::facade::foundation::WorthQueryRelationalConfig::enabled()
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
            let facade = WorthQueryApplicationFacade::runtime_backed_default();
            let error = facade
                .durable_artifact_capability()
                .expect_err("durable artifacts should remain deferred debt");
            UnifiedFacadeRejection::from_error(&error)
        }
        "invalid-unified-configuration" => {
            let error = WorthQueryApplicationFacade::new(
                WorthQueryConfig::runtime_backed_default()
                    .with_query(WorthQueryQueryConfig::disabled())
                    .with_signal(WorthQuerySignalConfig::enabled()),
            )
            .expect_err("invalid unified config should deny before facade construction");
            UnifiedFacadeRejection::from_config_error(&error)
        }
        "broad-collection-diff-denied" => {
            let facade = WorthQueryApplicationFacade::runtime_backed_default();
            let contexts = facade
                .query_context_capability()
                .expect("query context capability should admit");
            let left_preflight =
                execution_preflights::ordered_collection_without_traversal_preflight();
            let right_preflight =
                execution_preflights::alternate_basis_ordered_collection_preflight();
            let left = contexts
                .capability()
                .admit_basis_context(
                    basis_lifecycle().current_head(),
                    QueryContextBindingSource::RuntimeCurrent(&left_preflight),
                )
                .expect("left context should admit");
            let right = contexts
                .capability()
                .admit_basis_context(
                    basis_lifecycle().branch_head("branch:ordered-collection", true),
                    QueryContextBindingSource::RuntimeBranch(&right_preflight),
                )
                .expect("right context should admit");
            let diff = contexts
                .capability()
                .bind_diff_context(&left, &right)
                .expect("diff context should bind");
            let left_execution = contexts
                .capability()
                .execute_basis_context(&left)
                .expect("left context should execute");
            let right_execution = contexts
                .capability()
                .execute_basis_context(&right)
                .expect("right context should execute");
            let error = contexts
                .capability()
                .shape_diff_result_bundle(&diff, &left_execution, &right_execution)
                .expect_err("broad collection diff should deny through the unified facade");
            UnifiedFacadeRejection::from_query_context_error(contexts.counters(), &error)
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
