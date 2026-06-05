#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeAsyncSourceDeclarationFamilyKind {
    RequestResponse,
    SubscriptionBacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeAsyncSignalLoweringFamilyKind {
    ResourceDescriptor,
    AsyncNodeCapability,
}
