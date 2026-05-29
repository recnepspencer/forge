use std::collections::BTreeMap;
use std::sync::Arc;

use crate::capabilities::AspectPlanSource;
use crate::merge::data::{
    AspectComparisonState, AuthorizedAspectValueSurface, AuthorizedAspectValueUsage,
    DeletionExecutionClass, LoweredAspectAction, LoweredAspectDenialIntent,
    LoweredAspectExecutionIntent, LoweredAspectOutcome, LoweredMergeAction,
    LoweredMergeBlockedReason, LoweredMergePlan, LoweredMergePlanRecord, LoweredMergePlanSummary,
    LoweredMergeRejectedReason, LoweredRecordDecision, LoweredRecordDenialAspectIntent,
    LoweredRecordDenialBundle, LoweredRecordDenialKind, LoweredRecordExecutionAspectIntent,
    LoweredRecordExecutionBundle, LoweredRecordExecutionIntentKind, MergeExecutableClass,
    MergeExecutionReadiness, MergePlanningDecisionKind, MergePlanningDecisionLog,
    MergePlanningDecisionLogDigestBasis, MergePlanningError, MergePlanningRequest,
    MergePolicyDecisionBoundary, MergeResolutionClass, PolicyResolvedMergePlan,
    TopologyExecutionClass, TopologyRewireAdmissionPolicy, VisibleMergeRecordKind,
};
use crate::merge::logic::MergeAccess;
use crate::schema::data::LoweredAspectPlan;
use crate::transactions::data::RecordRef;

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn lower_planning_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<LoweredMergePlan, MergePlanningError> {
        let policy_plan = self.plan_policy_scope(request)?;
        self.lower_policy_plan(policy_plan)
    }

    fn lower_policy_plan(
        &self,
        policy_plan: PolicyResolvedMergePlan,
    ) -> Result<LoweredMergePlan, MergePlanningError> {
        let causal_by_record = policy_plan
            .causal_annotations
            .iter()
            .map(|annotation| (annotation.record.clone(), annotation))
            .collect::<BTreeMap<RecordRef, _>>();
        let source_records_by_ref = policy_plan
            .source_records
            .iter()
            .map(|record| (record.record_ref.clone(), record))
            .collect::<BTreeMap<_, _>>();
        let classifications_by_record = policy_plan
            .classifications
            .iter()
            .map(|classification| (classification.record.clone(), classification))
            .collect::<BTreeMap<_, _>>();
        let lowered_records = policy_plan
            .policy_records
            .iter()
            .map(|policy_record| {
                let causal = causal_by_record.get(&policy_record.record).ok_or_else(|| {
                    MergePlanningError::MissingCausalAnnotation {
                        record: policy_record.record.clone(),
                    }
                })?;
                let source_record = source_records_by_ref
                    .get(&policy_record.record)
                    .ok_or_else(|| MergePlanningError::MissingLoweringSourceRecord {
                        record: policy_record.record.clone(),
                    })?;
                let classification = classifications_by_record
                    .get(&policy_record.record)
                    .ok_or_else(
                        || MergePlanningError::MissingLoweringConflictClassification {
                            record: policy_record.record.clone(),
                        },
                    )?;
                let resolution_class = resolution_class_for_record(
                    policy_record.classification,
                    classification.relation_evidence.as_ref(),
                );
                let aspect_outcomes = lowered_aspect_outcomes_for_record(
                    self.runtime,
                    source_record,
                    policy_record,
                    resolution_class,
                )?;
                let policy_readiness =
                    readiness_for_policy_decision(policy_record.proof_boundary.decision_boundary);
                let aspect_readiness = if aspect_outcomes.is_empty() {
                    policy_readiness
                } else {
                    aggregate_record_readiness(aspect_outcomes.as_slice())
                };
                let readiness = match policy_readiness {
                    MergeExecutionReadiness::Rejected => MergeExecutionReadiness::Rejected,
                    MergeExecutionReadiness::Blocked => MergeExecutionReadiness::Blocked,
                    MergeExecutionReadiness::Admitted => aspect_readiness,
                };
                let lowered_action = lowered_action_for_record(
                    policy_record.classification,
                    aspect_outcomes.as_slice(),
                    readiness,
                );
                let execution_bundle = execution_bundle_for_record(
                    policy_record.classification,
                    aspect_outcomes.as_slice(),
                    readiness,
                );
                let executable_class = executable_class_for_record(
                    resolution_class,
                    readiness,
                    execution_bundle.as_ref().map(|bundle| bundle.kind),
                );
                let denial_bundle = denial_bundle_for_record(
                    policy_record.classification,
                    resolution_class,
                    aspect_outcomes.as_slice(),
                    readiness,
                );
                let blocked_reason = blocked_reason_for_record(
                    policy_record.classification,
                    resolution_class,
                    aspect_outcomes.as_slice(),
                    readiness,
                );
                let rejected_reason =
                    rejected_reason_for_record(aspect_outcomes.as_slice(), readiness);
                let record_decision = record_decision_for_record(
                    readiness,
                    policy_record.classification,
                    resolution_class,
                    lowered_action,
                    blocked_reason,
                    rejected_reason,
                    execution_bundle.clone(),
                    denial_bundle.clone(),
                )?;
                Ok(LoweredMergePlanRecord {
                    record: policy_record.record.clone(),
                    target_record: policy_record.target_record.clone(),
                    classification: policy_record.classification,
                    resolution_class,
                    executable_class,
                    causal_disposition: causal.disposition,
                    applied_policies: policy_record.applied_policies.clone(),
                    policy_proof_boundary: policy_record.proof_boundary,
                    readiness,
                    record_decision,
                    lowered_action,
                    blocked_reason,
                    rejected_reason,
                    aspect_outcomes: Arc::from(aspect_outcomes),
                })
            })
            .collect::<Result<Vec<_>, MergePlanningError>>()?;
        let lowered_summary = summarize_lowered_records(Arc::from(lowered_records.clone()));
        let decision_log = build_decision_log(&lowered_records);
        let decision_log_digest_basis = build_decision_log_digest_basis(&decision_log);

        Ok(LoweredMergePlan {
            request: policy_plan.request,
            target_head: policy_plan.target_head,
            source_head: policy_plan.source_head,
            merge_base: policy_plan.merge_base,
            ancestry: policy_plan.ancestry,
            target_delta: policy_plan.target_delta,
            source_delta: policy_plan.source_delta,
            effective_identity_declarations: policy_plan.effective_identity_declarations,
            source_records: policy_plan.source_records,
            candidates: policy_plan.candidates,
            validated_schema_correspondences: policy_plan.validated_schema_correspondences,
            identity_summary: policy_plan.identity_summary,
            classifications: policy_plan.classifications,
            conflict_summary: policy_plan.conflict_summary,
            causal_annotations: policy_plan.causal_annotations,
            causal_summary: policy_plan.causal_summary,
            policy_records: policy_plan.policy_records,
            policy_summary: policy_plan.policy_summary,
            lowered_records: Arc::from(lowered_records),
            lowered_summary,
            decision_log,
            decision_log_digest_basis,
        })
    }
}

fn record_decision_for_record(
    readiness: MergeExecutionReadiness,
    classification: crate::merge::data::MergeConflictClass,
    resolution_class: MergeResolutionClass,
    lowered_action: Option<LoweredMergeAction>,
    blocked_reason: Option<LoweredMergeBlockedReason>,
    rejected_reason: Option<LoweredMergeRejectedReason>,
    execution_bundle: Option<LoweredRecordExecutionBundle>,
    denial_bundle: Option<LoweredRecordDenialBundle>,
) -> Result<LoweredRecordDecision, MergePlanningError> {
    match readiness {
        MergeExecutionReadiness::Admitted => {
            if let Some(bundle) = execution_bundle {
                Ok(LoweredRecordDecision::Execute(bundle))
            } else {
                synthesized_execution_bundle(classification, lowered_action)
                    .map(LoweredRecordDecision::Execute)
                    .ok_or(MergePlanningError::MissingLoweredRecordExecutionBundle {
                        classification,
                        readiness,
                        lowered_action,
                    })
            }
        }
        MergeExecutionReadiness::Blocked => {
            if let Some(bundle) = denial_bundle {
                Ok(LoweredRecordDecision::Block(bundle))
            } else {
                synthesized_denial_bundle(
                    classification,
                    resolution_class,
                    blocked_reason,
                    readiness,
                )
                .map(LoweredRecordDecision::Block)
                .ok_or(MergePlanningError::MissingLoweredRecordDenialBundle)
            }
        }
        MergeExecutionReadiness::Rejected => {
            if let Some(bundle) = denial_bundle {
                Ok(LoweredRecordDecision::Reject(bundle))
            } else {
                let _ = rejected_reason;
                synthesized_denial_bundle(classification, resolution_class, None, readiness)
                    .map(LoweredRecordDecision::Reject)
                    .ok_or(MergePlanningError::MissingLoweredRecordDenialBundle)
            }
        }
    }
}

fn synthesized_execution_bundle(
    classification: crate::merge::data::MergeConflictClass,
    lowered_action: Option<LoweredMergeAction>,
) -> Option<LoweredRecordExecutionBundle> {
    let kind = match (classification, lowered_action) {
        (
            crate::merge::data::MergeConflictClass::SourceOnlyAddition,
            Some(LoweredMergeAction::KeepSourceAddition),
        ) => LoweredRecordExecutionIntentKind::AdoptSourceRecord,
        (
            crate::merge::data::MergeConflictClass::ExactSharedTruth,
            Some(LoweredMergeAction::KeepExactSharedTruth),
        ) => LoweredRecordExecutionIntentKind::PreserveSharedRecord,
        (
            crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence,
            Some(LoweredMergeAction::ReconcileSchemaCorrespondence),
        )
        | (
            crate::merge::data::MergeConflictClass::DivergentVisibleState,
            Some(LoweredMergeAction::ReconcileDivergentVisibleState),
        ) => LoweredRecordExecutionIntentKind::ReconcileRecord,
        (
            crate::merge::data::MergeConflictClass::Deletion(
                crate::merge::data::DeletionMergeClass::DeletedOnBothSides,
            ),
            Some(LoweredMergeAction::ConvergeDeletedOnBothSides),
        ) => LoweredRecordExecutionIntentKind::ConvergeDeletedOnBothSides,
        _ => return None,
    };
    Some(LoweredRecordExecutionBundle {
        kind,
        aspects: Arc::from(Vec::<LoweredRecordExecutionAspectIntent>::new()),
    })
}

fn synthesized_denial_bundle(
    classification: crate::merge::data::MergeConflictClass,
    resolution_class: MergeResolutionClass,
    blocked_reason: Option<LoweredMergeBlockedReason>,
    readiness: MergeExecutionReadiness,
) -> Option<LoweredRecordDenialBundle> {
    let kind = match readiness {
        MergeExecutionReadiness::Admitted => return None,
        MergeExecutionReadiness::Blocked => blocked_reason
            .map(blocked_denial_kind_from_reason)
            .unwrap_or_else(|| {
                blocked_denial_kind_for_record(classification, resolution_class, &[])
            }),
        MergeExecutionReadiness::Rejected => LoweredRecordDenialKind::RejectedPolicy,
    };
    Some(LoweredRecordDenialBundle {
        kind,
        aspects: Arc::from(Vec::<LoweredRecordDenialAspectIntent>::new()),
    })
}

fn blocked_denial_kind_from_reason(reason: LoweredMergeBlockedReason) -> LoweredRecordDenialKind {
    match reason {
        LoweredMergeBlockedReason::MissingVisibleState => {
            LoweredRecordDenialKind::BlockedMissingVisibleState
        }
        LoweredMergeBlockedReason::MissingAncestorValueBasis => {
            LoweredRecordDenialKind::BlockedMissingAncestorValueBasis
        }
        LoweredMergeBlockedReason::UnvalidatedSchemaCorrespondence => {
            LoweredRecordDenialKind::BlockedUnvalidatedSchemaCorrespondence
        }
        LoweredMergeBlockedReason::SourceDeletedTargetLive => {
            LoweredRecordDenialKind::BlockedSourceDeletedTargetLive
        }
        LoweredMergeBlockedReason::SourceLiveTargetDeleted => {
            LoweredRecordDenialKind::BlockedSourceLiveTargetDeleted
        }
        LoweredMergeBlockedReason::DeletedOnBothSides => {
            LoweredRecordDenialKind::BlockedDeletedOnBothSides
        }
        LoweredMergeBlockedReason::DeletedVsModified => {
            LoweredRecordDenialKind::BlockedDeletedVsModified
        }
        LoweredMergeBlockedReason::DeletedVsRewired => {
            LoweredRecordDenialKind::BlockedDeletedVsRewired
        }
        LoweredMergeBlockedReason::RelationEndpointRewiredLocal => {
            LoweredRecordDenialKind::BlockedRelationEndpointRewiredLocal
        }
        LoweredMergeBlockedReason::RelationEndpointRewiredEscalated => {
            LoweredRecordDenialKind::BlockedRelationEndpointRewiredEscalated
        }
        LoweredMergeBlockedReason::TopologyRegionConflict => {
            LoweredRecordDenialKind::BlockedTopologyRegionConflict
        }
        LoweredMergeBlockedReason::StrategyIntentConflictRequiresManualResolution => {
            LoweredRecordDenialKind::BlockedManualResolution
        }
        LoweredMergeBlockedReason::ManualConflictResolutionRequired => {
            LoweredRecordDenialKind::BlockedManualResolution
        }
    }
}

fn summarize_lowered_records(records: Arc<[LoweredMergePlanRecord]>) -> LoweredMergePlanSummary {
    let mut admitted_count = 0;
    let mut blocked_count = 0;
    let mut rejected_count = 0;

    for record in records.iter() {
        match record.readiness {
            MergeExecutionReadiness::Admitted => admitted_count += 1,
            MergeExecutionReadiness::Blocked => blocked_count += 1,
            MergeExecutionReadiness::Rejected => rejected_count += 1,
        }
    }

    LoweredMergePlanSummary {
        record_count: records.len(),
        admitted_count,
        blocked_count,
        rejected_count,
        fully_execution_ready: blocked_count == 0 && rejected_count == 0,
        records,
    }
}

fn aggregate_record_readiness(aspects: &[LoweredAspectOutcome]) -> MergeExecutionReadiness {
    if aspects
        .iter()
        .any(|aspect| aspect.readiness == MergeExecutionReadiness::Rejected)
    {
        MergeExecutionReadiness::Rejected
    } else if aspects
        .iter()
        .any(|aspect| aspect.readiness == MergeExecutionReadiness::Blocked)
    {
        MergeExecutionReadiness::Blocked
    } else {
        MergeExecutionReadiness::Admitted
    }
}

fn lowered_action_for_record(
    classification: crate::merge::data::MergeConflictClass,
    aspect_outcomes: &[LoweredAspectOutcome],
    readiness: MergeExecutionReadiness,
) -> Option<LoweredMergeAction> {
    if readiness != MergeExecutionReadiness::Admitted
        || (!aspect_outcomes.is_empty()
            && aspect_outcomes
                .iter()
                .any(|aspect| aspect.lowered_action.is_none()))
    {
        return None;
    }

    match classification {
        crate::merge::data::MergeConflictClass::SourceOnlyAddition => {
            Some(LoweredMergeAction::KeepSourceAddition)
        }
        crate::merge::data::MergeConflictClass::ExactSharedTruth => {
            Some(LoweredMergeAction::KeepExactSharedTruth)
        }
        crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence => {
            Some(LoweredMergeAction::ReconcileSchemaCorrespondence)
        }
        crate::merge::data::MergeConflictClass::DivergentVisibleState
        | crate::merge::data::MergeConflictClass::StrategyIntentConflict => {
            Some(LoweredMergeAction::ReconcileDivergentVisibleState)
        }
        crate::merge::data::MergeConflictClass::Deletion(
            crate::merge::data::DeletionMergeClass::DeletedOnBothSides,
        ) => Some(LoweredMergeAction::ConvergeDeletedOnBothSides),
        crate::merge::data::MergeConflictClass::Deletion(_)
        | crate::merge::data::MergeConflictClass::RelationEndpointDivergence => None,
    }
}

fn resolution_class_for_record(
    classification: crate::merge::data::MergeConflictClass,
    relation_evidence: Option<&crate::merge::data::RelationConflictEvidence>,
) -> MergeResolutionClass {
    match classification {
        crate::merge::data::MergeConflictClass::SourceOnlyAddition => {
            MergeResolutionClass::SourceOnlyAddition
        }
        crate::merge::data::MergeConflictClass::ExactSharedTruth => {
            MergeResolutionClass::ExactSharedTruth
        }
        crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence => {
            MergeResolutionClass::SchemaDeclaredCorrespondence
        }
        crate::merge::data::MergeConflictClass::Deletion(class) => {
            MergeResolutionClass::Deletion(match class {
                crate::merge::data::DeletionMergeClass::SourceDeletedTargetLive => {
                    DeletionExecutionClass::SourceDeletedTargetLive
                }
                crate::merge::data::DeletionMergeClass::SourceLiveTargetDeleted => {
                    DeletionExecutionClass::SourceLiveTargetDeleted
                }
                crate::merge::data::DeletionMergeClass::DeletedOnBothSides => {
                    DeletionExecutionClass::DeletedOnBothSides
                }
                crate::merge::data::DeletionMergeClass::DeletedVsModified => {
                    DeletionExecutionClass::DeletedVsModified
                }
                crate::merge::data::DeletionMergeClass::DeletedVsRewired => {
                    DeletionExecutionClass::DeletedVsRewired
                }
            })
        }
        crate::merge::data::MergeConflictClass::RelationEndpointDivergence => {
            MergeResolutionClass::Topology(topology_resolution_class_for_record(relation_evidence))
        }
        crate::merge::data::MergeConflictClass::DivergentVisibleState
        | crate::merge::data::MergeConflictClass::StrategyIntentConflict => {
            MergeResolutionClass::DivergentVisibleState
        }
    }
}

fn topology_resolution_class_for_record(
    relation_evidence: Option<&crate::merge::data::RelationConflictEvidence>,
) -> TopologyExecutionClass {
    let admission_policy = crate::merge::logic::policy::current_topology_rewire_admission_policy();
    let Some(evidence) = relation_evidence else {
        return TopologyExecutionClass::TopologyRegionConflict;
    };

    match evidence.propagation {
        crate::merge::data::RelationConflictPropagation::RelationLocalOnly => {
            match evidence.endpoint_continuity {
                crate::merge::data::EndpointContinuityClass::EndpointsStable => {
                    TopologyExecutionClass::RelationEndpointStable
                }
                crate::merge::data::EndpointContinuityClass::SourceEndpointRewired
                | crate::merge::data::EndpointContinuityClass::TargetEndpointRewired
                | crate::merge::data::EndpointContinuityClass::BothEndpointsRewired => {
                    TopologyExecutionClass::RelationEndpointRewiredLocal
                }
            }
        }
        crate::merge::data::RelationConflictPropagation::RelationLocalRewireCandidate => {
            match admission_policy {
                TopologyRewireAdmissionPolicy::AlwaysEscalateToTopologyRegion => {
                    TopologyExecutionClass::RelationEndpointRewiredEscalated
                }
            }
        }
        crate::merge::data::RelationConflictPropagation::EscalatesToTopologyRegionConflict => {
            match admission_policy {
                TopologyRewireAdmissionPolicy::AlwaysEscalateToTopologyRegion => {
                    TopologyExecutionClass::TopologyRegionConflict
                }
            }
        }
    }
}

fn executable_class_for_record(
    resolution_class: MergeResolutionClass,
    readiness: MergeExecutionReadiness,
    execution_bundle_kind: Option<LoweredRecordExecutionIntentKind>,
) -> Option<MergeExecutableClass> {
    if readiness != MergeExecutionReadiness::Admitted {
        return None;
    }
    match (resolution_class, execution_bundle_kind) {
        (
            MergeResolutionClass::SourceOnlyAddition,
            Some(LoweredRecordExecutionIntentKind::AdoptSourceRecord),
        ) => Some(MergeExecutableClass::AdoptSourceRecord),
        (
            MergeResolutionClass::ExactSharedTruth,
            Some(LoweredRecordExecutionIntentKind::PreserveSharedRecord),
        ) => Some(MergeExecutableClass::PreserveSharedRecord),
        (
            MergeResolutionClass::SchemaDeclaredCorrespondence,
            Some(LoweredRecordExecutionIntentKind::ReconcileRecord),
        )
        | (
            MergeResolutionClass::DivergentVisibleState,
            Some(LoweredRecordExecutionIntentKind::ReconcileRecord),
        ) => Some(MergeExecutableClass::ReconcileRecord),
        (
            MergeResolutionClass::Deletion(DeletionExecutionClass::DeletedOnBothSides),
            Some(LoweredRecordExecutionIntentKind::ConvergeDeletedOnBothSides),
        ) => Some(MergeExecutableClass::ConvergeDeletedOnBothSides),
        _ => None,
    }
}

fn blocked_reason_for_record(
    classification: crate::merge::data::MergeConflictClass,
    resolution_class: MergeResolutionClass,
    aspect_outcomes: &[LoweredAspectOutcome],
    readiness: MergeExecutionReadiness,
) -> Option<LoweredMergeBlockedReason> {
    if readiness != MergeExecutionReadiness::Blocked {
        return None;
    }
    if aspect_outcomes.is_empty() {
        return Some(classification_blocked_reason(
            classification,
            resolution_class,
        ));
    }
    if let Some(reason) = aspect_outcomes.iter().find_map(|aspect| {
        aspect
            .blocked_reason
            .filter(|reason| is_deletion_blocked_reason(*reason))
    }) {
        Some(reason)
    } else if let Some(reason) = aspect_outcomes.iter().find_map(|aspect| {
        aspect.blocked_reason.filter(|reason| {
            matches!(
                reason,
                LoweredMergeBlockedReason::RelationEndpointRewiredLocal
                    | LoweredMergeBlockedReason::RelationEndpointRewiredEscalated
                    | LoweredMergeBlockedReason::TopologyRegionConflict
            )
        })
    }) {
        Some(reason)
    } else if aspect_outcomes
        .iter()
        .any(|aspect| aspect.blocked_reason.is_some())
    {
        Some(LoweredMergeBlockedReason::ManualConflictResolutionRequired)
    } else if classification_requires_record_level_blocked_reason(classification) {
        Some(classification_blocked_reason(
            classification,
            resolution_class,
        ))
    } else if classification == crate::merge::data::MergeConflictClass::StrategyIntentConflict {
        Some(LoweredMergeBlockedReason::StrategyIntentConflictRequiresManualResolution)
    } else {
        None
    }
}

fn classification_requires_record_level_blocked_reason(
    classification: crate::merge::data::MergeConflictClass,
) -> bool {
    matches!(
        classification,
        crate::merge::data::MergeConflictClass::Deletion(_)
            | crate::merge::data::MergeConflictClass::RelationEndpointDivergence
    )
}

fn classification_blocked_reason(
    classification: crate::merge::data::MergeConflictClass,
    resolution_class: MergeResolutionClass,
) -> LoweredMergeBlockedReason {
    match classification {
        crate::merge::data::MergeConflictClass::Deletion(class) => {
            blocked_reason_for_deletion_class(class)
        }
        crate::merge::data::MergeConflictClass::RelationEndpointDivergence => {
            blocked_reason_for_topology_resolution_class(resolution_class)
        }
        crate::merge::data::MergeConflictClass::StrategyIntentConflict => {
            LoweredMergeBlockedReason::StrategyIntentConflictRequiresManualResolution
        }
        crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence
        | crate::merge::data::MergeConflictClass::DivergentVisibleState
        | crate::merge::data::MergeConflictClass::ExactSharedTruth
        | crate::merge::data::MergeConflictClass::SourceOnlyAddition => {
            LoweredMergeBlockedReason::ManualConflictResolutionRequired
        }
    }
}

fn execution_bundle_for_record(
    classification: crate::merge::data::MergeConflictClass,
    aspect_outcomes: &[LoweredAspectOutcome],
    readiness: MergeExecutionReadiness,
) -> Option<LoweredRecordExecutionBundle> {
    if readiness != MergeExecutionReadiness::Admitted {
        return None;
    }
    let aspect_intents = aspect_outcomes
        .iter()
        .filter_map(|outcome| {
            Some(LoweredRecordExecutionAspectIntent {
                aspect_key: outcome.aspect_key.clone(),
                intent: outcome.execution_intent?,
            })
        })
        .collect::<Vec<_>>();
    let kind = match classification {
        crate::merge::data::MergeConflictClass::SourceOnlyAddition => {
            LoweredRecordExecutionIntentKind::AdoptSourceRecord
        }
        crate::merge::data::MergeConflictClass::ExactSharedTruth => {
            LoweredRecordExecutionIntentKind::PreserveSharedRecord
        }
        crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence => {
            LoweredRecordExecutionIntentKind::ReconcileRecord
        }
        crate::merge::data::MergeConflictClass::DivergentVisibleState
        | crate::merge::data::MergeConflictClass::StrategyIntentConflict => {
            LoweredRecordExecutionIntentKind::ReconcileRecord
        }
        crate::merge::data::MergeConflictClass::Deletion(
            crate::merge::data::DeletionMergeClass::DeletedOnBothSides,
        ) if aspect_intents.is_empty() => {
            LoweredRecordExecutionIntentKind::ConvergeDeletedOnBothSides
        }
        crate::merge::data::MergeConflictClass::Deletion(_)
        | crate::merge::data::MergeConflictClass::RelationEndpointDivergence => {
            return None;
        }
    };
    Some(LoweredRecordExecutionBundle {
        kind,
        aspects: Arc::from(aspect_intents),
    })
}

fn rejected_reason_for_record(
    aspect_outcomes: &[LoweredAspectOutcome],
    readiness: MergeExecutionReadiness,
) -> Option<LoweredMergeRejectedReason> {
    if readiness != MergeExecutionReadiness::Rejected {
        return None;
    }

    let mut reject_reason: Option<LoweredMergeRejectedReason> = None;
    for aspect in aspect_outcomes
        .iter()
        .filter_map(|aspect| aspect.rejected_reason)
    {
        reject_reason = Some(match reject_reason {
            None => aspect,
            Some(existing) if existing == aspect => existing,
            Some(_) => LoweredMergeRejectedReason::MixedPolicyRejectClasses,
        });
    }
    reject_reason
}

fn denial_bundle_for_record(
    classification: crate::merge::data::MergeConflictClass,
    resolution_class: MergeResolutionClass,
    aspect_outcomes: &[LoweredAspectOutcome],
    readiness: MergeExecutionReadiness,
) -> Option<LoweredRecordDenialBundle> {
    match readiness {
        MergeExecutionReadiness::Admitted => None,
        MergeExecutionReadiness::Blocked => {
            let aspects = aspect_outcomes
                .iter()
                .filter_map(|outcome| {
                    Some(LoweredRecordDenialAspectIntent {
                        aspect_key: outcome.aspect_key.clone(),
                        intent: outcome.denial_intent?,
                    })
                })
                .collect::<Vec<_>>();
            Some(LoweredRecordDenialBundle {
                kind: blocked_denial_kind_for_record(
                    classification,
                    resolution_class,
                    aspects.as_slice(),
                ),
                aspects: Arc::from(aspects),
            })
        }
        MergeExecutionReadiness::Rejected => {
            let aspects = aspect_outcomes
                .iter()
                .filter_map(|outcome| {
                    Some(LoweredRecordDenialAspectIntent {
                        aspect_key: outcome.aspect_key.clone(),
                        intent: outcome.denial_intent?,
                    })
                })
                .collect::<Vec<_>>();
            Some(LoweredRecordDenialBundle {
                kind: rejected_denial_kind_for_record(aspects.as_slice()),
                aspects: Arc::from(aspects),
            })
        }
    }
}

fn rejected_denial_kind_for_record(
    aspects: &[LoweredRecordDenialAspectIntent],
) -> LoweredRecordDenialKind {
    if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::RejectedCustomPolicy)
    {
        LoweredRecordDenialKind::RejectedCustomPolicy
    } else if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::RejectedMixedPolicyClasses)
    {
        LoweredRecordDenialKind::RejectedMixedPolicyClasses
    } else {
        LoweredRecordDenialKind::RejectedPolicy
    }
}

fn blocked_denial_kind_for_record(
    classification: crate::merge::data::MergeConflictClass,
    resolution_class: MergeResolutionClass,
    aspects: &[LoweredRecordDenialAspectIntent],
) -> LoweredRecordDenialKind {
    if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::BlockedMissingVisibleState)
    {
        LoweredRecordDenialKind::BlockedMissingVisibleState
    } else if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::BlockedMissingAncestorValueBasis)
    {
        LoweredRecordDenialKind::BlockedMissingAncestorValueBasis
    } else if aspects.iter().any(|aspect| {
        aspect.intent == LoweredAspectDenialIntent::BlockedUnvalidatedSchemaCorrespondence
    }) {
        LoweredRecordDenialKind::BlockedUnvalidatedSchemaCorrespondence
    } else if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::BlockedSourceDeletedTargetLive)
    {
        LoweredRecordDenialKind::BlockedSourceDeletedTargetLive
    } else if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::BlockedSourceLiveTargetDeleted)
    {
        LoweredRecordDenialKind::BlockedSourceLiveTargetDeleted
    } else if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::BlockedDeletedOnBothSides)
    {
        LoweredRecordDenialKind::BlockedDeletedOnBothSides
    } else if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::BlockedDeletedVsModified)
    {
        LoweredRecordDenialKind::BlockedDeletedVsModified
    } else if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::BlockedDeletedVsRewired)
    {
        LoweredRecordDenialKind::BlockedDeletedVsRewired
    } else if aspects.iter().any(|aspect| {
        aspect.intent == LoweredAspectDenialIntent::BlockedRelationEndpointRewiredLocal
    }) {
        LoweredRecordDenialKind::BlockedRelationEndpointRewiredLocal
    } else if aspects.iter().any(|aspect| {
        aspect.intent == LoweredAspectDenialIntent::BlockedRelationEndpointRewiredEscalated
    }) {
        LoweredRecordDenialKind::BlockedRelationEndpointRewiredEscalated
    } else if aspects
        .iter()
        .any(|aspect| aspect.intent == LoweredAspectDenialIntent::BlockedTopologyRegionConflict)
    {
        LoweredRecordDenialKind::BlockedTopologyRegionConflict
    } else {
        match classification {
            crate::merge::data::MergeConflictClass::Deletion(class) => {
                blocked_denial_kind_from_reason(blocked_reason_for_deletion_class(class))
            }
            crate::merge::data::MergeConflictClass::RelationEndpointDivergence => {
                blocked_denial_kind_from_reason(blocked_reason_for_topology_resolution_class(
                    resolution_class,
                ))
            }
            crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence
            | crate::merge::data::MergeConflictClass::DivergentVisibleState
            | crate::merge::data::MergeConflictClass::StrategyIntentConflict
            | crate::merge::data::MergeConflictClass::ExactSharedTruth
            | crate::merge::data::MergeConflictClass::SourceOnlyAddition => {
                LoweredRecordDenialKind::BlockedManualResolution
            }
        }
    }
}

fn lowered_aspect_outcomes_for_record(
    runtime: &crate::logic::runtime::RelationalRuntime,
    source_record: &crate::merge::data::VisibleMergeRecord,
    policy_record: &crate::merge::data::MergePolicyResolutionRecord,
    resolution_class: MergeResolutionClass,
) -> Result<Vec<LoweredAspectOutcome>, MergePlanningError> {
    let Some(plan) = lowered_plan_for_source_record(runtime, source_record) else {
        return Ok(Vec::new());
    };
    let policy_by_aspect = policy_record
        .aspect_resolutions
        .iter()
        .map(|aspect| (aspect.aspect_key.clone(), aspect))
        .collect::<BTreeMap<_, _>>();

    Ok(plan
        .executable_bindings
        .iter()
        .map(|binding| {
            let aspect_resolution = policy_by_aspect.get(&binding.aspect_key).copied();
            let readiness = aspect_resolution
                .map(|aspect| readiness_for_policy_decision(aspect.decision_boundary))
                .unwrap_or(MergeExecutionReadiness::Blocked);
            let applied_policy = aspect_resolution.and_then(|aspect| aspect.applied_policy.clone());
            LoweredAspectOutcome {
                aspect_key: binding.aspect_key.clone(),
                applied_policy,
                readiness,
                lowered_action: aspect_resolution.and_then(|aspect| {
                    lowered_aspect_action_for_resolution(
                        policy_record.classification,
                        aspect.comparison,
                        readiness,
                    )
                }),
                authorized_values: aspect_resolution.and_then(|aspect| {
                    authorized_values_for_aspect(
                        policy_record.classification,
                        aspect.comparison,
                        readiness,
                    )
                }),
                execution_intent: aspect_resolution.and_then(|aspect| {
                    lowered_aspect_execution_intent(
                        policy_record.classification,
                        aspect.comparison,
                        readiness,
                    )
                }),
                resolved_value_strategy: aspect_resolution
                    .and_then(|aspect| aspect.resolved_value_strategy.clone()),
                denial_intent: aspect_resolution.and_then(|aspect| {
                    lowered_aspect_denial_intent(
                        policy_record.classification,
                        resolution_class,
                        aspect.comparison,
                        aspect.decision_boundary,
                        readiness,
                    )
                }),
                blocked_reason: aspect_resolution.and_then(|aspect| {
                    blocked_reason_for_aspect(
                        policy_record.classification,
                        resolution_class,
                        aspect.comparison,
                        aspect.decision_boundary,
                        readiness,
                    )
                }),
                rejected_reason: aspect_resolution.and_then(|aspect| {
                    rejected_reason_for_aspect(aspect.decision_boundary, readiness)
                }),
            }
        })
        .collect())
}

fn lowered_plan_for_source_record<'a>(
    runtime: &'a crate::logic::runtime::RelationalRuntime,
    source_record: &crate::merge::data::VisibleMergeRecord,
) -> Option<&'a LoweredAspectPlan> {
    let kind_id = source_record.source_kind_id.or(source_record.kind_id)?;
    match source_record.record_kind {
        VisibleMergeRecordKind::Entity => runtime.entity_aspect_plan(kind_id),
        VisibleMergeRecordKind::Relation => runtime.relation_aspect_plan(kind_id),
    }
}

fn readiness_for_policy_decision(
    decision_boundary: MergePolicyDecisionBoundary,
) -> MergeExecutionReadiness {
    match decision_boundary {
        MergePolicyDecisionBoundary::AutoResolved => MergeExecutionReadiness::Admitted,
        MergePolicyDecisionBoundary::RequiresManualResolution { .. } => {
            MergeExecutionReadiness::Blocked
        }
        MergePolicyDecisionBoundary::Reject { .. } => MergeExecutionReadiness::Rejected,
    }
}

fn lowered_aspect_action_for_resolution(
    classification: crate::merge::data::MergeConflictClass,
    comparison: AspectComparisonState,
    readiness: MergeExecutionReadiness,
) -> Option<LoweredAspectAction> {
    if readiness != MergeExecutionReadiness::Admitted {
        return None;
    }
    match (classification, comparison) {
        (_, AspectComparisonState::Unavailable) => None,
        (
            crate::merge::data::MergeConflictClass::SourceOnlyAddition,
            AspectComparisonState::SourceOnly,
        ) => Some(LoweredAspectAction::AdoptSourceAspect),
        (
            crate::merge::data::MergeConflictClass::ExactSharedTruth,
            AspectComparisonState::Equal,
        ) => Some(LoweredAspectAction::KeepSharedAspect),
        (
            crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence
            | crate::merge::data::MergeConflictClass::DivergentVisibleState,
            AspectComparisonState::Equal
            | AspectComparisonState::SourceOnly
            | AspectComparisonState::TargetOnly
            | AspectComparisonState::Divergent,
        ) => Some(LoweredAspectAction::ReconcileCorrespondedAspect),
        _ => None,
    }
}

fn lowered_aspect_denial_intent(
    classification: crate::merge::data::MergeConflictClass,
    resolution_class: MergeResolutionClass,
    comparison: AspectComparisonState,
    decision_boundary: MergePolicyDecisionBoundary,
    readiness: MergeExecutionReadiness,
) -> Option<LoweredAspectDenialIntent> {
    match readiness {
        MergeExecutionReadiness::Admitted => None,
        MergeExecutionReadiness::Blocked => match blocked_reason_for_aspect(
            classification,
            resolution_class,
            comparison,
            decision_boundary,
            readiness,
        )? {
            LoweredMergeBlockedReason::MissingVisibleState => {
                Some(LoweredAspectDenialIntent::BlockedMissingVisibleState)
            }
            LoweredMergeBlockedReason::MissingAncestorValueBasis => {
                Some(LoweredAspectDenialIntent::BlockedMissingAncestorValueBasis)
            }
            LoweredMergeBlockedReason::UnvalidatedSchemaCorrespondence => {
                Some(LoweredAspectDenialIntent::BlockedUnvalidatedSchemaCorrespondence)
            }
            LoweredMergeBlockedReason::SourceDeletedTargetLive => {
                Some(LoweredAspectDenialIntent::BlockedSourceDeletedTargetLive)
            }
            LoweredMergeBlockedReason::SourceLiveTargetDeleted => {
                Some(LoweredAspectDenialIntent::BlockedSourceLiveTargetDeleted)
            }
            LoweredMergeBlockedReason::DeletedOnBothSides => {
                Some(LoweredAspectDenialIntent::BlockedDeletedOnBothSides)
            }
            LoweredMergeBlockedReason::DeletedVsModified => {
                Some(LoweredAspectDenialIntent::BlockedDeletedVsModified)
            }
            LoweredMergeBlockedReason::DeletedVsRewired => {
                Some(LoweredAspectDenialIntent::BlockedDeletedVsRewired)
            }
            LoweredMergeBlockedReason::RelationEndpointRewiredLocal => {
                Some(LoweredAspectDenialIntent::BlockedRelationEndpointRewiredLocal)
            }
            LoweredMergeBlockedReason::RelationEndpointRewiredEscalated => {
                Some(LoweredAspectDenialIntent::BlockedRelationEndpointRewiredEscalated)
            }
            LoweredMergeBlockedReason::TopologyRegionConflict => {
                Some(LoweredAspectDenialIntent::BlockedTopologyRegionConflict)
            }
            LoweredMergeBlockedReason::StrategyIntentConflictRequiresManualResolution => {
                Some(LoweredAspectDenialIntent::BlockedStrategyIntentConflict)
            }
            LoweredMergeBlockedReason::ManualConflictResolutionRequired => {
                Some(LoweredAspectDenialIntent::BlockedManualResolution)
            }
        },
        MergeExecutionReadiness::Rejected => {
            rejected_reason_for_aspect(decision_boundary, readiness)?;
            Some(match decision_boundary {
                MergePolicyDecisionBoundary::Reject {
                    class: crate::merge::data::MergePolicyRejectClass::BuiltInFailOnConflict,
                } => LoweredAspectDenialIntent::RejectedPolicy,
                MergePolicyDecisionBoundary::Reject {
                    class:
                        crate::merge::data::MergePolicyRejectClass::LastWriterWinsCausalConflict
                        | crate::merge::data::MergePolicyRejectClass::InvalidBuiltInPolicyValueShape,
                } => LoweredAspectDenialIntent::RejectedPolicy,
                MergePolicyDecisionBoundary::Reject {
                    class: crate::merge::data::MergePolicyRejectClass::CustomPolicyRejected,
                } => LoweredAspectDenialIntent::RejectedCustomPolicy,
                MergePolicyDecisionBoundary::Reject {
                    class: crate::merge::data::MergePolicyRejectClass::MixedAspectRejectClasses,
                } => LoweredAspectDenialIntent::RejectedMixedPolicyClasses,
                _ => LoweredAspectDenialIntent::RejectedPolicy,
            })
        }
    }
}

fn authorized_values_for_aspect(
    classification: crate::merge::data::MergeConflictClass,
    comparison: AspectComparisonState,
    readiness: MergeExecutionReadiness,
) -> Option<AuthorizedAspectValueSurface> {
    if readiness != MergeExecutionReadiness::Admitted {
        return None;
    }
    match (classification, comparison) {
        (
            crate::merge::data::MergeConflictClass::SourceOnlyAddition,
            AspectComparisonState::SourceOnly,
        ) => Some(AuthorizedAspectValueSurface {
            source: AuthorizedAspectValueUsage::ConsumeVisibleValue,
            target: AuthorizedAspectValueUsage::NotAuthorized,
            base: AuthorizedAspectValueUsage::NotAuthorized,
        }),
        (
            crate::merge::data::MergeConflictClass::ExactSharedTruth,
            AspectComparisonState::Equal,
        ) => Some(AuthorizedAspectValueSurface {
            source: AuthorizedAspectValueUsage::EqualityWitnessOnly,
            target: AuthorizedAspectValueUsage::EqualityWitnessOnly,
            base: AuthorizedAspectValueUsage::NotAuthorized,
        }),
        (
            crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence
            | crate::merge::data::MergeConflictClass::DivergentVisibleState,
            AspectComparisonState::Equal
            | AspectComparisonState::SourceOnly
            | AspectComparisonState::TargetOnly
            | AspectComparisonState::Divergent,
        ) => Some(AuthorizedAspectValueSurface {
            source: AuthorizedAspectValueUsage::ConsumeVisibleValue,
            target: AuthorizedAspectValueUsage::ConsumeVisibleValue,
            base: AuthorizedAspectValueUsage::ConsumeBaseValue,
        }),
        _ => None,
    }
}

fn lowered_aspect_execution_intent(
    classification: crate::merge::data::MergeConflictClass,
    comparison: AspectComparisonState,
    readiness: MergeExecutionReadiness,
) -> Option<LoweredAspectExecutionIntent> {
    let authorized_values = authorized_values_for_aspect(classification, comparison, readiness)?;
    match (classification, comparison) {
        (
            crate::merge::data::MergeConflictClass::SourceOnlyAddition,
            AspectComparisonState::SourceOnly,
        ) => Some(LoweredAspectExecutionIntent::AdoptSourceValue { authorized_values }),
        (
            crate::merge::data::MergeConflictClass::ExactSharedTruth,
            AspectComparisonState::Equal,
        ) => Some(LoweredAspectExecutionIntent::PreserveSharedValue { authorized_values }),
        (
            crate::merge::data::MergeConflictClass::SchemaDeclaredCorrespondence
            | crate::merge::data::MergeConflictClass::DivergentVisibleState,
            AspectComparisonState::Equal
            | AspectComparisonState::SourceOnly
            | AspectComparisonState::TargetOnly
            | AspectComparisonState::Divergent,
        ) => Some(LoweredAspectExecutionIntent::ReconcileVisibleValues { authorized_values }),
        _ => None,
    }
}

fn blocked_reason_for_aspect(
    classification: crate::merge::data::MergeConflictClass,
    resolution_class: MergeResolutionClass,
    comparison: AspectComparisonState,
    decision_boundary: MergePolicyDecisionBoundary,
    readiness: MergeExecutionReadiness,
) -> Option<LoweredMergeBlockedReason> {
    if readiness != MergeExecutionReadiness::Blocked {
        return None;
    }
    if let MergePolicyDecisionBoundary::RequiresManualResolution { class } = decision_boundary {
        match class {
            crate::merge::data::MergeManualResolutionClass::MissingVisibleState => {
                return Some(LoweredMergeBlockedReason::MissingVisibleState);
            }
            crate::merge::data::MergeManualResolutionClass::MissingAncestorValueBasis => {
                return Some(LoweredMergeBlockedReason::MissingAncestorValueBasis);
            }
            crate::merge::data::MergeManualResolutionClass::UnvalidatedSchemaCorrespondence => {
                return Some(LoweredMergeBlockedReason::UnvalidatedSchemaCorrespondence);
            }
            crate::merge::data::MergeManualResolutionClass::StrategyIntentConflict => {
                return Some(
                    LoweredMergeBlockedReason::StrategyIntentConflictRequiresManualResolution,
                );
            }
            crate::merge::data::MergeManualResolutionClass::GenericRuntimeConflict
            | crate::merge::data::MergeManualResolutionClass::MixedAspectManualResolution => {}
        }
    }
    match (classification, comparison) {
        (crate::merge::data::MergeConflictClass::StrategyIntentConflict, _) => {
            Some(LoweredMergeBlockedReason::StrategyIntentConflictRequiresManualResolution)
        }
        (crate::merge::data::MergeConflictClass::Deletion(class), _) => {
            Some(blocked_reason_for_deletion_class(class))
        }
        (
            crate::merge::data::MergeConflictClass::RelationEndpointDivergence,
            AspectComparisonState::Divergent
            | AspectComparisonState::TargetOnly
            | AspectComparisonState::SourceOnly,
        ) => Some(blocked_reason_for_topology_resolution_class(
            resolution_class,
        )),
        (_, AspectComparisonState::Unavailable) => {
            Some(LoweredMergeBlockedReason::ManualConflictResolutionRequired)
        }
        _ => Some(LoweredMergeBlockedReason::ManualConflictResolutionRequired),
    }
}

fn blocked_reason_for_deletion_class(
    class: crate::merge::data::DeletionMergeClass,
) -> LoweredMergeBlockedReason {
    match class {
        crate::merge::data::DeletionMergeClass::SourceDeletedTargetLive => {
            LoweredMergeBlockedReason::SourceDeletedTargetLive
        }
        crate::merge::data::DeletionMergeClass::SourceLiveTargetDeleted => {
            LoweredMergeBlockedReason::SourceLiveTargetDeleted
        }
        crate::merge::data::DeletionMergeClass::DeletedOnBothSides => {
            LoweredMergeBlockedReason::DeletedOnBothSides
        }
        crate::merge::data::DeletionMergeClass::DeletedVsModified => {
            LoweredMergeBlockedReason::DeletedVsModified
        }
        crate::merge::data::DeletionMergeClass::DeletedVsRewired => {
            LoweredMergeBlockedReason::DeletedVsRewired
        }
    }
}

fn blocked_reason_for_topology_resolution_class(
    resolution_class: MergeResolutionClass,
) -> LoweredMergeBlockedReason {
    match resolution_class {
        MergeResolutionClass::Topology(TopologyExecutionClass::RelationEndpointStable) => {
            LoweredMergeBlockedReason::ManualConflictResolutionRequired
        }
        MergeResolutionClass::Topology(TopologyExecutionClass::RelationEndpointRewiredLocal) => {
            LoweredMergeBlockedReason::RelationEndpointRewiredLocal
        }
        MergeResolutionClass::Topology(
            TopologyExecutionClass::RelationEndpointRewiredEscalated,
        ) => LoweredMergeBlockedReason::RelationEndpointRewiredEscalated,
        MergeResolutionClass::Topology(TopologyExecutionClass::TopologyRegionConflict) => {
            LoweredMergeBlockedReason::TopologyRegionConflict
        }
        _ => LoweredMergeBlockedReason::ManualConflictResolutionRequired,
    }
}

fn is_deletion_blocked_reason(reason: LoweredMergeBlockedReason) -> bool {
    matches!(
        reason,
        LoweredMergeBlockedReason::SourceDeletedTargetLive
            | LoweredMergeBlockedReason::SourceLiveTargetDeleted
            | LoweredMergeBlockedReason::DeletedOnBothSides
            | LoweredMergeBlockedReason::DeletedVsModified
            | LoweredMergeBlockedReason::DeletedVsRewired
    )
}

fn rejected_reason_for_aspect(
    decision_boundary: MergePolicyDecisionBoundary,
    readiness: MergeExecutionReadiness,
) -> Option<LoweredMergeRejectedReason> {
    (readiness == MergeExecutionReadiness::Rejected
        && matches!(
            decision_boundary,
            MergePolicyDecisionBoundary::Reject { .. }
        ))
    .then_some(match decision_boundary {
        MergePolicyDecisionBoundary::Reject {
            class: crate::merge::data::MergePolicyRejectClass::BuiltInFailOnConflict,
        } => LoweredMergeRejectedReason::FailOnConflictPolicy,
        MergePolicyDecisionBoundary::Reject {
            class:
                crate::merge::data::MergePolicyRejectClass::LastWriterWinsCausalConflict
                | crate::merge::data::MergePolicyRejectClass::InvalidBuiltInPolicyValueShape,
        } => LoweredMergeRejectedReason::FailOnConflictPolicy,
        MergePolicyDecisionBoundary::Reject {
            class: crate::merge::data::MergePolicyRejectClass::CustomPolicyRejected,
        } => LoweredMergeRejectedReason::CustomPolicyRejected,
        MergePolicyDecisionBoundary::Reject {
            class: crate::merge::data::MergePolicyRejectClass::MixedAspectRejectClasses,
        } => LoweredMergeRejectedReason::MixedPolicyRejectClasses,
        _ => LoweredMergeRejectedReason::FailOnConflictPolicy,
    })
}

fn build_decision_log(lowered_records: &[LoweredMergePlanRecord]) -> MergePlanningDecisionLog {
    let mut decisions = lowered_records
        .iter()
        .map(|record| crate::merge::data::MergePlanningDecisionRecord {
            record: record.record.clone(),
            target_record: record.target_record.clone(),
            decision: match record.readiness {
                MergeExecutionReadiness::Admitted => MergePlanningDecisionKind::Admitted,
                MergeExecutionReadiness::Blocked => MergePlanningDecisionKind::Blocked,
                MergeExecutionReadiness::Rejected => MergePlanningDecisionKind::Rejected,
            },
            classification: record.classification,
            causal_disposition: record.causal_disposition,
            policy_proof_boundary: record.policy_proof_boundary,
        })
        .collect::<Vec<_>>();
    decisions.sort_by(|left, right| {
        left.decision
            .cmp(&right.decision)
            .then(left.record.cmp(&right.record))
            .then(left.target_record.cmp(&right.target_record))
    });
    MergePlanningDecisionLog {
        decisions: Arc::from(decisions),
    }
}

fn build_decision_log_digest_basis(
    decision_log: &MergePlanningDecisionLog,
) -> MergePlanningDecisionLogDigestBasis {
    MergePlanningDecisionLogDigestBasis {
        canonical_decisions: Arc::from(
            decision_log
                .decisions
                .iter()
                .map(|decision| decision.decision)
                .collect::<Vec<_>>(),
        ),
        canonical_records: Arc::from(
            decision_log
                .decisions
                .iter()
                .map(|decision| decision.record.clone())
                .collect::<Vec<_>>(),
        ),
        canonical_target_records: Arc::from(
            decision_log
                .decisions
                .iter()
                .map(|decision| decision.target_record.clone())
                .collect::<Vec<_>>(),
        ),
        canonical_classifications: Arc::from(
            decision_log
                .decisions
                .iter()
                .map(|decision| decision.classification)
                .collect::<Vec<_>>(),
        ),
        canonical_causal_dispositions: Arc::from(
            decision_log
                .decisions
                .iter()
                .map(|decision| decision.causal_disposition)
                .collect::<Vec<_>>(),
        ),
        canonical_policy_proof_boundaries: Arc::from(
            decision_log
                .decisions
                .iter()
                .map(|decision| decision.policy_proof_boundary)
                .collect::<Vec<_>>(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        blocked_denial_kind_from_reason, blocked_reason_for_aspect,
        blocked_reason_for_deletion_class, executable_class_for_record, is_deletion_blocked_reason,
        rejected_reason_for_aspect,
    };
    use crate::merge::data::{
        AspectComparisonState, DeletionExecutionClass, DeletionMergeClass,
        LoweredMergeBlockedReason, LoweredMergeRejectedReason, LoweredRecordDenialKind,
        LoweredRecordExecutionIntentKind, MergeExecutableClass, MergeExecutionReadiness,
        MergeManualResolutionClass, MergePolicyDecisionBoundary, MergePolicyRejectClass,
        MergeResolutionClass,
    };

    #[test]
    fn deletion_classes_map_to_distinct_blocked_reasons() {
        assert_eq!(
            blocked_reason_for_deletion_class(DeletionMergeClass::SourceDeletedTargetLive),
            LoweredMergeBlockedReason::SourceDeletedTargetLive
        );
        assert_eq!(
            blocked_reason_for_deletion_class(DeletionMergeClass::SourceLiveTargetDeleted),
            LoweredMergeBlockedReason::SourceLiveTargetDeleted
        );
        assert_eq!(
            blocked_reason_for_deletion_class(DeletionMergeClass::DeletedOnBothSides),
            LoweredMergeBlockedReason::DeletedOnBothSides
        );
        assert_eq!(
            blocked_reason_for_deletion_class(DeletionMergeClass::DeletedVsModified),
            LoweredMergeBlockedReason::DeletedVsModified
        );
        assert_eq!(
            blocked_reason_for_deletion_class(DeletionMergeClass::DeletedVsRewired),
            LoweredMergeBlockedReason::DeletedVsRewired
        );
    }

    #[test]
    fn deletion_blocked_reasons_map_to_distinct_denial_kinds() {
        assert_eq!(
            blocked_denial_kind_from_reason(LoweredMergeBlockedReason::SourceDeletedTargetLive),
            LoweredRecordDenialKind::BlockedSourceDeletedTargetLive
        );
        assert_eq!(
            blocked_denial_kind_from_reason(LoweredMergeBlockedReason::SourceLiveTargetDeleted),
            LoweredRecordDenialKind::BlockedSourceLiveTargetDeleted
        );
        assert_eq!(
            blocked_denial_kind_from_reason(LoweredMergeBlockedReason::DeletedOnBothSides),
            LoweredRecordDenialKind::BlockedDeletedOnBothSides
        );
        assert_eq!(
            blocked_denial_kind_from_reason(LoweredMergeBlockedReason::DeletedVsModified),
            LoweredRecordDenialKind::BlockedDeletedVsModified
        );
        assert_eq!(
            blocked_denial_kind_from_reason(LoweredMergeBlockedReason::DeletedVsRewired),
            LoweredRecordDenialKind::BlockedDeletedVsRewired
        );
        assert_eq!(
            blocked_denial_kind_from_reason(LoweredMergeBlockedReason::TopologyRegionConflict),
            LoweredRecordDenialKind::BlockedTopologyRegionConflict
        );
        assert_eq!(
            blocked_denial_kind_from_reason(LoweredMergeBlockedReason::MissingVisibleState),
            LoweredRecordDenialKind::BlockedMissingVisibleState
        );
        assert_eq!(
            blocked_denial_kind_from_reason(
                LoweredMergeBlockedReason::UnvalidatedSchemaCorrespondence
            ),
            LoweredRecordDenialKind::BlockedUnvalidatedSchemaCorrespondence
        );
    }

    #[test]
    fn deletion_reason_detector_is_specific_to_deletion_blocking() {
        assert!(is_deletion_blocked_reason(
            LoweredMergeBlockedReason::DeletedVsModified
        ));
        assert!(!is_deletion_blocked_reason(
            LoweredMergeBlockedReason::ManualConflictResolutionRequired
        ));
        assert!(!is_deletion_blocked_reason(
            LoweredMergeBlockedReason::TopologyRegionConflict
        ));
    }

    #[test]
    fn deleted_on_both_sides_maps_to_explicit_executable_class_when_admitted() {
        assert_eq!(
            executable_class_for_record(
                MergeResolutionClass::Deletion(DeletionExecutionClass::DeletedOnBothSides),
                MergeExecutionReadiness::Admitted,
                Some(LoweredRecordExecutionIntentKind::ConvergeDeletedOnBothSides),
            ),
            Some(MergeExecutableClass::ConvergeDeletedOnBothSides)
        );
    }

    #[test]
    fn blocked_reason_for_aspect_preserves_specific_manual_resolution_class() {
        assert_eq!(
            blocked_reason_for_aspect(
                crate::merge::data::MergeConflictClass::DivergentVisibleState,
                MergeResolutionClass::DivergentVisibleState,
                AspectComparisonState::Unavailable,
                MergePolicyDecisionBoundary::RequiresManualResolution {
                    class: MergeManualResolutionClass::MissingVisibleState,
                },
                MergeExecutionReadiness::Blocked,
            ),
            Some(LoweredMergeBlockedReason::MissingVisibleState)
        );
    }

    #[test]
    fn rejected_reason_for_aspect_preserves_specific_reject_class() {
        assert_eq!(
            rejected_reason_for_aspect(
                MergePolicyDecisionBoundary::Reject {
                    class: MergePolicyRejectClass::BuiltInFailOnConflict,
                },
                MergeExecutionReadiness::Rejected,
            ),
            Some(LoweredMergeRejectedReason::FailOnConflictPolicy)
        );
        assert_eq!(
            rejected_reason_for_aspect(
                MergePolicyDecisionBoundary::Reject {
                    class: MergePolicyRejectClass::CustomPolicyRejected,
                },
                MergeExecutionReadiness::Rejected,
            ),
            Some(LoweredMergeRejectedReason::CustomPolicyRejected)
        );
    }
}
