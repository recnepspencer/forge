//! Deterministic ordering rules for topology entities.
//!
//! DOMAIN: Ordering comparisons for stable, reproducible sorts.
//! INVARIANTS: Ordering is total and deterministic (D1).
//! DEPENDENCIES: forge-math (spatial Tie-breaking).
//!
//! Sort order: ID first → lineage hash → spatial hash.
//! This ensures that enumeration order is identical across runs
//! given the same input.

/// A composite ordering key for deterministic entity sorting (D1).
///
/// Compared lexicographically: `id` first, then `lineage_hash`,
/// then `spatial_hash`. This produces a total, deterministic order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderingKey {
    /// Primary: entity ID (unique within an epoch).
    id: u64,
    /// Secondary: lineage ancestry hash.
    lineage_hash: u128,
    /// Tertiary: quantized spatial hash for geometric tie-breaking.
    spatial_hash: u64,
}

impl OrderingKey {
    /// Construct a new ordering key.
    pub fn new(id: u64, lineage_hash: u128, spatial_hash: u64) -> Self {
        Self {
            id,
            lineage_hash,
            spatial_hash,
        }
    }

    /// The entity ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// The lineage ancestry hash.
    pub fn lineage_hash(&self) -> u128 {
        self.lineage_hash
    }

    /// The spatial hash.
    pub fn spatial_hash(&self) -> u64 {
        self.spatial_hash
    }
}

impl Ord for OrderingKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id
            .cmp(&other.id)
            .then(self.lineage_hash.cmp(&other.lineage_hash))
            .then(self.spatial_hash.cmp(&other.spatial_hash))
    }
}

impl PartialOrd for OrderingKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Trait for entities that can produce a deterministic ordering key.
pub trait DeterministicOrder {
    /// Produce the ordering key for this entity.
    fn ordering_key(&self) -> OrderingKey;
}

/// Compute a spatial hash from a 3D position.
///
/// `grid_scale` controls quantization resolution — pass `ToleranceConfig::get_spatial_hash_grid_scale()`
/// from the kernel layer. Never hardcode this value.
pub fn compute_entity_spatial_hash(position: &[f64; 3], grid_scale: f64) -> u64 {
    forge_math::linalg::compute_spatial_hash(position, grid_scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_by_id_first() {
        let a = OrderingKey::new(1, 0, 0);
        let b = OrderingKey::new(2, 0, 0);
        assert!(a < b);
    }

    #[test]
    fn ordering_falls_through_to_lineage() {
        let a = OrderingKey::new(1, 100, 0);
        let b = OrderingKey::new(1, 200, 0);
        assert!(a < b);
    }

    #[test]
    fn ordering_falls_through_to_spatial() {
        let a = OrderingKey::new(1, 100, 10);
        let b = OrderingKey::new(1, 100, 20);
        assert!(a < b);
    }

    #[test]
    fn equal_keys_are_equal() {
        let a = OrderingKey::new(1, 100, 10);
        let b = OrderingKey::new(1, 100, 10);
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
    }

    #[test]
    fn entity_spatial_hash_is_deterministic() {
        let pos = [1.5, 2.5, 3.5];
        assert_eq!(
            compute_entity_spatial_hash(&pos),
            compute_entity_spatial_hash(&pos)
        );
    }

    #[test]
    fn sort_is_stable_for_mixed_keys() {
        let mut keys = vec![
            OrderingKey::new(3, 0, 0),
            OrderingKey::new(1, 0, 0),
            OrderingKey::new(2, 0, 0),
            OrderingKey::new(1, 50, 0),
            OrderingKey::new(1, 50, 10),
        ];
        keys.sort();
        assert_eq!(keys[0], OrderingKey::new(1, 0, 0));
        assert_eq!(keys[1], OrderingKey::new(1, 50, 0));
        assert_eq!(keys[2], OrderingKey::new(1, 50, 10));
        assert_eq!(keys[3], OrderingKey::new(2, 0, 0));
        assert_eq!(keys[4], OrderingKey::new(3, 0, 0));
    }
}
