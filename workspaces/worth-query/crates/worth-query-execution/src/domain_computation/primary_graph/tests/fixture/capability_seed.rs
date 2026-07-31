use super::capability::*;
use super::IdentityExecutionSchema;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationEntityKey, WorthQueryApplicationEntitySeed,
    WorthQueryApplicationRelationSeed, WorthQueryPrimaryGraphBootstrap,
};

pub(super) fn bind_grant(
    bootstrap: &mut WorthQueryPrimaryGraphBootstrap<IdentityExecutionSchema>,
) {
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                CapabilityGrant::reference(),
                WorthQueryApplicationEntityKey::new("capability-1").unwrap(),
            )
            .field(CapabilityIdentity::reference(), "capability-1".to_owned())
            .field(CapabilityActionField::reference(), CapabilityAction::Touch)
            .field(
                CapabilityPurposeField::reference(),
                CapabilityPurpose::AccountMaintenance,
            )
            .field(CapabilityStatusField::reference(), CapabilityStatus::Active)
            .field(CapabilityWorkflowField::reference(), "open".to_owned())
            .field(CapabilityNotBeforeField::reference(), 90_u64)
            .field(CapabilityNotAfterField::reference(), 110_u64)
            .field(CapabilityDelegationLimitField::reference(), 0_u64),
        )
        .unwrap();
    for seed in [
        WorthQueryApplicationRelationSeed::new(
            CapabilityGrantee::reference(),
            "capability-grantee",
            WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
            WorthQueryApplicationEntityKey::new("capability-1").unwrap(),
        ),
        WorthQueryApplicationRelationSeed::new(
            CapabilityGrantor::reference(),
            "capability-grantor",
            WorthQueryApplicationEntityKey::new("principal-0").unwrap(),
            WorthQueryApplicationEntityKey::new("capability-1").unwrap(),
        ),
        WorthQueryApplicationRelationSeed::new(
            CapabilityResource::reference(),
            "capability-resource",
            WorthQueryApplicationEntityKey::new("capability-1").unwrap(),
            WorthQueryApplicationEntityKey::new("account-1").unwrap(),
        ),
    ] {
        bootstrap.bind_relation(seed).unwrap();
    }
}
