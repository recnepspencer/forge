use crate::application::ForgeQueryDeclarationEnvelope;

use super::{
    artifact::ForgeQueryDeclarationRelationalBinding,
    contract::{
        ForgeQueryDeclarationRelationalAuthorityFamily, ForgeQueryDeclarationRelationalTruthClaim,
        ForgeQueryDeclarationRelationalTruthContract,
    },
};

pub(crate) fn forge_query_lower_relational_binding<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    _envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    contract: ForgeQueryDeclarationRelationalTruthContract,
) -> (
    ForgeQueryDeclarationRelationalTruthClaim,
    ForgeQueryDeclarationRelationalAuthorityFamily,
    ForgeQueryDeclarationRelationalBinding,
) {
    let binding = match contract.authority_family() {
        ForgeQueryDeclarationRelationalAuthorityFamily::Runtime => {
            ForgeQueryDeclarationRelationalBinding::Runtime("forge_relational::facade::runtime")
        }
        ForgeQueryDeclarationRelationalAuthorityFamily::History => {
            ForgeQueryDeclarationRelationalBinding::History("forge_relational::facade::history")
        }
        ForgeQueryDeclarationRelationalAuthorityFamily::GroupedTruth => {
            ForgeQueryDeclarationRelationalBinding::GroupedTruth(
                "forge_relational::facade::grouped_truth",
            )
        }
        ForgeQueryDeclarationRelationalAuthorityFamily::CommitStrategies => {
            ForgeQueryDeclarationRelationalBinding::CommitStrategies(
                "forge_relational::facade::commit_strategies",
            )
        }
        ForgeQueryDeclarationRelationalAuthorityFamily::BridgeSource => {
            ForgeQueryDeclarationRelationalBinding::BridgeSource(
                "forge_relational::facade::bridge::RuntimeBridgeRelationalSource",
            )
        }
    };
    (contract.truth_claim(), contract.authority_family(), binding)
}
