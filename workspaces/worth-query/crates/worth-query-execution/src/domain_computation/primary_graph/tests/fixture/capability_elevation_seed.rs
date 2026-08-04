use super::capability::*;
use super::capability_seed::{bind_future_replacement_grant, bind_grant};
use super::IdentityExecutionSchema;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationEntityKey, WorthQueryApplicationEntitySeed,
    WorthQueryApplicationRelationSeed, WorthQueryPrimaryGraphBootstrap,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) enum CapabilityElevationScenario {
    Active,
    ConflictedApprover,
    Expired,
    Revoked,
    SelfApproved,
    WrongGrant,
}

pub(super) fn bind_elevated_capability(
    bootstrap: &mut WorthQueryPrimaryGraphBootstrap<IdentityExecutionSchema>,
    scenario: CapabilityElevationScenario,
) {
    bind_grant(bootstrap);
    if scenario == CapabilityElevationScenario::WrongGrant {
        bind_future_replacement_grant(bootstrap);
    }
    let status = match scenario {
        CapabilityElevationScenario::Expired => CapabilityElevationStatus::Expired,
        CapabilityElevationScenario::Revoked => CapabilityElevationStatus::Revoked,
        CapabilityElevationScenario::Active
        | CapabilityElevationScenario::ConflictedApprover
        | CapabilityElevationScenario::SelfApproved
        | CapabilityElevationScenario::WrongGrant => CapabilityElevationStatus::Approved,
    };
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                CapabilityElevation::reference(),
                WorthQueryApplicationEntityKey::new("elevation-1").unwrap(),
            )
            .field(
                CapabilityElevationIdentity::reference(),
                "elevation-1".to_owned(),
            )
            .field(
                CapabilityElevationReason::reference(),
                "protect-customer".to_owned(),
            )
            .field(CapabilityElevationStatusField::reference(), status)
            .field(CapabilityElevationNotBefore::reference(), 95)
            .field(CapabilityElevationNotAfter::reference(), 105),
        )
        .unwrap();
    bootstrap
        .bind_entity(
            WorthQueryApplicationEntitySeed::new(
                CapabilityReview::reference(),
                WorthQueryApplicationEntityKey::new("review-1").unwrap(),
            )
            .field(CapabilityReviewIdentity::reference(), "review-1".to_owned())
            .field(
                CapabilityReviewStatusField::reference(),
                CapabilityReviewStatus::Required,
            ),
        )
        .unwrap();
    bind_relation(
        bootstrap,
        CapabilityElevationRequester::reference(),
        "elevation-requester-1",
        "principal-0",
        "elevation-1",
    );
    bind_relation(
        bootstrap,
        CapabilityElevationApprover::reference(),
        "elevation-approver-1",
        if scenario == CapabilityElevationScenario::SelfApproved {
            "principal-0"
        } else {
            "principal-1"
        },
        "elevation-1",
    );
    bind_relation(
        bootstrap,
        CapabilityElevationGrant::reference(),
        "elevation-grant-1",
        "elevation-1",
        if scenario == CapabilityElevationScenario::WrongGrant {
            "capability-2"
        } else {
            "capability-1"
        },
    );
    bind_relation(
        bootstrap,
        CapabilityElevationReview::reference(),
        "elevation-review-1",
        "elevation-1",
        "review-1",
    );
    if scenario == CapabilityElevationScenario::ConflictedApprover {
        bind_relation(
            bootstrap,
            CapabilityConflictingBeneficiary::reference(),
            "elevation-approver-conflict-1",
            "principal-1",
            "account-1",
        );
    }
}

fn bind_relation<Relation, From, To>(
    bootstrap: &mut WorthQueryPrimaryGraphBootstrap<IdentityExecutionSchema>,
    relation: worth_query_declaration::facade::application_schema::ApplicationRelationRef<
        IdentityExecutionSchema,
        Relation,
        From,
        To,
    >,
    key: &str,
    from: &str,
    to: &str,
) {
    bootstrap
        .bind_relation(WorthQueryApplicationRelationSeed::new(
            relation,
            key,
            WorthQueryApplicationEntityKey::new(from).unwrap(),
            WorthQueryApplicationEntityKey::new(to).unwrap(),
        ))
        .unwrap();
}
