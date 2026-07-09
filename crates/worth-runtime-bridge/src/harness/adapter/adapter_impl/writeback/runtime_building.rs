use super::*;

pub(in crate::harness::adapter::adapter_impl::writeback) fn build_writeback_runtime(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    fixture: &BridgeHarnessFixture,
    bind_authority: bool,
) -> Result<crate::facade::RuntimeBridge, BridgeHarnessError> {
    let mut builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_policy(fixture.policy())
        .with_relational_source(runtime.source.clone())
        .with_truth_branch_head_source(runtime.source.clone())
        .with_signal_sink(runtime.sink.clone());
    if bind_authority {
        builder = builder.with_writeback_authority(runtime.writeback_authority.clone());
    }
    let (first_mapping, remaining_mappings) =
        fixture.mappings().split_first().ok_or_else(|| {
            BridgeHarnessError::new("writeback harness fixture requires at least one mapping")
        })?;
    let mut builder = builder.register_mapping(first_mapping.clone());
    for mapping in remaining_mappings {
        builder = builder.register_mapping(mapping.clone());
    }
    builder.build().map_err(|error| {
        BridgeHarnessError::new(format!(
            "failed to build writeback harness runtime with bind_authority={bind_authority}: {error}"
        ))
    })
}

pub(in crate::harness::adapter::adapter_impl::writeback) fn build_writeback_runtime_with_custom_authority<
    A,
>(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    fixture: &BridgeHarnessFixture,
    writeback_authority: A,
) -> Result<crate::facade::RuntimeBridge, BridgeHarnessError>
where
    A: crate::adapter::TruthWritebackAuthority,
{
    let builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_policy(fixture.policy())
        .with_relational_source(runtime.source.clone())
        .with_truth_branch_head_source(runtime.source.clone())
        .with_signal_sink(runtime.sink.clone())
        .with_writeback_authority(writeback_authority);
    let (first_mapping, remaining_mappings) =
        fixture.mappings().split_first().ok_or_else(|| {
            BridgeHarnessError::new("writeback harness fixture requires at least one mapping")
        })?;
    let mut builder = builder.register_mapping(first_mapping.clone());
    for mapping in remaining_mappings {
        builder = builder.register_mapping(mapping.clone());
    }
    builder.build().map_err(|error| {
        BridgeHarnessError::new(format!(
            "failed to build writeback harness runtime with custom authority: {error}"
        ))
    })
}
