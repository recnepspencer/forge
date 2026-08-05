use bank_domain::{estate::BankEstateWorld, schema::*};
use worth_query_host::facade::primary_graph::{
    WorthQueryPrimaryGraphBootstrap, WorthQueryPrimaryGraphInstallationDenial,
};

use super::{
    super::{account_key, employee_key, institution_key, principal_key},
    keys::{authority, branch, estate, notice, review},
    relation_seed,
};

pub(super) fn bind(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    bind_case_context(graph, world)?;
    bind_actors(graph, world)?;
    bind_authorities_and_reviews(graph, world)
}

fn bind_case_context(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for value in world.branches() {
        relation_seed::bind(
            graph,
            BranchInstitution::reference(),
            format!("branch-institution:{}", value.id.get()),
            branch(value.id.get()),
            institution_key(value.institution.get()),
        )?;
    }
    for value in world.death_notices() {
        relation_seed::bind(
            graph,
            DeathNoticeSubject::reference(),
            format!("death-notice-subject:{}", value.id.get()),
            notice(value.id.get()),
            principal_key(value.subject.get()),
        )?;
    }
    for value in world.cases() {
        let case = estate(value.id.get());
        relation_seed::bind(
            graph,
            EstateDeathNotice::reference(),
            format!("estate-notice:{}", value.id.get()),
            case.clone(),
            notice(value.death_notice.get()),
        )?;
        relation_seed::bind(
            graph,
            EstateDeceased::reference(),
            format!("estate-deceased:{}", value.id.get()),
            case.clone(),
            principal_key(value.deceased.get()),
        )?;
        relation_seed::bind(
            graph,
            EstateAccount::reference(),
            format!("estate-account:{}", value.id.get()),
            case.clone(),
            account_key(value.account),
        )?;
        relation_seed::bind(
            graph,
            EstateBranch::reference(),
            format!("estate-branch:{}", value.id.get()),
            case,
            branch(value.branch.get()),
        )?;
    }
    Ok(())
}

fn bind_actors(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for (case, principal) in world.executors() {
        relation_seed::bind(
            graph,
            EstateExecutor::reference(),
            format!("estate-executor:{}:{}", case.get(), principal.get()),
            principal_key(principal.get()),
            estate(case.get()),
        )?;
    }
    for (case, principal) in world.beneficiaries() {
        relation_seed::bind(
            graph,
            EstateBeneficiary::reference(),
            format!("estate-beneficiary:{}:{}", case.get(), principal.get()),
            principal_key(principal.get()),
            estate(case.get()),
        )?;
    }
    for (case, assignment) in world.estate_assignments() {
        relation_seed::bind(
            graph,
            EstateAssignment::reference(),
            format!("estate-assignment:{}:{}", case.get(), assignment.get()),
            employee_key(assignment.get()),
            estate(case.get()),
        )?;
    }
    for (account, principal) in world.joint_owners() {
        relation_seed::bind(
            graph,
            EstateJointOwner::reference(),
            format!(
                "estate-joint-owner:{}:{}",
                account.canonical_text(),
                principal.get()
            ),
            principal_key(principal.get()),
            account_key(account),
        )?;
    }
    for (account, principal) in world.authorized_signers() {
        relation_seed::bind(
            graph,
            EstateAuthorizedSigner::reference(),
            format!(
                "estate-authorized-signer:{}:{}",
                account.canonical_text(),
                principal.get()
            ),
            principal_key(principal.get()),
            account_key(account),
        )?;
    }
    Ok(())
}

fn bind_authorities_and_reviews(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for value in world.legal_authorities() {
        relation_seed::bind(
            graph,
            LegalAuthorityEstate::reference(),
            format!("legal-authority-estate:{}", value.id.get()),
            authority(value.id.get()),
            estate(value.estate.get()),
        )?;
        relation_seed::bind(
            graph,
            LegalAuthorityHolder::reference(),
            format!("legal-authority-holder:{}", value.id.get()),
            authority(value.id.get()),
            principal_key(value.holder.get()),
        )?;
    }
    for value in world.reviews() {
        relation_seed::bind(
            graph,
            ReviewEstate::reference(),
            format!("review-estate:{}", value.id.get()),
            review(value.id.get()),
            estate(value.estate.get()),
        )?;
        if let Some(reviewer) = value.reviewer {
            relation_seed::bind(
                graph,
                ReviewPrincipal::reference(),
                format!("review-principal:{}", value.id.get()),
                principal_key(reviewer.get()),
                review(value.id.get()),
            )?;
        }
    }
    Ok(())
}
