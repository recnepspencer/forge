use crate::basis_lifecycle::{RawBasisIntent, ScopedObservationBasis};
use crate::intent_admission::{
    ForgeQueryBasisObservationPlan, ForgeQueryProjectionConsumptionPlan,
};
use crate::projection_consumption::{
    MaterializedProjectionContract, ProjectionConsumptionDeclaration,
};
use crate::runtime::ForgeQueryIntentConsumerInspection;

use super::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentNonAdmittedStop, ForgeQueryRawIntentAdmissionRequest,
    ForgeQueryRuntimeIntentAdmissionReviewData,
};

pub fn forge_query_basis_observation_intent(
    raw: RawBasisIntent,
) -> Result<ForgeQueryBasisObservationIntentAuthoring, super::ForgeQueryIntentViolationDecision> {
    Ok(ForgeQueryBasisObservationIntentAuthoring {
        request: ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(raw)?,
    })
}

pub fn forge_query_projection_consumption_intent(
    declaration: ProjectionConsumptionDeclaration,
) -> Result<ForgeQueryProjectionConsumptionIntentAuthoring, super::ForgeQueryIntentViolationDecision>
{
    Ok(ForgeQueryProjectionConsumptionIntentAuthoring {
        request: ForgeQueryRawIntentAdmissionRequest::projection_consumption(declaration)?,
    })
}

pub struct ForgeQueryBasisObservationIntentAuthoring {
    request: ForgeQueryRawIntentAdmissionRequest,
}

impl ForgeQueryBasisObservationIntentAuthoring {
    pub fn review(self) -> ForgeQueryBasisObservationIntentReview {
        ForgeQueryBasisObservationIntentReview {
            review: ForgeQueryRuntimeIntentAdmissionReviewData::from_request(self.request),
        }
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryBasisObservationAdmittedIntent, ForgeQueryIntentNonAdmittedStop> {
        self.review().admit()
    }
}

pub struct ForgeQueryBasisObservationIntentReview {
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl ForgeQueryBasisObservationIntentReview {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }
    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }
    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }
    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.review.decision_trace_envelope()
    }
    pub fn consumer_inspection(&self) -> ForgeQueryIntentConsumerInspection<'_> {
        ForgeQueryIntentConsumerInspection::from_review(
            self.review.request().intent_name(),
            self.review.decision(),
            self.review.request().family(),
            self.review.request().entrypoint(),
            self.review.decision_trace_envelope(),
        )
    }
    pub fn admit(
        self,
    ) -> Result<ForgeQueryBasisObservationAdmittedIntent, ForgeQueryIntentNonAdmittedStop> {
        match self.review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                ForgeQueryAdmittedIntentPlan::BasisObservation(plan),
            ) => Ok(ForgeQueryBasisObservationAdmittedIntent { plan }),
            _ => Err(self
                .review
                .non_admitted_stop()
                .expect("non-admitted basis observation review should stop")),
        }
    }
}

pub struct ForgeQueryBasisObservationAdmittedIntent {
    plan: ForgeQueryBasisObservationPlan,
}

impl ForgeQueryBasisObservationAdmittedIntent {
    pub fn plan(&self) -> &ForgeQueryBasisObservationPlan {
        &self.plan
    }

    pub fn scope(self) -> ScopedObservationBasis {
        self.plan.scope()
    }
}

pub struct ForgeQueryProjectionConsumptionIntentAuthoring {
    request: ForgeQueryRawIntentAdmissionRequest,
}

impl ForgeQueryProjectionConsumptionIntentAuthoring {
    pub fn review(self) -> ForgeQueryProjectionConsumptionIntentReview {
        ForgeQueryProjectionConsumptionIntentReview {
            review: ForgeQueryRuntimeIntentAdmissionReviewData::from_request(self.request),
        }
    }

    pub fn admit(
        self,
    ) -> Result<ForgeQueryProjectionConsumptionAdmittedIntent, ForgeQueryIntentNonAdmittedStop>
    {
        self.review().admit()
    }
}

pub struct ForgeQueryProjectionConsumptionIntentReview {
    review: ForgeQueryRuntimeIntentAdmissionReviewData,
}

impl ForgeQueryProjectionConsumptionIntentReview {
    pub fn request(&self) -> &ForgeQueryRawIntentAdmissionRequest {
        self.review.request()
    }
    pub fn eligibility(&self) -> &ForgeQueryIntentAdmissionEligibility {
        self.review.eligibility()
    }
    pub fn decision(&self) -> &ForgeQueryIntentAdmissionDecision {
        self.review.decision()
    }
    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.review.decision_trace_envelope()
    }
    pub fn consumer_inspection(&self) -> ForgeQueryIntentConsumerInspection<'_> {
        ForgeQueryIntentConsumerInspection::from_review(
            self.review.request().intent_name(),
            self.review.decision(),
            self.review.request().family(),
            self.review.request().entrypoint(),
            self.review.decision_trace_envelope(),
        )
    }
    pub fn admit(
        self,
    ) -> Result<ForgeQueryProjectionConsumptionAdmittedIntent, ForgeQueryIntentNonAdmittedStop>
    {
        match self.review.decision().clone() {
            ForgeQueryIntentAdmissionDecision::Admitted(
                ForgeQueryAdmittedIntentPlan::ProjectionConsumption(plan),
            ) => Ok(ForgeQueryProjectionConsumptionAdmittedIntent { plan }),
            _ => Err(self
                .review
                .non_admitted_stop()
                .expect("non-admitted projection review should stop")),
        }
    }
}

pub struct ForgeQueryProjectionConsumptionAdmittedIntent {
    plan: ForgeQueryProjectionConsumptionPlan,
}

impl ForgeQueryProjectionConsumptionAdmittedIntent {
    pub fn plan(&self) -> &ForgeQueryProjectionConsumptionPlan {
        &self.plan
    }

    pub fn bind_contract(&self) -> MaterializedProjectionContract {
        self.plan.bind_contract()
    }
}
