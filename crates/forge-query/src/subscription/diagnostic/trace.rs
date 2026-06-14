use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::certification::SubscriptionLifecycleCertificationBundle;
use super::super::closeout::SubscriptionLifecycleCloseout;
use super::super::continuation::SubscriptionContinuationReport;
use super::super::declaration::QuerySubscriptionDeclarationArtifact;
use super::super::evidence_identities::{
    diagnostic_stage_trace_identity, diagnostic_trace_identity, typed_identity_drift,
};
use super::super::preview_isolation::PreviewSubscriptionIsolationArtifact;
use super::super::support::QuerySubscriptionSupportReport;
use super::bundle::{
    QuerySubscriptionDiagnosticBundleError, QuerySubscriptionDiagnosticBundleErrorKind,
    QuerySubscriptionDiagnosticCounters, QuerySubscriptionDiagnosticFailure,
};
use super::context::QuerySubscriptionDiagnosticSelectionContext;
use super::stage::{
    QuerySubscriptionDiagnosticEvidence, QuerySubscriptionDiagnosticOutcome,
    QuerySubscriptionDiagnosticStage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticStageTrace {
    stage: QuerySubscriptionDiagnosticStage,
    outcome: QuerySubscriptionDiagnosticOutcome,
    source_for_reporting: String,
    evidence_for_reporting: String,
    stage_trace_identity: ForgeQueryEvidenceIdentity,
    stage_trace_for_reporting: String,
}

impl QuerySubscriptionDiagnosticStageTrace {
    fn from_evidence(evidence: &QuerySubscriptionDiagnosticEvidence) -> Self {
        let source_identity = ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_diagnostic_stage_source_projection_v1",
        )
        .field_shape(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("source"),
            evidence.source_digest(),
        )
        .seal();
        let stage_trace_identity = diagnostic_stage_trace_identity(
            evidence.stage().as_str(),
            evidence.outcome().as_str(),
            &source_identity,
            evidence.evidence_identity(),
        );
        Self {
            stage: *evidence.stage(),
            outcome: *evidence.outcome(),
            source_for_reporting: evidence.source_digest().to_string(),
            evidence_for_reporting: evidence.evidence_for_reporting().to_string(),
            stage_trace_for_reporting: stage_trace_identity.as_str().to_string(),
            stage_trace_identity,
        }
    }

    pub fn stage(&self) -> &QuerySubscriptionDiagnosticStage {
        &self.stage
    }

    pub fn outcome(&self) -> &QuerySubscriptionDiagnosticOutcome {
        &self.outcome
    }

    pub fn source_digest(&self) -> &str {
        &self.source_for_reporting
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_for_reporting
    }

    pub fn stage_trace_digest(&self) -> &str {
        self.stage_trace_for_reporting()
    }

    pub fn stage_trace_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.stage_trace_identity
    }

    pub fn stage_trace_for_reporting(&self) -> &str {
        &self.stage_trace_for_reporting
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticTrace {
    terminal_stage: QuerySubscriptionDiagnosticStage,
    stage_traces: Vec<QuerySubscriptionDiagnosticStageTrace>,
    counter_snapshot: String,
    trace_identity: ForgeQueryEvidenceIdentity,
    trace_for_reporting: String,
    counters: QuerySubscriptionDiagnosticCounters,
}

impl QuerySubscriptionDiagnosticTrace {
    pub fn terminal_stage(&self) -> &QuerySubscriptionDiagnosticStage {
        &self.terminal_stage
    }

    pub fn stage_traces(&self) -> &[QuerySubscriptionDiagnosticStageTrace] {
        &self.stage_traces
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn trace_digest(&self) -> &str {
        self.trace_for_reporting()
    }

    pub fn trace_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.trace_identity
    }

    pub fn trace_for_reporting(&self) -> &str {
        &self.trace_for_reporting
    }

    pub fn counters(&self) -> &QuerySubscriptionDiagnosticCounters {
        &self.counters
    }
}

#[allow(clippy::too_many_arguments)]
pub fn trace_admitted_query_subscription_diagnostics(
    selection: &super::super::selection::QuerySubscriptionFamilySelection,
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    admission: &QuerySubscriptionAdmissionArtifact,
    support: &QuerySubscriptionSupportReport,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
    continuation: Option<&SubscriptionContinuationReport>,
    preview: Option<&PreviewSubscriptionIsolationArtifact>,
    closeout: Option<&SubscriptionLifecycleCloseout>,
) -> Result<QuerySubscriptionDiagnosticTrace, QuerySubscriptionDiagnosticBundleError> {
    validate_admitted_sources(
        selection,
        declaration,
        lowering,
        admission,
        support,
        lifecycle,
    )?;

    let mut stage_evidence = vec![
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::FamilySelection,
            format!(
                "query subscription family {} selected through canonical admission classification",
                selection.family().as_str()
            ),
            selection.equivalence_basis().evidence_identity(),
            &selection.counters().evidence_identity(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::Declaration,
            format!(
                "query subscription declaration {} preserved canonical family semantics",
                declaration.family().as_str()
            ),
            declaration.declaration_identity(),
            &declaration.counters().evidence_identity(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            format!(
                "bridge lowering admitted family {} for canonical query declaration",
                lowering.bridge_family().as_str()
            ),
            lowering.bridge_declaration_identity(),
            &lowering.counters().evidence_identity(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            "runtime-backed subscription admission preserved declaration, bridge, basis, and signal identity",
            admission.evidence_identity(),
            &admission.counters().evidence_identity(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::SupportReporting,
            format!(
                "support reporting certified {} for subject class {}",
                support.support_posture().as_str(),
                support.support_subject().support_class().as_str()
            ),
            support.report_identity(),
            &support.counters().evidence_identity(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::Certification,
            "subscription lifecycle certification closed the admitted runtime-backed proof chain",
            lifecycle.certification_bundle_identity(),
            lifecycle.counter_sequence_identity(),
        ),
    ];

    if let Some(continuation) = continuation {
        stage_evidence.push(QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::Continuation,
            "subscription continuation report preserved canonical lane identity",
            continuation.evidence_identity(),
            continuation.performance_receipt().performance_receipt_identity(),
        ));
    }

    if let Some(preview) = preview {
        stage_evidence.push(QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::PreviewIsolation,
            "preview isolation artifact remained distinct from authoritative lifecycle state",
            preview.isolation_identity(),
            &preview.counters().evidence_identity(),
        ));
    }

    if let Some(closeout) = closeout {
        stage_evidence.push(QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::LifecycleCloseout,
            format!(
                "lifecycle closeout {} preserved terminal runtime evidence",
                closeout.closeout_kind().as_str()
            ),
            closeout.evidence_identity(),
            &closeout.counters().evidence_identity(),
        ));
    }

    Ok(trace_from_stage_evidence(stage_evidence))
}

pub fn trace_denied_query_subscription_diagnostics(
    selection_context: &QuerySubscriptionDiagnosticSelectionContext,
    declaration: Option<&QuerySubscriptionDeclarationArtifact>,
    lowering: Option<&BridgeSubscriptionLoweringPlan>,
    admission: Option<&QuerySubscriptionAdmissionArtifact>,
    support: Option<&QuerySubscriptionSupportReport>,
    failure: QuerySubscriptionDiagnosticFailure,
) -> Result<QuerySubscriptionDiagnosticTrace, QuerySubscriptionDiagnosticBundleError> {
    validate_denied_selection_context(
        selection_context,
        failure.stage(),
        &failure,
        declaration.is_some() || lowering.is_some() || admission.is_some() || support.is_some(),
    )?;

    if selection_context.selection().is_some() {
        if failure_is_selection_stage(*failure.stage()) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic trace assembly may not bind an admitted family selection context to a family-selection denial",
                &[
                    format!("selection_context:{}", selection_context.digest()),
                    format!("failure_stage:{}", failure.stage().as_str()),
                ],
            ));
        }
    }

    if let Some(declaration) = declaration {
        if selection_context
            .selection()
            .map(|selection| selection.family())
            != Some(declaration.family())
        {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
                "diagnostic trace assembly requires declaration and family selection to preserve the same query subscription family",
                &[
                    format!(
                        "selection_family:{}",
                        selection_context.query_family_label()
                    ),
                    format!("declaration_family:{}", declaration.family().as_str()),
                ],
            ));
        }
    }
    if let (Some(declaration), Some(lowering)) = (declaration, lowering) {
        if typed_identity_drift(
            declaration.declaration_identity(),
            lowering.query_declaration_identity(),
        ) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
                "diagnostic trace assembly requires bridge lowering to bind the same declaration artifact",
                &[
                    format!("declaration:{}", declaration.declaration_for_reporting()),
                    format!("lowering:{}", lowering.query_declaration_for_reporting()),
                ],
            ));
        }
    }
    if let (Some(declaration), Some(admission)) = (declaration, admission) {
        if typed_identity_drift(
            declaration.declaration_identity(),
            admission.query_declaration_identity(),
        ) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
                "diagnostic trace assembly requires admission to preserve declaration identity",
                &[
                    format!("declaration:{}", declaration.declaration_for_reporting()),
                    format!("admission:{}", admission.query_declaration_for_reporting()),
                ],
            ));
        }
    }
    if let (Some(declaration), Some(support)) = (declaration, support) {
        if typed_identity_drift(
            declaration.declaration_identity(),
            support.support_subject().declaration_identity(),
        ) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
                "diagnostic trace assembly requires support reporting to preserve declaration identity",
                &[
                    format!("declaration:{}", declaration.declaration_for_reporting()),
                    format!(
                        "support_declaration:{}",
                        support.support_subject().declaration_digest()
                    ),
                ],
            ));
        }
    }

    let failure_source_identity = failure.source_identity();
    let failure_counter_identity = failure.counter_identity();

    let mut stage_evidence = if let Some(selection) = selection_context.selection() {
        vec![QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::FamilySelection,
            format!(
                "query subscription family {} selected through canonical admission classification",
                selection.family().as_str()
            ),
            selection.equivalence_basis().evidence_identity(),
            &selection.counters().evidence_identity(),
        )]
    } else {
        vec![stage_evidence_from_state(
            *failure.stage(),
            false,
            failure.reason().to_string(),
            failure_source_identity,
            failure_counter_identity,
        )]
    };

    if selection_context.selection().is_some() {
        if let Some(declaration) = declaration {
            let declaration_counters = declaration.counters().evidence_identity();
            let stage = if *failure.stage() == QuerySubscriptionDiagnosticStage::Declaration
                || *failure.stage() == QuerySubscriptionDiagnosticStage::DeliveryIntent
            {
                *failure.stage()
            } else {
                QuerySubscriptionDiagnosticStage::Declaration
            };
            let admitted = stage != *failure.stage();
            stage_evidence.push(stage_evidence_from_state(
                stage,
                admitted,
                if admitted {
                    format!(
                        "query subscription declaration {} preserved canonical family semantics",
                        declaration.family().as_str()
                    )
                } else {
                    failure.reason().to_string()
                },
                if admitted {
                    declaration.declaration_identity()
                } else {
                    failure_source_identity
                },
                if admitted {
                    &declaration_counters
                } else {
                    failure_counter_identity
                },
            ));
        } else if matches!(
            failure.stage(),
            QuerySubscriptionDiagnosticStage::Declaration
                | QuerySubscriptionDiagnosticStage::DeliveryIntent
        ) {
            stage_evidence.push(stage_evidence_from_state(
                *failure.stage(),
                false,
                failure.reason().to_string(),
                failure_source_identity,
                failure_counter_identity,
            ));
        }
    }

    if let Some(lowering) = lowering {
        let lowering_counters = lowering.counters().evidence_identity();
        let bridge_failure = matches!(
            failure.stage(),
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering
                | QuerySubscriptionDiagnosticStage::BridgeSliceLowering
                | QuerySubscriptionDiagnosticStage::BasisBinding
        );
        stage_evidence.push(stage_evidence_from_state(
            if bridge_failure {
                *failure.stage()
            } else {
                QuerySubscriptionDiagnosticStage::BridgeFamilyLowering
            },
            !bridge_failure,
            if bridge_failure {
                failure.reason().to_string()
            } else {
                format!(
                    "bridge lowering admitted family {} for canonical query declaration",
                    lowering.bridge_family().as_str()
                )
            },
            if bridge_failure {
                failure_source_identity
            } else {
                lowering.bridge_declaration_identity()
            },
            if bridge_failure {
                failure_counter_identity
            } else {
                &lowering_counters
            },
        ));
    }

    if let Some(admission) = admission {
        let admission_counters = admission.counters().evidence_identity();
        let admission_failure = matches!(
            failure.stage(),
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission
                | QuerySubscriptionDiagnosticStage::AdmissionBudget
                | QuerySubscriptionDiagnosticStage::DurableReloadOverclaim
                | QuerySubscriptionDiagnosticStage::ActiveLifecycleAllocation
                | QuerySubscriptionDiagnosticStage::ActivationReadiness
        );
        stage_evidence.push(stage_evidence_from_state(
            if admission_failure {
                *failure.stage()
            } else {
                QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission
            },
            !admission_failure,
            if admission_failure {
                failure.reason().to_string()
            } else {
                "runtime-backed subscription admission preserved declaration, bridge, basis, and signal identity".to_string()
            },
            if admission_failure {
                failure_source_identity
            } else {
                admission.evidence_identity()
            },
            if admission_failure {
                failure_counter_identity
            } else {
                &admission_counters
            },
        ));
    }

    if let Some(support) = support {
        let support_counters = support.counters().evidence_identity();
        let support_failure =
            *failure.stage() == QuerySubscriptionDiagnosticStage::SupportReporting;
        stage_evidence.push(stage_evidence_from_state(
            QuerySubscriptionDiagnosticStage::SupportReporting,
            !support_failure,
            if support_failure {
                failure.reason().to_string()
            } else {
                format!(
                    "support reporting certified {} for subject class {}",
                    support.support_posture().as_str(),
                    support.support_subject().support_class().as_str()
                )
            },
            if support_failure {
                failure_source_identity
            } else {
                support.report_identity()
            },
            if support_failure {
                failure_counter_identity
            } else {
                &support_counters
            },
        ));
    } else if *failure.stage() == QuerySubscriptionDiagnosticStage::SupportReporting {
        stage_evidence.push(stage_evidence_from_state(
            QuerySubscriptionDiagnosticStage::SupportReporting,
            false,
            failure.reason().to_string(),
            failure_source_identity,
            failure_counter_identity,
        ));
    }

    if *failure.stage() == QuerySubscriptionDiagnosticStage::Certification {
        stage_evidence.push(stage_evidence_from_state(
            QuerySubscriptionDiagnosticStage::Certification,
            false,
            failure.reason().to_string(),
            failure_source_identity,
            failure_counter_identity,
        ));
    }

    Ok(trace_from_stage_evidence(stage_evidence))
}

fn validate_admitted_sources(
    selection: &super::super::selection::QuerySubscriptionFamilySelection,
    declaration: &QuerySubscriptionDeclarationArtifact,
    lowering: &BridgeSubscriptionLoweringPlan,
    admission: &QuerySubscriptionAdmissionArtifact,
    support: &QuerySubscriptionSupportReport,
    lifecycle: &SubscriptionLifecycleCertificationBundle,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if selection.family() != declaration.family() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::DeclarationSourceMismatch,
            "admitted diagnostic trace requires declaration and family selection to preserve the same query subscription family",
            &[
                format!("selection_family:{}", selection.family().as_str()),
                format!("declaration_family:{}", declaration.family().as_str()),
            ],
        ));
    }
    if typed_identity_drift(
        declaration.declaration_identity(),
        lowering.query_declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
            "admitted diagnostic trace requires bridge lowering to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_for_reporting()),
                format!("lowering:{}", lowering.query_declaration_for_reporting()),
            ],
        ));
    }
    if typed_identity_drift(
        declaration.declaration_identity(),
        admission.query_declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
            "admitted diagnostic trace requires admission to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_for_reporting()),
                format!("admission:{}", admission.query_declaration_for_reporting()),
            ],
        ));
    }
    if typed_identity_drift(
        declaration.declaration_identity(),
        support.support_subject().declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
            "admitted diagnostic trace requires support reporting to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_for_reporting()),
                format!(
                    "support_declaration:{}",
                    support.support_subject().declaration_digest()
                ),
            ],
        ));
    }
    if typed_identity_drift(
        declaration.declaration_identity(),
        lifecycle.subscription_declaration_identity(),
    ) {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::LifecycleSourceMismatch,
            "admitted diagnostic trace requires lifecycle certification to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_for_reporting()),
                format!(
                    "lifecycle_declaration:{}",
                    lifecycle.query_declaration_for_reporting()
                ),
            ],
        ));
    }
    Ok(())
}

fn validate_denied_selection_context(
    selection_context: &QuerySubscriptionDiagnosticSelectionContext,
    failure_stage: &QuerySubscriptionDiagnosticStage,
    failure: &QuerySubscriptionDiagnosticFailure,
    carries_later_artifacts: bool,
) -> Result<(), QuerySubscriptionDiagnosticBundleError> {
    if selection_context.is_selection_denied() {
        if !failure_is_selection_stage(*failure_stage) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic trace assembly may only use a selection-denied context for family-selection failures",
                &[
                    format!("selection_context:{}", selection_context.digest()),
                    format!("failure_stage:{}", failure_stage.as_str()),
                ],
            ));
        }
        if typed_identity_drift(
            &selection_context.source_identity(),
            failure.source_identity(),
        ) {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic trace assembly requires the selection-denied context and failure to bind the same canonical source digest",
                &[
                    format!("selection_source:{}", selection_context.source_digest()),
                    format!("failure_source:{}", failure.source_digest()),
                ],
            ));
        }
        if carries_later_artifacts {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic trace assembly may not attach declaration, lowering, admission, or support artifacts after family-selection denial",
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

fn stage_evidence_from_state(
    stage: QuerySubscriptionDiagnosticStage,
    admitted: bool,
    reason: String,
    source_identity: &ForgeQueryEvidenceIdentity,
    counter_identity: &ForgeQueryEvidenceIdentity,
) -> QuerySubscriptionDiagnosticEvidence {
    if admitted {
        QuerySubscriptionDiagnosticEvidence::admitted(stage, reason, source_identity, counter_identity)
    } else {
        QuerySubscriptionDiagnosticEvidence::denied(stage, reason, source_identity, counter_identity)
    }
}

fn trace_from_stage_evidence(
    stage_evidence: Vec<QuerySubscriptionDiagnosticEvidence>,
) -> QuerySubscriptionDiagnosticTrace {
    let terminal_stage = *stage_evidence
        .last()
        .map(|evidence| evidence.stage())
        .expect("diagnostic trace requires at least one stage");
    let stage_traces = stage_evidence
        .iter()
        .map(QuerySubscriptionDiagnosticStageTrace::from_evidence)
        .collect::<Vec<_>>();
    let counters = QuerySubscriptionDiagnosticCounters::trace_emitted(stage_traces.len() as u64);
    let counter_snapshot = counters.digest();
    let stage_trace_refs: Vec<&ForgeQueryEvidenceIdentity> = stage_traces
        .iter()
        .map(|trace| trace.stage_trace_identity())
        .collect();
    let trace_identity = diagnostic_trace_identity(
        terminal_stage.as_str(),
        &counters.evidence_identity(),
        stage_trace_refs,
    );
    QuerySubscriptionDiagnosticTrace {
        terminal_stage,
        stage_traces,
        counter_snapshot,
        trace_for_reporting: trace_identity.as_str().to_string(),
        trace_identity,
        counters,
    }
}
