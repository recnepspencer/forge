use crate::identity::hash_parts;

use super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::admission_error::QuerySubscriptionAdmissionError;
use super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::bridge_lowering_error::QuerySubscriptionBridgeLoweringError;
use super::super::certification::{
    SubscriptionLifecycleCertificationBundle, SubscriptionLifecycleCertificationError,
};
use super::super::closeout::SubscriptionLifecycleCloseout;
use super::super::continuation::SubscriptionContinuationReport;
use super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::declaration_error::QuerySubscriptionDeclarationDenial;
use super::super::error::QuerySubscriptionFamilySelectionError;
use super::super::preview_isolation::PreviewSubscriptionIsolationArtifact;
use super::super::support::{
    QuerySubscriptionSupportPosture, QuerySubscriptionSupportReport,
    QuerySubscriptionSupportReportError,
};
use super::context::QuerySubscriptionDiagnosticSelectionContext;
use super::stage::{QuerySubscriptionDiagnosticOutcome, QuerySubscriptionDiagnosticStage};
use super::trace::QuerySubscriptionDiagnosticTrace;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BundleAssemblyPosture {
    ComposedFromCanonicalArtifacts,
    PartialRediscoveryDebtExplicit,
    PartialRediscoveryDenied,
}

impl BundleAssemblyPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ComposedFromCanonicalArtifacts => "composed_from_canonical_artifacts",
            Self::PartialRediscoveryDebtExplicit => "partial_rediscovery_debt_explicit",
            Self::PartialRediscoveryDenied => "partial_rediscovery_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticBundleWidth {
    stage_evidence_count: usize,
    failure_evidence_count: usize,
    hostile_row_reference_count: usize,
    digest: String,
}

impl QuerySubscriptionDiagnosticBundleWidth {
    fn new(
        stage_evidence_count: usize,
        failure_evidence_count: usize,
        hostile_row_reference_count: usize,
    ) -> Self {
        let digest = hash_parts(&[
            "query_subscription_diagnostic_bundle_width_v1".to_string(),
            format!("stage_evidence_count:{stage_evidence_count}"),
            format!("failure_evidence_count:{failure_evidence_count}"),
            format!("hostile_row_reference_count:{hostile_row_reference_count}"),
        ]);
        Self {
            stage_evidence_count,
            failure_evidence_count,
            hostile_row_reference_count,
            digest,
        }
    }

    pub fn stage_evidence_count(&self) -> usize {
        self.stage_evidence_count
    }

    pub fn failure_evidence_count(&self) -> usize {
        self.failure_evidence_count
    }

    pub fn hostile_row_reference_count(&self) -> usize {
        self.hostile_row_reference_count
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticCounters {
    diagnostic_trace_emission_count: u64,
    diagnostic_bundle_emission_count: u64,
    denied_bundle_emission_count: u64,
    diagnostic_missing_stage_denial_count: u64,
    diagnostic_bundle_width: u64,
}

impl QuerySubscriptionDiagnosticCounters {
    pub fn digest(&self) -> String {
        hash_parts(&[
            format!(
                "diagnostic_trace_emission:{}",
                self.diagnostic_trace_emission_count
            ),
            format!(
                "diagnostic_bundle_emission:{}",
                self.diagnostic_bundle_emission_count
            ),
            format!(
                "diagnostic_denied_bundle_emission:{}",
                self.denied_bundle_emission_count
            ),
            format!(
                "diagnostic_missing_stage_denial:{}",
                self.diagnostic_missing_stage_denial_count
            ),
            format!("diagnostic_bundle_width:{}", self.diagnostic_bundle_width),
        ])
    }

    pub fn diagnostic_trace_emission_count(&self) -> u64 {
        self.diagnostic_trace_emission_count
    }

    pub fn diagnostic_bundle_emission_count(&self) -> u64 {
        self.diagnostic_bundle_emission_count
    }

    pub fn denied_bundle_emission_count(&self) -> u64 {
        self.denied_bundle_emission_count
    }

    pub fn diagnostic_missing_stage_denial_count(&self) -> u64 {
        self.diagnostic_missing_stage_denial_count
    }

    pub fn diagnostic_bundle_width(&self) -> u64 {
        self.diagnostic_bundle_width
    }

    pub(crate) fn trace_emitted(width: u64) -> Self {
        Self {
            diagnostic_trace_emission_count: 1,
            diagnostic_bundle_width: width,
            ..Default::default()
        }
    }

    pub(crate) fn admitted_bundle_emitted(trace_count: u64, width: u64) -> Self {
        Self {
            diagnostic_trace_emission_count: trace_count,
            diagnostic_bundle_emission_count: 1,
            diagnostic_bundle_width: width,
            ..Default::default()
        }
    }

    pub(crate) fn denied_bundle_emitted(trace_count: u64, width: u64) -> Self {
        Self {
            diagnostic_trace_emission_count: trace_count,
            denied_bundle_emission_count: 1,
            diagnostic_bundle_width: width,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticAssemblyReceipt {
    bundle_assembly_posture: BundleAssemblyPosture,
    stage_evidence_composition_count: usize,
    semantic_label_carry_forward_count: usize,
    stage_rederivation_count: usize,
    bundle_width: QuerySubscriptionDiagnosticBundleWidth,
    digest: String,
}

impl DiagnosticAssemblyReceipt {
    fn new(
        bundle_assembly_posture: BundleAssemblyPosture,
        stage_evidence_composition_count: usize,
        semantic_label_carry_forward_count: usize,
        stage_rederivation_count: usize,
        bundle_width: QuerySubscriptionDiagnosticBundleWidth,
    ) -> Self {
        let digest = hash_parts(&[
            "query_subscription_diagnostic_assembly_receipt_v1".to_string(),
            bundle_assembly_posture.as_str().to_string(),
            format!("stage_evidence_composition_count:{stage_evidence_composition_count}"),
            format!("semantic_label_carry_forward_count:{semantic_label_carry_forward_count}"),
            format!("stage_rederivation_count:{stage_rederivation_count}"),
            format!("bundle_width:{}", bundle_width.digest()),
        ]);
        Self {
            bundle_assembly_posture,
            stage_evidence_composition_count,
            semantic_label_carry_forward_count,
            stage_rederivation_count,
            bundle_width,
            digest,
        }
    }

    pub fn bundle_assembly_posture(&self) -> &BundleAssemblyPosture {
        &self.bundle_assembly_posture
    }

    pub fn stage_evidence_composition_count(&self) -> usize {
        self.stage_evidence_composition_count
    }

    pub fn semantic_label_carry_forward_count(&self) -> usize {
        self.semantic_label_carry_forward_count
    }

    pub fn stage_rederivation_count(&self) -> usize {
        self.stage_rederivation_count
    }

    pub fn bundle_width(&self) -> &QuerySubscriptionDiagnosticBundleWidth {
        &self.bundle_width
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticSemanticLabels {
    query_family_label: String,
    declaration_family_label: String,
    bridge_family_label: String,
    bridge_slice_labels: Vec<String>,
    basis_posture_label: String,
    signal_strategy_class_label: String,
    support_posture_label: String,
    denial_or_coverage_class_label: String,
    digest: String,
}

impl QuerySubscriptionDiagnosticSemanticLabels {
    fn new(
        query_family_label: String,
        declaration_family_label: String,
        bridge_family_label: String,
        bridge_slice_labels: Vec<String>,
        basis_posture_label: String,
        signal_strategy_class_label: String,
        support_posture_label: String,
        denial_or_coverage_class_label: String,
    ) -> Self {
        let mut digest_parts = vec![
            "query_subscription_diagnostic_semantic_labels_v1".to_string(),
            format!("query_family:{query_family_label}"),
            format!("declaration_family:{declaration_family_label}"),
            format!("bridge_family:{bridge_family_label}"),
            format!("basis_posture:{basis_posture_label}"),
            format!("signal_strategy_class:{signal_strategy_class_label}"),
            format!("support_posture:{support_posture_label}"),
            format!("denial_or_coverage_class:{denial_or_coverage_class_label}"),
        ];
        digest_parts.extend(
            bridge_slice_labels
                .iter()
                .enumerate()
                .map(|(index, label)| format!("bridge_slice:{index}:{label}")),
        );
        let digest = hash_parts(&digest_parts);
        Self {
            query_family_label,
            declaration_family_label,
            bridge_family_label,
            bridge_slice_labels,
            basis_posture_label,
            signal_strategy_class_label,
            support_posture_label,
            denial_or_coverage_class_label,
            digest,
        }
    }

    pub fn query_family_label(&self) -> &str {
        &self.query_family_label
    }

    pub fn declaration_family_label(&self) -> &str {
        &self.declaration_family_label
    }

    pub fn bridge_family_label(&self) -> &str {
        &self.bridge_family_label
    }

    pub fn bridge_slice_labels(&self) -> &[String] {
        &self.bridge_slice_labels
    }

    pub fn basis_posture_label(&self) -> &str {
        &self.basis_posture_label
    }

    pub fn signal_strategy_class_label(&self) -> &str {
        &self.signal_strategy_class_label
    }

    pub fn support_posture_label(&self) -> &str {
        &self.support_posture_label
    }

    pub fn denial_or_coverage_class_label(&self) -> &str {
        &self.denial_or_coverage_class_label
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionDiagnosticBundleErrorKind {
    MissingRequiredStage,
    SelectionContextMismatch,
    DeclarationSourceMismatch,
    BridgeLoweringSourceMismatch,
    AdmissionSourceMismatch,
    SupportSourceMismatch,
    LifecycleSourceMismatch,
    FailureSourceMismatch,
}

impl QuerySubscriptionDiagnosticBundleErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingRequiredStage => "missing_required_stage",
            Self::SelectionContextMismatch => "selection_context_mismatch",
            Self::DeclarationSourceMismatch => "declaration_source_mismatch",
            Self::BridgeLoweringSourceMismatch => "bridge_lowering_source_mismatch",
            Self::AdmissionSourceMismatch => "admission_source_mismatch",
            Self::SupportSourceMismatch => "support_source_mismatch",
            Self::LifecycleSourceMismatch => "lifecycle_source_mismatch",
            Self::FailureSourceMismatch => "failure_source_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticBundleError {
    error_kind: QuerySubscriptionDiagnosticBundleErrorKind,
    message: &'static str,
    failure_digest: String,
    counters: QuerySubscriptionDiagnosticCounters,
}

impl QuerySubscriptionDiagnosticBundleError {
    pub(crate) fn new(
        error_kind: QuerySubscriptionDiagnosticBundleErrorKind,
        message: &'static str,
        evidence_parts: &[String],
    ) -> Self {
        let mut parts = vec![
            "query_subscription_diagnostic_bundle_error_v1".to_string(),
            error_kind.as_str().to_string(),
            message.to_string(),
        ];
        parts.extend(evidence_parts.iter().cloned());
        Self {
            error_kind,
            message,
            failure_digest: hash_parts(&parts),
            counters: QuerySubscriptionDiagnosticCounters {
                diagnostic_missing_stage_denial_count: 1,
                ..Default::default()
            },
        }
    }

    pub fn error_kind(&self) -> &QuerySubscriptionDiagnosticBundleErrorKind {
        &self.error_kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionDiagnosticCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticFailure {
    stage: QuerySubscriptionDiagnosticStage,
    outcome: QuerySubscriptionDiagnosticOutcome,
    reason: String,
    source_digest: String,
    counter_digest: String,
    failure_digest: String,
}

impl QuerySubscriptionDiagnosticFailure {
    fn new(
        stage: QuerySubscriptionDiagnosticStage,
        reason: impl Into<String>,
        source_digest: impl Into<String>,
        counter_digest: impl Into<String>,
        failure_digest: impl Into<String>,
    ) -> Self {
        Self {
            stage,
            outcome: QuerySubscriptionDiagnosticOutcome::Denied,
            reason: reason.into(),
            source_digest: source_digest.into(),
            counter_digest: counter_digest.into(),
            failure_digest: failure_digest.into(),
        }
    }

    pub fn from_family_selection_error(error: &QuerySubscriptionFamilySelectionError) -> Self {
        Self::new(
            *error.diagnostic().stage(),
            error.message(),
            error.diagnostic().source_digest(),
            error.diagnostic().counter_digest(),
            hash_parts(&[
                "query_subscription_diagnostic_failure_v1".to_string(),
                error.failure_class().as_str().to_string(),
                error.diagnostic().digest().to_string(),
            ]),
        )
    }

    pub fn from_declaration_denial(error: &QuerySubscriptionDeclarationDenial) -> Self {
        Self::new(
            *error.diagnostic().stage(),
            error.message(),
            error.diagnostic().source_digest(),
            error.diagnostic().counter_digest(),
            hash_parts(&[
                "query_subscription_diagnostic_failure_v1".to_string(),
                error.denial_kind().as_str().to_string(),
                error.diagnostic().digest().to_string(),
            ]),
        )
    }

    pub fn from_bridge_lowering_error(error: &QuerySubscriptionBridgeLoweringError) -> Self {
        Self::new(
            *error.diagnostic().stage(),
            error.message(),
            error.diagnostic().source_digest(),
            error.diagnostic().counter_digest(),
            hash_parts(&[
                "query_subscription_diagnostic_failure_v1".to_string(),
                error.denial_kind().as_str().to_string(),
                error.diagnostic().digest().to_string(),
            ]),
        )
    }

    pub fn from_admission_error(error: &QuerySubscriptionAdmissionError) -> Self {
        Self::new(
            *error.pipeline_diagnostic().stage(),
            error.message(),
            error.pipeline_diagnostic().source_digest(),
            error.pipeline_diagnostic().counter_digest(),
            hash_parts(&[
                "query_subscription_diagnostic_failure_v1".to_string(),
                error.denial_kind().as_str().to_string(),
                error.pipeline_diagnostic().digest().to_string(),
            ]),
        )
    }

    pub fn from_support_report_error(error: &QuerySubscriptionSupportReportError) -> Self {
        Self::new(
            QuerySubscriptionDiagnosticStage::SupportReporting,
            error.message(),
            error.failure_digest(),
            error.failure_digest(),
            error.failure_digest(),
        )
    }

    pub fn from_lifecycle_certification_error(
        error: &SubscriptionLifecycleCertificationError,
    ) -> Self {
        Self::new(
            QuerySubscriptionDiagnosticStage::Certification,
            error.message(),
            error.failure_digest(),
            error.failure_digest(),
            error.failure_digest(),
        )
    }

    pub fn stage(&self) -> &QuerySubscriptionDiagnosticStage {
        &self.stage
    }

    pub fn outcome(&self) -> &QuerySubscriptionDiagnosticOutcome {
        &self.outcome
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDeniedDiagnosticBundle {
    trace: QuerySubscriptionDiagnosticTrace,
    semantic_labels: QuerySubscriptionDiagnosticSemanticLabels,
    failure: QuerySubscriptionDiagnosticFailure,
    omitted_stages: Vec<QuerySubscriptionDiagnosticStage>,
    support_report_digest: Option<String>,
    counter_snapshot: String,
    bundle_digest: String,
    counters: QuerySubscriptionDiagnosticCounters,
}

impl QuerySubscriptionDeniedDiagnosticBundle {
    pub fn trace(&self) -> &QuerySubscriptionDiagnosticTrace {
        &self.trace
    }

    pub fn semantic_labels(&self) -> &QuerySubscriptionDiagnosticSemanticLabels {
        &self.semantic_labels
    }

    pub fn failure(&self) -> &QuerySubscriptionDiagnosticFailure {
        &self.failure
    }

    pub fn omitted_stages(&self) -> &[QuerySubscriptionDiagnosticStage] {
        &self.omitted_stages
    }

    pub fn support_report_digest(&self) -> Option<&str> {
        self.support_report_digest.as_deref()
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionDiagnosticCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionAdmittedDiagnosticBundle {
    trace: QuerySubscriptionDiagnosticTrace,
    semantic_labels: QuerySubscriptionDiagnosticSemanticLabels,
    support_report_digest: String,
    lifecycle_certification_digest: String,
    continuation_digest: Option<String>,
    preview_isolation_digest: Option<String>,
    lifecycle_closeout_digest: Option<String>,
    counter_snapshot: String,
    bundle_digest: String,
    counters: QuerySubscriptionDiagnosticCounters,
}

impl QuerySubscriptionAdmittedDiagnosticBundle {
    pub fn trace(&self) -> &QuerySubscriptionDiagnosticTrace {
        &self.trace
    }

    pub fn semantic_labels(&self) -> &QuerySubscriptionDiagnosticSemanticLabels {
        &self.semantic_labels
    }

    pub fn support_report_digest(&self) -> &str {
        &self.support_report_digest
    }

    pub fn lifecycle_certification_digest(&self) -> &str {
        &self.lifecycle_certification_digest
    }

    pub fn continuation_digest(&self) -> Option<&str> {
        self.continuation_digest.as_deref()
    }

    pub fn preview_isolation_digest(&self) -> Option<&str> {
        self.preview_isolation_digest.as_deref()
    }

    pub fn lifecycle_closeout_digest(&self) -> Option<&str> {
        self.lifecycle_closeout_digest.as_deref()
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionDiagnosticCounters {
        &self.counters
    }
}

#[allow(clippy::too_many_arguments)]
pub fn bundle_admitted_query_subscription_diagnostics(
    trace: QuerySubscriptionDiagnosticTrace,
    selection: &super::super::selection::QuerySubscriptionFamilySelection,
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    admission: &QuerySubscriptionAdmissionArtifact,
    support: &QuerySubscriptionSupportReport,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
    continuation: Option<&SubscriptionContinuationReport>,
    preview: Option<&PreviewSubscriptionIsolationArtifact>,
    lifecycle_closeout: Option<&SubscriptionLifecycleCloseout>,
) -> Result<
    (
        QuerySubscriptionAdmittedDiagnosticBundle,
        DiagnosticAssemblyReceipt,
    ),
    QuerySubscriptionDiagnosticBundleError,
> {
    let selection_context = QuerySubscriptionDiagnosticSelectionContext::from_selection(selection);
    validate_selection_and_declaration(&selection_context, declaration)?;
    validate_declaration_and_lowering(declaration, lowering)?;
    validate_declaration_and_admission(declaration, admission)?;
    validate_declaration_and_support(declaration, support)?;
    validate_admitted_sources(declaration, lowering, lifecycle)?;
    validate_admitted_trace_terminal_stage(&trace)?;
    validate_admitted_trace_sources(
        &trace,
        selection,
        declaration,
        lowering,
        admission,
        support,
        lifecycle,
        continuation,
        preview,
        lifecycle_closeout,
    )?;

    let semantic_labels = semantic_labels_for_support(
        selection.family().as_str(),
        declaration,
        lowering,
        support.support_posture(),
        "runtime_lifecycle_certified",
    );
    let bundle_width =
        QuerySubscriptionDiagnosticBundleWidth::new(trace.stage_traces().len(), 0, 0);
    let receipt = DiagnosticAssemblyReceipt::new(
        BundleAssemblyPosture::ComposedFromCanonicalArtifacts,
        trace.stage_traces().len(),
        semantic_label_count(&semantic_labels),
        0,
        bundle_width.clone(),
    );
    let counters = QuerySubscriptionDiagnosticCounters::admitted_bundle_emitted(
        trace.counters().diagnostic_trace_emission_count(),
        bundle_width.stage_evidence_count() as u64,
    );
    let counter_snapshot = counters.digest();
    let bundle_digest = hash_parts(&[
        "query_subscription_admitted_diagnostic_bundle_v1".to_string(),
        format!("trace:{}", trace.trace_digest()),
        format!("labels:{}", semantic_labels.digest()),
        format!("support:{}", support.report_digest()),
        format!(
            "lifecycle_certification:{}",
            lifecycle.certification_bundle_digest()
        ),
        format!("receipt:{}", receipt.digest()),
        format!("counters:{counter_snapshot}"),
        format!("admission:{}", admission.admission_digest()),
        format!(
            "continuation:{}",
            continuation
                .map(|value| value.report_digest())
                .unwrap_or("none")
        ),
        format!(
            "preview:{}",
            preview
                .map(|value| value.isolation_digest())
                .unwrap_or("none")
        ),
        format!(
            "closeout:{}",
            lifecycle_closeout
                .map(|value| value.closeout_digest())
                .unwrap_or("none")
        ),
    ]);

    Ok((
        QuerySubscriptionAdmittedDiagnosticBundle {
            trace,
            semantic_labels,
            support_report_digest: support.report_digest().to_string(),
            lifecycle_certification_digest: lifecycle.certification_bundle_digest().to_string(),
            continuation_digest: continuation.map(|value| value.report_digest().to_string()),
            preview_isolation_digest: preview.map(|value| value.isolation_digest().to_string()),
            lifecycle_closeout_digest: lifecycle_closeout
                .map(|value| value.closeout_digest().to_string()),
            counter_snapshot,
            bundle_digest,
            counters,
        },
        receipt,
    ))
}

pub fn bundle_denied_query_subscription_diagnostics(
    trace: QuerySubscriptionDiagnosticTrace,
    selection_context: &QuerySubscriptionDiagnosticSelectionContext,
    declaration: Option<&QuerySubscriptionDeclarationArtifact>,
    lowering: Option<&BridgeSubscriptionLoweringPlan>,
    admission: Option<&QuerySubscriptionAdmissionArtifact>,
    support: Option<&QuerySubscriptionSupportReport>,
    failure: QuerySubscriptionDiagnosticFailure,
) -> Result<
    (
        QuerySubscriptionDeniedDiagnosticBundle,
        DiagnosticAssemblyReceipt,
    ),
    QuerySubscriptionDiagnosticBundleError,
> {
    validate_denied_selection_context(
        selection_context,
        failure.stage(),
        failure.source_digest(),
        declaration.is_some() || lowering.is_some() || admission.is_some() || support.is_some(),
    )?;
    if let Some(declaration) = declaration {
        validate_selection_and_declaration(selection_context, declaration)?;
    }
    if let (Some(declaration), Some(lowering)) = (declaration, lowering) {
        validate_declaration_and_lowering(declaration, lowering)?;
    }
    if let (Some(declaration), Some(admission)) = (declaration, admission) {
        validate_declaration_and_admission(declaration, admission)?;
    }
    if let (Some(declaration), Some(support)) = (declaration, support) {
        validate_declaration_and_support(declaration, support)?;
    }
    validate_trace_terminal_stage(&trace, *failure.stage())?;
    validate_denied_trace_sources(
        &trace,
        selection_context,
        declaration,
        lowering,
        admission,
        support,
        &failure,
    )?;

    let semantic_labels = semantic_labels_for_denied_bundle(
        selection_context,
        declaration,
        lowering,
        support,
        failure.stage().as_str(),
    );
    let omitted_stages = omitted_stages_after_failure(*failure.stage());
    let bundle_width =
        QuerySubscriptionDiagnosticBundleWidth::new(trace.stage_traces().len(), 1, 0);
    let receipt = DiagnosticAssemblyReceipt::new(
        BundleAssemblyPosture::ComposedFromCanonicalArtifacts,
        trace.stage_traces().len(),
        semantic_label_count(&semantic_labels),
        0,
        bundle_width.clone(),
    );
    let counters = QuerySubscriptionDiagnosticCounters::denied_bundle_emitted(
        trace.counters().diagnostic_trace_emission_count(),
        (bundle_width.stage_evidence_count() + bundle_width.failure_evidence_count()) as u64,
    );
    let counter_snapshot = counters.digest();
    let bundle_digest = hash_parts(&[
        "query_subscription_denied_diagnostic_bundle_v1".to_string(),
        format!("trace:{}", trace.trace_digest()),
        format!("labels:{}", semantic_labels.digest()),
        format!("failure:{}", failure.failure_digest()),
        format!("receipt:{}", receipt.digest()),
        format!("counters:{counter_snapshot}"),
        format!(
            "support:{}",
            support.map(|value| value.report_digest()).unwrap_or("none")
        ),
    ]);

    Ok((
        QuerySubscriptionDeniedDiagnosticBundle {
            trace,
            semantic_labels,
            failure,
            omitted_stages,
            support_report_digest: support.map(|value| value.report_digest().to_string()),
            counter_snapshot,
            bundle_digest,
            counters,
        },
        receipt,
    ))
}

fn validate_selection_and_declaration(
    selection: &QuerySubscriptionDiagnosticSelectionContext,
    declaration: &QuerySubscriptionDeclarationArtifact,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if selection.selection().map(|value| value.family()) != Some(declaration.family()) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
            "diagnostic bundle assembly requires declaration and family selection to preserve the same query subscription family",
            &[
                format!("selection_family:{}", selection.query_family_label()),
                format!("declaration_family:{}", declaration.family().as_str()),
            ],
        ));
    }
    Ok(())
}

fn validate_declaration_and_lowering(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if lowering.query_declaration_digest() != declaration.declaration_digest().as_str() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
            "diagnostic bundle assembly requires bridge lowering to bind the same declaration artifact",
            &[
                format!("declaration:{}", declaration.declaration_digest().as_str()),
                format!("lowering:{}", lowering.query_declaration_digest()),
            ],
        ));
    }
    Ok(())
}

fn validate_declaration_and_admission(
    declaration: &QuerySubscriptionDeclarationArtifact,
    admission: &QuerySubscriptionAdmissionArtifact,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if admission.query_declaration_digest() != declaration.declaration_digest().as_str() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
            "diagnostic bundle assembly requires admission and declaration to preserve the same canonical declaration digest",
            &[
                format!("declaration:{}", declaration.declaration_digest().as_str()),
                format!("admission:{}", admission.query_declaration_digest()),
            ],
        ));
    }
    Ok(())
}

fn validate_declaration_and_support(
    declaration: &QuerySubscriptionDeclarationArtifact,
    support: &QuerySubscriptionSupportReport,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if support.support_subject().declaration_digest() != declaration.declaration_digest().as_str() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
            "diagnostic bundle assembly requires support reporting to bind the same declaration artifact",
            &[
                format!("declaration:{}", declaration.declaration_digest().as_str()),
                format!(
                    "support_declaration:{}",
                    support.support_subject().declaration_digest()
                ),
            ],
        ));
    }
    Ok(())
}

fn validate_admitted_sources(
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if lifecycle.subscription_declaration_digest() != declaration.declaration_digest().as_str()
        || lifecycle.bridge_declaration_digest() != lowering.bridge_declaration_digest()
    {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::LifecycleSourceMismatch,
            "diagnostic bundle assembly requires lifecycle certification to preserve declaration and bridge lowering identity",
            &[
                format!("declaration:{}", declaration.declaration_digest().as_str()),
                format!(
                    "lifecycle_declaration:{}",
                    lifecycle.subscription_declaration_digest()
                ),
                format!("bridge:{}", lowering.bridge_declaration_digest()),
                format!("lifecycle_bridge:{}", lifecycle.bridge_declaration_digest()),
            ],
        ));
    }
    Ok(())
}

fn validate_trace_terminal_stage(
    trace: &QuerySubscriptionDiagnosticTrace,
    expected_terminal_stage: QuerySubscriptionDiagnosticStage,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if trace.terminal_stage() != &expected_terminal_stage {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
            "diagnostic bundle assembly requires the trace terminal stage to match the assembled outcome",
            &[
                format!("trace_terminal_stage:{}", trace.terminal_stage().as_str()),
                format!("expected_terminal_stage:{}", expected_terminal_stage.as_str()),
            ],
        ));
    }
    Ok(())
}

fn validate_admitted_trace_terminal_stage(
    trace: &QuerySubscriptionDiagnosticTrace,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if matches!(
        trace.terminal_stage(),
        QuerySubscriptionDiagnosticStage::Certification
            | QuerySubscriptionDiagnosticStage::Continuation
            | QuerySubscriptionDiagnosticStage::PreviewIsolation
            | QuerySubscriptionDiagnosticStage::LifecycleCloseout
    ) {
        Ok(())
    } else {
        Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
            "admitted diagnostic bundle assembly requires a certification-stage trace that may extend through continuation, preview, or closeout evidence",
            &[format!(
                "trace_terminal_stage:{}",
                trace.terminal_stage().as_str()
            )],
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_admitted_trace_sources(
    trace: &QuerySubscriptionDiagnosticTrace,
    selection: &super::super::selection::QuerySubscriptionFamilySelection,
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    admission: &QuerySubscriptionAdmissionArtifact,
    support: &QuerySubscriptionSupportReport,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
    continuation: Option<&SubscriptionContinuationReport>,
    preview: Option<&PreviewSubscriptionIsolationArtifact>,
    lifecycle_closeout: Option<&SubscriptionLifecycleCloseout>,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::FamilySelection,
        selection.equivalence_basis().digest().as_str(),
        "admitted diagnostic bundle assembly requires family-selection trace evidence for the supplied canonical family selection",
        QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::Declaration,
        declaration.declaration_digest().as_str(),
        "admitted diagnostic bundle assembly requires declaration trace evidence for the supplied canonical declaration artifact",
        QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
        lowering.bridge_declaration_digest(),
        "admitted diagnostic bundle assembly requires bridge-lowering trace evidence for the supplied bridge declaration artifact",
        QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
        admission.admission_digest(),
        "admitted diagnostic bundle assembly requires runtime-admission trace evidence for the supplied admission artifact",
        QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::SupportReporting,
        support.report_digest(),
        "admitted diagnostic bundle assembly requires support-reporting trace evidence for the supplied support report",
        QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
    )?;
    validate_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::Certification,
        lifecycle.certification_bundle_digest(),
        "admitted diagnostic bundle assembly requires lifecycle-certification trace evidence for the supplied certification bundle",
        QuerySubscriptionDiagnosticBundleErrorKind::LifecycleSourceMismatch,
    )?;
    validate_optional_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::Continuation,
        continuation.map(|value| value.report_digest()),
        "admitted diagnostic bundle assembly may only carry continuation trace evidence when the supplied continuation artifact is present",
        QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
    )?;
    validate_optional_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::PreviewIsolation,
        preview.map(|value| value.isolation_digest()),
        "admitted diagnostic bundle assembly may only carry preview-isolation trace evidence when the supplied preview artifact is present",
        QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
    )?;
    validate_optional_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::LifecycleCloseout,
        lifecycle_closeout.map(|value| value.closeout_digest()),
        "admitted diagnostic bundle assembly may only carry lifecycle-closeout trace evidence when the supplied closeout artifact is present",
        QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
    )?;
    Ok(())
}

fn validate_denied_trace_sources(
    trace: &QuerySubscriptionDiagnosticTrace,
    selection_context: &QuerySubscriptionDiagnosticSelectionContext,
    declaration: Option<&QuerySubscriptionDeclarationArtifact>,
    lowering: Option<&BridgeSubscriptionLoweringPlan>,
    admission: Option<&QuerySubscriptionAdmissionArtifact>,
    support: Option<&QuerySubscriptionSupportReport>,
    failure: &QuerySubscriptionDiagnosticFailure,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    validate_trace_stage_source(
        trace,
        if selection_context.is_selection_denied() {
            *failure.stage()
        } else {
            QuerySubscriptionDiagnosticStage::FamilySelection
        },
        if selection_context.is_selection_denied() {
            failure.source_digest()
        } else {
            selection_context.source_digest()
        },
        "denied diagnostic bundle assembly requires trace family-selection evidence for the supplied selection context",
        QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
    )?;

    if let Some(declaration) = declaration {
        validate_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::Declaration,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::Declaration
                    | QuerySubscriptionDiagnosticStage::DeliveryIntent
            ) {
                failure.source_digest()
            } else {
                declaration.declaration_digest().as_str()
            },
            "denied diagnostic bundle assembly requires declaration trace evidence aligned with the supplied declaration or declaration-stage failure",
            QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
        )?;
    } else {
        validate_optional_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::Declaration,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::Declaration
                    | QuerySubscriptionDiagnosticStage::DeliveryIntent
            ) {
                Some(failure.source_digest())
            } else {
                None
            },
            "diagnostic bundle assembly requires the trace to carry every stage that the assembled artifacts claim",
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
        )?;
    }

    if let Some(lowering) = lowering {
        validate_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::BridgeFamilyLowering
                    | QuerySubscriptionDiagnosticStage::BridgeSliceLowering
                    | QuerySubscriptionDiagnosticStage::BasisBinding
            ) {
                failure.source_digest()
            } else {
                lowering.bridge_declaration_digest()
            },
            "denied diagnostic bundle assembly requires bridge-lowering trace evidence aligned with the supplied lowering artifact or bridge-stage failure",
            QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
        )?;
    } else {
        validate_optional_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::BridgeFamilyLowering
                    | QuerySubscriptionDiagnosticStage::BridgeSliceLowering
                    | QuerySubscriptionDiagnosticStage::BasisBinding
            ) {
                Some(failure.source_digest())
            } else {
                None
            },
            "diagnostic bundle assembly requires the trace to carry every stage that the assembled artifacts claim",
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
        )?;
    }

    if let Some(admission) = admission {
        validate_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission
                    | QuerySubscriptionDiagnosticStage::AdmissionBudget
                    | QuerySubscriptionDiagnosticStage::DurableReloadOverclaim
                    | QuerySubscriptionDiagnosticStage::ActiveLifecycleAllocation
                    | QuerySubscriptionDiagnosticStage::ActivationReadiness
            ) {
                failure.source_digest()
            } else {
                admission.admission_digest()
            },
            "denied diagnostic bundle assembly requires runtime-admission trace evidence aligned with the supplied admission artifact or admission-stage failure",
            QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
        )?;
    } else {
        validate_optional_trace_stage_source(
            trace,
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            if matches!(
                failure.stage(),
                QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission
                    | QuerySubscriptionDiagnosticStage::AdmissionBudget
                    | QuerySubscriptionDiagnosticStage::DurableReloadOverclaim
                    | QuerySubscriptionDiagnosticStage::ActiveLifecycleAllocation
                    | QuerySubscriptionDiagnosticStage::ActivationReadiness
            ) {
                Some(failure.source_digest())
            } else {
                None
            },
            "diagnostic bundle assembly requires the trace to carry every stage that the assembled artifacts claim",
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
        )?;
    }

    validate_optional_trace_stage_source(
        trace,
        QuerySubscriptionDiagnosticStage::SupportReporting,
        if *failure.stage() == QuerySubscriptionDiagnosticStage::SupportReporting {
            Some(failure.source_digest())
        } else {
            support.map(|value| value.report_digest())
        },
        "denied diagnostic bundle assembly may only carry support-reporting trace evidence when the supplied support report is present",
        QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
    )?;
    Ok(())
}

fn validate_trace_stage_source(
    trace: &QuerySubscriptionDiagnosticTrace,
    stage: QuerySubscriptionDiagnosticStage,
    expected_source_digest: &str,
    message: &'static str,
    error_kind: QuerySubscriptionDiagnosticBundleErrorKind,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    let stage_trace = trace_stage(trace, stage).ok_or_else(|| {
        QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
            "diagnostic bundle assembly requires the trace to carry every stage that the assembled artifacts claim",
            &[
                format!("trace_terminal_stage:{}", trace.terminal_stage().as_str()),
                format!("missing_stage:{}", stage.as_str()),
            ],
        )
    })?;

    if stage_trace.source_digest() != expected_source_digest {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            error_kind,
            message,
            &[
                format!("stage:{}", stage.as_str()),
                format!("trace_source:{}", stage_trace.source_digest()),
                format!("expected_source:{expected_source_digest}"),
            ],
        ));
    }

    Ok(())
}

fn validate_optional_trace_stage_source(
    trace: &QuerySubscriptionDiagnosticTrace,
    stage: QuerySubscriptionDiagnosticStage,
    expected_source_digest: Option<&str>,
    message: &'static str,
    error_kind: QuerySubscriptionDiagnosticBundleErrorKind,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    match (trace_stage(trace, stage), expected_source_digest) {
        (Some(stage_trace), Some(expected_source_digest)) => {
            if stage_trace.source_digest() != expected_source_digest {
                return Err(QuerySubscriptionDiagnosticBundleError::new(
                    error_kind,
                    message,
                    &[
                        format!("stage:{}", stage.as_str()),
                        format!("trace_source:{}", stage_trace.source_digest()),
                        format!("expected_source:{expected_source_digest}"),
                    ],
                ));
            }
        }
        (Some(_), None) => {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
                message,
                &[format!("unexpected_stage:{}", stage.as_str())],
            ));
        }
        (None, Some(_)) => {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::MissingRequiredStage,
                "diagnostic bundle assembly requires the trace to carry every optional stage claimed by the assembled artifacts",
                &[format!("missing_stage:{}", stage.as_str())],
            ));
        }
        (None, None) => {}
    }

    Ok(())
}

fn trace_stage(
    trace: &QuerySubscriptionDiagnosticTrace,
    stage: QuerySubscriptionDiagnosticStage,
) -> Option<&super::trace::QuerySubscriptionDiagnosticStageTrace> {
    trace
        .stage_traces()
        .iter()
        .find(|stage_trace| stage_trace.stage() == &stage)
}

fn semantic_labels_for_support(
    query_family_label: &str,
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    support_posture: &QuerySubscriptionSupportPosture,
    denial_or_coverage_class_label: &str,
) -> QuerySubscriptionDiagnosticSemanticLabels {
    QuerySubscriptionDiagnosticSemanticLabels::new(
        query_family_label.to_string(),
        declaration.family().as_str().to_string(),
        lowering.bridge_family().as_str().to_string(),
        lowering
            .bridge_slices()
            .iter()
            .map(|slice| slice.as_str().to_string())
            .collect(),
        declaration.basis_posture().as_str().to_string(),
        lowering
            .signal_strategy_request()
            .request_kind()
            .as_str()
            .to_string(),
        support_posture.as_str().to_string(),
        denial_or_coverage_class_label.to_string(),
    )
}

fn semantic_labels_for_denied_bundle(
    selection: &QuerySubscriptionDiagnosticSelectionContext,
    declaration: Option<&QuerySubscriptionDeclarationArtifact>,
    lowering: Option<&BridgeSubscriptionLoweringPlan>,
    support: Option<&QuerySubscriptionSupportReport>,
    denial_class_label: &str,
) -> QuerySubscriptionDiagnosticSemanticLabels {
    QuerySubscriptionDiagnosticSemanticLabels::new(
        selection.query_family_label().to_string(),
        declaration
            .map(|value| value.family().as_str().to_string())
            .unwrap_or_else(|| selection.declaration_family_label().to_string()),
        lowering
            .map(|value| value.bridge_family().as_str().to_string())
            .unwrap_or_else(|| "not_lowered".to_string()),
        lowering
            .map(|value| {
                value
                    .bridge_slices()
                    .iter()
                    .map(|slice| slice.as_str().to_string())
                    .collect()
            })
            .unwrap_or_default(),
        declaration
            .map(|value| value.basis_posture().as_str().to_string())
            .unwrap_or_else(|| selection.basis_posture_label().to_string()),
        lowering
            .map(|value| {
                value
                    .signal_strategy_request()
                    .request_kind()
                    .as_str()
                    .to_string()
            })
            .unwrap_or_else(|| "not_lowered".to_string()),
        support
            .map(|value| value.support_posture().as_str().to_string())
            .unwrap_or_else(|| "not_reported".to_string()),
        denial_class_label.to_string(),
    )
}

fn omitted_stages_after_failure(
    failure_stage: QuerySubscriptionDiagnosticStage,
) -> Vec<QuerySubscriptionDiagnosticStage> {
    match failure_stage {
        QuerySubscriptionDiagnosticStage::FamilySelection
        | QuerySubscriptionDiagnosticStage::ViewMismatch
        | QuerySubscriptionDiagnosticStage::RelationshipProofDrift => vec![
            QuerySubscriptionDiagnosticStage::Declaration,
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            QuerySubscriptionDiagnosticStage::SupportReporting,
            QuerySubscriptionDiagnosticStage::Certification,
        ],
        QuerySubscriptionDiagnosticStage::Declaration
        | QuerySubscriptionDiagnosticStage::DeliveryIntent => vec![
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            QuerySubscriptionDiagnosticStage::SupportReporting,
            QuerySubscriptionDiagnosticStage::Certification,
        ],
        QuerySubscriptionDiagnosticStage::BridgeFamilyLowering
        | QuerySubscriptionDiagnosticStage::BridgeSliceLowering
        | QuerySubscriptionDiagnosticStage::BasisBinding => vec![
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            QuerySubscriptionDiagnosticStage::SupportReporting,
            QuerySubscriptionDiagnosticStage::Certification,
        ],
        QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission
        | QuerySubscriptionDiagnosticStage::AdmissionBudget
        | QuerySubscriptionDiagnosticStage::DurableReloadOverclaim
        | QuerySubscriptionDiagnosticStage::ActiveLifecycleAllocation
        | QuerySubscriptionDiagnosticStage::ActivationReadiness => vec![
            QuerySubscriptionDiagnosticStage::SupportReporting,
            QuerySubscriptionDiagnosticStage::Certification,
        ],
        QuerySubscriptionDiagnosticStage::SupportReporting => {
            vec![QuerySubscriptionDiagnosticStage::Certification]
        }
        _ => Vec::new(),
    }
}

fn semantic_label_count(labels: &QuerySubscriptionDiagnosticSemanticLabels) -> usize {
    7 + labels.bridge_slice_labels().len()
}

fn validate_denied_selection_context(
    selection_context: &QuerySubscriptionDiagnosticSelectionContext,
    failure_stage: &QuerySubscriptionDiagnosticStage,
    failure_source_digest: &str,
    carries_later_artifacts: bool,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if selection_context.is_selection_denied() {
        if !failure_is_selection_stage(*failure_stage) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic bundle assembly may only use a selection-denied context for family-selection failures",
                &[
                    format!("selection_context:{}", selection_context.digest()),
                    format!("failure_stage:{}", failure_stage.as_str()),
                ],
            ));
        }
        if selection_context.source_digest() != failure_source_digest {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic bundle assembly requires the selection-denied context and failure to bind the same canonical source digest",
                &[
                    format!("selection_source:{}", selection_context.source_digest()),
                    format!("failure_source:{failure_source_digest}"),
                ],
            ));
        }
        if carries_later_artifacts {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic bundle assembly may not attach declaration, lowering, or support artifacts after family-selection denial",
                &[
                    format!("selection_context:{}", selection_context.digest()),
                    format!("failure_stage:{}", failure_stage.as_str()),
                ],
            ));
        }
    }
    Ok(())
}

fn failure_is_selection_stage(stage: QuerySubscriptionDiagnosticStage) -> bool {
    matches!(
        stage,
        QuerySubscriptionDiagnosticStage::FamilySelection
            | QuerySubscriptionDiagnosticStage::ViewMismatch
            | QuerySubscriptionDiagnosticStage::RelationshipProofDrift
    )
}
