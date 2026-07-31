use super::capability::*;
use super::IdentityExecutionSchema;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationEntityKey, WorthQueryApplicationEntitySeed,
    WorthQueryApplicationRelationSeed, WorthQueryPrimaryGraphBootstrap,
};

pub(super) fn bind_grant(bootstrap: &mut WorthQueryPrimaryGraphBootstrap<IdentityExecutionSchema>) {
    bind_grant_window(bootstrap, "capability-1", 90, 110);
}

pub(super) fn bind_future_replacement_grant(
    bootstrap: &mut WorthQueryPrimaryGraphBootstrap<IdentityExecutionSchema>,
) {
    bind_grant_window(bootstrap, "capability-2", 111, 200);
}

fn bind_grant_window(
    bootstrap: &mut WorthQueryPrimaryGraphBootstrap<IdentityExecutionSchema>,
    key: &str,
    not_before: u64,
    not_after: u64,
) {
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                CapabilityGrant::reference(),
                WorthQueryApplicationEntityKey::new(key).unwrap(),
            )
            .field(CapabilityIdentity::reference(), key.to_owned())
            .field(CapabilityActionField::reference(), CapabilityAction::Touch)
            .field(
                CapabilityPurposeField::reference(),
                CapabilityPurpose::AccountMaintenance,
            )
            .field(
                CapabilityDisclosureField::reference(),
                CapabilityDisclosure::AccountActivity,
            )
            .field(CapabilityStatusField::reference(), CapabilityStatus::Active)
            .field(CapabilityWorkflowField::reference(), "open".to_owned())
            .field(CapabilityNotBeforeField::reference(), not_before)
            .field(CapabilityNotAfterField::reference(), not_after)
            .field(CapabilityDelegationLimitField::reference(), 0_u64),
        )
        .unwrap();
    bootstrap
        .bind_relation(WorthQueryApplicationRelationSeed::new(
            CapabilityGrantee::reference(),
            format!("{key}-grantee"),
            WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
            WorthQueryApplicationEntityKey::new(key).unwrap(),
        ))
        .unwrap();
    bootstrap
        .bind_relation(WorthQueryApplicationRelationSeed::new(
            CapabilityGrantor::reference(),
            format!("{key}-grantor"),
            WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
            WorthQueryApplicationEntityKey::new(key).unwrap(),
        ))
        .unwrap();
    bootstrap
        .bind_relation(WorthQueryApplicationRelationSeed::new(
            CapabilityResource::reference(),
            format!("{key}-resource"),
            WorthQueryApplicationEntityKey::new(key).unwrap(),
            WorthQueryApplicationEntityKey::new("account-1").unwrap(),
        ))
        .unwrap();
}
