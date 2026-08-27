use worth_foundational::{
    FoundationalBranchComparisonBasis, FoundationalBranchForkBasis, FoundationalBranchId,
    FoundationalBranchIdConstructionDenial, FoundationalBranchReferenceGeneration,
    FoundationalBranchReferenceObservation, FoundationalBranchTarget,
};

use super::target::{SignalBranchTarget, SignalBranchTargetConstructionDenial};

pub type SignalBranchObservation = FoundationalBranchReferenceObservation<SignalBranchTarget>;
pub type SignalBranchForkBasis = FoundationalBranchForkBasis<SignalBranchTarget>;
pub type SignalBranchComparisonBasis = FoundationalBranchComparisonBasis<SignalBranchTarget>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalBranchObservationConstructionDenial {
    InvalidBranchId(FoundationalBranchIdConstructionDenial),
    EmptyOwnerComponent,
    InvalidTarget(SignalBranchTargetConstructionDenial),
    GraphInstanceMismatch {
        observation_graph_instance_id: String,
        target_graph_instance_id: String,
    },
}

pub fn signal_branch_observation(
    graph_instance_id: impl AsRef<str>,
    branch_id: u64,
    branch_name: impl AsRef<str>,
    target: FoundationalBranchTarget<SignalBranchTarget>,
    generation: FoundationalBranchReferenceGeneration,
) -> Result<SignalBranchObservation, SignalBranchObservationConstructionDenial> {
    let graph_instance_id = graph_instance_id.as_ref();
    if let FoundationalBranchTarget::Basis(target) = &target {
        if target.graph_instance_id() != graph_instance_id {
            return Err(
                SignalBranchObservationConstructionDenial::GraphInstanceMismatch {
                    observation_graph_instance_id: graph_instance_id.to_owned(),
                    target_graph_instance_id: target.graph_instance_id().to_owned(),
                },
            );
        }
    }
    let graph_instance_id = encode_owner_component(graph_instance_id)?;
    let branch_id = encode_owner_component(&branch_id.to_string())?;
    let branch_name = encode_owner_component(branch_name.as_ref())?;
    let branch_identity = FoundationalBranchId::new(format!(
        "signal/{graph_instance_id}/{branch_id}/{branch_name}"
    ))?;
    Ok(SignalBranchObservation::new(
        branch_identity,
        target,
        generation,
    ))
}

fn encode_owner_component(
    component: &str,
) -> Result<String, SignalBranchObservationConstructionDenial> {
    if component.trim().is_empty() {
        return Err(SignalBranchObservationConstructionDenial::EmptyOwnerComponent);
    }
    Ok(format!("{}:{component}", component.len()))
}

impl From<FoundationalBranchIdConstructionDenial> for SignalBranchObservationConstructionDenial {
    fn from(denial: FoundationalBranchIdConstructionDenial) -> Self {
        Self::InvalidBranchId(denial)
    }
}

impl From<SignalBranchTargetConstructionDenial> for SignalBranchObservationConstructionDenial {
    fn from(denial: SignalBranchTargetConstructionDenial) -> Self {
        Self::InvalidTarget(denial)
    }
}
