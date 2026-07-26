#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiStateOwnershipClass {
    PlatformShell,
    NodeIdentity,
    ShellLocalInteraction,
}
