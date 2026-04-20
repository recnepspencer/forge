use super::inspection_projection::{
    relational_merge_class_admission, relational_merge_class_label,
};
use super::*;
use crate::identity::hash_parts;
use forge_relational::facade::merge::RelationalMergeInspectionArtifact;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowStalenessOutcome {
    StillFresh,
    StaleDenied,
    ExplicitRebindRequired,
}

impl WorkflowStalenessOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StillFresh => "still_fresh",
            Self::StaleDenied => "stale_denied",
            Self::ExplicitRebindRequired => "explicit_rebind_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowExplicitRebindArtifact {
    declaration_digest: String,
    basis_family: WorkflowBasisFamily,
    basis_digest: String,
    authority_target_family: WorkflowAuthorityTargetFamily,
    rebind_reason: &'static str,
    digest: String,
}

impl WorkflowExplicitRebindArtifact {
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn rebind_reason(&self) -> &'static str {
        self.rebind_reason
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowInspectionFailureClass {
    UnsupportedInspectionFamily,
    RelationalInspectionMismatch,
    NonAuthoritativeOutcomeForbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowInspectionError {
    failure_class: WorkflowInspectionFailureClass,
    message: &'static str,
    counters: WorkflowInspectionCounters,
}

impl WorkflowInspectionError {
    fn new(
        failure_class: WorkflowInspectionFailureClass,
        message: &'static str,
        counters: WorkflowInspectionCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> &WorkflowInspectionFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &WorkflowInspectionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConflictInspectionFamily {
    MergeWorkflowNarrow,
}

impl ConflictInspectionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MergeWorkflowNarrow => "merge_workflow_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MergeClassAdmission {
    ExecutionAdmissible,
    ExecutionDenied,
}

impl MergeClassAdmission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExecutionAdmissible => "execution_admissible",
            Self::ExecutionDenied => "execution_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictInspectionRow {
    workflow_basis_digest: String,
    merge_class: String,
    merge_class_admission: MergeClassAdmission,
    target_basis_digest: String,
    source_basis_digest: String,
    conflict_scope_digest: String,
    authority_target_family: WorkflowAuthorityTargetFamily,
}

impl ConflictInspectionRow {
    pub fn workflow_basis_digest(&self) -> &str {
        &self.workflow_basis_digest
    }

    pub fn merge_class(&self) -> &str {
        &self.merge_class
    }

    pub fn merge_class_admission(&self) -> &MergeClassAdmission {
        &self.merge_class_admission
    }

    pub fn target_basis_digest(&self) -> &str {
        &self.target_basis_digest
    }

    pub fn source_basis_digest(&self) -> &str {
        &self.source_basis_digest
    }

    pub fn conflict_scope_digest(&self) -> &str {
        &self.conflict_scope_digest
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryConflictInspectionArtifact {
    declaration_digest: String,
    family: ConflictInspectionFamily,
    budget: WorkflowInspectionBudget,
    prediction_report: WorkflowPredictionReport,
    drift_outcome: WorkflowPredictionDriftOutcome,
    rows: Vec<ConflictInspectionRow>,
    counters: WorkflowInspectionCounters,
}

impl QueryConflictInspectionArtifact {
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn family(&self) -> &ConflictInspectionFamily {
        &self.family
    }

    pub fn budget(&self) -> &WorkflowInspectionBudget {
        &self.budget
    }

    pub fn prediction_report(&self) -> &WorkflowPredictionReport {
        &self.prediction_report
    }

    pub fn drift_outcome(&self) -> &WorkflowPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn rows(&self) -> &[ConflictInspectionRow] {
        &self.rows
    }

    pub fn counters(&self) -> &WorkflowInspectionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PostMergeInspectionFamily {
    AuthoritativeOutcomeNarrow,
}

impl PostMergeInspectionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeOutcomeNarrow => "authoritative_outcome_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostMergeInspectionRow {
    authoritative_outcome_basis_digest: String,
    authority_target_family: WorkflowAuthorityTargetFamily,
    authoritative_commit_or_outcome_digest: String,
    post_merge_scope_digest: String,
    merge_or_writeback_origin_digest: String,
    inspection_result_family: String,
}

impl PostMergeInspectionRow {
    pub fn authoritative_outcome_basis_digest(&self) -> &str {
        &self.authoritative_outcome_basis_digest
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn authoritative_commit_or_outcome_digest(&self) -> &str {
        &self.authoritative_commit_or_outcome_digest
    }

    pub fn post_merge_scope_digest(&self) -> &str {
        &self.post_merge_scope_digest
    }

    pub fn merge_or_writeback_origin_digest(&self) -> &str {
        &self.merge_or_writeback_origin_digest
    }

    pub fn inspection_result_family(&self) -> &str {
        &self.inspection_result_family
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPostMergeInspectionArtifact {
    origin_digest: String,
    family: PostMergeInspectionFamily,
    budget: WorkflowInspectionBudget,
    prediction_report: WorkflowPredictionReport,
    drift_outcome: WorkflowPredictionDriftOutcome,
    rows: Vec<PostMergeInspectionRow>,
    counters: WorkflowInspectionCounters,
}

impl QueryPostMergeInspectionArtifact {
    pub fn origin_digest(&self) -> &str {
        &self.origin_digest
    }

    pub fn family(&self) -> &PostMergeInspectionFamily {
        &self.family
    }

    pub fn budget(&self) -> &WorkflowInspectionBudget {
        &self.budget
    }

    pub fn prediction_report(&self) -> &WorkflowPredictionReport {
        &self.prediction_report
    }

    pub fn drift_outcome(&self) -> &WorkflowPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn rows(&self) -> &[PostMergeInspectionRow] {
        &self.rows
    }

    pub fn counters(&self) -> &WorkflowInspectionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowAuthorityOutcomeFamily {
    MutationLoweringAdmitted,
    MergeLoweringAdmitted,
    WritebackLoweringAdmitted,
}

impl WorkflowAuthorityOutcomeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MutationLoweringAdmitted => "mutation_lowering_admitted",
            Self::MergeLoweringAdmitted => "merge_lowering_admitted",
            Self::WritebackLoweringAdmitted => "writeback_lowering_admitted",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowAuthorityOutcomeArtifact {
    family: WorkflowAuthorityOutcomeFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
    source_query_digest: String,
    source_plan_digest: String,
    source_basis_digest: String,
    source_declaration_digest: String,
    authority_request_digest: String,
    authoritative_outcome_digest: String,
    cost_class: WorkflowCostClass,
    budget_class: WorkflowBudgetClass,
    budget_outcome: WorkflowBudgetOutcome,
    prediction_report: WorkflowPredictionReport,
    prediction_drift_outcome: WorkflowPredictionDriftOutcome,
    freshness_outcome: WorkflowStalenessOutcome,
    explicit_rebind: Option<WorkflowExplicitRebindArtifact>,
    realized_width: usize,
    counters: WorkflowLoweringCounters,
}

impl WorkflowAuthorityOutcomeArtifact {
    pub fn family(&self) -> &WorkflowAuthorityOutcomeFamily {
        &self.family
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn source_query_digest(&self) -> &str {
        &self.source_query_digest
    }

    pub fn source_plan_digest(&self) -> &str {
        &self.source_plan_digest
    }

    pub fn source_basis_digest(&self) -> &str {
        &self.source_basis_digest
    }

    pub fn source_declaration_digest(&self) -> &str {
        &self.source_declaration_digest
    }

    pub fn authority_request_digest(&self) -> &str {
        &self.authority_request_digest
    }

    pub fn authoritative_outcome_digest(&self) -> &str {
        &self.authoritative_outcome_digest
    }

    pub fn cost_class(&self) -> &WorkflowCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &WorkflowBudgetClass {
        &self.budget_class
    }

    pub fn budget_outcome(&self) -> &WorkflowBudgetOutcome {
        &self.budget_outcome
    }

    pub fn prediction_report(&self) -> &WorkflowPredictionReport {
        &self.prediction_report
    }

    pub fn prediction_drift_outcome(&self) -> &WorkflowPredictionDriftOutcome {
        &self.prediction_drift_outcome
    }

    pub fn freshness_outcome(&self) -> &WorkflowStalenessOutcome {
        &self.freshness_outcome
    }

    pub fn explicit_rebind(&self) -> Option<&WorkflowExplicitRebindArtifact> {
        self.explicit_rebind.as_ref()
    }

    pub fn realized_width(&self) -> usize {
        self.realized_width
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowReplayBundle {
    bundle_digest: String,
    query_digest: String,
    plan_digest: String,
    basis_digest: String,
    declaration_digest: String,
    authority_target_family: WorkflowAuthorityTargetFamily,
    authority_request_digest: String,
    authoritative_outcome_digest: String,
    delivery_or_failure_digest: String,
    counters: WorkflowLoweringCounters,
}

impl WorkflowReplayBundle {
    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn plan_digest(&self) -> &str {
        &self.plan_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn authority_request_digest(&self) -> &str {
        &self.authority_request_digest
    }

    pub fn authoritative_outcome_digest(&self) -> &str {
        &self.authoritative_outcome_digest
    }

    pub fn delivery_or_failure_digest(&self) -> &str {
        &self.delivery_or_failure_digest
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

pub fn inspect_merge_conflicts(
    declaration: &QueryWorkflowDeclaration,
    merge_declaration: &LoweredMergeWorkflowDeclaration,
    relational_inspection: &RelationalMergeInspectionArtifact,
) -> Result<QueryConflictInspectionArtifact, WorkflowInspectionError> {
    if declaration.request().declaration_family()
        != &WorkflowDeclarationFamily::ConflictInspectionNarrow
    {
        return Err(WorkflowInspectionError::new(
            WorkflowInspectionFailureClass::UnsupportedInspectionFamily,
            "conflict inspection requires an admitted conflict inspection declaration",
            WorkflowInspectionCounters {
                workflow_inspection_denial_width: 1,
                ..WorkflowInspectionCounters::default()
            },
        ));
    }

    if declaration.binding().query_identity_digest()
        != merge_declaration
            .declaration()
            .binding()
            .query_identity_digest()
        || declaration.binding().basis_digest()
            != merge_declaration.declaration().binding().basis_digest()
    {
        return Err(WorkflowInspectionError::new(
            WorkflowInspectionFailureClass::UnsupportedInspectionFamily,
            "conflict inspection declaration must bind the same query and basis identity as the lowered merge declaration",
            WorkflowInspectionCounters {
                workflow_inspection_denial_width: 1,
                ..WorkflowInspectionCounters::default()
            },
        ));
    }

    if merge_declaration.merge_request() != relational_inspection.request() {
        return Err(WorkflowInspectionError::new(
            WorkflowInspectionFailureClass::RelationalInspectionMismatch,
            "relational merge inspection artifact must match the lowered merge request exactly",
            WorkflowInspectionCounters {
                workflow_inspection_denial_width: 1,
                ..WorkflowInspectionCounters::default()
            },
        ));
    }

    let rows = relational_inspection
        .rows()
        .iter()
        .map(|row| ConflictInspectionRow {
            workflow_basis_digest: declaration.binding().basis_digest().to_string(),
            merge_class: relational_merge_class_label(row),
            merge_class_admission: relational_merge_class_admission(row),
            target_basis_digest: merge_declaration.merge_request().target_branch().0.clone(),
            source_basis_digest: merge_declaration.merge_request().source_branch().0.clone(),
            conflict_scope_digest: hash_parts(&[
                format!(
                    "target:{}",
                    merge_declaration.merge_request().target_branch().0
                ),
                format!(
                    "source:{}",
                    merge_declaration.merge_request().source_branch().0
                ),
                format!("merge_intent:{}", merge_declaration.merge_intent().as_str()),
                format!("record:{:?}", row.record()),
                format!("row_digest:{}", row.row_digest()),
                format!("merge_class:{}", relational_merge_class_label(row)),
                format!(
                    "merge_class_admission:{}",
                    relational_merge_class_admission(row).as_str()
                ),
            ]),
            authority_target_family: merge_declaration
                .declaration()
                .report()
                .authority_target_family()
                .clone(),
        })
        .collect::<Vec<_>>();
    let row_width = rows.len();

    Ok(QueryConflictInspectionArtifact {
        declaration_digest: declaration.report().declaration_digest().to_string(),
        family: ConflictInspectionFamily::MergeWorkflowNarrow,
        budget: WorkflowInspectionBudget::ConflictInspectionNarrow,
        prediction_report: WorkflowPredictionReport {
            predicted_declaration_width: 1,
            predicted_inspection_width: row_width,
            predicted_lowering_width: 1,
            predicted_freshness_width: 1,
            predicted_denial_width: 1,
        },
        drift_outcome: WorkflowPredictionDriftOutcome::WithinBudget,
        rows,
        counters: WorkflowInspectionCounters {
            workflow_inspection_count: 1,
            workflow_conflict_inspection_count: 1,
            workflow_post_merge_inspection_count: 0,
            workflow_inspection_row_width: row_width,
            workflow_inspection_merge_class_width: row_width,
            workflow_inspection_denial_width: 0,
            workflow_executor_rediscovery_count: 0,
        },
    })
}

pub fn shape_mutation_authority_outcome(
    declaration: &LoweredMutationIntentDeclaration,
) -> WorkflowAuthorityOutcomeArtifact {
    shape_authority_outcome(
        declaration.declaration(),
        WorkflowAuthorityOutcomeFamily::MutationLoweringAdmitted,
        declaration
            .strategy_request()
            .caller_provenance()
            .correlation_id
            .as_deref(),
        declaration.lowering_digest(),
        declaration.counters().clone(),
        1,
    )
}

pub fn shape_merge_authority_outcome(
    declaration: &LoweredMergeWorkflowDeclaration,
) -> WorkflowAuthorityOutcomeArtifact {
    shape_authority_outcome(
        declaration.declaration(),
        WorkflowAuthorityOutcomeFamily::MergeLoweringAdmitted,
        None,
        declaration.lowering_digest(),
        declaration.counters().clone(),
        1,
    )
}

pub fn shape_writeback_authority_outcome(
    declaration: &QueryWritebackDeclaration,
) -> WorkflowAuthorityOutcomeArtifact {
    shape_authority_outcome(
        declaration.declaration(),
        WorkflowAuthorityOutcomeFamily::WritebackLoweringAdmitted,
        None,
        declaration.lowering_digest(),
        declaration.counters().clone(),
        1,
    )
}

pub fn inspect_post_merge_outcome(
    declaration: &QueryWorkflowDeclaration,
    outcome: &WorkflowAuthorityOutcomeArtifact,
) -> Result<QueryPostMergeInspectionArtifact, WorkflowInspectionError> {
    if declaration.request().declaration_family()
        != &WorkflowDeclarationFamily::PostMergeInspectionNarrow
    {
        return Err(WorkflowInspectionError::new(
            WorkflowInspectionFailureClass::UnsupportedInspectionFamily,
            "post-merge inspection requires an admitted post-merge inspection declaration",
            WorkflowInspectionCounters {
                workflow_inspection_denial_width: 1,
                ..WorkflowInspectionCounters::default()
            },
        ));
    }

    if !matches!(
        outcome.family(),
        WorkflowAuthorityOutcomeFamily::MergeLoweringAdmitted
            | WorkflowAuthorityOutcomeFamily::WritebackLoweringAdmitted
    ) {
        return Err(WorkflowInspectionError::new(
            WorkflowInspectionFailureClass::NonAuthoritativeOutcomeForbidden,
            "post-merge inspection requires a merge or writeback authority outcome artifact",
            WorkflowInspectionCounters {
                workflow_inspection_denial_width: 1,
                ..WorkflowInspectionCounters::default()
            },
        ));
    }

    if declaration.binding().query_identity_digest() != outcome.source_query_digest()
        || declaration.binding().basis_digest() != outcome.source_basis_digest()
    {
        return Err(WorkflowInspectionError::new(
            WorkflowInspectionFailureClass::UnsupportedInspectionFamily,
            "post-merge inspection declaration must bind the same query and basis identity as the authoritative outcome",
            WorkflowInspectionCounters {
                workflow_inspection_denial_width: 1,
                ..WorkflowInspectionCounters::default()
            },
        ));
    }

    let row = PostMergeInspectionRow {
        authoritative_outcome_basis_digest: outcome.source_basis_digest().to_string(),
        authority_target_family: outcome.authority_target_family().clone(),
        authoritative_commit_or_outcome_digest: outcome.authoritative_outcome_digest().to_string(),
        post_merge_scope_digest: hash_parts(&[
            format!("outcome:{}", outcome.authoritative_outcome_digest()),
            format!("family:{}", outcome.family().as_str()),
        ]),
        merge_or_writeback_origin_digest: outcome.source_declaration_digest().to_string(),
        inspection_result_family: PostMergeInspectionFamily::AuthoritativeOutcomeNarrow
            .as_str()
            .to_string(),
    };

    Ok(QueryPostMergeInspectionArtifact {
        origin_digest: declaration.report().declaration_digest().to_string(),
        family: PostMergeInspectionFamily::AuthoritativeOutcomeNarrow,
        budget: WorkflowInspectionBudget::PostMergeInspectionNarrow,
        prediction_report: WorkflowPredictionReport {
            predicted_declaration_width: 1,
            predicted_inspection_width: 1,
            predicted_lowering_width: 1,
            predicted_freshness_width: 1,
            predicted_denial_width: 1,
        },
        drift_outcome: WorkflowPredictionDriftOutcome::WithinBudget,
        rows: vec![row],
        counters: WorkflowInspectionCounters {
            workflow_inspection_count: 1,
            workflow_conflict_inspection_count: 0,
            workflow_post_merge_inspection_count: 1,
            workflow_inspection_row_width: 1,
            workflow_inspection_merge_class_width: 1,
            workflow_inspection_denial_width: 0,
            workflow_executor_rediscovery_count: 0,
        },
    })
}

pub fn build_workflow_replay_bundle(
    outcome: &WorkflowAuthorityOutcomeArtifact,
) -> WorkflowReplayBundle {
    let delivery_or_failure_digest = hash_parts(&[
        format!("outcome:{}", outcome.authoritative_outcome_digest()),
        format!("freshness:{}", outcome.freshness_outcome().as_str()),
        format!("budget:{}", outcome.budget_outcome().as_str()),
    ]);
    let bundle_digest = hash_parts(&[
        format!("query:{}", outcome.source_query_digest()),
        format!("plan:{}", outcome.source_plan_digest()),
        format!("basis:{}", outcome.source_basis_digest()),
        format!("declaration:{}", outcome.source_declaration_digest()),
        format!(
            "authority_target:{}",
            outcome.authority_target_family().as_str()
        ),
        format!("request:{}", outcome.authority_request_digest()),
        format!("outcome:{}", outcome.authoritative_outcome_digest()),
        format!("delivery:{delivery_or_failure_digest}"),
    ]);

    let counters = outcome.counters().with_replay_bundle_issued();

    WorkflowReplayBundle {
        bundle_digest,
        query_digest: outcome.source_query_digest().to_string(),
        plan_digest: outcome.source_plan_digest().to_string(),
        basis_digest: outcome.source_basis_digest().to_string(),
        declaration_digest: outcome.source_declaration_digest().to_string(),
        authority_target_family: outcome.authority_target_family().clone(),
        authority_request_digest: outcome.authority_request_digest().to_string(),
        authoritative_outcome_digest: outcome.authoritative_outcome_digest().to_string(),
        delivery_or_failure_digest,
        counters,
    }
}

fn shape_authority_outcome(
    declaration: &QueryWorkflowDeclaration,
    family: WorkflowAuthorityOutcomeFamily,
    authority_request_digest_hint: Option<&str>,
    lowering_digest: &str,
    counters: WorkflowLoweringCounters,
    realized_width: usize,
) -> WorkflowAuthorityOutcomeArtifact {
    let authority_request_digest = authority_request_digest_hint
        .unwrap_or(lowering_digest)
        .to_string();
    let authoritative_outcome_digest = hash_parts(&[
        format!("declaration:{}", declaration.report().declaration_digest()),
        format!("family:{}", family.as_str()),
        format!("authority_request:{authority_request_digest}"),
        format!("basis:{}", declaration.binding().basis_digest()),
    ]);

    WorkflowAuthorityOutcomeArtifact {
        family,
        authority_target_family: declaration.report().authority_target_family().clone(),
        source_query_digest: declaration.binding().query_identity_digest().to_string(),
        source_plan_digest: declaration.binding().source_digest().to_string(),
        source_basis_digest: declaration.binding().basis_digest().to_string(),
        source_declaration_digest: declaration.report().declaration_digest().to_string(),
        authority_request_digest,
        authoritative_outcome_digest,
        cost_class: declaration.report().cost_class().clone(),
        budget_class: declaration.report().budget_class().clone(),
        budget_outcome: WorkflowBudgetOutcome::WithinBudget,
        prediction_report: WorkflowPredictionReport {
            predicted_declaration_width: 1,
            predicted_inspection_width: 1,
            predicted_lowering_width: 1,
            predicted_freshness_width: 1,
            predicted_denial_width: 1,
        },
        prediction_drift_outcome: WorkflowPredictionDriftOutcome::WithinBudget,
        freshness_outcome: WorkflowStalenessOutcome::StillFresh,
        explicit_rebind: None,
        realized_width,
        counters,
    }
}
