//! Application-owner admission evidence for native mounted identities.

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiNativeClientAuthoredMountedInstanceObservation {
    authored_semantic_identity_digest: [u8; 32],
    mounted_instance: u64,
}

pub(super) type Observations = Box<[UiNativeClientAuthoredMountedInstanceObservation]>;

impl UiNativeClientAuthoredMountedInstanceObservation {
    pub fn reported(authored_semantic_identity_digest: [u8; 32], mounted_instance: u64) -> Self {
        Self {
            authored_semantic_identity_digest,
            mounted_instance,
        }
    }

    pub const fn authored_semantic_identity_digest(self) -> [u8; 32] {
        self.authored_semantic_identity_digest
    }

    pub const fn mounted_instance(self) -> u64 {
        self.mounted_instance
    }
}

impl super::UiNativeClientShutdownObservation {
    pub fn with_authored_mounted_instances(
        mut self,
        observations: Box<[UiNativeClientAuthoredMountedInstanceObservation]>,
    ) -> Self {
        self.authored_mounted_instances = observations;
        self
    }

    pub fn authored_mounted_instances(
        &self,
    ) -> &[UiNativeClientAuthoredMountedInstanceObservation] {
        &self.authored_mounted_instances
    }
}
