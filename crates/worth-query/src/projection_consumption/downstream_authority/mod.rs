mod authority;
mod closure;
mod contract;
mod contract_document;
mod counters;
mod denial;
mod dx;
mod evidence;
mod outcome;

pub use authority::WorthQueryConsumedProjectionAuthority;
pub use closure::{
    downstream_authority_closure_contract, DownstreamAuthorityClosureContract,
    DownstreamAuthorityClosureRole, DownstreamAuthorityClosureRow,
};
pub use contract::{ProjectionAuthorityContract, ProjectionAuthorityRequirement};
pub use contract_document::{
    load_projection_authority_contract_document, ExternalProjectionAuthorityContractDocument,
    ProjectionAuthorityContractDocument, ProjectionAuthorityContractDocumentError,
    ProjectionAuthorityContractDocumentErrorKind,
};
pub use counters::ConsumedProjectionAuthorityCounters;
pub use denial::{ConsumedProjectionAuthorityDenial, ConsumedProjectionAuthorityDenialKind};
pub use evidence::ConsumedProjectionAuthorityEvidence;
pub use outcome::ProjectionAuthorityOutcome;

pub(super) fn seal_completed_consumption(
    completed: super::CompletedProjectionFactConsumption,
) -> Result<WorthQueryConsumedProjectionAuthority, ConsumedProjectionAuthorityDenial> {
    let contract = ProjectionAuthorityContract::from_consumed_request(
        completed.declaration().requested().clone(),
    );
    WorthQueryConsumedProjectionAuthority::seal(completed, contract)
}

pub(super) fn seal_completed_consumption_with_contract(
    completed: super::CompletedProjectionFactConsumption,
    contract: ProjectionAuthorityContract,
) -> Result<WorthQueryConsumedProjectionAuthority, ConsumedProjectionAuthorityDenial> {
    WorthQueryConsumedProjectionAuthority::seal(completed, contract)
}

pub(super) fn declared_fact_request(
    contract: &ProjectionAuthorityContract,
) -> super::ProjectMaterializedFacts {
    contract.fact_request()
}
