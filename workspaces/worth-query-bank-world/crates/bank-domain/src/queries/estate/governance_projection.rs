use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::{
    reads::{
        EstateAssignmentView, EstateCapabilityContext, EstateCapabilityProjection,
        EstateEmergencyContext, EstateGovernanceContext,
    },
    schema::BankSchema,
};

use super::{governance::EstateGovernanceQuery, governance_fields::*, governance_relations::*};

impl WorthQueryApplicationProjection<BankSchema, EstateGovernanceQuery>
    for EstateGovernanceContext
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateGovernanceQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        Ok(Self::from_projection(
            row.field(estate_id())?,
            row.field(estate_stage())?,
            row.many(estate_beneficiaries())?
                .iter()
                .map(|value| value.field(beneficiary()))
                .collect::<Result<Vec<_>, _>>()?,
            row.many(estate_assignments())?
                .iter()
                .map(|value| project_assignment(&value))
                .collect::<Result<Vec<_>, _>>()?,
            row.many(estate_capabilities())?
                .iter()
                .map(|value| project_capability(&value))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

fn project_assignment(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateGovernanceQuery>,
) -> Result<EstateAssignmentView, WorthQueryApplicationProjectionDenial> {
    Ok(EstateAssignmentView::from_projection(
        row.field(assignment_id())?,
        row.one(assignment_principal())?
            .field(assignment_principal_identity())?,
        row.field(assignment_role())?,
    ))
}

fn project_capability(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateGovernanceQuery>,
) -> Result<EstateCapabilityContext, WorthQueryApplicationProjectionDenial> {
    Ok(EstateCapabilityContext::from_projection(
        EstateCapabilityProjection {
            id: row.field(capability_id())?,
            operation: row.field(capability_operation())?,
            purpose: row.field(capability_purpose())?,
            valid_from: row.field(capability_valid_from())?,
            valid_through: row.field(capability_valid_through())?,
            delegation: row.field(capability_delegation())?,
            workflow_stage: row.field(capability_workflow())?,
            status: row.field(capability_status())?,
            grantee: row
                .one(capability_grantee())?
                .field(capability_grantee_identity())?,
            grantor: row
                .one(capability_grantor())?
                .field(capability_grantor_identity())?,
            emergencies: row
                .many(capability_emergencies())?
                .iter()
                .map(|value| project_emergency(&value))
                .collect::<Result<Vec<_>, _>>()?,
        },
    ))
}

fn project_emergency(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateGovernanceQuery>,
) -> Result<EstateEmergencyContext, WorthQueryApplicationProjectionDenial> {
    let approver = row
        .optional(emergency_approver())?
        .map(|value| value.field(emergency_approver_identity()))
        .transpose()?;
    Ok(EstateEmergencyContext::from_projection(
        row.field(emergency_id())?,
        row.field(emergency_reason())?,
        row.field(emergency_status())?,
        row.one(emergency_requester())?
            .field(emergency_requester_identity())?,
        approver,
    ))
}
