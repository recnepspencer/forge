use worth_query::facade::{domain, read};

#[path = "domain_evidence/contract.rs"]
mod contract;
#[path = "domain_evidence/direct_executor.rs"]
mod direct_executor;
#[path = "domain_evidence/material.rs"]
mod material;
#[path = "domain_evidence/workspace.rs"]
mod workspace;

pub use material::EvidenceScenario;
pub use workspace::evidence_workspace;

#[derive(Clone, Copy, Debug)]
pub struct EvidenceRead;

#[derive(Clone, Copy, Debug)]
pub struct EvidenceFamily;

impl domain::WorthQueryExecutableDomainOperation<super::GeometryDomain, EvidenceFamily>
    for EvidenceRead
{
    type Input = ();
    type Output = read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}
