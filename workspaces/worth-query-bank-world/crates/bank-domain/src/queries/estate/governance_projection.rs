use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::{
    estate::{
        CapabilityValidity, EstateCapabilityGrant, EstateCapabilityScope, EstateCaseId,
        EstateEmergencyAccess, MandatoryEstateReview,
    },
    reads::{
        EstateAssignmentView, EstateCapabilityContext, EstateEmergencyContext,
        EstateGovernanceContext,
    },
    schema::BankSchema,
};

use super::{governance::EstateGovernanceQuery, governance_fields::*, governance_relations::*};

type Row<'row> = WorthQueryApplicationProjectionRow<'row, BankSchema, EstateGovernanceQuery>;

impl WorthQueryApplicationProjection<BankSchema, EstateGovernanceQuery>
    for EstateGovernanceContext
{
    fn project(row: &Row<'_>) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let estate = row.field(estate_id())?;
        Ok(Self::from_projection(
            estate,
            row.field(estate_stage())?,
            project_beneficiaries(row)?,
            project_assignments(row)?,
            row.many(estate_capabilities())?
                .iter()
                .map(|value| project_capability(&value, estate))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

fn project_beneficiaries(
    row: &Row<'_>,
) -> Result<Vec<crate::model::BankPrincipalId>, WorthQueryApplicationProjectionDenial> {
    row.many(estate_beneficiaries())?
        .iter()
        .map(|value| value.field(beneficiary()))
        .collect()
}

fn project_assignments(
    row: &Row<'_>,
) -> Result<Vec<EstateAssignmentView>, WorthQueryApplicationProjectionDenial> {
    row.many(estate_assignments())?
        .iter()
        .map(|value| project_assignment(&value))
        .collect()
}

fn project_assignment(
    row: &Row<'_>,
) -> Result<EstateAssignmentView, WorthQueryApplicationProjectionDenial> {
    Ok(EstateAssignmentView::from_projection(
        row.field(assignment_id())?,
        row.one(assignment_principal())?
            .field(assignment_principal_identity())?,
        row.field(assignment_role())?,
    ))
}

fn project_capability(
    row: &Row<'_>,
    estate: EstateCaseId,
) -> Result<EstateCapabilityContext, WorthQueryApplicationProjectionDenial> {
    let id = row.field(capability_id())?;
    let grant = EstateCapabilityGrant {
        id,
        grantor: row
            .one(capability_grantor())?
            .field(capability_grantor_identity())?,
        grantee: row
            .one(capability_grantee())?
            .field(capability_grantee_identity())?,
        scope: project_capability_scope(row, estate)?,
        parent: row
            .optional(capability_parent())?
            .map(|value| value.field(capability_parent_identity()))
            .transpose()?,
        status: row.field(capability_status())?,
    };
    let emergencies = row
        .many(capability_emergencies())?
        .iter()
        .map(|value| project_emergency(&value, id))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(EstateCapabilityContext::from_projection(grant, emergencies))
}

fn project_capability_scope(
    row: &Row<'_>,
    estate: EstateCaseId,
) -> Result<EstateCapabilityScope, WorthQueryApplicationProjectionDenial> {
    let valid_from = row.field(capability_valid_from())?;
    let valid_through = row.field(capability_valid_through())?;
    let validity = CapabilityValidity::new(valid_from, valid_through).ok_or_else(|| {
        WorthQueryApplicationProjectionDenial::reject("capability validity interval")
    })?;
    Ok(EstateCapabilityScope {
        account: row
            .optional(capability_account())?
            .map(|value| value.field(capability_account_identity()))
            .transpose()?,
        estate,
        institution: row
            .one(capability_institution())?
            .field(capability_institution_identity())?,
        branch: row
            .one(capability_branch())?
            .field(capability_branch_identity())?,
        operation: row.field(capability_operation())?,
        purpose: row.field(capability_purpose())?,
        field: row.optional_field(capability_field())?,
        amount_ceiling: row.optional_field(capability_amount())?,
        validity,
        delegation: row.field(capability_delegation())?,
        workflow_stage: row.field(capability_workflow())?,
    })
}

fn project_emergency(
    row: &Row<'_>,
    grant: crate::estate::CapabilityGrantId,
) -> Result<EstateEmergencyContext, WorthQueryApplicationProjectionDenial> {
    let review = project_review(&row.one(emergency_review())?)?;
    let access = EstateEmergencyAccess {
        id: row.field(emergency_id())?,
        requester: row
            .one(emergency_requester())?
            .field(emergency_requester_identity())?,
        approver: row
            .optional(emergency_approver())?
            .map(|value| value.field(emergency_approver_identity()))
            .transpose()?,
        reviewer: review.reviewer,
        grant,
        review: review.id,
        reason: row.field(emergency_reason())?,
        status: row.field(emergency_status())?,
        issued_at: row.field(emergency_issued_at())?,
        expires_at: row.field(emergency_expires_at())?,
    };
    Ok(EstateEmergencyContext::from_projection(access, review))
}

fn project_review(
    row: &Row<'_>,
) -> Result<MandatoryEstateReview, WorthQueryApplicationProjectionDenial> {
    Ok(MandatoryEstateReview {
        id: row.field(review_id())?,
        estate: row.one(review_estate())?.field(review_estate_identity())?,
        kind: row.field(review_kind())?,
        reviewer: row
            .optional(review_reviewer())?
            .map(|value| value.field(review_reviewer_identity()))
            .transpose()?,
        status: row.field(review_status())?,
    })
}
