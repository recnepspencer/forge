use worth_query_decl::facade::{
    application_query::ApplicationQueryResultFieldRef,
    application_schema::{EqualityPredicate, NoApplicationCurrency, ReadOnly, ReadWrite},
};

use crate::{
    estate::{
        CapabilityGrantId, CapabilityGrantStatus, DelegationLimit, EmergencyAccessId,
        EmergencyAccessReason, EmergencyAccessStatus, EstateCapabilityOperation,
        EstateCapabilityPurpose, EstateCaseId, EstateMoment, EstateWorkflowStage,
    },
    model::{BankPrincipalId, EmployeeAssignmentId, EmployeeRole},
    schema::{
        AssignmentRole, BankSchema, CapabilityDelegationLimitField, CapabilityGrant,
        CapabilityGrantIdentityField, CapabilityGrantRecord, CapabilityGrantStatusField,
        CapabilityOperationField, CapabilityPurposeField, CapabilityValidFromField,
        CapabilityValidThroughField, CapabilityWorkflowStageField, EmergencyAccess,
        EmergencyAccessIdentityField, EmergencyAccessReasonField, EmergencyAccessRecord,
        EmergencyAccessStatusField, EmployeeAssignment, EmployeeAssignmentIdentityField,
        EmployeeScope, EstateCase, EstateCaseIdentityField, EstateCaseRecord,
        EstateWorkflowStageField, Principal, PrincipalIdentity, PrincipalIdentityField,
    },
};

use super::governance::EstateGovernanceQuery;

macro_rules! slots {
    ($($name:ident),+ $(,)?) => { $(pub(super) struct $name;)+ };
}

slots!(
    EstateIdSlot,
    EstateStageSlot,
    BeneficiarySlot,
    AssignmentIdSlot,
    AssignmentRoleSlot,
    AssignmentPrincipalSlot,
    CapabilityIdSlot,
    CapabilityOperationSlot,
    CapabilityPurposeSlot,
    CapabilityValidFromSlot,
    CapabilityValidThroughSlot,
    CapabilityDelegationSlot,
    CapabilityWorkflowSlot,
    CapabilityStatusSlot,
    CapabilityGranteeSlot,
    CapabilityGrantorSlot,
    EmergencyIdSlot,
    EmergencyReasonSlot,
    EmergencyStatusSlot,
    EmergencyRequesterSlot,
    EmergencyApproverSlot,
);

macro_rules! selector {
    ($name:ident, $slot:ty, $entity:ty, $aspect:ty, $field:ty, $value:ty, $write:ty, $alias:literal) => {
        pub(super) fn $name() -> ApplicationQueryResultFieldRef<
            EstateGovernanceQuery,
            $slot,
            BankSchema,
            $entity,
            $aspect,
            $field,
            $value,
            $write,
            EqualityPredicate,
            NoApplicationCurrency,
        > {
            ApplicationQueryResultFieldRef::new($alias, <$field>::reference())
        }
    };
}

selector!(
    estate_id,
    EstateIdSlot,
    EstateCase,
    EstateCaseRecord,
    EstateCaseIdentityField,
    EstateCaseId,
    ReadOnly,
    "estate"
);
selector!(
    estate_stage,
    EstateStageSlot,
    EstateCase,
    EstateCaseRecord,
    EstateWorkflowStageField,
    EstateWorkflowStage,
    ReadWrite,
    "stage"
);
selector!(
    assignment_id,
    AssignmentIdSlot,
    EmployeeAssignment,
    EmployeeScope,
    EmployeeAssignmentIdentityField,
    EmployeeAssignmentId,
    ReadOnly,
    "assignment"
);
selector!(
    assignment_role,
    AssignmentRoleSlot,
    EmployeeAssignment,
    EmployeeScope,
    AssignmentRole,
    EmployeeRole,
    ReadWrite,
    "role"
);
selector!(
    capability_id,
    CapabilityIdSlot,
    CapabilityGrant,
    CapabilityGrantRecord,
    CapabilityGrantIdentityField,
    CapabilityGrantId,
    ReadOnly,
    "capability"
);
selector!(
    capability_operation,
    CapabilityOperationSlot,
    CapabilityGrant,
    CapabilityGrantRecord,
    CapabilityOperationField,
    EstateCapabilityOperation,
    ReadWrite,
    "operation"
);
selector!(
    capability_purpose,
    CapabilityPurposeSlot,
    CapabilityGrant,
    CapabilityGrantRecord,
    CapabilityPurposeField,
    EstateCapabilityPurpose,
    ReadWrite,
    "purpose"
);
selector!(
    capability_valid_from,
    CapabilityValidFromSlot,
    CapabilityGrant,
    CapabilityGrantRecord,
    CapabilityValidFromField,
    EstateMoment,
    ReadWrite,
    "valid_from"
);
selector!(
    capability_valid_through,
    CapabilityValidThroughSlot,
    CapabilityGrant,
    CapabilityGrantRecord,
    CapabilityValidThroughField,
    EstateMoment,
    ReadWrite,
    "valid_through"
);
selector!(
    capability_delegation,
    CapabilityDelegationSlot,
    CapabilityGrant,
    CapabilityGrantRecord,
    CapabilityDelegationLimitField,
    DelegationLimit,
    ReadWrite,
    "delegation"
);
selector!(
    capability_workflow,
    CapabilityWorkflowSlot,
    CapabilityGrant,
    CapabilityGrantRecord,
    CapabilityWorkflowStageField,
    EstateWorkflowStage,
    ReadWrite,
    "workflow_stage"
);
selector!(
    capability_status,
    CapabilityStatusSlot,
    CapabilityGrant,
    CapabilityGrantRecord,
    CapabilityGrantStatusField,
    CapabilityGrantStatus,
    ReadWrite,
    "status"
);
selector!(
    emergency_id,
    EmergencyIdSlot,
    EmergencyAccess,
    EmergencyAccessRecord,
    EmergencyAccessIdentityField,
    EmergencyAccessId,
    ReadOnly,
    "emergency"
);
selector!(
    emergency_reason,
    EmergencyReasonSlot,
    EmergencyAccess,
    EmergencyAccessRecord,
    EmergencyAccessReasonField,
    EmergencyAccessReason,
    ReadWrite,
    "reason"
);
selector!(
    emergency_status,
    EmergencyStatusSlot,
    EmergencyAccess,
    EmergencyAccessRecord,
    EmergencyAccessStatusField,
    EmergencyAccessStatus,
    ReadWrite,
    "status"
);

macro_rules! principal_selector {
    ($name:ident, $slot:ty, $alias:literal) => {
        selector!(
            $name,
            $slot,
            Principal,
            PrincipalIdentity,
            PrincipalIdentityField,
            BankPrincipalId,
            ReadOnly,
            $alias
        );
    };
}

principal_selector!(beneficiary, BeneficiarySlot, "beneficiary");
principal_selector!(
    assignment_principal_identity,
    AssignmentPrincipalSlot,
    "principal"
);
principal_selector!(
    capability_grantee_identity,
    CapabilityGranteeSlot,
    "grantee"
);
principal_selector!(
    capability_grantor_identity,
    CapabilityGrantorSlot,
    "grantor"
);
principal_selector!(
    emergency_requester_identity,
    EmergencyRequesterSlot,
    "requester"
);
principal_selector!(
    emergency_approver_identity,
    EmergencyApproverSlot,
    "approver"
);
