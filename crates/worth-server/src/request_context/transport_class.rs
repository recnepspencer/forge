#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthServerTransportClass {
    WorthNativeInProcess,
    CompatHttp,
    SyncSocket,
    BinaryTransfer,
    IntegrationCallback,
}
