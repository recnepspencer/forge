use bank_domain::{estate::BankEstateWorld, model::EmployeeRole, proposals::BankSnapshot};
use worth_query_host::facade::declaration::authentication::WorthQueryExternalPrincipalIdentity;

use super::{super::*, FixtureWorldComposition, FixtureWorldSpec};
use crate::{BankEmployeeAssignmentSeed, BankPrincipalSeed, BankWorldSeed};

pub(super) fn identities(scenario: &str) -> [WorthQueryExternalPrincipalIdentity; 5] {
    [
        external_identity(scenario, "deceased"),
        external_identity(scenario, "specialist"),
        external_identity(scenario, "executor"),
        external_identity(scenario, "approver"),
        external_identity(scenario, "reviewer"),
    ]
}

pub(super) fn assemble(
    snapshot: BankSnapshot,
    estate: BankEstateWorld,
    identities: &[WorthQueryExternalPrincipalIdentity; 5],
    spec: &FixtureWorldSpec<'_>,
) -> BankWorldSeed {
    let seed = base_seed(snapshot, estate, identities);
    let seed = install_delegation_assignments(seed, spec.composition);
    install_additional_principals(seed, spec)
}

fn base_seed(
    snapshot: BankSnapshot,
    estate: BankEstateWorld,
    identities: &[WorthQueryExternalPrincipalIdentity; 5],
) -> BankWorldSeed {
    BankWorldSeed::new(snapshot)
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
        .estate(estate)
}

fn install_delegation_assignments(
    seed: BankWorldSeed,
    composition: FixtureWorldComposition,
) -> BankWorldSeed {
    if matches!(composition, FixtureWorldComposition::Delegation { .. }) {
        seed.employee(BankEmployeeAssignmentSeed::new(
            DELEGATION_EXECUTOR_ASSIGNMENT,
            INSTITUTION,
            EXECUTOR,
            EmployeeRole::EstateSpecialist,
        ))
        .employee(BankEmployeeAssignmentSeed::new(
            DELEGATION_REVIEWER_ASSIGNMENT,
            INSTITUTION,
            REVIEWER,
            EmployeeRole::EstateSpecialist,
        ))
    } else {
        seed
    }
}

fn install_additional_principals(
    mut seed: BankWorldSeed,
    spec: &FixtureWorldSpec<'_>,
) -> BankWorldSeed {
    let additional_principals = match spec.composition {
        FixtureWorldComposition::WarmLocality {
            axis: super::WarmLocalityAxis::Relationships,
            count,
        } => spec.unrelated_grants.max(count),
        _ => spec.unrelated_grants,
    };
    for ordinal in 0..additional_principals {
        seed = seed.principal(BankPrincipalSeed::enabled(
            extra_principal(ordinal),
            external_identity(spec.scenario, &format!("extra-{ordinal}")),
        ));
    }
    seed
}
