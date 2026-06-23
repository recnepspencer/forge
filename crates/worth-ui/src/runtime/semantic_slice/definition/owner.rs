#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSemanticSliceOwner {
    AuthoredSource,
    CapabilityAuthority,
    QueryAuthority,
    RuntimeInteractionState,
    DurableRuntimeState,
    CompiledPlatformAuthority,
}
