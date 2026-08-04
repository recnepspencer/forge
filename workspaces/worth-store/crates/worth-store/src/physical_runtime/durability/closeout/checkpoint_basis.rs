pub enum PhysicalRecoveryCheckpointBasis {
    GenerationZero,
    NamespaceDurable(crate::physical_runtime::CompletedPhysicalCheckpoint),
}

impl PhysicalRecoveryCheckpointBasis {
    pub(in crate::physical_runtime) fn from_latest(
        latest: Option<crate::physical_runtime::CompletedPhysicalCheckpoint>,
    ) -> Self {
        latest.map_or(Self::GenerationZero, Self::NamespaceDurable)
    }

    pub const fn completed(&self) -> Option<&crate::physical_runtime::CompletedPhysicalCheckpoint> {
        match self {
            Self::GenerationZero => None,
            Self::NamespaceDurable(completed) => Some(completed),
        }
    }
}
