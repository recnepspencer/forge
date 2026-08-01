use worth_query_declaration::worth_query_application_schema;

use super::*;

worth_query_application_schema! {
    pub schema IdentityExecutionSchema {
        owner: identity_execution_test,
        version: (1, 0),
        members: |schema| {
            let schema = schema
                .entity(ExternalMapping::reference())
                .entity(Principal::reference())
                .entity(Account::reference())
                .entity(Activity::reference())
                .aspect(ExternalMapping::reference(), ExternalIdentity::reference())
                .aspect(Principal::reference(), PrincipalIdentity::reference())
                .field(ExternalMapping::reference(), ExternalIdentityField::reference())
                .field(ExternalMapping::reference(), MappingStatusField::reference())
                .field(Principal::reference(), PrincipalIdentityField::reference())
                .aspect(Account::reference(), AccountPolicy::reference())
                .field(Account::reference(), AccountIdentity::reference())
                .field(Account::reference(), AccountStatus::reference())
                .field(Account::reference(), AccountLabel::reference())
                .aspect(Activity::reference(), ActivityFacts::reference())
                .field(Activity::reference(), ActivityIdentity::reference())
                .field(Activity::reference(), ActivitySequence::reference())
                .relation(
                    MappingTarget::reference(),
                    ExternalMapping::reference(),
                    Principal::reference(),
                )
                .relation(
                    AccountOwner::reference(),
                    Principal::reference(),
                    Account::reference(),
                )
                .relation(
                    AccountBlocked::reference(),
                    Principal::reference(),
                    Account::reference(),
                )
                .relation(
                    AccountPrimaryActivity::reference(),
                    Account::reference(),
                    Activity::reference(),
                )
                .relation(
                    AccountSecondaryActivity::reference(),
                    Account::reference(),
                    Activity::reference(),
                )
                .relation(
                    AccountAllActivity::reference(),
                    Account::reference(),
                    Activity::reference(),
                )
                .relation(
                    ActivityAccount::reference(),
                    Activity::reference(),
                    Account::reference(),
                )
                .principal_binding(IdentityBinding::reference())
                .ability(ViewAccount::reference())
                .ability(EditAccount::reference())
                .ability(ManageOwnership::reference())
                .effect(AccountActivityEffect::reference())
                .effect(LiveActivityEffect::reference())
                .operation(TouchAccountOperation::reference())
                .operation_decision_fact_budget(TouchAccountOperation::reference(), 1)
                .operation_projection_work_budget(TouchAccountOperation::reference(), 32)
                .operation_requires_ability(
                    TouchAccountOperation::reference(),
                    ViewAccount::reference(),
                )
                .operation_write(TouchAccountOperation::reference(), AccountStatus::reference())
                .operation_emit(
                    TouchAccountOperation::reference(),
                    AccountActivityEffect::reference(),
                )
                .operation_emit(
                    TouchAccountOperation::reference(),
                    LiveActivityEffect::reference(),
                )
                .operation_read_field(
                    TouchAccountOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation_expected_fact(
                    TouchAccountOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation(PublishLiveActivityOperation::reference())
                .operation_decision_fact_budget(PublishLiveActivityOperation::reference(), 2)
                .operation_projection_work_budget(PublishLiveActivityOperation::reference(), 32)
                .operation_requires_ability(
                    PublishLiveActivityOperation::reference(),
                    ViewAccount::reference(),
                )
                .operation_read_field(
                    PublishLiveActivityOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation_read_field(
                    PublishLiveActivityOperation::reference(),
                    AccountLabel::reference(),
                )
                .operation_write(
                    PublishLiveActivityOperation::reference(),
                    AccountLabel::reference(),
                )
                .operation_emit(
                    PublishLiveActivityOperation::reference(),
                    LiveActivityEffect::reference(),
                )
                .operation_expected_fact(
                    PublishLiveActivityOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation(MultiTouchOperation::reference())
                .operation_decision_fact_budget(MultiTouchOperation::reference(), 2)
                .operation_projection_work_budget(MultiTouchOperation::reference(), 32)
                .operation_requires_ability(
                    MultiTouchOperation::reference(),
                    ViewAccount::reference(),
                )
                .operation_requires_ability(
                    MultiTouchOperation::reference(),
                    EditAccount::reference(),
                )
                .operation_write(MultiTouchOperation::reference(), AccountStatus::reference())
                .operation_read_field(
                    MultiTouchOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation(ChangeOwnershipOperation::reference())
                .operation_decision_fact_budget(ChangeOwnershipOperation::reference(), 2)
                .operation_projection_work_budget(ChangeOwnershipOperation::reference(), 32)
                .operation_requires_ability(
                    ChangeOwnershipOperation::reference(),
                    ManageOwnership::reference(),
                )
                .operation_read_relation(
                    ChangeOwnershipOperation::reference(),
                    AccountOwner::reference(),
                )
                .operation_read_field(
                    ChangeOwnershipOperation::reference(),
                    AccountStatus::reference(),
                )
                .operation_link(
                    ChangeOwnershipOperation::reference(),
                    AccountOwner::reference(),
                )
                .operation_unlink(
                    ChangeOwnershipOperation::reference(),
                    AccountOwner::reference(),
                )
                .policy(AccountAccessPolicy::reference())
                .ability_policy(
                    ViewAccount::reference(),
                    AccountAccessPolicy::reference(),
                    [
                        worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                            Principal::reference(),
                        )
                        .forward(AccountOwner::reference())
                        .allow(Account::reference()),
                        worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                            Principal::reference(),
                        )
                        .forward(AccountBlocked::reference())
                        .deny(Account::reference()),
                    ],
                )
                .ability_policy(
                    EditAccount::reference(),
                    AccountAccessPolicy::reference(),
                    [
                        worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                            Principal::reference(),
                        )
                        .forward(AccountOwner::reference())
                        .allow(Account::reference()),
                        worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                            Principal::reference(),
                        )
                        .forward(AccountBlocked::reference())
                        .deny(Account::reference()),
                    ],
                )
                .ability_policy(
                    ManageOwnership::reference(),
                    AccountAccessPolicy::reference(),
                    [worth_query_declaration::facade::application_schema::ApplicationAuthorizationPathBuilder::from_principal(
                        Principal::reference(),
                    )
                    .allow(Principal::reference())],
                )
                .application_query(application_queries::account_summary_definition())
                .application_query(application_queries::scoped_account_summary_definition())
                .application_query(application_queries::cross_root_definition("open"))
                .application_query(application_queries::governed_account_summary_definition())
                .application_query(application_queries::ordered_account_summary_definition())
                .application_query(nested_account::nested_account_definition())
                .application_query(forged_selector::forged_selector_definition())
                .application_query(live_account_query::live_account_activity_definition())
                .application_query(governed_live_query::governed_live_account_definition())
                .application_query(governed_omission_query::governed_account_omission_definition())
                .application_query(invalid_disclosure_queries::incomplete_disclosure_definition())
                .application_query(invalid_disclosure_queries::forbidden_influence_definition());
            capability::install(schema)
        }
    }
}
