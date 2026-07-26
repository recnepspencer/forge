use worth_query::facade::{domain, read};

#[path = "domain_evidence/contract.rs"]
mod contract;
#[path = "domain_evidence/direct_executor.rs"]
mod direct_executor;
#[path = "domain_evidence/material.rs"]
mod material;
#[path = "domain_evidence/workflow_executor.rs"]
mod workflow_executor;
#[path = "domain_evidence/workflow_workspace.rs"]
mod workflow_workspace;
#[path = "domain_evidence/workspace.rs"]
mod workspace;

pub use contract::EvidenceGovernance;
pub use material::EvidenceScenario;
pub use workflow_executor::EvidenceWorkflowMode;
pub use workflow_workspace::{evidence_workflow_intent, evidence_workflow_workspace};
pub use workspace::{evidence_workspace, evidence_workspace_with_governance};

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
