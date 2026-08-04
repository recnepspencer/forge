use worth_foundational::facade::CanonicalDigestDerivationDenial;

use crate::canonical_digest_derivation::InstallationCanonicalIdentityBasis;
use crate::domain_operation::{
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
    WorthQueryOperationTouchContract,
};

use super::super::{
    WorthQueryInstalledGraphObligationContract, WorthQueryInstalledGraphObligationResourcePosture,
};

pub(super) fn encode_contract(
    basis: &mut InstallationCanonicalIdentityBasis,
    index: usize,
    contract: &WorthQueryInstalledGraphObligationContract,
) -> Result<(), CanonicalDigestDerivationDenial> {
    let prefix = format!("obligation[{index}]");
    match contract {
        WorthQueryInstalledGraphObligationContract::QueryGraphRead { graph } => {
            basis.text(format!("{prefix}.kind"), "query-graph-read")?;
            basis.digest(format!("{prefix}.graph"), *graph.digest())?;
        }
        WorthQueryInstalledGraphObligationContract::OperationGraphRead { role } => {
            basis.text(format!("{prefix}.kind"), "operation-graph-read")?;
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
            basis.text(
                format!("{prefix}.access"),
                match role.access {
                    WorthQueryOperationGraphAccess::Observe => "observe",
                    WorthQueryOperationGraphAccess::Project => "project",
                },
            )?;
            basis.unsigned_usize(
                format!("{prefix}.semantic-read-count"),
                role.semantic_reads.len(),
            )?;
            for (read_index, read) in role.semantic_reads.iter().enumerate() {
                basis.embedded_basis(
                    &format!("{prefix}.semantic-read[{read_index}].contract"),
                    read.canonical_contract_basis(),
                )?;
                basis.embedded_basis(
                    &format!("{prefix}.semantic-read[{read_index}].mask"),
                    read.canonical_mask_basis(),
                )?;
            }
        }
        WorthQueryInstalledGraphObligationContract::PrincipalAuthorization => {
            basis.text(format!("{prefix}.kind"), "principal-authorization")?;
        }
        WorthQueryInstalledGraphObligationContract::AbilityAuthorization { requirements } => {
            basis.text(format!("{prefix}.kind"), "ability-authorization")?;
            basis.unsigned_usize(format!("{prefix}.count"), requirements.len())?;
            for (requirement_index, requirement) in requirements.iter().enumerate() {
                basis.digest(
                    format!("{prefix}.requirement[{requirement_index}]"),
                    *requirement.identity(),
                )?;
            }
        }
        WorthQueryInstalledGraphObligationContract::CapabilityAuthorization { requirements } => {
            basis.text(format!("{prefix}.kind"), "capability-authorization")?;
            basis.unsigned_usize(format!("{prefix}.count"), requirements.len())?;
            for (requirement_index, requirement) in requirements.iter().enumerate() {
                basis.digest(
                    format!("{prefix}.requirement[{requirement_index}]"),
                    *requirement.identity().digest(),
                )?;
            }
        }
        WorthQueryInstalledGraphObligationContract::MutationTouch { contract } => {
            basis.text(format!("{prefix}.kind"), "mutation-touch")?;
            encode_touch(basis, &prefix, contract)?;
        }
        WorthQueryInstalledGraphObligationContract::EffectApplication { family } => {
            basis.text(format!("{prefix}.kind"), "effect-application")?;
            basis.text(format!("{prefix}.family"), family.as_str())?;
        }
        WorthQueryInstalledGraphObligationContract::InvariantExecution { requirement } => {
            basis.text(format!("{prefix}.kind"), "invariant-execution")?;
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
            for (load_index, family) in requirement.state_load_families().iter().enumerate() {
                basis.text(format!("{prefix}.state-load[{load_index}]"), family)?;
            }
            basis.unsigned_usize(
                format!("{prefix}.maximum-state-facts"),
                requirement.max_state_facts(),
            )?;
            basis.unsigned_u64(
                format!("{prefix}.maximum-work-units"),
                requirement.max_work_units(),
            )?;
        }
    }
    Ok(())
}

pub(super) fn encode_resources(
    basis: &mut InstallationCanonicalIdentityBasis,
    resources: &WorthQueryInstalledGraphObligationResourcePosture,
) -> Result<(), CanonicalDigestDerivationDenial> {
    match resources {
        WorthQueryInstalledGraphObligationResourcePosture::ApplicationQuery {
            maximum_traversal_depth,
            maximum_result_count,
            maximum_authorization_facts,
        } => {
            basis.text("resources.kind", "application-query")?;
            basis.unsigned_usize(
                "resources.maximum-traversal-depth",
                *maximum_traversal_depth,
            )?;
            basis.unsigned_usize("resources.maximum-result-count", *maximum_result_count)?;
            basis.unsigned_usize(
                "resources.maximum-authorization-facts",
                *maximum_authorization_facts,
            )?;
        }
        WorthQueryInstalledGraphObligationResourcePosture::ApplicationOperation(contract) => {
            basis.text("resources.kind", "application-operation")?;
            basis.text("resources.contract", contract.canonical_identity())?;
        }
    }
    Ok(())
}

fn encode_touch(
    basis: &mut InstallationCanonicalIdentityBasis,
    prefix: &str,
    contract: &WorthQueryOperationTouchContract,
) -> Result<(), CanonicalDigestDerivationDenial> {
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
