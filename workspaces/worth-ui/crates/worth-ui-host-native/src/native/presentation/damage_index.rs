use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use worth_ui_host_contract::{UiMountedCanonicalBox, UiMountedCoordinateSpace};

#[path = "damage_index/aabb.rs"]
mod aabb;
#[path = "damage_index/arena.rs"]
mod arena;
#[path = "damage_index/hierarchy.rs"]
mod hierarchy;

use arena::{NodeId, MAX_LEAVES};
use hierarchy::DamageHierarchy;

pub(super) struct UiNativeDamageIndex<Identity> {
    hierarchy: DamageHierarchy<Identity>,
    leaves: HashMap<Identity, DamageRecord>,
    high_water: usize,
}

#[derive(Clone, Copy)]
struct DamageRecord {
    leaf: NodeId,
    space: UiMountedCoordinateSpace,
}

pub(super) struct UiNativeDamageQuery<Identity> {
    pub(super) identities: HashSet<Identity>,
    pub(super) branch_aabb_probes: usize,
    pub(super) leaf_command_bounds_probes: usize,
    pub(super) stored_records: usize,
    pub(super) high_water_records: usize,
    #[cfg(test)]
    pub(super) hierarchy_height: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiNativeDamageIndexDenial {
    DuplicateIdentity,
    MissingIdentity,
    CapacityExceeded,
}

impl<Identity> UiNativeDamageIndex<Identity>
where
    Identity: Copy + Eq + Hash,
{
    pub(super) fn new() -> Self {
        Self {
            hierarchy: DamageHierarchy::new(),
            leaves: HashMap::with_capacity(MAX_LEAVES),
            high_water: 0,
        }
    }

    pub(super) fn insert(
        &mut self,
        identity: Identity,
        bounds: UiMountedCanonicalBox,
    ) -> Result<(), UiNativeDamageIndexDenial> {
        if self.leaves.contains_key(&identity) {
            return Err(UiNativeDamageIndexDenial::DuplicateIdentity);
        }
        if self.leaves.len() == MAX_LEAVES {
            return Err(UiNativeDamageIndexDenial::CapacityExceeded);
        }
        let space = bounds.coordinate_space();
        let leaf = self.hierarchy.insert(identity, bounds);
        self.leaves.insert(identity, DamageRecord { leaf, space });
        self.high_water = self.high_water.max(self.leaves.len());
        Ok(())
    }

    pub(super) fn validate_bounds(
        &self,
        _bounds: UiMountedCanonicalBox,
    ) -> Result<(), UiNativeDamageIndexDenial> {
        Ok(())
    }

    pub(super) fn remove(&mut self, identity: Identity) -> Result<(), UiNativeDamageIndexDenial> {
        let record = self
            .leaves
            .remove(&identity)
            .ok_or(UiNativeDamageIndexDenial::MissingIdentity)?;
        self.hierarchy.remove(record.leaf, record.space);
        Ok(())
    }

    pub(super) fn replace(
        &mut self,
        identity: Identity,
        bounds: UiMountedCanonicalBox,
    ) -> Result<(), UiNativeDamageIndexDenial> {
        let record = self
            .leaves
            .get_mut(&identity)
            .ok_or(UiNativeDamageIndexDenial::MissingIdentity)?;
        self.hierarchy.replace(record.leaf, record.space, bounds);
        record.space = bounds.coordinate_space();
        Ok(())
    }

    pub(super) fn intersecting(
        &self,
        damage: UiMountedCanonicalBox,
    ) -> Result<UiNativeDamageQuery<Identity>, UiNativeDamageIndexDenial> {
        let query = self.hierarchy.query(damage);
        Ok(UiNativeDamageQuery {
            identities: query.identities,
            branch_aabb_probes: query.branch_aabb_probes,
            leaf_command_bounds_probes: query.leaf_command_bounds_probes,
            stored_records: self.leaves.len(),
            high_water_records: self.high_water,
            #[cfg(test)]
            hierarchy_height: query.height,
        })
    }
}

#[cfg(test)]
#[path = "damage_index/tests.rs"]
mod tests;
