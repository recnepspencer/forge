use std::collections::HashSet;
use std::hash::Hash;

use worth_ui_host_contract::{UiMountedCanonicalBox, UiMountedCoordinateSpace};

use super::aabb::Aabb;
use super::arena::{Node, NodeArena, NodeId, NodeKind};

pub(super) struct DamageHierarchy<Identity> {
    arena: NodeArena<Identity>,
    roots: [Option<NodeId>; 5],
}

pub(super) struct HierarchyQuery<Identity> {
    pub(super) identities: HashSet<Identity>,
    pub(super) branch_aabb_probes: usize,
    pub(super) leaf_command_bounds_probes: usize,
    #[cfg(test)]
    pub(super) height: u16,
}

impl<Identity> DamageHierarchy<Identity>
where
    Identity: Copy + Eq + Hash,
{
    pub(super) fn new() -> Self {
        Self {
            arena: NodeArena::new(),
            roots: [None; 5],
        }
    }

    pub(super) fn insert(&mut self, identity: Identity, bounds: UiMountedCanonicalBox) -> NodeId {
        let leaf = self
            .arena
            .allocate(Node::leaf(identity, bounds))
            .expect("the leaf limit reserves one arena leaf per retained command");
        self.attach_leaf(leaf, bounds.coordinate_space());
        leaf
    }

    pub(super) fn remove(&mut self, leaf: NodeId, space: UiMountedCoordinateSpace) {
        self.detach_leaf(leaf, space);
        self.arena.release(leaf);
    }

    pub(super) fn replace(
        &mut self,
        leaf: NodeId,
        old_space: UiMountedCoordinateSpace,
        bounds: UiMountedCanonicalBox,
    ) {
        self.detach_leaf(leaf, old_space);
        let node = self.arena.node_mut(leaf);
        let identity = match node.kind {
            NodeKind::Leaf { identity, .. } => identity,
            NodeKind::Branch { .. } => unreachable!("identity map addresses hierarchy leaves"),
        };
        *node = Node::leaf(identity, bounds);
        self.attach_leaf(leaf, bounds.coordinate_space());
    }

    pub(super) fn query(&self, damage: UiMountedCanonicalBox) -> HierarchyQuery<Identity> {
        let root = self.roots[space_index(damage.coordinate_space())];
        let height = root.map_or(0, |identity| self.arena.node(identity).height);
        let mut result = HierarchyQuery {
            identities: HashSet::new(),
            branch_aabb_probes: 0,
            leaf_command_bounds_probes: 0,
            #[cfg(test)]
            height,
        };
        let Some(root) = root else { return result };
        let damage_aabb = Aabb::from_bounds(damage);
        let mut pending = Vec::with_capacity(usize::from(height) + 1);
        pending.push(root);
        while let Some(identity) = pending.pop() {
            let node = self.arena.node(identity);
            match node.kind {
                NodeKind::Branch { left, right } => {
                    result.branch_aabb_probes += 1;
                    if node.aabb.intersects(damage_aabb) {
                        pending.push(right);
                        pending.push(left);
                    }
                }
                NodeKind::Leaf {
                    identity,
                    command_bounds,
                } => {
                    result.leaf_command_bounds_probes += 1;
                    if Aabb::from_bounds(command_bounds).intersects(damage_aabb) {
                        result.identities.insert(identity);
                    }
                }
            }
        }
        result
    }

    fn attach_leaf(&mut self, leaf: NodeId, space: UiMountedCoordinateSpace) {
        let root_index = space_index(space);
        let Some(root) = self.roots[root_index] else {
            self.roots[root_index] = Some(leaf);
            return;
        };
        let sibling = self.best_sibling(root, self.arena.node(leaf).aabb);
        let old_parent = self.arena.node(sibling).parent;
        let branch = Node::branch(
            sibling,
            leaf,
            self.arena
                .node(sibling)
                .aabb
                .union(self.arena.node(leaf).aabb),
            self.arena.node(sibling).height + 1,
        );
        let parent = self
            .arena
            .allocate(branch)
            .expect("a bounded binary forest uses at most one branch per non-root command leaf");
        self.arena.node_mut(sibling).parent = Some(parent);
        self.arena.node_mut(leaf).parent = Some(parent);
        self.arena.node_mut(parent).parent = old_parent;
        self.replace_parent_link(old_parent, sibling, parent, root_index);
        self.refit_from(Some(parent), root_index);
    }

    fn detach_leaf(&mut self, leaf: NodeId, space: UiMountedCoordinateSpace) {
        let root_index = space_index(space);
        if self.roots[root_index] == Some(leaf) {
            self.roots[root_index] = None;
            self.arena.node_mut(leaf).parent = None;
            return;
        }
        let parent = self
            .arena
            .node(leaf)
            .parent
            .expect("non-root leaf has a parent");
        let grandparent = self.arena.node(parent).parent;
        let sibling = self.other_child(parent, leaf);
        self.replace_parent_link(grandparent, parent, sibling, root_index);
        self.arena.node_mut(sibling).parent = grandparent;
        self.arena.node_mut(leaf).parent = None;
        self.arena.release(parent);
        self.refit_from(grandparent, root_index);
    }

    fn best_sibling(&self, mut node: NodeId, leaf: Aabb) -> NodeId {
        loop {
            let NodeKind::Branch { left, right } = self.arena.node(node).kind else {
                return node;
            };
            let left_cost = enlargement(self.arena.node(left).aabb, leaf);
            let right_cost = enlargement(self.arena.node(right).aabb, leaf);
            node = if (left_cost, self.arena.node(left).height)
                <= (right_cost, self.arena.node(right).height)
            {
                left
            } else {
                right
            };
        }
    }

    fn refit_from(&mut self, mut node: Option<NodeId>, root_index: usize) {
        while let Some(identity) = node {
            self.refresh(identity);
            let balanced = self.balance(identity, root_index);
            self.refresh(balanced);
            node = self.arena.node(balanced).parent;
        }
    }

    fn balance(&mut self, root: NodeId, root_index: usize) -> NodeId {
        let NodeKind::Branch { left, right } = self.arena.node(root).kind else {
            return root;
        };
        let factor =
            i32::from(self.arena.node(right).height) - i32::from(self.arena.node(left).height);
        if factor > 1 {
            if self.child_balance(right) < 0 {
                self.rotate_right(right, root_index);
            }
            return self.rotate_left(root, root_index);
        }
        if factor < -1 {
            if self.child_balance(left) > 0 {
                self.rotate_left(left, root_index);
            }
            return self.rotate_right(root, root_index);
        }
        root
    }

    fn rotate_left(&mut self, root: NodeId, root_index: usize) -> NodeId {
        let old_parent = self.arena.node(root).parent;
        let right = self.right_child(root);
        let middle = self.left_child(right);
        self.replace_parent_link(old_parent, root, right, root_index);
        self.set_left(right, root);
        self.set_right(root, middle);
        self.refresh(root);
        self.refresh(right);
        right
    }

    fn rotate_right(&mut self, root: NodeId, root_index: usize) -> NodeId {
        let old_parent = self.arena.node(root).parent;
        let left = self.left_child(root);
        let middle = self.right_child(left);
        self.replace_parent_link(old_parent, root, left, root_index);
        self.set_right(left, root);
        self.set_left(root, middle);
        self.refresh(root);
        self.refresh(left);
        left
    }

    fn refresh(&mut self, identity: NodeId) {
        let NodeKind::Branch { left, right } = self.arena.node(identity).kind else {
            return;
        };
        let aabb = self
            .arena
            .node(left)
            .aabb
            .union(self.arena.node(right).aabb);
        let height = 1 + self
            .arena
            .node(left)
            .height
            .max(self.arena.node(right).height);
        let node = self.arena.node_mut(identity);
        node.aabb = aabb;
        node.height = height;
    }

    fn replace_parent_link(
        &mut self,
        parent: Option<NodeId>,
        old: NodeId,
        new: NodeId,
        root_index: usize,
    ) {
        let Some(parent) = parent else {
            self.roots[root_index] = Some(new);
            self.arena.node_mut(new).parent = None;
            return;
        };
        let NodeKind::Branch { left, right } = self.arena.node(parent).kind else {
            unreachable!("only branches parent hierarchy nodes")
        };
        self.arena.node_mut(parent).kind = if left == old {
            NodeKind::Branch { left: new, right }
        } else {
            debug_assert_eq!(right, old);
            NodeKind::Branch { left, right: new }
        };
        self.arena.node_mut(new).parent = Some(parent);
    }

    fn set_left(&mut self, parent: NodeId, child: NodeId) {
        let right = self.right_child(parent);
        self.arena.node_mut(parent).kind = NodeKind::Branch { left: child, right };
        self.arena.node_mut(child).parent = Some(parent);
    }

    fn set_right(&mut self, parent: NodeId, child: NodeId) {
        let left = self.left_child(parent);
        self.arena.node_mut(parent).kind = NodeKind::Branch { left, right: child };
        self.arena.node_mut(child).parent = Some(parent);
    }

    fn left_child(&self, identity: NodeId) -> NodeId {
        match self.arena.node(identity).kind {
            NodeKind::Branch { left, .. } => left,
            NodeKind::Leaf { .. } => unreachable!("balanced branch has branch child"),
        }
    }

    fn right_child(&self, identity: NodeId) -> NodeId {
        match self.arena.node(identity).kind {
            NodeKind::Branch { right, .. } => right,
            NodeKind::Leaf { .. } => unreachable!("balanced branch has branch child"),
        }
    }

    fn other_child(&self, parent: NodeId, child: NodeId) -> NodeId {
        let NodeKind::Branch { left, right } = self.arena.node(parent).kind else {
            unreachable!("leaf parent is a branch")
        };
        if left == child {
            right
        } else {
            left
        }
    }

    fn child_balance(&self, identity: NodeId) -> i32 {
        let NodeKind::Branch { left, right } = self.arena.node(identity).kind else {
            return 0;
        };
        i32::from(self.arena.node(right).height) - i32::from(self.arena.node(left).height)
    }
}

fn enlargement(current: Aabb, added: Aabb) -> f32 {
    current.union(added).perimeter() - current.perimeter()
}

fn space_index(space: UiMountedCoordinateSpace) -> usize {
    match space {
        UiMountedCoordinateSpace::Viewport => 0,
        UiMountedCoordinateSpace::Window => 1,
        UiMountedCoordinateSpace::GraphNodeLocal => 2,
        UiMountedCoordinateSpace::HostSurface => 3,
        UiMountedCoordinateSpace::PortalLayer => 4,
    }
}
