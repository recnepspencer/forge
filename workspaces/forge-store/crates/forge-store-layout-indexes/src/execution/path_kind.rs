#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessPathKind {
    ExactForegroundRead,
    ReadmissionBoundary,
    BaselineBTreePointLookup,
    BaselineBTreeRootPublication,
    BaselineBTreeReplayRecovery,
    BaselineLsmPointLookup,
    BaselineLsmManifestPublication,
    BaselineLsmWalReplay,
}
