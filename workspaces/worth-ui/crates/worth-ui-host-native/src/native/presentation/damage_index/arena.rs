use worth_ui_host_contract::UiMountedCanonicalBox;

use super::aabb::Aabb;

pub(super) const MAX_LEAVES: usize =
    crate::native_profile::UiNativeMechanicsCapacities::QUALIFIED.retained_commands as usize;
const MAX_NODES: usize = MAX_LEAVES * 2 - 1;

pub(super) type NodeId = usize;

#[derive(Clone, Copy)]
pub(super) enum NodeKind<Identity> {
    Leaf {
        identity: Identity,
        command_bounds: UiMountedCanonicalBox,
    },
    Branch {
        left: NodeId,
        right: NodeId,
    },
}

pub(super) struct Node<Identity> {
    pub(super) parent: Option<NodeId>,
    pub(super) aabb: Aabb,
    pub(super) height: u16,
    pub(super) kind: NodeKind<Identity>,
}

impl<Identity: Copy> Node<Identity> {
    pub(super) fn leaf(identity: Identity, bounds: UiMountedCanonicalBox) -> Self {
        Self {
            parent: None,
            aabb: Aabb::from_bounds(bounds),
            height: 0,
            kind: NodeKind::Leaf {
                identity,
                command_bounds: bounds,
            },
        }
    }

    pub(super) fn branch(left: NodeId, right: NodeId, aabb: Aabb, height: u16) -> Self {
        Self {
            parent: None,
            aabb,
            height,
            kind: NodeKind::Branch { left, right },
        }
    }
}

pub(super) struct NodeArena<Identity> {
    slots: Vec<Option<Node<Identity>>>,
    free: Vec<NodeId>,
}

impl<Identity> NodeArena<Identity> {
    pub(super) fn new() -> Self {
        Self {
            slots: Vec::with_capacity(MAX_NODES),
            free: Vec::with_capacity(MAX_NODES),
        }
    }

    pub(super) fn allocate(&mut self, node: Node<Identity>) -> Option<NodeId> {
        if let Some(identity) = self.free.pop() {
            self.slots[identity] = Some(node);
            return Some(identity);
        }
        if self.slots.len() == MAX_NODES {
            return None;
        }
        let identity = self.slots.len();
        self.slots.push(Some(node));
        Some(identity)
    }

    pub(super) fn release(&mut self, identity: NodeId) {
        let released = self.slots[identity].take();
        debug_assert!(released.is_some());
        self.free.push(identity);
    }

    pub(super) fn node(&self, identity: NodeId) -> &Node<Identity> {
        self.slots[identity]
            .as_ref()
            .expect("live hierarchy links address live arena nodes")
    }

    pub(super) fn node_mut(&mut self, identity: NodeId) -> &mut Node<Identity> {
        self.slots[identity]
            .as_mut()
            .expect("live hierarchy links address live arena nodes")
    }
}
