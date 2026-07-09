#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalSimulationActorAdmissionDenial {
    EmptyActorId,
    FutureExtensionActorCannotExecute,
}
