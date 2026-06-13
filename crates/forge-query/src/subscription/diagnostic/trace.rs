use crate::identity::hash_parts;

use super::super::admission::QuerySubscriptionAdmissionArtifact;
use super::super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::super::certification::SubscriptionLifecycleCertificationBundle;
use super::super::closeout::SubscriptionLifecycleCloseout;
use super::super::continuation::SubscriptionContinuationReport;
use super::super::declaration::QuerySubscriptionDeclarationArtifact;
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
    source_digest: String,
    evidence_digest: String,
    stage_trace_digest: String,
}

impl QuerySubscriptionDiagnosticStageTrace {
    fn from_evidence(evidence: &QuerySubscriptionDiagnosticEvidence) -> Self {
        let stage_trace_digest = hash_parts(&[
            "query_subscription_diagnostic_stage_trace_v1".to_string(),
            evidence.stage().as_str().to_string(),
            evidence.outcome().as_str().to_string(),
            format!("source:{}", evidence.source_digest()),
            format!("evidence:{}", evidence.digest()),
        ]);
        Self {
            stage: *evidence.stage(),
            outcome: *evidence.outcome(),
            source_digest: evidence.source_digest().to_string(),
            evidence_digest: evidence.digest().to_string(),
            stage_trace_digest,
        }
    }

    pub fn stage(&self) -> &QuerySubscriptionDiagnosticStage {
        &self.stage
    }

    pub fn outcome(&self) -> &QuerySubscriptionDiagnosticOutcome {
        &self.outcome
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn stage_trace_digest(&self) -> &str {
        &self.stage_trace_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticTrace {
    terminal_stage: QuerySubscriptionDiagnosticStage,
    stage_traces: Vec<QuerySubscriptionDiagnosticStageTrace>,
    counter_snapshot: String,
    trace_digest: String,
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
        &self.trace_digest
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
            selection.equivalence_basis().digest().as_str(),
            selection.counters().digest(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::Declaration,
            format!(
                "query subscription declaration {} preserved canonical family semantics",
                declaration.family().as_str()
            ),
            declaration.declaration_digest().as_str(),
            declaration.counters().digest(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::BridgeFamilyLowering,
            format!(
                "bridge lowering admitted family {} for canonical query declaration",
                lowering.bridge_family().as_str()
            ),
            lowering.bridge_declaration_for_reporting(),
            lowering.counters().digest(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission,
            "runtime-backed subscription admission preserved declaration, bridge, basis, and signal identity",
            admission.admission_for_reporting(),
            admission.counters().digest(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::SupportReporting,
            format!(
                "support reporting certified {} for subject class {}",
                support.support_posture().as_str(),
                support.support_subject().support_class().as_str()
            ),
            support.report_digest(),
            support.counter_snapshot(),
        ),
        QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::Certification,
            "subscription lifecycle certification closed the admitted runtime-backed proof chain",
            lifecycle.certification_bundle_for_reporting(),
            lifecycle.counter_snapshot(),
        ),
    ];

    if let Some(continuation) = continuation {
        stage_evidence.push(QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::Continuation,
            "subscription continuation report preserved canonical lane identity",
            continuation.report_digest(),
            continuation.continuation_digest(),
        ));
    }

    if let Some(preview) = preview {
        stage_evidence.push(QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::PreviewIsolation,
            "preview isolation artifact remained distinct from authoritative lifecycle state",
            preview.isolation_digest(),
            preview.counters().digest(),
        ));
    }

    if let Some(closeout) = closeout {
        stage_evidence.push(QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::LifecycleCloseout,
            format!(
                "lifecycle closeout {} preserved terminal runtime evidence",
                closeout.closeout_kind().as_str()
            ),
            closeout.closeout_digest(),
            closeout.counters().digest(),
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
        failure.source_digest(),
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
        if declaration.declaration_digest().as_str() != lowering.query_declaration_for_reporting() {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
                "diagnostic trace assembly requires bridge lowering to bind the same declaration artifact",
                &[
                    format!("declaration:{}", declaration.declaration_digest().as_str()),
                    format!("lowering:{}", lowering.query_declaration_for_reporting()),
                ],
            ));
        }
    }
    if let (Some(declaration), Some(admission)) = (declaration, admission) {
        if declaration.declaration_digest().as_str() != admission.query_declaration_for_reporting() {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
                "diagnostic trace assembly requires admission to preserve declaration identity",
                &[
                    format!("declaration:{}", declaration.declaration_digest().as_str()),
                    format!("admission:{}", admission.query_declaration_for_reporting()),
                ],
            ));
        }
    }
    if let (Some(declaration), Some(support)) = (declaration, support) {
        if declaration.declaration_digest().as_str()
            != support.support_subject().declaration_digest()
        {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
                "diagnostic trace assembly requires support reporting to preserve declaration identity",
                &[
                    format!("declaration:{}", declaration.declaration_digest().as_str()),
                    format!(
                        "support_declaration:{}",
                        support.support_subject().declaration_digest()
                    ),
                ],
            ));
        }
    }

    let mut stage_evidence = if let Some(selection) = selection_context.selection() {
        vec![QuerySubscriptionDiagnosticEvidence::admitted(
            QuerySubscriptionDiagnosticStage::FamilySelection,
            format!(
                "query subscription family {} selected through canonical admission classification",
                selection.family().as_str()
            ),
            selection.equivalence_basis().digest().as_str(),
            selection.counters().digest(),
        )]
    } else {
        vec![stage_evidence_from_state(
            *failure.stage(),
            false,
            failure.reason().to_string(),
            failure.source_digest().to_string(),
            failure.counter_digest().to_string(),
        )]
    };

    if selection_context.selection().is_some() {
        if let Some(declaration) = declaration {
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
                    declaration.declaration_digest().as_str().to_string()
                } else {
                    failure.source_digest().to_string()
                },
                if admitted {
                    declaration.counters().digest()
                } else {
                    failure.counter_digest().to_string()
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
                failure.source_digest().to_string(),
                failure.counter_digest().to_string(),
            ));
        }
    }

    if let Some(lowering) = lowering {
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
                failure.source_digest().to_string()
            } else {
                lowering.bridge_declaration_for_reporting().to_string()
            },
            if bridge_failure {
                failure.counter_digest().to_string()
            } else {
                lowering.counters().digest()
            },
        ));
    }

    if let Some(admission) = admission {
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
                failure.source_digest().to_string()
            } else {
                admission.admission_for_reporting().to_string()
            },
            if admission_failure {
                failure.counter_digest().to_string()
            } else {
                admission.counters().digest()
            },
        ));
    }

    if let Some(support) = support {
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
                failure.source_digest().to_string()
            } else {
                support.report_digest().to_string()
            },
            if support_failure {
                failure.counter_digest().to_string()
            } else {
                support.counter_snapshot().to_string()
            },
        ));
    } else if *failure.stage() == QuerySubscriptionDiagnosticStage::SupportReporting {
        stage_evidence.push(stage_evidence_from_state(
            QuerySubscriptionDiagnosticStage::SupportReporting,
            false,
            failure.reason().to_string(),
            failure.source_digest().to_string(),
            failure.counter_digest().to_string(),
        ));
    }

    if *failure.stage() == QuerySubscriptionDiagnosticStage::Certification {
        stage_evidence.push(stage_evidence_from_state(
            QuerySubscriptionDiagnosticStage::Certification,
            false,
            failure.reason().to_string(),
            failure.source_digest().to_string(),
            failure.counter_digest().to_string(),
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
    if declaration.declaration_digest().as_str() != lowering.query_declaration_for_reporting() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::BridgeLoweringSourceMismatch,
            "admitted diagnostic trace requires bridge lowering to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_digest().as_str()),
                format!("lowering:{}", lowering.query_declaration_for_reporting()),
            ],
        ));
    }
    if declaration.declaration_digest().as_str() != admission.query_declaration_for_reporting() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::AdmissionSourceMismatch,
            "admitted diagnostic trace requires admission to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_digest().as_str()),
                format!("admission:{}", admission.query_declaration_for_reporting()),
            ],
        ));
    }
    if declaration.declaration_digest().as_str() != support.support_subject().declaration_digest() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::SupportSourceMismatch,
            "admitted diagnostic trace requires support reporting to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_digest().as_str()),
                format!(
                    "support_declaration:{}",
                    support.support_subject().declaration_digest()
                ),
            ],
        ));
    }
    if lifecycle.query_declaration_for_reporting() != declaration.declaration_digest().as_str() {
        return Err(QuerySubscriptionDiagnosticBundleError::new(
            QuerySubscriptionDiagnosticBundleErrorKind::LifecycleSourceMismatch,
            "admitted diagnostic trace requires lifecycle certification to preserve declaration identity",
            &[
                format!("declaration:{}", declaration.declaration_digest().as_str()),
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
    failure_source_digest: &str,
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
        if selection_context.source_digest() != failure_source_digest {
            return Err(QuerySubscriptionDiagnosticBundleError::new(
                QuerySubscriptionDiagnosticBundleErrorKind::SelectionContextMismatch,
                "diagnostic trace assembly requires the selection-denied context and failure to bind the same canonical source digest",
                &[
                    format!("selection_source:{}", selection_context.source_digest()),
                    format!("failure_source:{failure_source_digest}"),
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
    source_digest: String,
    counter_digest: String,
) -> QuerySubscriptionDiagnosticEvidence {
    if admitted {
        QuerySubscriptionDiagnosticEvidence::admitted(stage, reason, source_digest, counter_digest)
    } else {
        QuerySubscriptionDiagnosticEvidence::denied(stage, reason, source_digest, counter_digest)
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
    let trace_digest = hash_parts(&[
        "query_subscription_diagnostic_trace_v1".to_string(),
        format!("terminal_stage:{}", terminal_stage.as_str()),
        format!("counters:{counter_snapshot}"),
        format!(
            "stages:{}",
            stage_traces
                .iter()
                .map(|trace| trace.stage_trace_digest())
                .collect::<Vec<_>>()
                .join("|")
        ),
    ]);
    QuerySubscriptionDiagnosticTrace {
        terminal_stage,
        stage_traces,
        counter_snapshot,
        trace_digest,
        counters,
    }
}
