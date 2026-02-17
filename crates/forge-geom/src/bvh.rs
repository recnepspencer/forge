//! Bounding Volume Hierarchy (BVH) for spatial indexing.
//!
//! Uses a median-split construction strategy to build a balanced tree.
//! Supports querying for overlapping pairs of objects between two trees.

use crate::aabb::Aabb;

/// A node in the BVH tree.
#[derive(Debug, Clone)]
pub enum BvhNode {
    /// Leaf node containing a face index and its AABB.
    Leaf {
        face_index: usize,
        aabb: Aabb,
    },
    /// Internal node containing the union AABB and two children.
    Internal {
        aabb: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl BvhNode {
    /// Get the AABB of this node.
    pub fn aabb(&self) -> &Aabb {
        match self {
            BvhNode::Leaf { aabb, .. } => aabb,
            BvhNode::Internal { aabb, .. } => aabb,
        }
    }

    /// Build a BVH from a list of face AABBs.
    pub fn build(mut items: Vec<(usize, Aabb)>) -> Option<Box<Self>> {
        if items.is_empty() {
            return None;
        }
        Some(Self::build_recursive(&mut items))
    }

    fn build_recursive(items: &mut [(usize, Aabb)]) -> Box<Self> {
        if items.len() == 1 {
            let (face_index, aabb) = items[0];
            return Box::new(BvhNode::Leaf { face_index, aabb });
        }

        // Calculate bounding box of all items
        let mut union_aabb = items[0].1;
        for (_, aabb) in items.iter().skip(1) {
            union_aabb = union_aabb.union(aabb);
        }

        // Split along the longest axis
        let axis = union_aabb.largest_axis();
        let mid = items.len() / 2;

        items.select_nth_unstable_by(mid, |a, b| {
            a.1.min[axis]
                .partial_cmp(&b.1.min[axis])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let (left_items, right_items) = items.split_at_mut(mid);
        let left = Self::build_recursive(left_items);
        let right = Self::build_recursive(right_items);

        Box::new(BvhNode::Internal {
            aabb: union_aabb,
            left,
            right,
        })
    }
}

/// Query overlapping pairs between two BVH trees.
///
/// Returns a vector of (index_a, index_b) pairs where the AABBs overlap.
pub fn query_overlapping_pairs(
    root_a: &BvhNode,
    root_b: &BvhNode,
) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    // Use a stack to avoid recursion depth issues
    let mut stack = vec![(root_a, root_b)];

    while let Some((node_a, node_b)) = stack.pop() {
        if !node_a.aabb().intersects(node_b.aabb()) {
            continue;
        }

        match (node_a, node_b) {
            (BvhNode::Leaf { face_index: idx_a, .. }, BvhNode::Leaf { face_index: idx_b, .. }) => {
                results.push((*idx_a, *idx_b));
            }
            (BvhNode::Leaf { .. }, BvhNode::Internal { left, right, .. }) => {
                stack.push((node_a, left));
                stack.push((node_a, right));
            }
            (BvhNode::Internal { left, right, .. }, BvhNode::Leaf { .. }) => {
                stack.push((left, node_b));
                stack.push((right, node_b));
            }
            (BvhNode::Internal { left: la, right: ra, .. }, BvhNode::Internal { left: lb, right: rb, .. }) => {
                // Heuristic: descend into the larger node first? or just cross product?
                // Standard BVH traversal:
                stack.push((la, lb));
                stack.push((la, rb));
                stack.push((ra, lb));
                stack.push((ra, rb));
            }
        }
    }
    results
}
