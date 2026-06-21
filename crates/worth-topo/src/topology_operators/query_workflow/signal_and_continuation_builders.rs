#[cfg(test)]
use forge_query::facade::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationSignalCompatibilityInput, ForgeQueryResolveContinuationFromTargetRequest,
    ForgeQuerySignalCompatibilityOrchestrationInput,
};

#[cfg(test)]
use crate::query_domain::TopologyQueryDomain;

#[cfg(test)]
use super::{
    TopologyOperatorContinuationTarget, TopologyOperatorEnvelope,
    TopologyOperatorSignalCompatibilityInput,
};

#[cfg(test)]
pub fn topology_operator_signal_workflow<I>(
    envelope: TopologyOperatorEnvelope<I>,
) -> TopologyOperatorSignalCompatibilityInput<I>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    ForgeQuerySignalCompatibilityOrchestrationInput::new(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
    )
}

#[cfg(test)]
pub fn topology_operator_continuation_target<I>(
    envelope: TopologyOperatorEnvelope<I>,
) -> TopologyOperatorContinuationTarget<I>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    ForgeQueryResolveContinuationFromTargetRequest::new(envelope, I::Family::aspect_contract())
}
