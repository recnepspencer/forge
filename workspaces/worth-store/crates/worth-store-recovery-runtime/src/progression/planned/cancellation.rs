#[derive(Debug)]
pub struct PhysicalRecoveryStagingCancellation {
    plan: [u8; 32],
    settled_commands: u64,
}

impl PhysicalRecoveryStagingCancellation {
    pub(super) const fn new(plan: [u8; 32], settled_commands: u64) -> Self {
        Self {
            plan,
            settled_commands,
        }
    }

    pub(super) fn admit(self, plan: [u8; 32], command_count: u64) -> Option<u64> {
        (self.plan == plan && self.settled_commands != 0 && self.settled_commands <= command_count)
            .then_some(self.settled_commands)
    }
}
