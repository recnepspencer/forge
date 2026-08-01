use worth_foundational::facade::CanonicalDigestDerivationDenial;

use crate::canonical_digest_derivation::InstallationCanonicalIdentityBasis;
use crate::domain_computation::WorthQueryExecutionResourceContract;

use super::super::WorthQueryInstalledGraphObligationResourcePosture;

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
            encode_resource_contract(basis, contract)?;
        }
    }
    Ok(())
}

fn encode_resource_contract(
    basis: &mut InstallationCanonicalIdentityBasis,
    contract: &WorthQueryExecutionResourceContract,
) -> Result<(), CanonicalDigestDerivationDenial> {
    basis.unsigned_usize("resources.strategy-count", contract.strategies().len())?;
    for (index, strategy) in contract.strategies().iter().enumerate() {
        encode_strategy(basis, index, strategy)?;
    }
    Ok(())
}

fn encode_strategy(
    basis: &mut InstallationCanonicalIdentityBasis,
    index: usize,
    strategy: &crate::domain_computation::WorthQueryExecutionStrategyContract,
) -> Result<(), CanonicalDigestDerivationDenial> {
    let prefix = format!("resources.strategy[{index}]");
    let envelope = strategy.envelope();
    let provider = strategy.provider_requirements();
    basis.text(format!("{prefix}.name"), strategy.name().as_str())?;
    basis.text(format!("{prefix}.provider"), provider.provider().as_str())?;
    basis.text(
        format!("{prefix}.access-product"),
        provider.access_product().as_str(),
    )?;
    basis.text(format!("{prefix}.allocator"), provider.allocator().as_str())?;
    basis.text(format!("{prefix}.mode"), envelope.mode().as_str())?;
    basis.text(
        format!("{prefix}.safe-point"),
        envelope.cancellation_safe_point().as_str(),
    )?;
    basis.text(
        format!("{prefix}.degradation"),
        envelope
            .degradation()
            .map_or("complete", |posture| posture.as_str()),
    )?;
    basis.text(
        format!("{prefix}.partial-effect"),
        envelope.partial_effect_posture().as_str(),
    )?;
    basis.text(
        format!("{prefix}.yielded-state"),
        envelope.yielded_state_posture().as_str(),
    )?;
    basis.text(
        format!("{prefix}.retained-progress"),
        envelope.retained_progress_posture().as_str(),
    )?;
    for (axis, value) in envelope.scale_ceilings().iter() {
        basis.unsigned_u64(format!("{prefix}.scale.{}", axis.as_str()), value)?;
    }
    for (dimension, value) in envelope.resource_ceilings().iter() {
        basis.unsigned_u64(format!("{prefix}.resource.{}", dimension.as_str()), value)?;
    }
    Ok(())
}
