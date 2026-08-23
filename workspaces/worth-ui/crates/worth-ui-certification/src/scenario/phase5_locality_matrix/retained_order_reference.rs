//! Independent AVL path model for exact retained-order touch accounting.

use super::case::{Phase5LocalityAxis as Axis, Phase5LocalityCase};

pub(super) fn expected_touches(case: Phase5LocalityCase) -> u64 {
    if case.axis() == Axis::Dpi {
        return 0;
    }
    let commands = case.retained_paragraphs() * 2;
    let mut tree = ReferenceOrder::sequential(commands);
    tree.touches = if case.axis() == Axis::PinRelease {
        commands.saturating_sub(2)
    } else {
        commands
    };
    match case.axis() {
        Axis::Width => {
            for _ in 0..2 {
                tree.touch_rank(commands - 2);
                tree.touch_rank(commands - 1);
            }
        }
        Axis::PinRelease => tree.remove_last_pair_and_replay(),
        _ => {
            for rank in 0..commands {
                tree.touch_rank(rank);
            }
        }
    }
    tree.touches as u64
}

struct ReferenceOrder {
    nodes: Vec<Node>,
    root: Option<usize>,
    touches: usize,
}

#[derive(Clone, Copy)]
struct Node {
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
    height: u16,
    size: usize,
    active: bool,
}

impl ReferenceOrder {
    fn sequential(count: usize) -> Self {
        let mut tree = Self {
            nodes: Vec::with_capacity(count),
            root: None,
            touches: 0,
        };
        for rank in 0..count {
            let node = tree.nodes.len();
            tree.nodes.push(Node::leaf());
            tree.root = Some(tree.insert(tree.root, node, rank));
            tree.set_parent(tree.root, None);
        }
        tree.touches = 0;
        tree
    }

    fn touch_rank(&mut self, mut rank: usize) {
        let mut node = self.root.expect("reference order is nonempty");
        loop {
            self.touches += 1;
            let left = self.size(self.nodes[node].left);
            if rank < left {
                node = self.nodes[node].left.unwrap();
            } else if rank == left {
                return;
            } else {
                rank -= left + 1;
                node = self.nodes[node].right.unwrap();
            }
        }
    }

    fn remove_last_pair_and_replay(&mut self) {
        let original = self.size(self.root);
        let identities = [original - 2, original - 1];
        for identity in identities {
            let rank = self.rank(identity);
            if rank > 0 {
                self.touch_rank(rank - 1);
            }
        }
        for identity in identities {
            let rank = self.rank(identity);
            if rank > 0 {
                self.touch_rank(rank - 1);
            }
            self.touch_rank_or_absent(rank + 1);
            self.remove_identity(identity);
        }
        for rank in 0..self.size(self.root) {
            self.touch_rank(rank);
        }
    }

    fn touch_rank_or_absent(&mut self, mut rank: usize) {
        let Some(mut node) = self.root else {
            return;
        };
        loop {
            self.touches += 1;
            let left = self.size(self.nodes[node].left);
            if rank < left {
                let Some(left) = self.nodes[node].left else {
                    return;
                };
                node = left;
            } else if rank == left {
                return;
            } else {
                rank -= left + 1;
                let Some(right) = self.nodes[node].right else {
                    return;
                };
                node = right;
            }
        }
    }

    fn rank(&mut self, identity: usize) -> usize {
        let mut node = identity;
        let mut rank = self.size(self.nodes[node].left);
        self.touches += 1;
        while let Some(parent) = self.nodes[node].parent {
            self.touches += 1;
            if self.nodes[parent].right == Some(node) {
                rank += self.size(self.nodes[parent].left) + 1;
            }
            node = parent;
        }
        rank
    }

    fn remove_identity(&mut self, identity: usize) {
        let rank = self.rank(identity);
        let root = self.root.unwrap();
        let (next, removed) = self.remove_at(root, rank);
        self.root = next;
        self.set_parent(next, None);
        self.nodes[removed].active = false;
    }

    fn remove_at(&mut self, root: usize, rank: usize) -> (Option<usize>, usize) {
        self.touches += 1;
        let left = self.size(self.nodes[root].left);
        if rank < left {
            let child = self.nodes[root].left.unwrap();
            let (next, removed) = self.remove_at(child, rank);
            self.nodes[root].left = next;
            self.set_parent(next, Some(root));
            return (Some(self.rebalance(root)), removed);
        }
        if rank > left {
            let child = self.nodes[root].right.unwrap();
            let (next, removed) = self.remove_at(child, rank - left - 1);
            self.nodes[root].right = next;
            self.set_parent(next, Some(root));
            return (Some(self.rebalance(root)), removed);
        }
        match (self.nodes[root].left, self.nodes[root].right) {
            (None, child) | (child, None) => {
                self.set_parent(child, self.nodes[root].parent);
                (child, root)
            }
            (Some(_), Some(right)) => {
                let successor = self.minimum(right);
                let (next, removed) = self.remove_at(right, 0);
                self.nodes[root].right = next;
                self.set_parent(next, Some(root));
                self.swap_identity_positions(root, successor);
                (Some(self.rebalance(root)), removed)
            }
        }
    }

    fn swap_identity_positions(&mut self, _root: usize, _successor: usize) {
        // The qualified pin-release world removes the final two ranks, so this
        // branch is unreachable. Keeping it explicit prevents a silent model.
        panic!("reference pin removal unexpectedly moved an interior successor")
    }

    fn minimum(&mut self, mut node: usize) -> usize {
        self.touches += 1;
        while let Some(left) = self.nodes[node].left {
            self.touches += 1;
            node = left;
        }
        node
    }

    fn insert(&mut self, root: Option<usize>, node: usize, rank: usize) -> usize {
        let Some(root) = root else {
            return node;
        };
        let left = self.size(self.nodes[root].left);
        if rank <= left {
            let child = self.insert(self.nodes[root].left, node, rank);
            self.nodes[root].left = Some(child);
            self.nodes[child].parent = Some(root);
        } else {
            let child = self.insert(self.nodes[root].right, node, rank - left - 1);
            self.nodes[root].right = Some(child);
            self.nodes[child].parent = Some(root);
        }
        self.rebalance(root)
    }

    fn rebalance(&mut self, root: usize) -> usize {
        self.refresh(root);
        let balance =
            self.height(self.nodes[root].left) as i32 - self.height(self.nodes[root].right) as i32;
        if balance > 1 {
            return self.rotate_right(root);
        }
        if balance < -1 {
            return self.rotate_left(root);
        }
        root
    }

    fn rotate_left(&mut self, root: usize) -> usize {
        let pivot = self.nodes[root].right.unwrap();
        let middle = self.nodes[pivot].left;
        let parent = self.nodes[root].parent;
        self.nodes[root].right = middle;
        self.set_parent(middle, Some(root));
        self.nodes[pivot].left = Some(root);
        self.nodes[root].parent = Some(pivot);
        self.nodes[pivot].parent = parent;
        self.refresh(root);
        self.refresh(pivot);
        pivot
    }

    fn rotate_right(&mut self, root: usize) -> usize {
        let pivot = self.nodes[root].left.unwrap();
        let middle = self.nodes[pivot].right;
        let parent = self.nodes[root].parent;
        self.nodes[root].left = middle;
        self.set_parent(middle, Some(root));
        self.nodes[pivot].right = Some(root);
        self.nodes[root].parent = Some(pivot);
        self.nodes[pivot].parent = parent;
        self.refresh(root);
        self.refresh(pivot);
        pivot
    }

    fn refresh(&mut self, node: usize) {
        self.nodes[node].height = 1 + self
            .height(self.nodes[node].left)
            .max(self.height(self.nodes[node].right));
        self.nodes[node].size =
            1 + self.size(self.nodes[node].left) + self.size(self.nodes[node].right);
    }

    fn set_parent(&mut self, node: Option<usize>, parent: Option<usize>) {
        if let Some(node) = node {
            self.nodes[node].parent = parent;
        }
    }

    fn size(&self, node: Option<usize>) -> usize {
        node.map(|node| self.nodes[node].size).unwrap_or(0)
    }

    fn height(&self, node: Option<usize>) -> u16 {
        node.map(|node| self.nodes[node].height).unwrap_or(0)
    }
}

impl Node {
    const fn leaf() -> Self {
        Self {
            left: None,
            right: None,
            parent: None,
            height: 1,
            size: 1,
            active: true,
        }
    }
}
