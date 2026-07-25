use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MountedIdentityModelOperation {
    Mount(u8),
    Unmount(u8),
    Reorder(&'static [u8]),
    RebindSurface,
    AdvanceFrame,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MountedIdentityModelSnapshot {
    live_instances: BTreeSet<u8>,
    visible_order: Vec<u8>,
    binding_generation: u16,
    frame_generation: u16,
    frame_current: bool,
    incarnation_generation: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MountedIdentityModel {
    snapshot: MountedIdentityModelSnapshot,
}

impl MountedIdentityModel {
    pub(crate) fn known_empty_surface() -> Self {
        Self {
            snapshot: MountedIdentityModelSnapshot {
                live_instances: BTreeSet::new(),
                visible_order: Vec::new(),
                binding_generation: 1,
                frame_generation: 0,
                frame_current: false,
                incarnation_generation: 0,
            },
        }
    }

    pub(crate) fn apply(&mut self, operation: MountedIdentityModelOperation) {
        match operation {
            MountedIdentityModelOperation::Mount(identity) => {
                assert!(self.snapshot.live_instances.insert(identity));
                self.snapshot.visible_order.push(identity);
                self.snapshot.incarnation_generation += 1;
            }
            MountedIdentityModelOperation::Unmount(identity) => {
                assert!(self.snapshot.live_instances.remove(&identity));
                self.snapshot
                    .visible_order
                    .retain(|candidate| *candidate != identity);
            }
            MountedIdentityModelOperation::Reorder(order) => {
                let requested = order.iter().copied().collect::<BTreeSet<_>>();
                assert_eq!(requested, self.snapshot.live_instances);
                assert_eq!(requested.len(), order.len());
                self.snapshot.visible_order.clear();
                self.snapshot.visible_order.extend_from_slice(order);
            }
            MountedIdentityModelOperation::RebindSurface => {
                self.snapshot.binding_generation += 1;
                self.snapshot.frame_current = false;
            }
            MountedIdentityModelOperation::AdvanceFrame => {
                self.snapshot.frame_generation += 1;
                self.snapshot.frame_current = true;
            }
        }
    }

    pub(crate) fn snapshot(&self) -> &MountedIdentityModelSnapshot {
        &self.snapshot
    }
}

impl MountedIdentityModelSnapshot {
    pub(crate) fn live_count(&self) -> usize {
        self.live_instances.len()
    }

    pub(crate) fn visible_order(&self) -> &[u8] {
        &self.visible_order
    }

    pub(crate) fn binding_generation(&self) -> u16 {
        self.binding_generation
    }

    pub(crate) fn frame_generation(&self) -> u16 {
        self.frame_generation
    }

    pub(crate) fn frame_current(&self) -> bool {
        self.frame_current
    }

    pub(crate) fn incarnation_generation(&self) -> u16 {
        self.incarnation_generation
    }
}
