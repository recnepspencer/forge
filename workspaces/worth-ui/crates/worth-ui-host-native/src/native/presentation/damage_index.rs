use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use worth_ui_host_contract::UiMountedCanonicalBox;

const LOGICAL_CELL_EXTENT: f32 = 64.0;
const MAX_CELLS_PER_COMMAND: usize = 65_536;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DamageCell {
    x: i64,
    y: i64,
}

pub(super) struct UiNativeDamageIndex<Identity> {
    cells: HashMap<DamageCell, HashSet<Identity>>,
    records: HashMap<Identity, DamageRecord>,
}

struct DamageRecord {
    bounds: UiMountedCanonicalBox,
    cells: Box<[DamageCell]>,
}

pub(super) struct UiNativeDamageQuery<Identity> {
    pub(super) identities: HashSet<Identity>,
    pub(super) cell_probes: usize,
    pub(super) candidate_probes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiNativeDamageIndexDenial {
    DuplicateIdentity,
    MissingIdentity,
    CellCapacityExceeded,
}

impl<Identity> UiNativeDamageIndex<Identity>
where
    Identity: Copy + Eq + Hash,
{
    pub(super) fn new() -> Self {
        Self {
            cells: HashMap::new(),
            records: HashMap::new(),
        }
    }

    pub(super) fn insert(
        &mut self,
        identity: Identity,
        bounds: UiMountedCanonicalBox,
    ) -> Result<(), UiNativeDamageIndexDenial> {
        if self.records.contains_key(&identity) {
            return Err(UiNativeDamageIndexDenial::DuplicateIdentity);
        }
        let cells = cells_for(bounds)?;
        for cell in &cells {
            self.cells.entry(*cell).or_default().insert(identity);
        }
        self.records.insert(
            identity,
            DamageRecord {
                bounds,
                cells: cells.into_boxed_slice(),
            },
        );
        Ok(())
    }

    pub(super) fn validate_bounds(
        &self,
        bounds: UiMountedCanonicalBox,
    ) -> Result<(), UiNativeDamageIndexDenial> {
        cells_for(bounds).map(|_| ())
    }

    pub(super) fn remove(&mut self, identity: Identity) -> Result<(), UiNativeDamageIndexDenial> {
        let record = self
            .records
            .remove(&identity)
            .ok_or(UiNativeDamageIndexDenial::MissingIdentity)?;
        for cell in record.cells {
            let remove_cell = self.cells.get_mut(&cell).is_some_and(|identities| {
                identities.remove(&identity);
                identities.is_empty()
            });
            if remove_cell {
                self.cells.remove(&cell);
            }
        }
        Ok(())
    }

    pub(super) fn replace(
        &mut self,
        identity: Identity,
        bounds: UiMountedCanonicalBox,
    ) -> Result<(), UiNativeDamageIndexDenial> {
        self.remove(identity)?;
        self.insert(identity, bounds)
    }

    pub(super) fn intersecting(
        &self,
        damage: UiMountedCanonicalBox,
    ) -> Result<UiNativeDamageQuery<Identity>, UiNativeDamageIndexDenial> {
        let cells = cells_for(damage)?;
        let mut candidates = HashSet::new();
        let mut candidate_probes = 0usize;
        for cell in &cells {
            if let Some(identities) = self.cells.get(cell) {
                candidate_probes = candidate_probes.saturating_add(identities.len());
                candidates.extend(identities.iter().copied());
            }
        }
        candidates.retain(|identity| {
            self.records
                .get(identity)
                .is_some_and(|record| intersects(record.bounds, damage))
        });
        Ok(UiNativeDamageQuery {
            identities: candidates,
            cell_probes: cells.len(),
            candidate_probes,
        })
    }
}

fn cells_for(bounds: UiMountedCanonicalBox) -> Result<Vec<DamageCell>, UiNativeDamageIndexDenial> {
    let x_start = cell_floor(bounds.x());
    let y_start = cell_floor(bounds.y());
    let x_end = cell_ceiling(bounds.x() + bounds.width());
    let y_end = cell_ceiling(bounds.y() + bounds.height());
    let width = usize::try_from(x_end.saturating_sub(x_start))
        .map_err(|_| UiNativeDamageIndexDenial::CellCapacityExceeded)?;
    let height = usize::try_from(y_end.saturating_sub(y_start))
        .map_err(|_| UiNativeDamageIndexDenial::CellCapacityExceeded)?;
    let count = width
        .checked_mul(height)
        .filter(|count| *count <= MAX_CELLS_PER_COMMAND)
        .ok_or(UiNativeDamageIndexDenial::CellCapacityExceeded)?;
    let mut cells = Vec::with_capacity(count);
    for y in y_start..y_end {
        for x in x_start..x_end {
            cells.push(DamageCell { x, y });
        }
    }
    Ok(cells)
}

fn cell_floor(value: f32) -> i64 {
    (value / LOGICAL_CELL_EXTENT).floor() as i64
}

fn cell_ceiling(value: f32) -> i64 {
    (value / LOGICAL_CELL_EXTENT).ceil() as i64
}

fn intersects(left: UiMountedCanonicalBox, right: UiMountedCanonicalBox) -> bool {
    left.coordinate_space() == right.coordinate_space()
        && left.x() < right.x() + right.width()
        && right.x() < left.x() + left.width()
        && left.y() < right.y() + right.height()
        && right.y() < left.y() + left.height()
}

#[cfg(test)]
mod tests {
    use super::UiNativeDamageIndex;
    use worth_ui_host_contract::{
        UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedCoordinateSpace,
    };

    fn bounds(x: f32, y: f32, width: f32, height: f32) -> UiMountedCanonicalBox {
        UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
            x,
            y,
            width,
            height,
            coordinate_space: UiMountedCoordinateSpace::HostSurface,
        })
        .unwrap()
    }

    #[test]
    fn sparse_damage_probes_only_intersecting_cells_and_exact_bounds() {
        let mut index = UiNativeDamageIndex::new();
        index.insert(1_u64, bounds(0.0, 0.0, 20.0, 20.0)).unwrap();
        index
            .insert(2_u64, bounds(2_000.0, 2_000.0, 20.0, 20.0))
            .unwrap();
        let query = index.intersecting(bounds(4.0, 4.0, 2.0, 2.0)).unwrap();
        assert_eq!(query.identities.into_iter().collect::<Vec<_>>(), vec![1]);
        assert_eq!(query.cell_probes, 1);
        assert_eq!(query.candidate_probes, 1);
    }

    #[test]
    fn replacement_and_removal_update_only_owned_cells() {
        let mut index = UiNativeDamageIndex::new();
        index.insert(1_u64, bounds(0.0, 0.0, 10.0, 10.0)).unwrap();
        index.replace(1, bounds(128.0, 128.0, 10.0, 10.0)).unwrap();
        assert!(index
            .intersecting(bounds(0.0, 0.0, 10.0, 10.0))
            .unwrap()
            .identities
            .is_empty());
        assert!(index
            .intersecting(bounds(128.0, 128.0, 10.0, 10.0))
            .unwrap()
            .identities
            .contains(&1));
        index.remove(1).unwrap();
        assert!(index
            .intersecting(bounds(128.0, 128.0, 10.0, 10.0))
            .unwrap()
            .identities
            .is_empty());
    }
}
