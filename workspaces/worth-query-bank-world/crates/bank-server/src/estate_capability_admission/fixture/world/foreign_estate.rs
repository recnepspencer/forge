use bank_domain::estate::{
    BankEstateWorld, CapabilityGrantId, DeathNoticeId, DeathNoticeStatus, DelegationLimit,
    EstateCase, EstateCaseId, EstateCaseStatus, EstateDeathNotice, EstateWorkflowStage,
};

use super::{FixtureWorldComposition, FixtureWorldSpec};
use crate::estate_capability_admission::fixture::{
    grant, CapabilityFixture, GrantSpec, ACCOUNT, ASSIGNMENT, BRANCH, DECEASED, INSTITUTION,
    SPECIALIST,
};

pub(crate) const FOREIGN_ESTATE: EstateCaseId = EstateCaseId::new(103).unwrap();
pub(crate) const FOREIGN_GRANT: CapabilityGrantId = CapabilityGrantId::new(120).unwrap();
const FOREIGN_DEATH_NOTICE: DeathNoticeId = DeathNoticeId::new(112).unwrap();
const FOREIGN_REVOCATION_GRANT: CapabilityGrantId = CapabilityGrantId::new(121).unwrap();
const FOREIGN_GOVERNANCE_GRANT: CapabilityGrantId = CapabilityGrantId::new(122).unwrap();
const PRIMARY_GOVERNANCE_GRANT: CapabilityGrantId = CapabilityGrantId::new(123).unwrap();

pub(crate) fn foreign_estate_revocation_world(scenario: &str) -> CapabilityFixture {
    super::super::capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: GrantSpec::emergency_view(),
        case_stage: EstateWorkflowStage::Administration,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::ForeignEstateRevocation,
        alternate_emergency_bound: None,
    })
}

pub(crate) fn install_foreign_estate_revocation(estate: BankEstateWorld) -> BankEstateWorld {
    estate
        .with_grant(grant(
            PRIMARY_GOVERNANCE_GRANT,
            SPECIALIST,
            GrantSpec::governance_view(),
        ))
        .with_death_notice(EstateDeathNotice {
            id: FOREIGN_DEATH_NOTICE,
            subject: DECEASED,
            status: DeathNoticeStatus::Verified,
        })
        .with_case(EstateCase {
            id: FOREIGN_ESTATE,
            institution: INSTITUTION,
            branch: BRANCH,
            deceased: DECEASED,
            account: ACCOUNT,
            death_notice: FOREIGN_DEATH_NOTICE,
            stage: EstateWorkflowStage::Administration,
            status: EstateCaseStatus::Open,
        })
        .with_estate_assignment(FOREIGN_ESTATE, ASSIGNMENT)
        .with_grant(foreign_grant(FOREIGN_GRANT, GrantSpec::emergency_view()))
        .with_grant(foreign_grant(
            FOREIGN_REVOCATION_GRANT,
            GrantSpec::revoke_capability(),
        ))
        .with_grant(foreign_grant(
            FOREIGN_GOVERNANCE_GRANT,
            GrantSpec::governance_view(),
        ))
}

fn foreign_grant(
    id: CapabilityGrantId,
    spec: GrantSpec,
) -> bank_domain::estate::EstateCapabilityGrant {
    let mut grant = grant(id, SPECIALIST, spec);
    grant.scope.estate = FOREIGN_ESTATE;
    grant.scope.delegation = DelegationLimit::none();
    grant
}
