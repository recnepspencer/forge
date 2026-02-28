use crate::primitives::aabb::Aabb;

/// A node in the BVH tree.
#[derive(Debug, Clone)]
pub enum BvhNode<T> {
    /// Leaf node containing an item and its AABB.
    Leaf { item: T, aabb: Aabb },
    /// Internal node containing the union AABB and two children.
    Internal {
        aabb: Aabb,
        left: Box<BvhNode<T>>,
        right: Box<BvhNode<T>>,
    },
}
