use super::{
    ConfigurationAdmissionFailureClass, ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily,
    ForgeQueryCapabilityStatus, ForgeQueryConfig, ForgeQueryConfigSectionFamily,
    ForgeQueryFacadeFailureClass, ForgeQueryQueryConfig, ForgeQueryRelationalConfig,
    ForgeQueryRuntimeBridgeConfig, ForgeQuerySignalConfig,
};
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::historical::{
    HistoricalCapabilityDescriptor, HistoricalEvaluationRequest, HistoricalPathReuseDescriptor,
};
use crate::identity_evolution::{
    CorrespondenceIdentityComparison, IdentityEvolutionComparisonBasisFamily,
    IdentityEvolutionQueryContext,
};
use crate::preview::{PreviewEvaluationClass, PreviewSessionQueryContext};
use crate::query_context::{QueryBasisContextRequest, QueryContextBindingSource};
use crate::workflow::{
    WorkflowAuthorityTargetFamily, WorkflowBindingSource, WorkflowBudgetClass, WorkflowCostClass,
    WorkflowDeclarationFamily, WorkflowDeclarationRequest, WorkflowFreshnessPolicy,
};

#[test]
fn facade_support_matrix_stays_in_sync_with_capability_admission() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let support = facade.support_matrix();
    let report = facade.support_report();

    let live_descriptor = support
        .descriptor(ForgeQueryCapabilityFamily::LiveQuery)
        .expect("support matrix should include live query capability");
    assert_eq!(
        live_descriptor.status(),
        ForgeQueryCapabilityStatus::Admitted
    );
    assert!(facade.live_query_capability().is_ok());

    let durability_descriptor = support
        .descriptor(ForgeQueryCapabilityFamily::DurableArtifacts)
        .expect("support matrix should include durability capability");
    assert_eq!(
        durability_descriptor.status(),
        ForgeQueryCapabilityStatus::DeferredDebt
    );
    let deferred = facade
        .durable_artifact_capability()
        .expect_err("durable artifacts should remain deferred debt");
    assert_eq!(
        deferred.failure_class(),
        ForgeQueryFacadeFailureClass::DeferredCapabilityFamily
    );
    assert_eq!(
        deferred.counters().unsupported_composition_denial_count(),
        0
    );
    assert_eq!(deferred.counters().deferred_capability_denial_count(), 1);
    assert_eq!(
        report.support_matrix().support_matrix_digest(),
        support.support_matrix_digest()
    );
    assert_eq!(
        report.validated_config_digest(),
        facade.validated_config().validated_digest()
    );
    assert_eq!(report.counters().support_report_generation_count(), 1);
    assert_eq!(report.deferred_capability_count(), 1);
    assert!(report.admitted_capability_count() >= 1);
    assert!(report
        .admitted_capability_families()
        .contains(&ForgeQueryCapabilityFamily::QueryRead));
    assert_eq!(
        report.deferred_capability_families(),
        &[ForgeQueryCapabilityFamily::DurableArtifacts]
    );
    assert_eq!(
        support.admitted_capability_count(),
        report.admitted_capability_count()
    );
    assert_eq!(
        support.deferred_capability_count(),
        report.deferred_capability_count()
    );
    assert_eq!(
        support.unsupported_capability_count(),
        report.unsupported_capability_count()
    );
    assert_eq!(report.section_postures().len(), 5);
    assert!(report
        .section_postures()
        .iter()
        .any(
            |posture| posture.section() == ForgeQueryConfigSectionFamily::Signal
                && posture.owner() == super::ForgeQuerySubsystemOwner::Signal
                && posture.enabled()
        ));
    assert!(!report.report_digest().is_empty());
}

#[test]
fn query_read_capability_executes_runtime_preflight() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let query_reads = facade
        .query_read_capability()
        .expect("runtime-backed facade should admit query reads");
    let admission = query_reads.admission();
    let preflight = execution_preflights::direct_runtime_preflight();
    let execution = query_reads
        .capability()
        .execute_preflight(&preflight)
        .expect("query read capability should execute admitted preflight");

    assert_eq!(
        execution.report().query_digest().as_str(),
        preflight.plan().query().validated_query_digest().as_str()
    );
    assert_eq!(
        admission.descriptor().family(),
        ForgeQueryCapabilityFamily::QueryRead
    );
    assert!(!admission.decision_digest().is_empty());
    assert_eq!(
        admission.validated_config_digest(),
        facade.validated_config().validated_digest()
    );
    assert_eq!(query_reads.counters().capability_lookup_count(), 1);
    assert_eq!(
        query_reads
            .counters()
            .configuration_section_resolution_count(),
        1
    );
}

#[test]
fn durable_artifact_admission_fails_as_deferred_decision() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let error = facade
        .durable_artifact_capability()
        .expect_err("durable artifact admission should remain deferred");

    assert_eq!(
        error.failure_class(),
        super::CapabilityAdmissionFailureClass::DeferredCapabilityFamily
    );
    assert_eq!(
        error.capability_family(),
        Some(ForgeQueryCapabilityFamily::DurableArtifacts)
    );
    assert_eq!(error.counters().deferred_capability_denial_count(), 1);
    assert_eq!(error.counters().unsupported_composition_denial_count(), 0);
}

#[test]
fn preview_workflow_and_historical_decisions_preserve_family_and_validated_config() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();

    let preview = facade
        .preview_query_capability()
        .expect("preview capability should admit")
        .admission()
        .clone();
    let workflow = facade
        .workflow_query_capability()
        .expect("workflow capability should admit")
        .admission()
        .clone();
    let historical = facade
        .historical_query_capability()
        .expect("historical capability should admit")
        .admission()
        .clone();

    assert_eq!(
        preview.descriptor().family(),
        ForgeQueryCapabilityFamily::PreviewSession
    );
    assert_eq!(
        workflow.descriptor().family(),
        ForgeQueryCapabilityFamily::WorkflowOrchestration
    );
    assert_eq!(
        historical.descriptor().family(),
        ForgeQueryCapabilityFamily::HistoricalEvaluation
    );
    assert_eq!(
        preview.validated_config_digest(),
        facade.validated_config().validated_digest()
    );
    assert_eq!(
        workflow.validated_config_digest(),
        facade.validated_config().validated_digest()
    );
    assert_eq!(
        historical.validated_config_digest(),
        facade.validated_config().validated_digest()
    );
}

#[test]
fn preview_and_workflow_capabilities_compose_without_bypassing_authority() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let preview = facade
        .preview_query_capability()
        .expect("runtime-backed facade should admit preview sessions");
    let workflow = facade
        .workflow_query_capability()
        .expect("runtime-backed facade should admit workflow orchestration");
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("app-facade");
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
        .expect("preview capability should bind admitted preview context");
    let foundation = crate::preview::admit_preview_workflow_foundation(&preview_binding)
        .expect("preview workflow foundation should admit");
    let workflow_binding = workflow
        .capability()
        .bind_context(WorkflowBindingSource::PreviewFoundation(&foundation))
        .expect("workflow capability should bind preview workflow foundation");
    let declaration = workflow
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
        .expect("workflow capability should admit a query inspection declaration");

    assert_eq!(
        declaration.report().declaration_family(),
        &WorkflowDeclarationFamily::ConflictInspectionNarrow
    );
}

#[test]
fn live_capability_disables_typed_and_early_when_config_section_is_off() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_signal(ForgeQuerySignalConfig::disabled()),
    )
    .expect("disabling live alone should preserve a valid facade config");
    let support = facade.support_matrix();
    assert_eq!(
        support
            .descriptor(ForgeQueryCapabilityFamily::LiveQuery)
            .expect("live capability descriptor should exist")
            .status(),
        ForgeQueryCapabilityStatus::Unsupported
    );

    let error = facade
        .live_query_capability()
        .expect_err("disabled live section should deny live capability admission");
    assert_eq!(
        error.failure_class(),
        ForgeQueryFacadeFailureClass::MissingOwningSection
    );
    assert_eq!(
        error.capability_family(),
        Some(ForgeQueryCapabilityFamily::LiveQuery)
    );
    assert_eq!(error.counters().capability_lookup_count(), 1);
    assert_eq!(error.counters().unsupported_composition_denial_count(), 1);
    assert_eq!(error.counters().deferred_capability_denial_count(), 0);
}

#[test]
fn workflow_capability_can_be_disabled_inside_enabled_relational_section() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_relational(
            ForgeQueryRelationalConfig::enabled()
                .with_workflow_orchestration(false)
                .with_historical_evaluation(true),
        ),
    )
    .expect("disabling workflow within relational should preserve a valid facade config");

    let error = facade
        .workflow_query_capability()
        .expect_err("disabled workflow capability should deny workflow admission");
    assert_eq!(
        error.failure_class(),
        ForgeQueryFacadeFailureClass::InvalidComposedSupportPosture
    );
    assert_eq!(
        error.capability_family(),
        Some(ForgeQueryCapabilityFamily::WorkflowOrchestration)
    );
    assert_eq!(error.counters().unsupported_composition_denial_count(), 1);
    assert_eq!(error.counters().deferred_capability_denial_count(), 0);
}

#[test]
fn config_section_resolution_remains_owned_by_subsystem() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let (relational_section, counters) =
        facade.resolve_config_section(ForgeQueryConfigSectionFamily::Relational);

    assert_eq!(
        relational_section.section(),
        ForgeQueryConfigSectionFamily::Relational
    );
    assert_eq!(
        relational_section.owner(),
        super::ForgeQuerySubsystemOwner::Relational
    );
    assert!(relational_section.enabled());
    assert_eq!(counters.configuration_section_resolution_count(), 1);
}

#[test]
fn invalid_config_rejects_composed_capabilities_without_query_execution() {
    let error = ForgeQueryConfig::runtime_backed_default()
        .with_query(ForgeQueryQueryConfig::disabled())
        .with_signal(ForgeQuerySignalConfig::enabled())
        .validate()
        .expect_err("signal without query execution should reject config");

    assert_eq!(
        error.failure_class(),
        ConfigurationAdmissionFailureClass::MissingRequiredSection
    );
    assert_eq!(error.section(), Some(ForgeQueryConfigSectionFamily::Query));
}

#[test]
fn invalid_store_config_rejects_before_facade_construction() {
    let error = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_relational(
                ForgeQueryRelationalConfig::disabled().with_historical_evaluation(false),
            )
            .with_store(super::ForgeQueryStoreConfig::enabled()),
    )
    .expect_err("store-backed config should reject before facade construction");

    assert_eq!(
        error.failure_class(),
        ConfigurationAdmissionFailureClass::ContradictorySectionPosture
    );
}

#[test]
fn historical_capability_admits_runtime_retained_snapshot_request() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let historical = facade
        .historical_query_capability()
        .expect("runtime-backed facade should admit historical evaluation");
    let request = HistoricalEvaluationRequest::retained_snapshot(
        "basis:historical-facade",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot(
        "basis:historical-facade",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = historical
        .capability()
        .admit_path(request, capability)
        .expect("historical capability should admit a retained snapshot request");

    assert_eq!(
        admission.requested_path().basis_identity(),
        "basis:historical-facade"
    );
    assert_eq!(historical.counters().capability_lookup_count(), 1);
}

#[test]
fn historical_capability_can_be_disabled_typed_and_early() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_relational(
            ForgeQueryRelationalConfig::enabled().with_historical_evaluation(false),
        ),
    )
    .expect("disabling historical alone should preserve a valid facade config");

    let error = facade
        .historical_query_capability()
        .expect_err("disabled historical section should deny historical capability admission");
    assert_eq!(
        error.failure_class(),
        ForgeQueryFacadeFailureClass::InvalidComposedSupportPosture
    );
    assert_eq!(
        error.capability_family(),
        Some(ForgeQueryCapabilityFamily::HistoricalEvaluation)
    );
    assert_eq!(error.counters().unsupported_composition_denial_count(), 1);
    assert_eq!(error.counters().deferred_capability_denial_count(), 0);
}

#[test]
fn query_context_capability_binds_branch_and_diff_contexts_without_mode_flags() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let contexts = facade
        .query_context_capability()
        .expect("runtime-backed facade should admit query context capability");
    let left_preflight = execution_preflights::direct_runtime_preflight();
    let right_preflight = execution_preflights::alternate_basis_runtime_preflight();
    let left_binding = contexts
        .capability()
        .bind_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("current branch context should bind");
    let right_binding = contexts
        .capability()
        .bind_basis_context(
            QueryBasisContextRequest::branch_head("branch:snapshot-2"),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("branch context should bind");
    let left = contexts
        .capability()
        .admit_basis_context(left_binding)
        .expect("current branch context should admit");
    let right = contexts
        .capability()
        .admit_basis_context(right_binding)
        .expect("branch context should admit");
    let diff = contexts
        .capability()
        .bind_diff_context(&left, &right)
        .expect("diff context should bind");
    let left_execution = contexts
        .capability()
        .execute_basis_context(&left)
        .expect("current context should execute through query-context capability");
    let right_execution = contexts
        .capability()
        .execute_basis_context(&right)
        .expect("branch context should execute through query-context capability");
    let basis_bundle = contexts
        .capability()
        .execute_basis_result_bundle(&left)
        .expect("basis result bundle should remain query-owned");
    let diff_bundle = contexts
        .capability()
        .shape_diff_result_bundle(&diff, &left_execution, &right_execution)
        .expect("diff result bundle should remain query-owned");
    let change_set = contexts
        .capability()
        .shape_diff_change_set(&diff, &left_execution, &right_execution)
        .expect("diff change-set should remain query-shaped");

    assert_eq!(left.family().as_str(), "current_branch_head");
    assert_eq!(right.family().as_str(), "branch_head");
    assert_eq!(diff.family().as_str(), "branch_to_branch");
    assert_eq!(
        basis_bundle.context().family().as_str(),
        "current_branch_head"
    );
    assert_eq!(
        basis_bundle.metadata().basis_digest(),
        basis_bundle.context().basis_digest()
    );
    assert_eq!(
        basis_bundle.metadata().result_digest(),
        basis_bundle.execution().result_digest()
    );
    assert!(!basis_bundle.replay_digest().is_empty());
    assert_eq!(change_set.comparison_basis_family(), diff.family());
    assert_eq!(
        diff_bundle.metadata().comparison_basis_family(),
        diff.family()
    );
    assert_eq!(
        diff_bundle.metadata().comparison_result_digest(),
        diff_bundle.change_set().result_digest()
    );
    assert_eq!(
        diff_bundle.metadata().prediction_drift_outcome(),
        diff_bundle.change_set().prediction_drift_outcome()
    );
    assert!(!diff_bundle.replay_digest().is_empty());
    assert!(!change_set.rows().is_empty());
    assert_eq!(contexts.counters().capability_lookup_count(), 1);
}

#[test]
fn support_report_includes_query_context_capability() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let support = facade.support_matrix();
    let report = facade.support_report();

    assert_eq!(
        support
            .descriptor(ForgeQueryCapabilityFamily::QueryContext)
            .expect("query context descriptor should exist")
            .status(),
        ForgeQueryCapabilityStatus::Admitted
    );
    assert!(report
        .admitted_capability_families()
        .contains(&ForgeQueryCapabilityFamily::QueryContext));
    let profile = report
        .query_context_support_profile()
        .expect("query context support profile should be present");
    assert_eq!(
        profile
            .admitted_basis_families()
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>(),
        vec![
            "current_branch_head",
            "branch_head",
            "historical_snapshot",
            "historical_commit",
            "preview_derived_historical"
        ]
    );
    assert_eq!(
        profile
            .admitted_comparison_families()
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>(),
        vec![
            "branch_to_branch",
            "current_to_historical",
            "historical_to_historical",
            "preview_to_authoritative"
        ]
    );
    assert_eq!(
        profile
            .deferred_scope_markers()
            .iter()
            .map(|marker| marker.as_str())
            .collect::<Vec<_>>(),
        vec![
            "store_backed_historical",
            "store_backed_diff",
            "broad_collection_diff"
        ]
    );
}

#[test]
fn support_report_includes_identity_evolution_capability_and_profile() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let support = facade.support_matrix();
    let report = facade.support_report();

    assert_eq!(
        support
            .descriptor(ForgeQueryCapabilityFamily::IdentityEvolution)
            .expect("identity evolution descriptor should exist")
            .status(),
        ForgeQueryCapabilityStatus::Admitted
    );
    assert!(report
        .admitted_capability_families()
        .contains(&ForgeQueryCapabilityFamily::IdentityEvolution));
    let profile = report
        .identity_evolution_support_profile()
        .expect("identity evolution profile should be present");
    assert_eq!(
        profile
            .admitted_traversal_families()
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>(),
        vec![
            "direct_predecessor",
            "direct_successor",
            "direct_replacement",
            "direct_split_successors",
            "direct_merge_successor",
            "branch_local_direct_evolution"
        ]
    );
}

#[test]
fn identity_evolution_capability_admits_and_executes_query_surface() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let identity_evolution = facade
        .identity_evolution_capability()
        .expect("runtime-backed facade should admit identity evolution");
    let admission = identity_evolution.admission().clone();

    let admitted = identity_evolution
        .capability()
        .admit_query(IdentityEvolutionQueryContext::correspondence_identity_comparison(
            crate::identity::CanonicalQueryDigest::from_parts(&["app:identity".to_string()]),
            IdentityEvolutionComparisonBasisFamily::BranchToBranch,
            crate::identity::BasisDigest::from_parts(&["basis:left".to_string()]),
            crate::identity::BasisDigest::from_parts(&["basis:right".to_string()]),
            CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
        ))
        .expect("identity evolution comparison should admit");
    let execution = identity_evolution
        .capability()
        .execute_query(&admitted)
        .expect("identity evolution comparison should execute");

    assert_eq!(
        admission.descriptor().family(),
        ForgeQueryCapabilityFamily::IdentityEvolution
    );
    assert_eq!(
        execution.family().as_str(),
        "branch_to_branch_comparison"
    );
    assert!(execution
        .result_bundle()
        .as_advisory_identity_candidate_set()
        .is_some());
    assert_eq!(identity_evolution.counters().capability_lookup_count(), 1);
}

#[test]
fn identity_evolution_capability_disables_typed_and_early_when_query_section_is_off() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_query(ForgeQueryQueryConfig::disabled())
            .with_signal(ForgeQuerySignalConfig::disabled())
            .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled())
            .with_relational(
                ForgeQueryRelationalConfig::disabled().with_historical_evaluation(false),
            ),
    )
    .expect("query-disabled facade config should still validate");

    let error = facade
        .identity_evolution_capability()
        .expect_err("disabled query section should deny identity evolution");
    assert_eq!(
        error.failure_class(),
        ForgeQueryFacadeFailureClass::MissingOwningSection
    );
    assert_eq!(
        error.capability_family(),
        Some(ForgeQueryCapabilityFamily::IdentityEvolution)
    );
}

#[test]
fn broad_collection_diff_remains_denied_before_diff_bundle_construction() {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let contexts = facade
        .query_context_capability()
        .expect("query context capability should admit");
    let left_preflight = execution_preflights::ordered_collection_without_traversal_preflight();
    let right_preflight = execution_preflights::alternate_basis_ordered_collection_preflight();
    let left = contexts
        .capability()
        .admit_basis_context(
            contexts
                .capability()
                .bind_basis_context(
                    QueryBasisContextRequest::current_branch_head(),
                    QueryContextBindingSource::RuntimeCurrent(&left_preflight),
                )
                .expect("left context should bind"),
        )
        .expect("left context should admit");
    let right = contexts
        .capability()
        .admit_basis_context(
            contexts
                .capability()
                .bind_basis_context(
                    QueryBasisContextRequest::branch_head("branch:ordered-collection"),
                    QueryContextBindingSource::RuntimeBranch(&right_preflight),
                )
                .expect("right context should bind"),
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
        .expect_err("broad collection diff should deny before bundle construction");

    assert_eq!(
        error.failure_class().clone(),
        crate::query_context::QueryContextAdmissionFailureClass::ComparisonBroadeningRequired
    );
}

#[test]
fn support_report_tracks_disabled_signal_section_posture() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_signal(ForgeQuerySignalConfig::disabled()),
    )
    .expect("signal-disabled facade config should still validate");
    let report = facade.support_report();

    let signal_posture = report
        .section_postures()
        .iter()
        .find(|posture| posture.section() == ForgeQueryConfigSectionFamily::Signal)
        .expect("signal posture should be present");
    assert_eq!(
        signal_posture.owner(),
        super::ForgeQuerySubsystemOwner::Signal
    );
    assert!(!signal_posture.enabled());
    assert_eq!(
        report
            .unsupported_capability_families()
            .contains(&ForgeQueryCapabilityFamily::LiveQuery),
        true
    );
}

#[test]
fn runtime_bridge_section_controls_preview_capability_without_query_section_drift() {
    let facade = ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled()),
    )
    .expect("disabling runtime bridge alone should preserve a valid facade config");

    let error = facade
        .preview_query_capability()
        .expect_err("disabled runtime bridge section should deny preview capability admission");
    assert_eq!(
        error.failure_class(),
        ForgeQueryFacadeFailureClass::MissingOwningSection
    );
    assert_eq!(
        error.capability_family(),
        Some(ForgeQueryCapabilityFamily::PreviewSession)
    );
    assert_eq!(error.counters().unsupported_composition_denial_count(), 1);
    assert_eq!(error.counters().deferred_capability_denial_count(), 0);
}
