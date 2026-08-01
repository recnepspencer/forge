use worth_foundational::facade::CanonicalDigestDerivationDenial;

use crate::application_operation::WorthQueryInstalledAbilityRequirement;
use crate::canonical_digest_derivation::InstallationCanonicalIdentityBasis;
use crate::domain_operation::{
    WorthQueryInstalledInvariantExecutionRequirement, WorthQueryOperationGraphParticipation,
    WorthQueryOperationGraphReadRole, WorthQueryOperationTouchContract,
};

use super::super::{
    WorthQueryInstalledGraphCapabilityRequirement, WorthQueryInstalledGraphObligationContract,
};

pub(super) fn encode_contract(
    basis: &mut InstallationCanonicalIdentityBasis,
    index: usize,
    contract: &WorthQueryInstalledGraphObligationContract,
) -> Result<(), CanonicalDigestDerivationDenial> {
    let prefix = format!("obligation[{index}]");
    match contract {
        WorthQueryInstalledGraphObligationContract::QueryGraphRead { graph } => {
            basis.text(format!("{prefix}.route"), "query-graph-read")?;
            basis.digest(format!("{prefix}.graph"), *graph.digest())?;
        }
        WorthQueryInstalledGraphObligationContract::OperationGraphRead { role } => {
            encode_operation_read(basis, &prefix, role)?;
        }
        WorthQueryInstalledGraphObligationContract::PrincipalAuthorization => {
            basis.text(format!("{prefix}.route"), "principal-authorization")?;
        }
        WorthQueryInstalledGraphObligationContract::AbilityAuthorization { requirements } => {
            encode_abilities(basis, &prefix, requirements)?;
        }
        WorthQueryInstalledGraphObligationContract::CapabilityAuthorization { requirements } => {
            encode_capabilities(basis, &prefix, requirements)?;
        }
        WorthQueryInstalledGraphObligationContract::MutationTouch { contract } => {
            encode_touch(basis, &prefix, contract)?;
        }
        WorthQueryInstalledGraphObligationContract::EffectApplication { family } => {
            basis.text(format!("{prefix}.route"), "effect-application")?;
            basis.text(format!("{prefix}.family"), family.as_str())?;
        }
        WorthQueryInstalledGraphObligationContract::InvariantExecution { requirement } => {
            encode_invariant(basis, &prefix, requirement)?;
        }
    }
    Ok(())
}

fn encode_operation_read(
    basis: &mut InstallationCanonicalIdentityBasis,
    prefix: &str,
    role: &WorthQueryOperationGraphReadRole,
) -> Result<(), CanonicalDigestDerivationDenial> {
    basis.text(format!("{prefix}.route"), "operation-graph-read")?;
    basis.text(format!("{prefix}.role"), &role.role)?;
    match &role.participation {
        WorthQueryOperationGraphParticipation::PrimaryLogicalGraph => {
            basis.text(format!("{prefix}.participation"), "primary")?;
        }
        WorthQueryOperationGraphParticipation::SeparateAuthority { role } => {
            basis.text(format!("{prefix}.participation"), "separate-authority")?;
            basis.text(format!("{prefix}.authority-role"), role)?;
        }
    }
    basis.text(format!("{prefix}.access"), role.access.as_str())?;
    basis.unsigned_usize(
        format!("{prefix}.semantic-read-count"),
        role.semantic_reads.len(),
    )?;
    for (index, read) in role.semantic_reads.iter().enumerate() {
        basis.embedded_basis(
            &format!("{prefix}.semantic-read[{index}].contract"),
            read.canonical_contract_basis(),
        )?;
        basis.embedded_basis(
            &format!("{prefix}.semantic-read[{index}].mask"),
            read.canonical_mask_basis(),
        )?;
    }
    Ok(())
}

fn encode_abilities(
    basis: &mut InstallationCanonicalIdentityBasis,
    prefix: &str,
    requirements: &[WorthQueryInstalledAbilityRequirement],
) -> Result<(), CanonicalDigestDerivationDenial> {
    basis.text(format!("{prefix}.route"), "ability-authorization")?;
    basis.unsigned_usize(format!("{prefix}.count"), requirements.len())?;
    for (index, requirement) in requirements.iter().enumerate() {
        basis.digest(
            format!("{prefix}.requirement[{index}]"),
            *requirement.identity(),
        )?;
    }
    Ok(())
}

fn encode_capabilities(
    basis: &mut InstallationCanonicalIdentityBasis,
    prefix: &str,
    requirements: &[WorthQueryInstalledGraphCapabilityRequirement],
) -> Result<(), CanonicalDigestDerivationDenial> {
    basis.text(format!("{prefix}.route"), "capability-authorization")?;
    basis.unsigned_usize(format!("{prefix}.count"), requirements.len())?;
    for (index, requirement) in requirements.iter().enumerate() {
        basis.digest(
            format!("{prefix}.requirement[{index}]"),
            *requirement.identity().digest(),
        )?;
    }
    Ok(())
}

fn encode_touch(
    basis: &mut InstallationCanonicalIdentityBasis,
    prefix: &str,
    contract: &WorthQueryOperationTouchContract,
) -> Result<(), CanonicalDigestDerivationDenial> {
    basis.text(format!("{prefix}.route"), "mutation-touch")?;
    let WorthQueryOperationTouchContract::Declared {
        graph_roles,
        scopes,
    } = contract
    else {
        return Ok(());
    };
    basis.unsigned_usize(format!("{prefix}.graph-role-count"), graph_roles.len())?;
    for (index, role) in graph_roles.iter().enumerate() {
        basis.text(format!("{prefix}.graph-role[{index}]"), role)?;
    }
    basis.unsigned_usize(format!("{prefix}.scope-count"), scopes.len())?;
    for (index, scope) in scopes.iter().enumerate() {
        basis.text(format!("{prefix}.scope[{index}]"), scope)?;
    }
    Ok(())
}

fn encode_invariant(
    basis: &mut InstallationCanonicalIdentityBasis,
    prefix: &str,
    requirement: &WorthQueryInstalledInvariantExecutionRequirement,
) -> Result<(), CanonicalDigestDerivationDenial> {
    basis.text(format!("{prefix}.route"), "invariant-execution")?;
    basis.text(format!("{prefix}.slot"), requirement.slot())?;
    basis.text(format!("{prefix}.family"), requirement.family())?;
    basis.unsigned_u32(format!("{prefix}.version"), requirement.version().get())?;
    basis.text(
        format!("{prefix}.enforcement"),
        requirement.enforcement().as_str(),
    )?;
    basis.text(
        format!("{prefix}.executor-role"),
        requirement.executor_role(),
    )?;
    basis.unsigned_usize(
        format!("{prefix}.state-load-count"),
        requirement.state_load_families().len(),
    )?;
    for (index, family) in requirement.state_load_families().iter().enumerate() {
        basis.text(format!("{prefix}.state-load[{index}]"), family)?;
    }
    basis.unsigned_usize(
        format!("{prefix}.maximum-state-facts"),
        requirement.max_state_facts(),
    )?;
    basis.unsigned_u64(
        format!("{prefix}.maximum-work-units"),
        requirement.max_work_units(),
    )?;
    Ok(())
}
