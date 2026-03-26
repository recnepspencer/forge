use crate::capabilities::{SchemaSource, SchemaVersionSource};
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::replay::data::CanonicalCommitEnvelope;
use crate::schema::data::{
    DescriptorSemanticsVersion, FreeFormSchemaDiffIntent, LoweredSchemaTransitionPlan,
    SchemaContinuationDescriptor, SchemaDiffDetail, SchemaReconciliationDescriptor, SchemaStratum,
    SchemaTransitionArtifact, SchemaTransitionSummary,
};
use crate::schema::logic::{
    lower_schema_transition, validate_schema_continuity_bundle, validate_schema_transition,
    SchemaContinuityBundleIssue,
};
use crate::transactions::data::{CommitConflict, ConflictClass, TransactionCommitError};
use serde::Serialize;
use serde_json::{to_value, Value};

#[derive(Debug, Clone)]
pub(crate) struct SchemaContinuityPlan {
    pub(crate) descriptor_semantics_version: DescriptorSemanticsVersion,
    pub(crate) schema_transition: Option<SchemaTransitionArtifact>,
    pub(crate) schema_continuation_descriptor: Option<SchemaContinuationDescriptor>,
    pub(crate) schema_reconciliation_descriptor: Option<SchemaReconciliationDescriptor>,
}

enum FailureTransitionView<'a> {
    Proposed(&'a crate::schema::data::ProposedSchemaTransition),
    Artifact(&'a SchemaTransitionArtifact),
}

#[derive(Debug, Serialize)]
struct SchemaTransitionSummaryFields {
    branch_id: String,
    source_schema_id: String,
    source_schema_version_id: u32,
    target_schema_id: String,
    target_schema_version_id: u32,
    changed_atom_count: usize,
    changed_strata: Vec<String>,
    historical_interpretation: String,
    continuation: String,
    bridgeability: String,
    reconciliation: String,
    descriptor_semantics_version: u32,
    descriptor_canonicalization_version: u32,
    normalized_boundary_count: usize,
}

#[derive(Debug, Serialize)]
struct SchemaContinuityFailureFields {
    branch_id: String,
    conflict_class: String,
    detail: String,
    conflict_fields: Option<Value>,
    previous_schema_version: Option<u32>,
    previous_descriptor_semantics_version: Option<u32>,
}

#[derive(Debug, Serialize)]
struct SchemaTransitionRejectedFields {
    source_schema_id: String,
    source_schema_version_id: u32,
    target_schema_id: String,
    target_schema_version_id: u32,
    changed_atom_count: usize,
}

#[derive(Debug, Serialize)]
struct SchemaDiffAtomTraceFields {
    diff_atom_index: usize,
    element_kind: String,
    schema_id: String,
    schema_version_id: u32,
    kind_id: Option<u32>,
    element_name: String,
    strata: Vec<String>,
    publication_impact: String,
    subscriber_impact: String,
    historical_interpretation: String,
    detail: SchemaDiffDetailFields,
}

#[derive(Debug, Serialize)]
struct SchemaBridgeDescriptorFields {
    boundary_fingerprint: String,
    continuation: String,
    bridgeability: String,
    normalized_boundary_count: usize,
    descriptor_canonicalization_version: u32,
}

#[derive(Debug, Serialize)]
struct SchemaInterpretationFields {
    boundary_fingerprint: String,
    historical_interpretation: String,
    changed_strata: Vec<String>,
}

#[derive(Debug, Serialize)]
struct SchemaReconciliationFields {
    classification: String,
    policy: String,
    resulting_schema_id: String,
    resulting_schema_version_id: u32,
}

#[derive(Debug, Serialize)]
struct SchemaDescriptorVersionFields {
    descriptor_semantics_version: u32,
    continuation_canonicalization_version: u32,
    reconciliation_canonicalization_version: u32,
}

#[derive(Debug, Serialize)]
struct SchemaTransitionClassificationFields {
    branch_id: String,
    boundary_fingerprint: String,
    continuation: String,
    bridgeability: String,
    historical_interpretation: String,
    changed_strata: Vec<String>,
    reconciliation: String,
    policy: String,
}

#[derive(Debug, Serialize)]
struct SchemaLineageFields {
    resulting_schema_id: String,
    resulting_schema_version_id: u32,
    parent_schema_ids: Vec<String>,
    parent_schema_version_ids: Vec<u32>,
    ordering_mode: String,
    ordering_semantics: String,
    branch_context: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
enum SchemaDiffDetailFields {
    AddedField {
        field_name: String,
        required: bool,
        default_expression: Option<String>,
    },
    RemovedField {
        field_name: String,
    },
    TypeChanged {
        field_name: String,
        from_type: String,
        to_type: String,
    },
    EnumDomainExpanded {
        field_name: String,
        added_variants: Vec<String>,
    },
    InvariantContractChanged {
        contract_name: String,
    },
    ProjectionContractChanged {
        projection_name: String,
    },
    SubscriberContractChanged {
        contract_name: String,
    },
    FreeText {
        detail: String,
        declared_intent: String,
    },
}

impl SchemaContinuityPlan {
    pub(crate) fn current(descriptor_semantics_version: DescriptorSemanticsVersion) -> Self {
        Self {
            descriptor_semantics_version,
            schema_transition: None,
            schema_continuation_descriptor: None,
            schema_reconciliation_descriptor: None,
        }
    }
}

pub(crate) fn resolve_schema_continuity(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
    options: &crate::transactions::data::TransactionOptions,
) -> Result<SchemaContinuityPlan, TransactionCommitError> {
    let descriptor_policy = runtime.config.schema.descriptor_semantics_policy.clone();
    let current_descriptor_semantics_version = descriptor_policy.current_write_version();
    let current_descriptor_canonicalization_version = runtime
        .config
        .schema
        .descriptor_canonicalization_policy
        .current_write_version();
    let current_schema_version = runtime.primary_schema_version_id();
    let current_schema_registry = runtime.schema_registry().clone();
    let current_schema_basis = current_schema_registry
        .authoritative_schema_basis()
        .map_err(|error| {
            schema_continuity_conflict(
                runtime,
                branch_id,
                options.proposed_schema_transition.as_ref(),
                None,
                ConflictClass::InvalidSchemaTransitionShape {
                    detail: error.detail,
                },
            )
        })?;
    let previous_head = {
        let history = runtime.history_access();
        history.branch_head(branch_id).cloned()
    };
    let Some(previous_head) = previous_head else {
        if let Some(proposed_transition) = &options.proposed_schema_transition {
            return materialize_declared_transition(
                runtime,
                proposed_transition.clone(),
                options.schema_reconciliation_policy,
                current_descriptor_semantics_version,
                current_descriptor_canonicalization_version,
                branch_id,
                None,
                current_schema_basis,
                current_schema_version,
            );
        }
        return Ok(SchemaContinuityPlan::current(
            current_descriptor_semantics_version,
        ));
    };
    let previous_envelope = {
        let history = runtime.history_access();
        history.commit_envelope(previous_head.commit_id).cloned()
    };
    let Some(previous_envelope) = previous_envelope.as_ref() else {
        return Ok(SchemaContinuityPlan::current(
            current_descriptor_semantics_version,
        ));
    };

    if !descriptor_policy.supports(previous_envelope.descriptor_semantics_version) {
        runtime
            .performance_access()
            .count_descriptor_version_mismatch();
        return Err(schema_continuity_conflict(
            runtime,
            branch_id,
            options.proposed_schema_transition.as_ref(),
            Some(previous_envelope),
            ConflictClass::DescriptorVersionIncompatibility {
                previous_descriptor_semantics_version: previous_envelope
                    .descriptor_semantics_version,
                current_descriptor_semantics_version,
            },
        ));
    }

    let drift_detected = previous_envelope.schema_version != current_schema_version
        || previous_envelope.schema_registry != current_schema_registry;
    match &options.proposed_schema_transition {
        Some(proposed_transition) => materialize_declared_transition(
            runtime,
            proposed_transition.clone(),
            options.schema_reconciliation_policy,
            current_descriptor_semantics_version,
            current_descriptor_canonicalization_version,
            branch_id,
            Some(previous_envelope),
            current_schema_basis,
            current_schema_version,
        ),
        None if drift_detected => Err(schema_continuity_conflict(
            runtime,
            branch_id,
            None,
            Some(previous_envelope),
            ConflictClass::UndeclaredSchemaTransition {
                previous_schema_version: previous_envelope.schema_version,
                current_schema_version,
                previous_descriptor_semantics_version: previous_envelope
                    .descriptor_semantics_version,
                current_descriptor_semantics_version,
            },
        )),
        None => Ok(SchemaContinuityPlan::current(
            current_descriptor_semantics_version,
        )),
    }
}

fn materialize_declared_transition(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    proposed_transition: crate::schema::data::ProposedSchemaTransition,
    policy: Option<crate::schema::data::SchemaReconciliationPolicy>,
    descriptor_semantics_version: DescriptorSemanticsVersion,
    descriptor_canonicalization_version: crate::schema::data::DescriptorCanonicalizationVersion,
    branch_id: &crate::history::data::BranchId,
    previous_envelope: Option<&crate::replay::data::CanonicalCommitEnvelope>,
    current_schema_basis: Option<(
        crate::schema::data::SchemaId,
        crate::schema::data::SchemaVersionId,
    )>,
    current_schema_version: crate::schema::data::SchemaVersionId,
) -> Result<SchemaContinuityPlan, TransactionCommitError> {
    if let Some(previous_envelope) = previous_envelope {
        let previous_schema_basis = previous_envelope
            .schema_registry
            .authoritative_schema_basis()
            .map_err(|error| {
                schema_continuity_conflict(
                    runtime,
                    branch_id,
                    Some(&proposed_transition),
                    Some(previous_envelope),
                    ConflictClass::InvalidSchemaTransitionShape {
                        detail: error.detail,
                    },
                )
            })?;
        let Some((previous_schema_id, previous_schema_version_id)) = previous_schema_basis else {
            return Err(schema_continuity_conflict(
                runtime,
                branch_id,
                Some(&proposed_transition),
                Some(previous_envelope),
                ConflictClass::MissingSchemaBasisForTransition {
                    role: "prior".to_string(),
                },
            ));
        };
        if proposed_transition.source_schema_id != previous_schema_id
            || proposed_transition.source_schema_version_id != previous_schema_version_id
        {
            return Err(schema_continuity_conflict(
                runtime,
                branch_id,
                Some(&proposed_transition),
                Some(previous_envelope),
                ConflictClass::InvalidSchemaTransitionSourceBasis {
                    declared_schema_id: proposed_transition.source_schema_id.clone(),
                    declared_schema_version: proposed_transition.source_schema_version_id,
                    expected_schema_id: previous_schema_id,
                    expected_schema_version: previous_schema_version_id,
                },
            ));
        }
    }

    let Some((current_schema_id, current_schema_version_id)) = current_schema_basis else {
        return Err(schema_continuity_conflict(
            runtime,
            branch_id,
            Some(&proposed_transition),
            previous_envelope,
            ConflictClass::MissingSchemaBasisForTransition {
                role: "runtime".to_string(),
            },
        ));
    };
    if proposed_transition.target_schema_id != current_schema_id
        || proposed_transition.target_schema_version_id != current_schema_version_id
        || proposed_transition.target_schema_version_id != current_schema_version
    {
        return Err(schema_continuity_conflict(
            runtime,
            branch_id,
            Some(&proposed_transition),
            previous_envelope,
            ConflictClass::InvalidSchemaTransitionTargetBasis {
                declared_schema_id: proposed_transition.target_schema_id.clone(),
                declared_schema_version: proposed_transition.target_schema_version_id,
                expected_schema_id: current_schema_id,
                expected_schema_version: current_schema_version_id,
            },
        ));
    }

    let validated =
        validate_schema_transition(proposed_transition.clone(), policy).map_err(|error| {
            schema_continuity_conflict(
                runtime,
                branch_id,
                Some(&proposed_transition),
                previous_envelope,
                ConflictClass::InvalidSchemaTransitionShape {
                    detail: error.detail(),
                },
            )
        })?;
    match validated.reconciliation {
        crate::schema::data::SchemaReconciliationClassification::TypeIncompatible => {
            return Err(schema_continuity_conflict(
                runtime,
                branch_id,
                Some(&proposed_transition),
                previous_envelope,
                ConflictClass::TypeIncompatibleSchemaTransition {
                    detail: "declared schema transition contains a type-incompatible boundary that cannot continue honestly"
                        .to_string(),
                },
            ));
        }
        crate::schema::data::SchemaReconciliationClassification::StructuralIncompatible => {
            return Err(schema_continuity_conflict(
                runtime,
                branch_id,
                Some(&proposed_transition),
                previous_envelope,
                ConflictClass::StructuralIncompatibleSchemaTransition {
                    detail: "declared schema transition contains a structural/semantic incompatibility that cannot continue honestly"
                        .to_string(),
                },
            ));
        }
        _ => {}
    }
    let lowered = lower_schema_transition(
        validated,
        policy,
        descriptor_semantics_version,
        descriptor_canonicalization_version,
    );
    let atoms_inspected = proposed_transition.diff_atoms.len();
    // Milestone 5 does not yet reuse unchanged subtrees by fingerprint, so each
    // diff atom is currently both the inspected atom and the inspected change unit.
    let changed_subtrees_inspected = proposed_transition.diff_atoms.len();
    let unchanged_subtrees_reused_by_fingerprint = 0;
    runtime
        .performance_access()
        .count_schema_transition_classification(
            atoms_inspected,
            changed_subtrees_inspected,
            unchanged_subtrees_reused_by_fingerprint,
        );
    runtime.performance_access().count_schema_bridge_descriptor(
        lowered.continuation_descriptor.bridge.continuation,
        lowered
            .continuation_descriptor
            .bridge
            .historical_interpretation,
        lowered.reconciliation_descriptor.policy,
    );
    Ok(schema_continuity_plan_from_lowered(
        proposed_transition,
        lowered,
        descriptor_semantics_version,
        branch_id,
    ))
}

fn schema_continuity_plan_from_lowered(
    proposed_transition: crate::schema::data::ProposedSchemaTransition,
    lowered: LoweredSchemaTransitionPlan,
    descriptor_semantics_version: DescriptorSemanticsVersion,
    branch_id: &crate::history::data::BranchId,
) -> SchemaContinuityPlan {
    let continuation_descriptor = lowered.continuation_descriptor.clone();
    let mut reconciliation_descriptor = lowered.reconciliation_descriptor.clone();
    reconciliation_descriptor.resulting_lineage.branch_context = Some(branch_id.clone());
    let schema_transition = SchemaTransitionArtifact::new(
        proposed_transition.source_schema_id,
        proposed_transition.source_schema_version_id,
        proposed_transition.target_schema_id,
        proposed_transition.target_schema_version_id,
        proposed_transition.diff_atoms,
        continuation_descriptor.clone(),
        reconciliation_descriptor.clone(),
    );

    SchemaContinuityPlan {
        descriptor_semantics_version,
        schema_transition: Some(schema_transition),
        schema_continuation_descriptor: Some(continuation_descriptor),
        schema_reconciliation_descriptor: Some(reconciliation_descriptor),
    }
}

fn schema_continuity_conflict(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
    proposed_transition: Option<&crate::schema::data::ProposedSchemaTransition>,
    previous_envelope: Option<&crate::replay::data::CanonicalCommitEnvelope>,
    class: ConflictClass,
) -> TransactionCommitError {
    let conflict = CommitConflict::new(class);
    emit_schema_continuity_failure_diagnostic(
        runtime,
        branch_id,
        proposed_transition.map(FailureTransitionView::Proposed),
        previous_envelope,
        &conflict,
    );
    TransactionCommitError::conflict(conflict)
}

pub(crate) fn emit_schema_continuity_diagnostic(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
    plan: &SchemaContinuityPlan,
) {
    let Some(transition) = &plan.schema_transition else {
        return;
    };
    let transition_summary = SchemaTransitionSummary::from_artifact(transition);
    runtime.publication_authority().push_bounded_diagnostic(
        DiagnosticsScope::Schema,
        DiagnosticsArtifactKind::MinimalSummary,
        vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaTransitionTraced,
            message: "schema continuity transition lowered into canonical commit artifacts"
                .to_string(),
            fields: diagnostics_fields(&SchemaTransitionSummaryFields {
                branch_id: branch_id.0.clone(),
                source_schema_id: transition.source_schema_id.0.clone(),
                source_schema_version_id: transition.source_schema_version_id.0,
                target_schema_id: transition.target_schema_id.0.clone(),
                target_schema_version_id: transition.target_schema_version_id.0,
                changed_atom_count: transition_summary.changed_atom_count,
                changed_strata: format_strata(&transition_summary.changed_strata),
                historical_interpretation: format!(
                    "{:?}",
                    transition_summary.historical_interpretation
                ),
                continuation: format!("{:?}", transition_summary.continuation),
                bridgeability: format!("{:?}", transition_summary.bridgeability),
                reconciliation: format!("{:?}", transition_summary.reconciliation),
                descriptor_semantics_version: plan.descriptor_semantics_version.0,
                descriptor_canonicalization_version: transition
                    .continuation_descriptor
                    .bridge
                    .canonicalization_version
                    .0,
                normalized_boundary_count: transition
                    .continuation_descriptor
                    .normalized_boundary_count,
            }),
        }],
    );
    runtime.publication_authority().push_bounded_diagnostic(
        DiagnosticsScope::Schema,
        DiagnosticsArtifactKind::DetailedTrace,
        schema_transition_trace_entries(branch_id, transition),
    );
}

pub(crate) fn validate_schema_continuity_publication(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
    _plan: &SchemaContinuityPlan,
    envelope: &CanonicalCommitEnvelope,
) -> Result<(), TransactionCommitError> {
    let validated_bundle = validate_schema_continuity_bundle(envelope).map_err(|issue| {
        schema_continuity_conflict_from_issue(
            runtime,
            branch_id,
            envelope.schema_transition.as_ref(),
            issue,
            envelope,
        )
    })?;
    let _ = (
        validated_bundle.envelope(),
        validated_bundle.transition(),
        validated_bundle.continuation(),
        validated_bundle.reconciliation(),
    );

    Ok(())
}

fn schema_continuity_conflict_from_issue(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
    transition: Option<&SchemaTransitionArtifact>,
    issue: SchemaContinuityBundleIssue,
    envelope: &CanonicalCommitEnvelope,
) -> TransactionCommitError {
    if matches!(
        issue,
        SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch { .. }
            | SchemaContinuityBundleIssue::DescriptorCanonicalizationVersionMismatch { .. }
    ) {
        runtime
            .performance_access()
            .count_descriptor_version_mismatch();
    }
    let class = match issue {
        SchemaContinuityBundleIssue::IncompleteBundle
        | SchemaContinuityBundleIssue::ContinuationDescriptorDrift { .. }
        | SchemaContinuityBundleIssue::ContinuationBoundaryFingerprintMismatch { .. }
        | SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch { .. }
        | SchemaContinuityBundleIssue::DescriptorCanonicalizationVersionMismatch { .. }
        | SchemaContinuityBundleIssue::VisibleBridgeProofMismatch => {
            ConflictClass::UnsupportedBridgeDescriptor {
                detail: issue.detail(),
            }
        }
        SchemaContinuityBundleIssue::ReconciliationDescriptorDrift => {
            ConflictClass::UnsupportedBridgeDescriptor {
                detail: issue.detail(),
            }
        }
        SchemaContinuityBundleIssue::TargetSchemaVersionMismatch => {
            ConflictClass::UnsupportedBridgeDescriptor {
                detail: format!(
                    "{}: target {:?} envelope {:?}",
                    issue.detail(),
                    transition.map(|candidate| candidate.target_schema_version_id),
                    envelope.schema_version
                ),
            }
        }
        SchemaContinuityBundleIssue::LineageSchemaVersionMismatch => {
            ConflictClass::DirectionalityMismatchUnderCanonicalReconciliation {
                detail: format!(
                    "{}: lineage {:?} envelope {:?}",
                    issue.detail(),
                    envelope
                        .schema_reconciliation_descriptor
                        .as_ref()
                        .map(|descriptor| descriptor.resulting_lineage.resulting_schema_version_id),
                    envelope.schema_version
                ),
            }
        }
        SchemaContinuityBundleIssue::HistoricalReinterpretationViolation => {
            ConflictClass::HistoricalReinterpretationViolation {
                detail: issue.detail(),
            }
        }
    };
    let conflict = CommitConflict::new(class);
    emit_schema_continuity_failure_diagnostic(
        runtime,
        branch_id,
        transition.map(FailureTransitionView::Artifact),
        None,
        &conflict,
    );
    TransactionCommitError::conflict(conflict)
}

fn emit_schema_continuity_failure_diagnostic(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    branch_id: &crate::history::data::BranchId,
    proposed_transition: Option<FailureTransitionView<'_>>,
    previous_envelope: Option<&crate::replay::data::CanonicalCommitEnvelope>,
    conflict: &CommitConflict,
) {
    let mut entries = vec![RelationalDiagnosticsEntry {
        code: DiagnosticCode::SchemaContinuityViolation,
        message: "schema continuity decision rejected during commit planning".to_string(),
        fields: diagnostics_fields(&SchemaContinuityFailureFields {
            branch_id: branch_id.0.clone(),
            conflict_class: format!("{:?}", conflict.class),
            detail: conflict.detail.clone(),
            conflict_fields: conflict.fields.clone(),
            previous_schema_version: previous_envelope.map(|envelope| envelope.schema_version.0),
            previous_descriptor_semantics_version: previous_envelope
                .map(|envelope| envelope.descriptor_semantics_version.0),
        }),
    }];

    if let Some(proposed_transition) = proposed_transition {
        let (
            source_schema_id,
            source_schema_version_id,
            target_schema_id,
            target_schema_version_id,
            diff_atoms,
        ) = match proposed_transition {
            FailureTransitionView::Proposed(transition) => (
                &transition.source_schema_id,
                transition.source_schema_version_id,
                &transition.target_schema_id,
                transition.target_schema_version_id,
                transition.diff_atoms.as_slice(),
            ),
            FailureTransitionView::Artifact(transition) => (
                &transition.source_schema_id,
                transition.source_schema_version_id,
                &transition.target_schema_id,
                transition.target_schema_version_id,
                transition.diff_atoms.as_slice(),
            ),
        };
        entries.push(RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaTransitionClassified,
            message: "rejected schema transition proposal captured for diagnosis".to_string(),
            fields: diagnostics_fields(&SchemaTransitionRejectedFields {
                source_schema_id: source_schema_id.0.clone(),
                source_schema_version_id: source_schema_version_id.0,
                target_schema_id: target_schema_id.0.clone(),
                target_schema_version_id: target_schema_version_id.0,
                changed_atom_count: diff_atoms.len(),
            }),
        });
        entries.extend(diff_atoms.iter().enumerate().map(|(index, atom)| {
            RelationalDiagnosticsEntry {
                code: DiagnosticCode::SchemaTransitionClassified,
                message: format!("rejected schema diff atom {index} traced for diagnosis"),
                fields: diagnostics_fields(&schema_diff_atom_trace_fields(index, atom)),
            }
        }));
    }

    runtime.publication_authority().push_bounded_diagnostic(
        DiagnosticsScope::Schema,
        DiagnosticsArtifactKind::Failure,
        entries,
    );
}

fn schema_transition_trace_entries(
    branch_id: &crate::history::data::BranchId,
    transition: &SchemaTransitionArtifact,
) -> Vec<RelationalDiagnosticsEntry> {
    let mut entries = vec![
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaBridgeDescriptorConstructed,
            message: "schema bridge descriptor constructed for continuity boundary".to_string(),
            fields: diagnostics_fields(&SchemaBridgeDescriptorFields {
                boundary_fingerprint: format!(
                    "{:?}",
                    transition.continuation_descriptor.boundary_fingerprint
                ),
                continuation: format!(
                    "{:?}",
                    transition.continuation_descriptor.bridge.continuation
                ),
                bridgeability: format!(
                    "{:?}",
                    transition.continuation_descriptor.bridge.bridgeability
                ),
                normalized_boundary_count: transition
                    .continuation_descriptor
                    .normalized_boundary_count,
                descriptor_canonicalization_version: transition
                    .continuation_descriptor
                    .bridge
                    .canonicalization_version
                    .0,
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaInterpretationSensitivityClassified,
            message: "schema historical interpretation sensitivity classified".to_string(),
            fields: diagnostics_fields(&SchemaInterpretationFields {
                boundary_fingerprint: format!(
                    "{:?}",
                    transition.continuation_descriptor.boundary_fingerprint
                ),
                historical_interpretation: format!(
                    "{:?}",
                    transition
                        .continuation_descriptor
                        .bridge
                        .historical_interpretation
                ),
                changed_strata: format_strata(
                    &transition.continuation_descriptor.bridge.changed_strata,
                ),
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaReconciliationResolved,
            message: "schema reconciliation result resolved for continuity boundary".to_string(),
            fields: diagnostics_fields(&SchemaReconciliationFields {
                classification: format!(
                    "{:?}",
                    transition.reconciliation_descriptor.classification
                ),
                policy: format!("{:?}", transition.reconciliation_descriptor.policy),
                resulting_schema_id: transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .resulting_schema_id
                    .0
                    .clone(),
                resulting_schema_version_id: transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .resulting_schema_version_id
                    .0,
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaDescriptorVersionSelected,
            message: "schema descriptor semantics version selected for continuity boundary"
                .to_string(),
            fields: diagnostics_fields(&SchemaDescriptorVersionFields {
                descriptor_semantics_version: transition
                    .continuation_descriptor
                    .bridge
                    .semantics_version
                    .0,
                continuation_canonicalization_version: transition
                    .continuation_descriptor
                    .bridge
                    .canonicalization_version
                    .0,
                reconciliation_canonicalization_version: transition
                    .reconciliation_descriptor
                    .canonicalization_version
                    .0,
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaTransitionClassified,
            message: "schema transition classified into continuation and reconciliation outcomes"
                .to_string(),
            fields: diagnostics_fields(&SchemaTransitionClassificationFields {
                branch_id: branch_id.0.clone(),
                boundary_fingerprint: format!(
                    "{:?}",
                    transition.continuation_descriptor.boundary_fingerprint
                ),
                continuation: format!(
                    "{:?}",
                    transition.continuation_descriptor.bridge.continuation
                ),
                bridgeability: format!(
                    "{:?}",
                    transition.continuation_descriptor.bridge.bridgeability
                ),
                historical_interpretation: format!(
                    "{:?}",
                    transition
                        .continuation_descriptor
                        .bridge
                        .historical_interpretation
                ),
                changed_strata: format_strata(
                    &transition.continuation_descriptor.bridge.changed_strata,
                ),
                reconciliation: format!(
                    "{:?}",
                    transition.reconciliation_descriptor.classification
                ),
                policy: format!("{:?}", transition.reconciliation_descriptor.policy),
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaLineageTraced,
            message: "schema reconciliation lineage recorded for continuity boundary".to_string(),
            fields: diagnostics_fields(&SchemaLineageFields {
                resulting_schema_id: transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .resulting_schema_id
                    .0
                    .clone(),
                resulting_schema_version_id: transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .resulting_schema_version_id
                    .0,
                parent_schema_ids: transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .parent_schema_ids
                    .iter()
                    .map(|id| id.0.clone())
                    .collect(),
                parent_schema_version_ids: transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .parent_schema_version_ids
                    .iter()
                    .map(|id| id.0)
                    .collect(),
                ordering_mode: format!(
                    "{:?}",
                    transition
                        .reconciliation_descriptor
                        .resulting_lineage
                        .ordering_mode
                ),
                ordering_semantics: format!(
                    "{:?}",
                    transition
                        .reconciliation_descriptor
                        .resulting_lineage
                        .ordering_semantics
                ),
                branch_context: transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .branch_context
                    .as_ref()
                    .map(|branch| branch.0.clone()),
            }),
        },
    ];

    entries.extend(
        transition
            .diff_atoms
            .iter()
            .enumerate()
            .map(|(index, atom)| RelationalDiagnosticsEntry {
                code: DiagnosticCode::SchemaTransitionClassified,
                message: format!("schema diff atom {index} classified for continuity"),
                fields: diagnostics_fields(&schema_diff_atom_trace_fields(index, atom)),
            }),
    );

    entries
}

fn schema_diff_detail_fields(detail: &SchemaDiffDetail) -> SchemaDiffDetailFields {
    match detail {
        SchemaDiffDetail::AddedField {
            field_name,
            required,
            default_expression,
        } => SchemaDiffDetailFields::AddedField {
            field_name: field_name.to_string(),
            required: *required,
            default_expression: default_expression.as_ref().map(|expr| expr.to_string()),
        },
        SchemaDiffDetail::RemovedField { field_name } => SchemaDiffDetailFields::RemovedField {
            field_name: field_name.to_string(),
        },
        SchemaDiffDetail::TypeChanged {
            field_name,
            from_type,
            to_type,
        } => SchemaDiffDetailFields::TypeChanged {
            field_name: field_name.to_string(),
            from_type: from_type.to_string(),
            to_type: to_type.to_string(),
        },
        SchemaDiffDetail::EnumDomainExpanded {
            field_name,
            added_variants,
        } => SchemaDiffDetailFields::EnumDomainExpanded {
            field_name: field_name.to_string(),
            added_variants: added_variants
                .iter()
                .map(|variant| variant.to_string())
                .collect(),
        },
        SchemaDiffDetail::InvariantContractChanged { contract_name } => {
            SchemaDiffDetailFields::InvariantContractChanged {
                contract_name: contract_name.to_string(),
            }
        }
        SchemaDiffDetail::ProjectionContractChanged { projection_name } => {
            SchemaDiffDetailFields::ProjectionContractChanged {
                projection_name: projection_name.to_string(),
            }
        }
        SchemaDiffDetail::SubscriberContractChanged { contract_name } => {
            SchemaDiffDetailFields::SubscriberContractChanged {
                contract_name: contract_name.to_string(),
            }
        }
        SchemaDiffDetail::FreeText {
            detail,
            declared_intent,
        } => SchemaDiffDetailFields::FreeText {
            detail: detail.to_string(),
            declared_intent: match declared_intent {
                FreeFormSchemaDiffIntent::Additive => "Additive".to_string(),
                FreeFormSchemaDiffIntent::StructuralIncompatible => {
                    "StructuralIncompatible".to_string()
                }
            },
        },
    }
}

fn schema_diff_atom_trace_fields(
    index: usize,
    atom: &crate::schema::data::SchemaDiffAtom,
) -> SchemaDiffAtomTraceFields {
    SchemaDiffAtomTraceFields {
        diff_atom_index: index,
        element_kind: format!("{:?}", atom.element.kind),
        schema_id: atom.element.schema_id.0.clone(),
        schema_version_id: atom.element.schema_version_id.0,
        kind_id: atom.element.kind_id.map(|kind_id| kind_id.0),
        element_name: atom.element.element_name.to_string(),
        strata: format_strata(&atom.strata),
        publication_impact: format!("{:?}", atom.publication_impact),
        subscriber_impact: format!("{:?}", atom.subscriber_impact),
        historical_interpretation: format!("{:?}", atom.historical_interpretation),
        detail: schema_diff_detail_fields(&atom.detail),
    }
}

fn format_strata(strata: &[SchemaStratum]) -> Vec<String> {
    strata
        .iter()
        .map(|stratum| format!("{stratum:?}"))
        .collect()
}

fn diagnostics_fields<T: Serialize>(fields: &T) -> Value {
    to_value(fields).unwrap_or_else(|error| {
        Value::String(format!(
            "schema continuity diagnostics serialization failed: {error}"
        ))
    })
}
