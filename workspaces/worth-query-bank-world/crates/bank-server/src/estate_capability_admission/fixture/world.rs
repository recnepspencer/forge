use bank_domain::{
    estate::{BankEstateWorld, CapabilityGrantId, EstateWorkflowStage},
    model::{AccountName, BankSnapshotVersion, EmployeeRole},
    proposals::{BankSnapshot, BankSnapshotBuilder},
    schema::AccountStatus,
};
use worth_query_host::facade::declaration::authentication::WorthQueryExternalPrincipalIdentity;

use super::{
    base_estate, external_identity, extra_principal, grant, GrantSpec, ACCOUNT, APPROVAL_GRANT,
    APPROVER, APPROVER_ASSIGNMENT, APPROVER_REQUEST_GRANT, APPROVER_UPPER_BOUND_GRANT, ASSIGNMENT,
    CLOSE_GRANT, COMMAND_GRANT, DECEASED, EXECUTOR, GRANT, INSTITUTION, OTHER_ACCOUNT, REVIEWER,
    REVIEWER_ASSIGNMENT, REVIEW_GRANT, SELF_APPROVAL_GRANT, SPECIALIST,
};
use crate::{
    BankEmployeeAssignmentSeed, BankIdentityRuntime, BankPrincipalSeed, BankWorldSeed,
};

pub(super) struct FixtureWorldSpec<'a> {
    pub(super) scenario: &'a str,
    pub(super) spec: GrantSpec,
    pub(super) case_stage: EstateWorkflowStage,
    pub(super) specialist_holds_authority: bool,
    pub(super) unrelated_grants: usize,
    pub(super) install_command_grants: bool,
}

pub(super) struct InstalledFixtureWorld {
    pub(super) runtime: BankIdentityRuntime,
    pub(super) estate_world: BankEstateWorld,
    pub(super) identities: [WorthQueryExternalPrincipalIdentity; 5],
}

pub(super) fn install_fixture_world(spec: FixtureWorldSpec<'_>) -> InstalledFixtureWorld {
    let identities = identities(spec.scenario);
    let snapshot = snapshot(spec.unrelated_grants);
    let estate = estate(&spec);
    let estate_world = estate.clone();
    let seed = seed(
        snapshot,
        estate,
        &identities,
        spec.scenario,
        spec.unrelated_grants,
    );
    let runtime = BankIdentityRuntime::install_world(seed)
        .expect("capability fixture runtime should install");
    InstalledFixtureWorld {
        runtime,
        estate_world,
        identities,
    }
}

fn identities(scenario: &str) -> [WorthQueryExternalPrincipalIdentity; 5] {
    [
        external_identity(scenario, "deceased"),
        external_identity(scenario, "specialist"),
        external_identity(scenario, "executor"),
        external_identity(scenario, "approver"),
        external_identity(scenario, "reviewer"),
    ]
}

fn snapshot(unrelated_grants: usize) -> BankSnapshot {
    let mut snapshot = BankSnapshotBuilder::new(BankSnapshotVersion::new(1).unwrap())
        .institution(INSTITUTION)
        .principal(DECEASED)
        .principal(SPECIALIST)
        .principal(EXECUTOR)
        .principal(APPROVER)
        .principal(REVIEWER)
        .personal_account(
            ACCOUNT,
            INSTITUTION,
            DECEASED,
            AccountName::new("Estate Operating").unwrap(),
            AccountStatus::Frozen,
        )
        .personal_account(
            OTHER_ACCOUNT,
            INSTITUTION,
            EXECUTOR,
            AccountName::new("Executor Settlement").unwrap(),
            AccountStatus::Open,
        );
    for ordinal in 0..unrelated_grants {
        snapshot = snapshot.principal(extra_principal(ordinal));
    }
    snapshot
        .build()
        .expect("capability fixture snapshot should be valid")
}

fn estate(spec: &FixtureWorldSpec<'_>) -> BankEstateWorld {
    let holder = if spec.specialist_holds_authority {
        SPECIALIST
    } else {
        EXECUTOR
    };
    let mut estate =
        base_estate(spec.case_stage, holder).with_grant(grant(GRANT, SPECIALIST, spec.spec));
    if spec.install_command_grants {
        estate = install_lifecycle_grants(estate);
    }
    for ordinal in 0..spec.unrelated_grants {
        estate = estate.with_grant(grant(
            CapabilityGrantId::new(2_000 + ordinal as u64).unwrap(),
            extra_principal(ordinal),
            GrantSpec::view(),
        ));
    }
    estate
}

fn install_lifecycle_grants(estate: BankEstateWorld) -> BankEstateWorld {
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
        .with_grant(grant(
            CLOSE_GRANT,
            APPROVER,
            GrantSpec::emergency_close(),
        ))
        .with_grant(grant(
            REVIEW_GRANT,
            REVIEWER,
            GrantSpec::mandatory_review(),
        ))
}

fn seed(
    snapshot: BankSnapshot,
    estate: BankEstateWorld,
    identities: &[WorthQueryExternalPrincipalIdentity; 5],
    scenario: &str,
    unrelated_grants: usize,
) -> BankWorldSeed {
    let mut seed = BankWorldSeed::new(snapshot)
        .principal(BankPrincipalSeed::enabled(DECEASED, identities[0].clone()))
        .principal(BankPrincipalSeed::enabled(
            SPECIALIST,
            identities[1].clone(),
        ))
        .principal(BankPrincipalSeed::enabled(EXECUTOR, identities[2].clone()))
        .principal(BankPrincipalSeed::enabled(APPROVER, identities[3].clone()))
        .principal(BankPrincipalSeed::enabled(REVIEWER, identities[4].clone()))
        .employee(BankEmployeeAssignmentSeed::new(
            ASSIGNMENT,
            INSTITUTION,
            SPECIALIST,
            EmployeeRole::EstateSpecialist,
        ))
        .employee(BankEmployeeAssignmentSeed::new(
            APPROVER_ASSIGNMENT,
            INSTITUTION,
            APPROVER,
            EmployeeRole::EstateSpecialist,
        ))
        .employee(BankEmployeeAssignmentSeed::new(
            REVIEWER_ASSIGNMENT,
            INSTITUTION,
            REVIEWER,
            EmployeeRole::Compliance,
        ))
        .estate(estate);
    for ordinal in 0..unrelated_grants {
        seed = seed.principal(BankPrincipalSeed::enabled(
            extra_principal(ordinal),
            external_identity(scenario, &format!("extra-{ordinal}")),
        ));
    }
    seed
}
