mod mappings;

#[cfg(test)]
mod tests;

use std::sync::Arc;

use forge_relational::facade::bridge::RuntimeBridgeRelationalSource;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::{
    BridgeBuildError, BridgeWritebackOutcomeClass, RuntimeBridge, RuntimeBridgeBuilder,
    SignalBridgeSink, TruthWritebackAuthority, TruthWritebackAuthorityError, TruthWritebackReceipt,
    TruthWritebackRequest,
};

pub(crate) use mappings::{
    milestone_one_bridge_aspect_registrations, milestone_one_bridge_mapping_registrations,
};

pub(crate) fn build_milestone_one_bridge<S>(
    runtime: Arc<RelationalRuntime>,
    sink: S,
) -> Result<RuntimeBridge, BridgeBuildError>
where
    S: SignalBridgeSink + Clone + 'static,
{
    let source = RuntimeBridgeRelationalSource::new(runtime);
    let builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(sink)
        .with_writeback_authority(TopologyProductionWritebackAuthority);
    let mut mappings = milestone_one_bridge_mapping_registrations().into_iter();
    let first = mappings
        .next()
        .expect(" milestone 1 bridge mapping pack should not be empty");
    let builder = mappings.fold(builder.register_mapping(first), |builder, registration| {
        builder.register_mapping(registration)
    });
    let builder = milestone_one_bridge_aspect_registrations()
        .into_iter()
        .fold(builder, |builder, registration| {
            builder.register_aspect_mapping(registration)
        });
    builder.build()
}

#[derive(Clone, Debug)]
struct TopologyProductionWritebackAuthority;

impl TruthWritebackAuthority for TopologyProductionWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        Ok(TruthWritebackReceipt::new(
            BridgeWritebackOutcomeClass::AuthoritativeCommit,
            &request,
        ))
    }
}
