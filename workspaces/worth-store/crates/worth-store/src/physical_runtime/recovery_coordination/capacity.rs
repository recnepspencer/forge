#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecoveryCoordinationCapacity {
    commands: usize,
    semantic_bytes: usize,
}

impl PhysicalRecoveryCoordinationCapacity {
    pub fn admit(commands: u64, semantic_bytes: u64) -> Option<Self> {
        let commands = usize::try_from(commands).ok()?;
        let semantic_bytes = usize::try_from(semantic_bytes).ok()?;
        (commands != 0 && semantic_bytes != 0).then_some(Self {
            commands,
            semantic_bytes,
        })
    }

    pub(super) fn work_capacity(self) -> crate::physical_runtime::PhysicalWorkCapacity {
        crate::physical_runtime::PhysicalWorkCapacity::new(
            self.commands,
            1,
            self.commands,
            self.semantic_bytes,
            self.semantic_bytes,
        )
        .expect("admission established nonzero recovery coordination capacity")
    }
}
