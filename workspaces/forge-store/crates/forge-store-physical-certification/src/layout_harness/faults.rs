#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutFaultLane {
    NoFaultControl,
    CrashInterruption,
    ByteCorruption,
    StaleGeneration,
    ReorderedPersistence,
    TerminalProjectionShortcut,
}
