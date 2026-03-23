use crate::capabilities::{SchemaSource, SchemaVersionSource};
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::replay::data::CanonicalCommitEnvelope;
use crate::schema::data::{
    DescriptorSemanticsVersion, LoweredSchemaTransitionPlan, SchemaContinuationDescriptor, SchemaDiffDetail,
    SchemaReconciliationDescriptor, SchemaStratum, SchemaTransitionArtifact,
    SchemaTransitionSummary,
};
use crate::schema::logic::{
    lower_schema_transition, validate_schema_continuity_bundle, validate_schema_transition,
    SchemaContinuityBundleIssue,
};
use crate::transactions::data::{CommitConflict, ConflictClass, TransactionCommitError};
use serde_json::json;

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
        runtime.performance_access().count_descriptor_version_mismatch();
        return Err(schema_continuity_conflict(
            runtime,
            branch_id,
            options.proposed_schema_transition.as_ref(),
            Some(previous_envelope),
            ConflictClass::DescriptorVersionIncompatibility {
                previous_descriptor_semantics_version:
                    previous_envelope.descriptor_semantics_version,
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
                previous_descriptor_semantics_version:
                    previous_envelope.descriptor_semantics_version,
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
    current_schema_basis: Option<(crate::schema::data::SchemaId, crate::schema::data::SchemaVersionId)>,
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

    let validated = validate_schema_transition(proposed_transition.clone(), policy).map_err(|error| {
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
    runtime.performance_access().count_schema_transition_classification(
        proposed_transition.diff_atoms.len(),
        proposed_transition.diff_atoms.len(),
        0,
    );
    runtime.performance_access().count_schema_bridge_descriptor(
        lowered.continuation_descriptor.bridge.continuation,
        lowered.continuation_descriptor.bridge.historical_interpretation,
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
            fields: json!({
                "branch_id": branch_id.0,
                "source_schema_id": transition.source_schema_id.0,
                "source_schema_version_id": transition.source_schema_version_id.0,
                "target_schema_id": transition.target_schema_id.0,
                "target_schema_version_id": transition.target_schema_version_id.0,
                "changed_atom_count": transition_summary.changed_atom_count,
                "changed_strata": transition_summary
                    .changed_strata
                    .iter()
                    .map(schema_stratum_name)
                    .collect::<Vec<_>>(),
                "historical_interpretation": format!("{:?}", transition_summary.historical_interpretation),
                "continuation": format!("{:?}", transition_summary.continuation),
                "bridgeability": format!("{:?}", transition_summary.bridgeability),
                "reconciliation": format!("{:?}", transition_summary.reconciliation),
                "descriptor_semantics_version": plan.descriptor_semantics_version.0,
                "descriptor_canonicalization_version": transition.continuation_descriptor
                    .bridge
                    .canonicalization_version
                    .0,
                "normalized_boundary_count": transition.continuation_descriptor
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
        runtime.performance_access().count_descriptor_version_mismatch();
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
        fields: json!({
            "branch_id": branch_id.0,
            "conflict_class": format!("{:?}", conflict.class),
            "detail": conflict.detail,
            "conflict_fields": conflict.fields,
            "previous_schema_version": previous_envelope.map(|envelope| envelope.schema_version.0),
            "previous_descriptor_semantics_version": previous_envelope
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
            fields: json!({
                "source_schema_id": source_schema_id.0,
                "source_schema_version_id": source_schema_version_id.0,
                "target_schema_id": target_schema_id.0,
                "target_schema_version_id": target_schema_version_id.0,
                "changed_atom_count": diff_atoms.len(),
            }),
        });
        entries.extend(
            diff_atoms
                .iter()
                .enumerate()
                .map(|(index, atom)| RelationalDiagnosticsEntry {
                    code: DiagnosticCode::SchemaTransitionClassified,
                    message: format!("rejected schema diff atom {index} traced for diagnosis"),
                    fields: json!({
                        "diff_atom_index": index,
                        "element_kind": format!("{:?}", atom.element.kind),
                        "schema_id": atom.element.schema_id.0,
                        "schema_version_id": atom.element.schema_version_id.0,
                        "kind_id": atom.element.kind_id.map(|kind_id| kind_id.0),
                        "element_name": atom.element.element_name.as_ref(),
                        "strata": atom.strata.iter().map(schema_stratum_name).collect::<Vec<_>>(),
                        "publication_impact": format!("{:?}", atom.publication_impact),
                        "subscriber_impact": format!("{:?}", atom.subscriber_impact),
                        "historical_interpretation": format!("{:?}", atom.historical_interpretation),
                        "detail": schema_diff_detail_fields(&atom.detail),
                    }),
                }),
        );
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
            fields: json!({
                "boundary_fingerprint": format!("{:?}", transition.continuation_descriptor.boundary_fingerprint),
                "continuation": format!("{:?}", transition.continuation_descriptor.bridge.continuation),
                "bridgeability": format!("{:?}", transition.continuation_descriptor.bridge.bridgeability),
                "normalized_boundary_count": transition.continuation_descriptor.normalized_boundary_count,
                "descriptor_canonicalization_version": transition.continuation_descriptor.bridge.canonicalization_version.0,
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaInterpretationSensitivityClassified,
            message: "schema historical interpretation sensitivity classified".to_string(),
            fields: json!({
                "boundary_fingerprint": format!("{:?}", transition.continuation_descriptor.boundary_fingerprint),
                "historical_interpretation": format!("{:?}", transition.continuation_descriptor.bridge.historical_interpretation),
                "changed_strata": transition
                    .continuation_descriptor
                    .bridge
                    .changed_strata
                    .iter()
                    .map(schema_stratum_name)
                    .collect::<Vec<_>>(),
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaReconciliationResolved,
            message: "schema reconciliation result resolved for continuity boundary".to_string(),
            fields: json!({
                "classification": format!("{:?}", transition.reconciliation_descriptor.classification),
                "policy": format!("{:?}", transition.reconciliation_descriptor.policy),
                "resulting_schema_id": transition.reconciliation_descriptor.resulting_lineage.resulting_schema_id.0,
                "resulting_schema_version_id": transition.reconciliation_descriptor.resulting_lineage.resulting_schema_version_id.0,
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaDescriptorVersionSelected,
            message: "schema descriptor semantics version selected for continuity boundary".to_string(),
            fields: json!({
                "descriptor_semantics_version": transition.continuation_descriptor.bridge.semantics_version.0,
                "continuation_canonicalization_version": transition.continuation_descriptor.bridge.canonicalization_version.0,
                "reconciliation_canonicalization_version": transition.reconciliation_descriptor.canonicalization_version.0,
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaTransitionClassified,
            message: "schema transition classified into continuation and reconciliation outcomes"
                .to_string(),
            fields: json!({
                "branch_id": branch_id.0,
                "boundary_fingerprint": format!("{:?}", transition.continuation_descriptor.boundary_fingerprint),
                "continuation": format!("{:?}", transition.continuation_descriptor.bridge.continuation),
                "bridgeability": format!("{:?}", transition.continuation_descriptor.bridge.bridgeability),
                "historical_interpretation": format!("{:?}", transition.continuation_descriptor.bridge.historical_interpretation),
                "changed_strata": transition
                    .continuation_descriptor
                    .bridge
                    .changed_strata
                    .iter()
                    .map(schema_stratum_name)
                    .collect::<Vec<_>>(),
                "reconciliation": format!("{:?}", transition.reconciliation_descriptor.classification),
                "policy": format!("{:?}", transition.reconciliation_descriptor.policy),
            }),
        },
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SchemaLineageTraced,
            message: "schema reconciliation lineage recorded for continuity boundary".to_string(),
            fields: json!({
                "resulting_schema_id": transition.reconciliation_descriptor.resulting_lineage.resulting_schema_id.0,
                "resulting_schema_version_id": transition.reconciliation_descriptor.resulting_lineage.resulting_schema_version_id.0,
                "parent_schema_ids": transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .parent_schema_ids
                    .iter()
                    .map(|id| id.0.clone())
                    .collect::<Vec<_>>(),
                "parent_schema_version_ids": transition
                    .reconciliation_descriptor
                    .resulting_lineage
                    .parent_schema_version_ids
                    .iter()
                    .map(|id| id.0)
                    .collect::<Vec<_>>(),
                "ordering_mode": format!("{:?}", transition.reconciliation_descriptor.resulting_lineage.ordering_mode),
                "ordering_semantics": format!("{:?}", transition.reconciliation_descriptor.resulting_lineage.ordering_semantics),
                "branch_context": transition
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
                fields: json!({
                    "diff_atom_index": index,
                    "element_kind": format!("{:?}", atom.element.kind),
                    "schema_id": atom.element.schema_id.0,
                    "schema_version_id": atom.element.schema_version_id.0,
                    "kind_id": atom.element.kind_id.map(|kind_id| kind_id.0),
                    "element_name": atom.element.element_name.as_ref(),
                    "strata": atom.strata.iter().map(schema_stratum_name).collect::<Vec<_>>(),
                    "publication_impact": format!("{:?}", atom.publication_impact),
                    "subscriber_impact": format!("{:?}", atom.subscriber_impact),
                    "historical_interpretation": format!("{:?}", atom.historical_interpretation),
                    "detail": schema_diff_detail_fields(&atom.detail),
                }),
            }),
    );

    entries
}

fn schema_stratum_name(stratum: &SchemaStratum) -> &'static str {
    match stratum {
        SchemaStratum::StructuralShape => "StructuralShape",
        SchemaStratum::ValueDomain => "ValueDomain",
        SchemaStratum::EntityIdentitySemantics => "EntityIdentitySemantics",
        SchemaStratum::CorrespondenceSemantics => "CorrespondenceSemantics",
        SchemaStratum::LineageSemantics => "LineageSemantics",
        SchemaStratum::BehavioralSemantics => "BehavioralSemantics",
        SchemaStratum::PublicationContract => "PublicationContract",
        SchemaStratum::SubscriberContract => "SubscriberContract",
    }
}

fn schema_diff_detail_fields(detail: &SchemaDiffDetail) -> serde_json::Value {
    match detail {
        SchemaDiffDetail::AddedField {
            field_name,
            required,
            default_expression,
        } => json!({
            "kind": "AddedField",
            "field_name": field_name.as_ref(),
            "required": required,
            "default_expression": default_expression.as_ref().map(|expr| expr.as_ref()),
        }),
        SchemaDiffDetail::RemovedField { field_name } => json!({
            "kind": "RemovedField",
            "field_name": field_name.as_ref(),
        }),
        SchemaDiffDetail::TypeChanged {
            field_name,
            from_type,
            to_type,
        } => json!({
            "kind": "TypeChanged",
            "field_name": field_name.as_ref(),
            "from_type": from_type.as_ref(),
            "to_type": to_type.as_ref(),
        }),
        SchemaDiffDetail::EnumDomainExpanded {
            field_name,
            added_variants,
        } => json!({
            "kind": "EnumDomainExpanded",
            "field_name": field_name.as_ref(),
            "added_variants": added_variants.iter().map(|variant| variant.as_ref()).collect::<Vec<_>>(),
        }),
        SchemaDiffDetail::InvariantContractChanged { contract_name } => json!({
            "kind": "InvariantContractChanged",
            "contract_name": contract_name.as_ref(),
        }),
        SchemaDiffDetail::ProjectionContractChanged { projection_name } => json!({
            "kind": "ProjectionContractChanged",
            "projection_name": projection_name.as_ref(),
        }),
        SchemaDiffDetail::SubscriberContractChanged { contract_name } => json!({
            "kind": "SubscriberContractChanged",
            "contract_name": contract_name.as_ref(),
        }),
        SchemaDiffDetail::FreeText { detail } => json!({
            "kind": "FreeText",
            "detail": detail.as_ref(),
        }),
    }
}
