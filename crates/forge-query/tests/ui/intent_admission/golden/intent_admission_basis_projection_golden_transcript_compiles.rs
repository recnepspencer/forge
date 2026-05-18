#![allow(dead_code)]

use forge_query::basis_lifecycle::RawBasisIntent;
use forge_query::facade::{
    forge_query_basis_observation_intent, forge_query_projection_consumption_intent,
    ForgeQueryIntentNonAdmittedStop, ForgeQueryIntentViolationDecision,
    ProjectionConsumptionDeclaration,
};

fn basis_observation_common_path() -> Result<(), ForgeQueryIntentNonAdmittedStop> {
    let scoped_basis = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)?
        .admit()?
        .scope();
    let _ = scoped_basis.scoped_basis_digest();
    Ok(())
}

fn basis_observation_advanced_path() -> Result<(), ForgeQueryIntentViolationDecision> {
    let review = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)?.review()?;
    let _ = review.request();
    let _ = review.eligibility();
    let _ = review.decision();
    let _ = review.admitted_plan();
    Ok(())
}

fn projection_consumption_advanced_path(
    declaration: ProjectionConsumptionDeclaration,
) -> Result<(), ForgeQueryIntentViolationDecision> {
    let review = forge_query_projection_consumption_intent(declaration)?.review()?;
    let _ = review.request();
    let _ = review.eligibility();
    let _ = review.decision();
    let _ = review.admitted_plan();
    Ok(())
}

fn main() {}
