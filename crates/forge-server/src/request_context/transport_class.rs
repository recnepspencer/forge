#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeServerTransportClass {
    ForgeNativeInProcess,
    CompatHttp,
    SyncSocket,
    BinaryTransfer,
    IntegrationCallback,
}
