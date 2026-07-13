#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutFaultLane {
    NoFaultControl,
    CrashInterruption,
    ByteCorruption,
    StaleGeneration,
    ReorderedPersistence,
    TerminalProjectionShortcut,
}
