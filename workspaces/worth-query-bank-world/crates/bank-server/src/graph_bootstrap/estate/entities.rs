use bank_domain::{estate::BankEstateWorld, schema::*};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationEntitySeed, WorthQueryPrimaryGraphBootstrap,
    WorthQueryPrimaryGraphInstallationDenial,
};

use super::{
    super::entity_key,
    keys::{authority, branch, emergency, estate, grant, notice, review},
};

pub(super) fn bind(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    bind_core(graph, world)?;
    bind_capabilities(graph, world)?;
    bind_emergency_access(graph, world)?;
    bind_reviews(graph, world)
}

fn bind_core(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for value in world.branches() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                Branch::reference(),
                entity_key(branch(value.id.get())),
            )
            .field(BranchIdentityField::reference(), value.id),
        )?;
    }
    for value in world.death_notices() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                DeathNotice::reference(),
                entity_key(notice(value.id.get())),
            )
            .field(DeathNoticeIdentityField::reference(), value.id)
            .field(DeathNoticeStatusField::reference(), value.status),
        )?;
    }
    for value in world.cases() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                EstateCase::reference(),
                entity_key(estate(value.id.get())),
            )
            .field(EstateCaseIdentityField::reference(), value.id)
            .field(EstateWorkflowStageField::reference(), value.stage)
            .field(EstateCaseStatusField::reference(), value.status),
        )?;
    }
    for value in world.legal_authorities() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                LegalAuthority::reference(),
                entity_key(authority(value.id.get())),
            )
            .field(LegalAuthorityIdentityField::reference(), value.id)
            .field(LegalAuthorityKindField::reference(), value.kind)
            .field(LegalAuthorityRecognizedField::reference(), value.recognized),
        )?;
    }
    Ok(())
}

fn bind_capabilities(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for value in world.grants() {
        let mut seed = WorthQueryApplicationEntitySeed::new(
            CapabilityGrant::reference(),
            entity_key(grant(value.id.get())),
        )
        .field(CapabilityGrantIdentityField::reference(), value.id)
        .field(CapabilityOperationField::reference(), value.scope.operation)
        .field(CapabilityPurposeField::reference(), value.scope.purpose)
        .field(
            CapabilityValidFromField::reference(),
            value.scope.validity.not_before(),
        )
        .field(
            CapabilityValidThroughField::reference(),
            value.scope.validity.not_after(),
        )
        .field(
            CapabilityDelegationLimitField::reference(),
            value.scope.delegation,
        )
        .field(
            CapabilityWorkflowStageField::reference(),
            value.scope.workflow_stage,
        )
        .field(CapabilityGrantStatusField::reference(), value.status);
        if let Some(field) = value.scope.field {
            seed = seed.field(CapabilityDisclosureField::reference(), field);
        }
        if let Some(amount) = value.scope.amount_ceiling {
            seed = seed.field(CapabilityAmountCeilingField::reference(), amount);
        }
        graph.bind_entity(seed)?;
    }
    Ok(())
}

fn bind_emergency_access(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for value in world.emergency_accesses() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                EmergencyAccess::reference(),
                entity_key(emergency(value.id.get())),
            )
            .field(EmergencyAccessIdentityField::reference(), value.id)
            .field(EmergencyAccessReasonField::reference(), value.reason)
            .field(EmergencyAccessStatusField::reference(), value.status)
            .field(EmergencyAccessIssuedAtField::reference(), value.issued_at)
            .field(EmergencyAccessExpiresAtField::reference(), value.expires_at),
        )?;
    }
    Ok(())
}

fn bind_reviews(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for value in world.reviews() {
        graph.bind_entity(
            WorthQueryApplicationEntitySeed::new(
                MandatoryReview::reference(),
                entity_key(review(value.id.get())),
            )
            .field(MandatoryReviewIdentityField::reference(), value.id)
            .field(MandatoryReviewKindField::reference(), value.kind)
            .field(MandatoryReviewStatusField::reference(), value.status),
        )?;
    }
    Ok(())
}
