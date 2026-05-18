use crate::basis_lifecycle::RawBasisIntent;
use crate::intent_admission::{
    forge_query_basis_observation_intent, forge_query_projection_consumption_intent,
    ForgeQueryAdmittedIntentPlan, ForgeQueryBasisObservationPlan, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryProjectionConsumptionPlan,
    ForgeQueryRawIntentAdmissionRequest,
};
use crate::projection_consumption::{
    intent_admission_admitted_projection_declaration, intent_admission_warning_projection_declaration,
};

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedBasisObservationIntentFixture {
    pub(in crate::intent_admission::certification) request: ForgeQueryRawIntentAdmissionRequest,
    pub(in crate::intent_admission::certification) eligibility: ForgeQueryIntentAdmissionEligibility,
    pub(in crate::intent_admission::certification) plan: ForgeQueryBasisObservationPlan,
    pub(in crate::intent_admission::certification) scoped_basis_digest: String,
}

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedProjectionConsumptionAdmittedFixture {
    pub(in crate::intent_admission::certification) request: ForgeQueryRawIntentAdmissionRequest,
    pub(in crate::intent_admission::certification) eligibility: ForgeQueryIntentAdmissionEligibility,
    pub(in crate::intent_admission::certification) plan: ForgeQueryProjectionConsumptionPlan,
    pub(in crate::intent_admission::certification) contract_digest: String,
}

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedProjectionConsumptionWarningFixture {
    pub(in crate::intent_admission::certification) request: ForgeQueryRawIntentAdmissionRequest,
    pub(in crate::intent_admission::certification) eligibility: ForgeQueryIntentAdmissionEligibility,
    pub(in crate::intent_admission::certification) plan: ForgeQueryProjectionConsumptionPlan,
    pub(in crate::intent_admission::certification) contract_digest: String,
}

pub(in crate::intent_admission::certification) fn certified_basis_observation_intent_fixture(
) -> CertifiedBasisObservationIntentFixture {
    let review = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .expect("basis observation intent should build")
        .review();
    let request = review.request().clone();
    let eligibility = review.eligibility().clone();
    let plan = match review.decision().clone() {
        ForgeQueryIntentAdmissionDecision::Admitted(ForgeQueryAdmittedIntentPlan::BasisObservation(
            plan,
        )) => plan,
        other => panic!("expected admitted basis plan, got {other:?}"),
    };
    let scoped_basis = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .expect("basis observation intent should build")
        .admit()
        .expect("basis observation should admit")
        .scope();
    CertifiedBasisObservationIntentFixture {
        request,
        eligibility,
        plan,
        scoped_basis_digest: scoped_basis.scoped_basis_digest().to_string(),
    }
}

pub(in crate::intent_admission::certification) fn certified_projection_consumption_admitted_fixture(
) -> CertifiedProjectionConsumptionAdmittedFixture {
    let declaration = intent_admission_admitted_projection_declaration();
    let review = forge_query_projection_consumption_intent(declaration)
        .expect("projection intent should build")
        .review();
    let request = review.request().clone();
    let eligibility = review.eligibility().clone();
    let plan = match review.decision().clone() {
        ForgeQueryIntentAdmissionDecision::Admitted(
            ForgeQueryAdmittedIntentPlan::ProjectionConsumption(plan),
        ) => plan,
        other => panic!("expected admitted projection plan, got {other:?}"),
    };
    let contract = review.admit().expect("projection should admit").bind_contract();
    CertifiedProjectionConsumptionAdmittedFixture {
        request,
        eligibility,
        plan,
        contract_digest: contract.contract_digest().to_string(),
    }
}

pub(in crate::intent_admission::certification) fn certified_projection_consumption_warning_fixture(
) -> CertifiedProjectionConsumptionWarningFixture {
    let declaration = intent_admission_warning_projection_declaration();
    let review = forge_query_projection_consumption_intent(declaration)
        .expect("warning projection intent should build")
        .review();
    let request = review.request().clone();
    let eligibility = review.eligibility().clone();
    let plan = match review.decision().clone() {
        ForgeQueryIntentAdmissionDecision::Admitted(
            ForgeQueryAdmittedIntentPlan::ProjectionConsumption(plan),
        ) => plan,
        other => panic!("expected warning-bearing admitted projection plan, got {other:?}"),
    };
    let contract = review.admit().expect("warning-bearing projection should admit").bind_contract();
    CertifiedProjectionConsumptionWarningFixture {
        request,
        eligibility,
        plan,
        contract_digest: contract.contract_digest().to_string(),
    }
}

