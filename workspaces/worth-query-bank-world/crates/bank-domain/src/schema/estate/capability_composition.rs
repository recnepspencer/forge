use worth_query_decl::facade::{
    application_capability::{
        ApplicationCapabilityAcceptedValues, ApplicationCapabilityActorComposition,
        ApplicationCapabilityAllowRule, ApplicationCapabilityComposition,
        ApplicationCapabilityConflictRule, ApplicationCapabilityDecisionComposition,
        ApplicationCapabilityDelegationRule, ApplicationCapabilityDenyRule,
        ApplicationCapabilityDisclosureRule, ApplicationCapabilityDistinctActorRule,
        ApplicationCapabilityGraphClause, ApplicationCapabilityGraphRule,
        ApplicationCapabilityPropagationComposition, ApplicationCapabilityScopeGuard,
        ApplicationCapabilitySeparationOfDutyRule,
    },
    application_schema::{ApplicationAuthorizationPath, ApplicationAuthorizationPathBuilder},
};

use crate::{
    estate::{EstateCapabilityOperation, EstateCapabilityPurpose, RestrictedBankField},
    model::EmployeeRole,
    schema::*,
};

pub(super) fn composition(
    action: EstateCapabilityOperation,
    purpose: EstateCapabilityPurpose,
) -> ApplicationCapabilityComposition {
    ApplicationCapabilityComposition::new(
        ApplicationCapabilityDecisionComposition::new(
            ApplicationCapabilityAllowRule::new(allow_rule(action)),
            deny_rule(action),
            conflict_rule(action),
        ),
        ApplicationCapabilityActorComposition::new(
            separation_of_duty_rule(action),
            distinct_actor_rule(action),
        ),
        ApplicationCapabilityPropagationComposition::new(
            ApplicationCapabilityDelegationRule::narrow_all_dimensions(),
            disclosure_rule(action, purpose),
        ),
    )
}

fn allow_rule(action: EstateCapabilityOperation) -> ApplicationCapabilityGraphRule {
    if action == EstateCapabilityOperation::ViewRestrictedEstate {
        return view_allow_rule();
    }
    let roles = if branch_manager_may_perform(action) {
        vec![
            EmployeeRole::BranchManager,
            EmployeeRole::EstateSpecialist,
            EmployeeRole::Compliance,
            EmployeeRole::Legal,
        ]
    } else {
        vec![
            EmployeeRole::EstateSpecialist,
            EmployeeRole::Compliance,
            EmployeeRole::Legal,
        ]
    };
    ApplicationCapabilityGraphRule::any(
        roles
            .into_iter()
            .map(|role| ApplicationCapabilityGraphClause::new(employee_allow_path(role))),
    )
}

fn view_allow_rule() -> ApplicationCapabilityGraphRule {
    ApplicationCapabilityGraphRule::any([
        guarded_employee_path(
            EmployeeRole::BranchManager,
            [
                RestrictedBankField::CustomerIdentity,
                RestrictedBankField::AccountDetails,
            ],
        ),
        guarded_employee_path(
            EmployeeRole::EstateSpecialist,
            [
                RestrictedBankField::CustomerIdentity,
                RestrictedBankField::BeneficiaryIdentity,
                RestrictedBankField::AccountDetails,
                RestrictedBankField::PostingHistory,
                RestrictedBankField::AuditTrail,
            ],
        ),
        guarded_employee_path(EmployeeRole::Compliance, RestrictedBankField::ALL),
        guarded_employee_path(EmployeeRole::Legal, RestrictedBankField::ALL),
    ])
}

fn guarded_employee_path(
    role: EmployeeRole,
    fields: impl IntoIterator<Item = RestrictedBankField>,
) -> ApplicationCapabilityGraphClause {
    ApplicationCapabilityGraphClause::when(
        employee_allow_path(role),
        [ApplicationCapabilityAcceptedValues::one_of(
            CapabilityDisclosureField::reference(),
            fields,
        )],
    )
}

fn employee_allow_path(role: EmployeeRole) -> ApplicationAuthorizationPath {
    ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .reverse(AssignmentPrincipal::reference())
        .where_equal(AssignmentRole::reference(), role)
        .forward(EstateAssignment::reference())
        .allow(EstateCase::reference())
}

fn deny_rule(action: EstateCapabilityOperation) -> ApplicationCapabilityDenyRule {
    if action == EstateCapabilityOperation::ViewRestrictedEstate {
        ApplicationCapabilityDenyRule::when(ApplicationCapabilityGraphRule::any([
            ApplicationCapabilityGraphClause::new(beneficiary_deny_path()),
        ]))
    } else {
        ApplicationCapabilityDenyRule::not_applicable()
    }
}

fn conflict_rule(action: EstateCapabilityOperation) -> ApplicationCapabilityConflictRule {
    if matches!(
        action,
        EstateCapabilityOperation::ApproveEmergencyAccess
            | EstateCapabilityOperation::CompleteMandatoryReview
            | EstateCapabilityOperation::ReleaseEstate
            | EstateCapabilityOperation::DisburseEstate
    ) {
        ApplicationCapabilityConflictRule::when(ApplicationCapabilityGraphRule::any([
            ApplicationCapabilityGraphClause::new(beneficiary_deny_path()),
        ]))
    } else {
        ApplicationCapabilityConflictRule::not_applicable()
    }
}

fn beneficiary_deny_path() -> ApplicationAuthorizationPath {
    ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(EstateBeneficiary::reference())
        .deny(EstateCase::reference())
}

fn separation_of_duty_rule(
    action: EstateCapabilityOperation,
) -> ApplicationCapabilitySeparationOfDutyRule {
    let path = match action {
        EstateCapabilityOperation::RecognizeExecutor => Some(
            ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
                .reverse(LegalAuthorityHolder::reference())
                .forward(LegalAuthorityEstate::reference())
                .deny(EstateCase::reference()),
        ),
        EstateCapabilityOperation::CompleteMandatoryReview
        | EstateCapabilityOperation::ReleaseEstate
        | EstateCapabilityOperation::DisburseEstate => Some(
            ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
                .forward(EstateExecutor::reference())
                .deny(EstateCase::reference()),
        ),
        _ => None,
    };
    path.map_or_else(
        ApplicationCapabilitySeparationOfDutyRule::not_applicable,
        |path| {
            ApplicationCapabilitySeparationOfDutyRule::when(ApplicationCapabilityGraphRule::any([
                ApplicationCapabilityGraphClause::new(path),
            ]))
        },
    )
}

fn distinct_actor_rule(
    action: EstateCapabilityOperation,
) -> ApplicationCapabilityDistinctActorRule {
    let paths = match action {
        EstateCapabilityOperation::ApproveEmergencyAccess => {
            vec![emergency_actor_path(EmergencyRequester::reference())]
        }
        EstateCapabilityOperation::CompleteMandatoryReview => vec![
            emergency_review_actor_path(EmergencyRequester::reference()),
            emergency_review_actor_path(EmergencyApprover::reference()),
        ],
        _ => Vec::new(),
    };
    if paths.is_empty() {
        ApplicationCapabilityDistinctActorRule::not_applicable()
    } else {
        ApplicationCapabilityDistinctActorRule::when(ApplicationCapabilityGraphRule::any(
            paths.into_iter().map(ApplicationCapabilityGraphClause::new),
        ))
    }
}

fn emergency_actor_path<Relation>(
    relation: worth_query_decl::facade::application_schema::ApplicationRelationRef<
        BankSchema,
        Relation,
        Principal,
        EmergencyAccess,
    >,
) -> ApplicationAuthorizationPath {
    ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(relation)
        .forward(EmergencyGrant::reference())
        .forward(CapabilityEstate::reference())
        .deny(EstateCase::reference())
}

fn emergency_review_actor_path<Relation>(
    relation: worth_query_decl::facade::application_schema::ApplicationRelationRef<
        BankSchema,
        Relation,
        Principal,
        EmergencyAccess,
    >,
) -> ApplicationAuthorizationPath {
    ApplicationAuthorizationPathBuilder::from_principal(Principal::reference())
        .forward(relation)
        .forward(EmergencyReview::reference())
        .forward(ReviewEstate::reference())
        .deny(EstateCase::reference())
}

fn disclosure_rule(
    action: EstateCapabilityOperation,
    purpose: EstateCapabilityPurpose,
) -> ApplicationCapabilityDisclosureRule {
    if action != EstateCapabilityOperation::ViewRestrictedEstate {
        return ApplicationCapabilityDisclosureRule::not_applicable();
    }
    ApplicationCapabilityDisclosureRule::permit([ApplicationCapabilityScopeGuard::requiring([
        ApplicationCapabilityAcceptedValues::one_of(
            CapabilityDisclosureField::reference(),
            permitted_fields(purpose),
        ),
    ])])
}

fn permitted_fields(purpose: EstateCapabilityPurpose) -> Vec<RestrictedBankField> {
    RestrictedBankField::ALL
        .into_iter()
        .filter(|field| field.permits(purpose))
        .collect()
}

const fn branch_manager_may_perform(action: EstateCapabilityOperation) -> bool {
    matches!(
        action,
        EstateCapabilityOperation::FreezeAccount
            | EstateCapabilityOperation::OpenEstateCase
            | EstateCapabilityOperation::DelegateCapability
            | EstateCapabilityOperation::RevokeCapability
            | EstateCapabilityOperation::RequestEmergencyAccess
            | EstateCapabilityOperation::RevokeEmergencyAccess
    )
}
