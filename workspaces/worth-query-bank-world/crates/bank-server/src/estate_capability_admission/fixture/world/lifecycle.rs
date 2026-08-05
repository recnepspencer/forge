use bank_domain::estate::BankEstateWorld;

use super::super::*;

pub(super) fn install_grants(estate: BankEstateWorld) -> BankEstateWorld {
    estate
        .with_grant(grant(
            COMMAND_GRANT,
            SPECIALIST,
            GrantSpec::emergency_request(),
        ))
        .with_grant(grant(
            APPROVAL_GRANT,
            APPROVER,
            GrantSpec::emergency_approval(),
        ))
        .with_grant(grant(
            SELF_APPROVAL_GRANT,
            SPECIALIST,
            GrantSpec::emergency_approval(),
        ))
        .with_grant(grant(
            APPROVER_REQUEST_GRANT,
            APPROVER,
            GrantSpec::emergency_request(),
        ))
        .with_grant(grant(
            APPROVER_UPPER_BOUND_GRANT,
            APPROVER,
            GrantSpec::emergency_view(),
        ))
        .with_grant(grant(CLOSE_GRANT, APPROVER, GrantSpec::emergency_close()))
        .with_grant(grant(REVIEW_GRANT, REVIEWER, GrantSpec::mandatory_review()))
        .with_grant(grant(
            LIFECYCLE_OBSERVER_GRANT,
            SPECIALIST,
            GrantSpec::governance_view(),
        ))
        .with_grant(grant(
            REVOKE_CAPABILITY_GRANT,
            SPECIALIST,
            GrantSpec::revoke_capability(),
        ))
}
