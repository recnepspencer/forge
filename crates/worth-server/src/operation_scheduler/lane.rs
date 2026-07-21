use crate::{WorthServerLoweredOperationPlan, WorthServerOperationAuthorityKind};

use super::WorthServerSchedulerConflictDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerSchedulerLane {
    SharedRead,
    DeterministicSubmission {
        submission_lane: String,
    },
    ProductDraftMutation {
        product_session_identity: String,
        draft_scope: String,
    },
    DurableProductMutation {
        scope_digest: String,
    },
    ProductSessionCoordination {
        product_session_identity: String,
        coordination_lane: String,
    },
}

impl WorthServerSchedulerLane {
    pub(crate) fn from_lowered_plan(
        plan: &WorthServerLoweredOperationPlan,
    ) -> Result<Self, WorthServerSchedulerConflictDenial> {
        let authority_metadata = plan
            .query_handoff()
            .operation_admission()
            .authority_metadata();
        let lane = match plan
            .query_handoff()
            .operation_admission()
            .authority_footprint()
            .authority_kind()
        {
            WorthServerOperationAuthorityKind::SharedReadOnly
            | WorthServerOperationAuthorityKind::DiagnosticsOnly => Self::SharedRead,
            WorthServerOperationAuthorityKind::DeterministicSubmission => {
                Self::DeterministicSubmission {
                    submission_lane: authority_metadata
                        .submission_lane()
                        .expect("deterministic submission plans must carry a submission lane")
                        .to_string(),
                }
            }
            WorthServerOperationAuthorityKind::ProductDraftMutation => {
                let (product_session_identity, draft_scope) = authority_metadata
                    .product_draft_scope()
                    .expect("product draft mutation plans must carry session and scope");
                Self::ProductDraftMutation {
                    product_session_identity: product_session_identity.to_string(),
                    draft_scope: draft_scope.to_string(),
                }
            }
            WorthServerOperationAuthorityKind::DurableProductMutation => {
                authority_metadata
                    .durable_product_mutation_scope()
                    .expect("durable product mutation plans must carry scope and preconditions");
                Self::DurableProductMutation {
                    scope_digest: plan
                        .query_handoff()
                        .operation_admission()
                        .authority_footprint()
                        .scope()
                        .canonical_digest(),
                }
            }
            WorthServerOperationAuthorityKind::ProductSessionCoordination => {
                let (target, coordination_lane) = authority_metadata
                    .product_session_coordination_target()
                    .expect("product session coordination plans must carry coordination metadata");
                match target {
                    crate::WorthServerProductSessionCoordinationTarget::ExistingSession {
                        product_session_identity,
                    } => Self::ProductSessionCoordination {
                        product_session_identity: product_session_identity.to_string(),
                        coordination_lane: coordination_lane.to_string(),
                    },
                    crate::WorthServerProductSessionCoordinationTarget::SessionCreation => {
                        Self::DeterministicSubmission {
                            submission_lane: format!(
                                "product-session-create:{}:{coordination_lane}",
                                plan.query_handoff()
                                    .operation_admission()
                                    .authority_footprint()
                                    .scope()
                                    .canonical_digest()
                            ),
                        }
                    }
                }
            }
            unsupported => {
                return Err(WorthServerSchedulerConflictDenial::unsupported_ordered_operation(
                    format!(
                        "scheduler cannot derive an execution lane for unsupported authority kind `{}`",
                        unsupported.as_str()
                    ),
                ));
            }
        };
        let expected_scheduler_lane = lane.canonical_label();
        if plan.receipt().expected_scheduler_lane() != expected_scheduler_lane {
            return Err(WorthServerSchedulerConflictDenial::unsupported_ordered_operation(
                format!(
                    "lowered plan expected scheduler lane `{}` but authority-derived lane was `{expected_scheduler_lane}`",
                    plan.receipt().expected_scheduler_lane()
                ),
            ));
        }
        Ok(lane)
    }

    pub(crate) fn canonical_label(&self) -> String {
        match self {
            Self::SharedRead => "shared-read".to_string(),
            Self::DeterministicSubmission { submission_lane } => submission_lane.clone(),
            Self::ProductDraftMutation {
                product_session_identity,
                draft_scope,
            } => format!("product-draft:{product_session_identity}:{draft_scope}"),
            Self::DurableProductMutation { scope_digest } => {
                format!("durable-product:{scope_digest}")
            }
            Self::ProductSessionCoordination {
                product_session_identity,
                coordination_lane,
            } => format!("product-session:{product_session_identity}:{coordination_lane}"),
        }
    }

    pub(crate) fn lane_scope_key(&self) -> String {
        self.canonical_label()
    }
}
