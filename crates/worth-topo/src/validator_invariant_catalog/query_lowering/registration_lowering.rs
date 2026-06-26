use forge_query::facade::{
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationRuleIdentity,
};

use super::operating_world_lowering::authoritative_operating_world_selector;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyLegalityFamilyIdentity,
    WorthTopologyLegalityFamilyRecord,
};

pub(in crate::validator_invariant_catalog::query_lowering) fn registration_from_family_record(
    record: &WorthTopologyLegalityFamilyRecord,
) -> Result<ForgeQueryGraphObligationRegistration, WorthTopologyLegalityCatalogError> {
    let rule_identity = query_rule_identity(record.identity())?;
    let touch_selector = record
        .touched_applicability()
        .query_touch_selector()
        .map_err(|error| WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string()))?;
    Ok(ForgeQueryGraphObligationRegistration::new(
        record.query_obligation_kind(),
        rule_identity,
        touch_selector,
        authoritative_operating_world_selector(),
    )
    .with_support_posture(record.query_support_posture().clone()))
}

fn query_rule_identity(
    identity: WorthTopologyLegalityFamilyIdentity,
) -> Result<ForgeQueryGraphObligationRuleIdentity, WorthTopologyLegalityCatalogError> {
    let semantic_version = match &identity {
        WorthTopologyLegalityFamilyIdentity::Validator(identity) => identity.semantic_version(),
        WorthTopologyLegalityFamilyIdentity::Invariant(identity) => identity.semantic_version(),
    };
    ForgeQueryGraphObligationRuleIdentity::new(
        "worth-topo-legality",
        identity.name(),
        semantic_version,
    )
    .map_err(|error| WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string()))
}
