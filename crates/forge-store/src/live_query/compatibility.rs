use crate::authority::DurableCursorResumeRequest;
use crate::failure::{StoreError, StoreErrorKind};
use crate::live_query::basis::{StableBasisHandle, StableBasisReadScope};
use crate::live_query::continuation::{
    ContinuationBatchBudget, ContinuationStrategy, CursorContinuationPlan,
};
use crate::live_query::restart::StableBasisSurvival;
use crate::DurableCursorResumePlan;
#[derive(Debug, Clone)]
pub struct CursorContinuationRequest {
    resume_request: DurableCursorResumeRequest,
    stable_basis: StableBasisHandle,
    batch_budget: ContinuationBatchBudget,
}

impl CursorContinuationRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cursor_id: impl Into<String>,
        subscriber_id: impl Into<String>,
        branch_id: forge_relational::facade::history::BranchId,
        feed_shape_id: impl Into<String>,
        schema_interpretation_id: impl Into<String>,
        cursor_semantics_version: u32,
        stable_basis: StableBasisHandle,
        batch_budget: ContinuationBatchBudget,
    ) -> Self {
        Self {
            resume_request: DurableCursorResumeRequest::new(
                cursor_id,
                subscriber_id,
                branch_id,
                feed_shape_id,
                schema_interpretation_id,
                cursor_semantics_version,
            ),
            stable_basis,
            batch_budget,
        }
    }

    pub fn resume_request(&self) -> &DurableCursorResumeRequest {
        &self.resume_request
    }

    pub fn stable_basis(&self) -> &StableBasisHandle {
        &self.stable_basis
    }

    pub fn batch_budget(&self) -> &ContinuationBatchBudget {
        &self.batch_budget
    }
}

#[derive(Debug, Clone)]
pub struct ContinuationCompatibilityWitness {
    request: CursorContinuationRequest,
    resume_plan: DurableCursorResumePlan,
}

impl ContinuationCompatibilityWitness {
    pub(crate) fn new(
        request: CursorContinuationRequest,
        resume_plan: DurableCursorResumePlan,
    ) -> Self {
        Self {
            request,
            resume_plan,
        }
    }

    pub fn request(&self) -> &CursorContinuationRequest {
        &self.request
    }

    pub fn resume_plan(&self) -> &DurableCursorResumePlan {
        &self.resume_plan
    }

    pub fn stable_basis(&self) -> &StableBasisHandle {
        self.request.stable_basis()
    }

    pub fn batch_budget(&self) -> &ContinuationBatchBudget {
        self.request.batch_budget()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContinuationPlanningEffect {
    SchemaMismatch,
    ScopeMismatch,
    DegradedBasis,
    StableBasisBroadening,
    ContinuationBroadening,
    RejectedBasis,
    ContinuationPlan,
}

#[derive(Debug)]
pub(crate) struct PlannedCursorContinuation {
    plan: CursorContinuationPlan,
    effects: Vec<ContinuationPlanningEffect>,
}

impl PlannedCursorContinuation {
    pub(crate) fn new(
        plan: CursorContinuationPlan,
        effects: Vec<ContinuationPlanningEffect>,
    ) -> Self {
        Self { plan, effects }
    }

    pub(crate) fn into_parts(self) -> (CursorContinuationPlan, Vec<ContinuationPlanningEffect>) {
        (self.plan, self.effects)
    }
}

#[derive(Debug)]
pub(crate) struct ContinuationPlanningFailure {
    error: StoreError,
    effects: Vec<ContinuationPlanningEffect>,
}

impl ContinuationPlanningFailure {
    fn new(error: StoreError, effects: Vec<ContinuationPlanningEffect>) -> Self {
        Self { error, effects }
    }

    pub(crate) fn into_parts(self) -> (StoreError, Vec<ContinuationPlanningEffect>) {
        (self.error, self.effects)
    }
}

pub(crate) fn plan_cursor_continuation(
    request: CursorContinuationRequest,
    resume_plan: DurableCursorResumePlan,
) -> Result<PlannedCursorContinuation, ContinuationPlanningFailure> {
    let basis = request.stable_basis();
    let latest_checkpoint = resume_plan.latest_checkpoint();
    let identity = resume_plan.identity();

    if identity.branch_id != *basis.branch_id() || latest_checkpoint.branch_id != *basis.branch_id()
    {
        return Err(ContinuationPlanningFailure::new(
            StoreError::new(
                StoreErrorKind::ContinuationBranchIncompatibility,
                "cursor continuation requires exact branch alignment with the stable basis handle",
            ),
            Vec::new(),
        ));
    }
    if latest_checkpoint.basis_commit_id.0 < basis.frontier_commit_id().0 {
        return Err(ContinuationPlanningFailure::new(
            StoreError::new(
                StoreErrorKind::ContinuationCursorIncompatibility,
                "cursor continuation requires the latest durable checkpoint basis to stay at or ahead of the stable basis frontier",
            ),
            Vec::new(),
        ));
    }
    if let Some(schema_support_id) = latest_checkpoint.schema_support_artifact_id.as_deref() {
        if schema_support_id != basis.schema_boundary_artifact_id() {
            return Err(ContinuationPlanningFailure::new(
                StoreError::new(
                    StoreErrorKind::ContinuationSchemaIncompatibility,
                    "cursor continuation requires an exact schema support artifact match with the stable basis handle",
                ),
                vec![ContinuationPlanningEffect::SchemaMismatch],
            ));
        }
    }

    let mut effects = Vec::new();
    let strategy = match StableBasisSurvival::from_handle(basis) {
        StableBasisSurvival::Retained => {
            if !matches!(basis.read_scope(), StableBasisReadScope::SingleEntity(_)) {
                return Err(ContinuationPlanningFailure::new(
                    StoreError::new(
                        StoreErrorKind::ContinuationScopeIncompatibility,
                        "retained continuation planning currently supports only single-entity stable-basis scopes",
                    ),
                    vec![ContinuationPlanningEffect::ScopeMismatch],
                ));
            }
            ContinuationStrategy::AdmittedLayoutNarrow
        }
        StableBasisSurvival::DegradedButRecoverable { .. } => {
            effects.push(ContinuationPlanningEffect::DegradedBasis);
            effects.push(ContinuationPlanningEffect::StableBasisBroadening);
            effects.push(ContinuationPlanningEffect::ContinuationBroadening);
            ContinuationStrategy::ExplicitBroadened
        }
        StableBasisSurvival::Rejected { .. } => {
            return Err(ContinuationPlanningFailure::new(
                StoreError::new(
                    StoreErrorKind::StableBasisRetainedStateRejected,
                    "rejected stable-basis handles cannot admit cursor continuation plans",
                ),
                vec![ContinuationPlanningEffect::RejectedBasis],
            ));
        }
    };

    effects.push(ContinuationPlanningEffect::ContinuationPlan);
    Ok(PlannedCursorContinuation::new(
        CursorContinuationPlan::new(
            ContinuationCompatibilityWitness::new(request, resume_plan),
            strategy,
        ),
        effects,
    ))
}
