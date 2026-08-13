use std::hash::Hash;

use super::{BoundedOrderIndex, Link};

pub(crate) struct Ordered<'a, Identity> {
    index: &'a BoundedOrderIndex<Identity>,
    stack: Vec<usize>,
    remaining: usize,
}

impl<'a, Identity> Ordered<'a, Identity>
where
    Identity: Copy + Eq + Hash,
{
    pub(super) fn new(index: &'a BoundedOrderIndex<Identity>) -> Self {
        let mut ordered = Self {
            index,
            stack: Vec::new(),
            remaining: index.len(),
        };
        ordered.push_left(index.root);
        ordered
    }

    fn push_left(&mut self, mut link: Link) {
        while let Some(node) = link {
            self.stack.push(node);
            link = self.index.nodes[node].left;
        }
    }
}

impl<Identity> Iterator for Ordered<'_, Identity>
where
    Identity: Copy + Eq + Hash,
{
    type Item = Identity;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        self.index.cost.node_touch();
        self.push_left(self.index.nodes[node].right);
        self.remaining -= 1;
        self.index.nodes[node].identity
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<Identity> ExactSizeIterator for Ordered<'_, Identity> where Identity: Copy + Eq + Hash {}
