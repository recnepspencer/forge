use forge_query::facade::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationSignalCompatibilityInput, ForgeQueryResolveContinuationFromTargetRequest,
    ForgeQuerySignalCompatibilityOrchestrationInput,
};

use crate::query_domain::TopologyQueryDomain;

use super::{
    TopologyOperatorContinuationTarget, TopologyOperatorEnvelope,
    TopologyOperatorSignalCompatibilityInput,
};

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

pub fn topology_operator_continuation_target<I>(
    envelope: TopologyOperatorEnvelope<I>,
) -> TopologyOperatorContinuationTarget<I>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    ForgeQueryResolveContinuationFromTargetRequest::new(envelope, I::Family::aspect_contract())
}
