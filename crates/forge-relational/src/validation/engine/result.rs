use crate::publication::data::{PublicationError, PublicationStage};
use crate::transactions::data::{CommitConflict, ConflictClass};
use crate::validation::data::{
    InvariantCheckResult, InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect,
    InvariantGroupSet, InvariantPlanContract, InvariantVerdict, InvariantViolation,
};
use crate::{
    authority::commit::preparation::diagnostics::failures::PreparationFailureClass,
    authority::commit::preparation::planning::strategy::PreparationStrategy,
    logic::planning::RelationalExecutionModel,
};
use serde::{Deserialize, Serialize};

use super::observation::InvariantObservationKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantPlanScopeClass {
    TouchedScope,
    PartitionScope,
    BroaderScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantScopeWideningCause {
    AllObservedPartitionScope,
    FullObservedReadSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantProofBoundarySummary {
    scope_class: InvariantPlanScopeClass,
    widened_causes: Vec<InvariantScopeWideningCause>,
    packet_count: usize,
    touched_partition_count: usize,
}

impl InvariantProofBoundarySummary {
    pub(crate) fn new(
        scope_class: InvariantPlanScopeClass,
        widened_causes: Vec<InvariantScopeWideningCause>,
        packet_count: usize,
        touched_partition_count: usize,
    ) -> Self {
        Self {
            scope_class,
            widened_causes,
            packet_count,
            touched_partition_count,
        }
    }

    pub fn scope_class(&self) -> InvariantPlanScopeClass {
        self.scope_class
    }

    pub fn widened_causes(&self) -> &[InvariantScopeWideningCause] {
        &self.widened_causes
    }

    pub fn packet_count(&self) -> usize {
        self.packet_count
    }

    pub fn touched_partition_count(&self) -> usize {
        self.touched_partition_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantExecutionDisposition {
    Executed,
    SkippedByPlanContract,
    SkippedByMayBreakMask,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantExecutionMetadata {
    execution_point: InvariantExecutionPoint,
    observation_kind: InvariantObservationKind,
    version_id: crate::identity::data::VersionId,
    current_version_id: crate::identity::data::VersionId,
    consumed_groups: InvariantGroupSet,
    applicable_groups: InvariantGroupSet,
    max_cost: InvariantCostClass,
    disposition: InvariantExecutionDisposition,
    plan_contract: Option<InvariantPlanContract>,
    has_merged_plan: bool,
    execution_model: RelationalExecutionModel,
    preparation_strategy: Option<PreparationStrategy>,
    preparation_failures: Vec<PreparationFailureClass>,
    proof_boundary: Option<InvariantProofBoundarySummary>,
}

impl InvariantExecutionMetadata {
    pub(crate) fn new(
        execution_point: InvariantExecutionPoint,
        observation_kind: InvariantObservationKind,
        version_id: crate::identity::data::VersionId,
        current_version_id: crate::identity::data::VersionId,
        consumed_groups: InvariantGroupSet,
        applicable_groups: InvariantGroupSet,
        max_cost: InvariantCostClass,
        disposition: InvariantExecutionDisposition,
        plan_contract: Option<InvariantPlanContract>,
        has_merged_plan: bool,
        execution_model: RelationalExecutionModel,
        preparation_strategy: Option<PreparationStrategy>,
        preparation_failures: Vec<PreparationFailureClass>,
        proof_boundary: Option<InvariantProofBoundarySummary>,
    ) -> Self {
        Self {
            execution_point,
            observation_kind,
            version_id,
            current_version_id,
            consumed_groups,
            applicable_groups,
            max_cost,
            disposition,
            plan_contract,
            has_merged_plan,
            execution_model,
            preparation_strategy,
            preparation_failures,
            proof_boundary,
        }
    }

    pub(crate) fn executed_with_strategy(
        execution_point: InvariantExecutionPoint,
        observation_kind: InvariantObservationKind,
        version_id: crate::identity::data::VersionId,
        current_version_id: crate::identity::data::VersionId,
        consumed_groups: InvariantGroupSet,
        applicable_groups: InvariantGroupSet,
        max_cost: InvariantCostClass,
        plan_contract: Option<InvariantPlanContract>,
        has_merged_plan: bool,
        preparation_strategy: PreparationStrategy,
        preparation_failures: Vec<PreparationFailureClass>,
        proof_boundary: Option<InvariantProofBoundarySummary>,
    ) -> Self {
        Self::new(
            execution_point,
            observation_kind,
            version_id,
            current_version_id,
            consumed_groups,
            applicable_groups,
            max_cost,
            InvariantExecutionDisposition::Executed,
            plan_contract,
            has_merged_plan,
            match preparation_strategy.selected_mode {
                crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection::Serial => {
                    RelationalExecutionModel::SerialAuthority
                }
                crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection::StagedParallel => {
                    RelationalExecutionModel::StagedParallelPreparation
                }
            },
            Some(preparation_strategy),
            preparation_failures,
            proof_boundary,
        )
    }

    pub fn execution_point(&self) -> InvariantExecutionPoint {
        self.execution_point
    }

    pub fn observation_kind(&self) -> InvariantObservationKind {
        self.observation_kind
    }

    pub fn version_id(&self) -> crate::identity::data::VersionId {
        self.version_id
    }

    pub fn current_version_id(&self) -> crate::identity::data::VersionId {
        self.current_version_id
    }

    pub fn consumed_groups(&self) -> InvariantGroupSet {
        self.consumed_groups
    }

    pub fn applicable_groups(&self) -> InvariantGroupSet {
        self.applicable_groups
    }

    pub fn max_cost(&self) -> InvariantCostClass {
        self.max_cost
    }

    pub fn disposition(&self) -> InvariantExecutionDisposition {
        self.disposition
    }

    pub fn plan_contract(&self) -> Option<InvariantPlanContract> {
        self.plan_contract
    }

    pub fn has_merged_plan(&self) -> bool {
        self.has_merged_plan
    }

    pub fn execution_model(&self) -> RelationalExecutionModel {
        self.execution_model
    }

    pub fn preparation_strategy(&self) -> Option<PreparationStrategy> {
        self.preparation_strategy
    }

    pub fn proof_boundary(&self) -> Option<&InvariantProofBoundarySummary> {
        self.proof_boundary.as_ref()
    }

    pub(crate) fn preparation_failures(&self) -> &[PreparationFailureClass] {
        &self.preparation_failures
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantFailure {
    execution_point: InvariantExecutionPoint,
    effect: InvariantFailureEffect,
    violation: InvariantViolation,
}

impl InvariantFailure {
    pub fn code(&self) -> crate::diagnostics::data::DiagnosticCode {
        self.violation.code
    }

    pub fn execution_point(&self) -> InvariantExecutionPoint {
        self.execution_point
    }

    pub fn effect(&self) -> InvariantFailureEffect {
        self.effect
    }

    pub fn violation(&self) -> &InvariantViolation {
        &self.violation
    }

    pub fn detail(&self) -> &str {
        &self.violation.detail
    }

    pub fn fields(&self) -> serde_json::Value {
        self.violation.fields_json()
    }

    pub fn into_commit_conflict(self) -> CommitConflict {
        CommitConflict::new(ConflictClass::InvariantViolation {
            code: self.violation.code,
            detail: self.violation.detail,
            fields: self.violation.fields.to_json_value(),
        })
    }

    pub fn into_publication_error(self, stage: PublicationStage) -> PublicationError {
        PublicationError::new(stage, self.violation.detail)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantExecutionResult {
    metadata: InvariantExecutionMetadata,
    summary: InvariantExecutionSummary,
    results: Vec<InvariantCheckResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantExecutionSummary {
    result_count: usize,
    advisory_count: usize,
    violation_count: usize,
    blocking_failure: Option<InvariantFailure>,
    publication_failure: Option<InvariantFailure>,
}

impl InvariantExecutionSummary {
    fn from_results(results: &[InvariantCheckResult]) -> Self {
        let mut advisory_count = 0;
        let mut violation_count = 0;
        let mut blocking_failure = None;
        let mut publication_failure = None;

        for result in results {
            match &result.verdict {
                InvariantVerdict::Pass => {}
                InvariantVerdict::Advisory { .. } => {
                    advisory_count += 1;
                }
                InvariantVerdict::Violation(violation) => {
                    violation_count += 1;
                    let failure = InvariantFailure {
                        execution_point: result.execution_point,
                        effect: result.failure_effect,
                        violation: violation.clone(),
                    };
                    match result.failure_effect {
                        InvariantFailureEffect::BlockCommit => {
                            if blocking_failure.is_none() {
                                blocking_failure = Some(failure);
                            }
                        }
                        InvariantFailureEffect::BlockPublication => {
                            if publication_failure.is_none() {
                                publication_failure = Some(failure);
                            }
                        }
                        InvariantFailureEffect::AuditOnly => {}
                    }
                }
            }
        }

        Self {
            result_count: results.len(),
            advisory_count,
            violation_count,
            blocking_failure,
            publication_failure,
        }
    }

    pub fn result_count(&self) -> usize {
        self.result_count
    }

    pub fn advisory_count(&self) -> usize {
        self.advisory_count
    }

    pub fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn blocking_failure(&self) -> Option<&InvariantFailure> {
        self.blocking_failure.as_ref()
    }

    pub fn blocking_failures(&self, results: &[InvariantCheckResult]) -> Vec<InvariantFailure> {
        results
            .iter()
            .filter_map(|result| match &result.verdict {
                InvariantVerdict::Violation(violation)
                    if result.failure_effect == InvariantFailureEffect::BlockCommit =>
                {
                    Some(InvariantFailure {
                        execution_point: result.execution_point,
                        effect: result.failure_effect,
                        violation: violation.clone(),
                    })
                }
                _ => None,
            })
            .collect()
    }

    pub fn publication_failure(&self) -> Option<&InvariantFailure> {
        self.publication_failure.as_ref()
    }

    pub fn publication_failures(
        &self,
        results: &[InvariantCheckResult],
    ) -> Vec<InvariantFailure> {
        results
            .iter()
            .filter_map(|result| match &result.verdict {
                InvariantVerdict::Violation(violation)
                    if result.failure_effect == InvariantFailureEffect::BlockPublication =>
                {
                    Some(InvariantFailure {
                        execution_point: result.execution_point,
                        effect: result.failure_effect,
                        violation: violation.clone(),
                    })
                }
                _ => None,
            })
            .collect()
    }

    pub fn has_blocking_violation(&self) -> bool {
        self.blocking_failure.is_some()
    }

    pub fn has_publication_violation(&self) -> bool {
        self.publication_failure.is_some()
    }
}

impl InvariantExecutionResult {
    pub fn executed(
        metadata: InvariantExecutionMetadata,
        results: Vec<InvariantCheckResult>,
    ) -> Self {
        assert_eq!(
            metadata.disposition(),
            InvariantExecutionDisposition::Executed,
            "executed invariant results require an executed disposition",
        );
        let summary = InvariantExecutionSummary::from_results(&results);
        Self {
            metadata,
            summary,
            results,
        }
    }

    pub fn skipped(metadata: InvariantExecutionMetadata) -> Self {
        assert_ne!(
            metadata.disposition(),
            InvariantExecutionDisposition::Executed,
            "skipped invariant results require a skipped disposition",
        );
        Self {
            metadata,
            summary: InvariantExecutionSummary::from_results(&[]),
            results: Vec::new(),
        }
    }

    pub fn metadata(&self) -> &InvariantExecutionMetadata {
        &self.metadata
    }

    pub fn summary(&self) -> &InvariantExecutionSummary {
        &self.summary
    }

    pub fn results(&self) -> &[InvariantCheckResult] {
        &self.results
    }

    pub fn blocking_failures(&self) -> Vec<InvariantFailure> {
        self.summary.blocking_failures(&self.results)
    }

    pub fn publication_failures(&self) -> Vec<InvariantFailure> {
        self.summary.publication_failures(&self.results)
    }

    pub fn into_results(self) -> Vec<InvariantCheckResult> {
        self.results
    }
}

#[cfg(test)]
mod tests {
    use super::InvariantFailure;
    use crate::diagnostics::data::DiagnosticCode;
    use crate::publication::data::PublicationStage;
    use crate::validation::data::{
        InvariantClass, InvariantCostClass, InvariantFailureEffect, InvariantViolation,
        InvariantViolationFields,
    };
    use crate::validation::engine::{
        InvariantExecutionDisposition, InvariantExecutionMetadata, InvariantObservationKind,
        InvariantPlanScopeClass, InvariantProofBoundarySummary, InvariantScopeWideningCause,
    };

    #[test]
    fn invariant_failure_converts_to_commit_and_publication_errors() {
        let failure = InvariantFailure {
            execution_point: crate::validation::data::InvariantExecutionPoint::SnapshotPublication,
            effect: InvariantFailureEffect::BlockPublication,
            violation: InvariantViolation {
                class: InvariantClass::SnapshotAudit,
                code: DiagnosticCode::InvariantViolation,
                detail: "detail".to_string(),
                fields: InvariantViolationFields::None,
            },
        };

        let conflict = failure.clone().into_commit_conflict();
        assert_eq!(conflict.code(), DiagnosticCode::InvariantViolation);
        assert_eq!(conflict.detail(), "detail".to_string());

        let publication = failure.into_publication_error(PublicationStage::InvariantCheck);
        assert_eq!(publication.stage, PublicationStage::InvariantCheck);
        assert_eq!(publication.detail, "detail".to_string());
    }

    #[test]
    fn skipped_result_retains_execution_metadata_without_checks() {
        let metadata = InvariantExecutionMetadata::new(
            crate::validation::data::InvariantExecutionPoint::CommitBoundary,
            InvariantObservationKind::Committed,
            crate::identity::data::VersionId(2),
            crate::identity::data::VersionId(1),
            crate::validation::data::InvariantGroupSet::from_mask(0b111),
            crate::validation::data::InvariantGroupSet::empty(),
            InvariantCostClass::Partition,
            InvariantExecutionDisposition::SkippedByMayBreakMask,
            None,
            true,
            crate::logic::planning::RelationalExecutionModel::SerialAuthority,
            None,
            Vec::new(),
            Some(InvariantProofBoundarySummary::new(
                InvariantPlanScopeClass::BroaderScope,
                vec![InvariantScopeWideningCause::AllObservedPartitionScope],
                1,
                0,
            )),
        );

        let result = crate::validation::engine::InvariantExecutionResult::skipped(metadata);

        assert!(result.results().is_empty());
        assert_eq!(
            result.metadata().disposition(),
            InvariantExecutionDisposition::SkippedByMayBreakMask
        );
        assert!(result.metadata().has_merged_plan());
    }
}
