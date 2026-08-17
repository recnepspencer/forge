use std::collections::BTreeSet;

use worth_ui_host_contract::{UiMountedInstanceIdentity, UiMountedSemanticTextTable};

use super::{UiMountedSemanticMechanicRows, UiMountedSemanticMechanicSource};
use crate::mounting::projection::{
    frame_storage::UiMountedSemanticProjection,
    semantic_text::{UiMountedSemanticTextSeed, UiMountedSemanticTextSeedContent},
};
use crate::mounting::UiMountedProjectionDenial;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct TextCapacity {
    rows: usize,
    bytes: usize,
}

impl UiMountedSemanticMechanicSource {
    pub(in crate::mounting::projection::frame_storage) const fn byte_len(&self) -> usize {
        self.byte_count
    }

    pub(super) fn replace_capacity(
        &mut self,
        predecessor: &UiMountedSemanticMechanicRows,
        successor: &UiMountedSemanticMechanicRows,
    ) -> Result<(), UiMountedProjectionDenial> {
        self.byte_count = self
            .byte_count
            .checked_sub(predecessor.byte_len())
            .and_then(|count| count.checked_add(successor.byte_len()))
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
        Ok(())
    }

    pub(super) fn remove_capacity(&mut self, predecessor: &UiMountedSemanticMechanicRows) {
        self.byte_count = self.byte_count.saturating_sub(predecessor.byte_len());
    }

    pub(super) fn update_capacity(
        &mut self,
        instance: UiMountedInstanceIdentity,
        successor: &UiMountedSemanticMechanicRows,
    ) -> Result<(), UiMountedProjectionDenial> {
        let predecessor = self
            .by_instance
            .get(&instance)
            .ok_or(UiMountedProjectionDenial::MissingSemanticCollectionPredecessor)?;
        let predecessor_bytes = predecessor.byte_len();
        self.byte_count = self
            .byte_count
            .checked_sub(predecessor_bytes)
            .and_then(|count| count.checked_add(successor.byte_len()))
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
        Ok(())
    }

    pub(in crate::mounting::projection::frame_storage) fn preflight(
        &self,
        changed: &[UiMountedInstanceIdentity],
        semantic: &UiMountedSemanticProjection,
    ) -> Result<(), UiMountedProjectionDenial> {
        let mut successor = TextCapacity {
            rows: self.row_count,
            bytes: self.byte_count,
        };
        let mut visited = BTreeSet::new();
        for instance in changed {
            if !visited.insert(*instance) {
                continue;
            }
            if let Some(predecessor) = self.by_instance.get(instance) {
                successor = subtract(successor, predecessor.len(), predecessor.byte_len())?;
            }
        }
        for instance in visited {
            let Some(seed) = semantic
                .node(instance)
                .and_then(|node| node.semantic_text.as_ref())
            else {
                continue;
            };
            let incoming = capacity_for_seed(seed)?;
            successor = add(successor, incoming)?;
            validate(successor)?;
        }
        validate(successor)
    }
}

impl UiMountedSemanticMechanicRows {
    fn byte_len(&self) -> usize {
        self.byte_count
    }
}

fn capacity_for_seed(
    seed: &UiMountedSemanticTextSeed,
) -> Result<TextCapacity, UiMountedProjectionDenial> {
    let mut result = TextCapacity::default();
    add_text(&mut result, seed.posture())?;
    match seed.content() {
        UiMountedSemanticTextSeedContent::Scalar(Some(value)) => add_text(&mut result, value)?,
        UiMountedSemanticTextSeedContent::Scalar(None) => {}
        UiMountedSemanticTextSeedContent::Collection(collection) => {
            result.rows = result
                .rows
                .checked_add(collection.selected_value_count())
                .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
            result.bytes = result
                .bytes
                .checked_add(collection.selected_byte_count())
                .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
        }
    }
    Ok(result)
}

fn add_text(result: &mut TextCapacity, value: &str) -> Result<(), UiMountedProjectionDenial> {
    if value.len() > worth_ui_host_contract::UiMountedSemanticTextMechanic::MAX_CONTENT_BYTES {
        return Err(UiMountedProjectionDenial::SemanticTextCapacityExceeded);
    }
    result.rows = result
        .rows
        .checked_add(1)
        .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
    result.bytes = result
        .bytes
        .checked_add(value.len())
        .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
    Ok(())
}

fn add(left: TextCapacity, right: TextCapacity) -> Result<TextCapacity, UiMountedProjectionDenial> {
    let result = TextCapacity {
        rows: left
            .rows
            .checked_add(right.rows)
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?,
        bytes: left
            .bytes
            .checked_add(right.bytes)
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?,
    };
    validate(result)?;
    Ok(result)
}

fn subtract(
    current: TextCapacity,
    rows: usize,
    bytes: usize,
) -> Result<TextCapacity, UiMountedProjectionDenial> {
    Ok(TextCapacity {
        rows: current
            .rows
            .checked_sub(rows)
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?,
        bytes: current
            .bytes
            .checked_sub(bytes)
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?,
    })
}

fn validate(capacity: TextCapacity) -> Result<(), UiMountedProjectionDenial> {
    if capacity.rows > UiMountedSemanticTextTable::MAX_ROWS
        || capacity.bytes > UiMountedSemanticTextTable::MAX_BYTES
    {
        Err(UiMountedProjectionDenial::SemanticTextCapacityExceeded)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_capacity_is_checked_before_any_row_is_materialized() {
        let maximum = TextCapacity {
            rows: UiMountedSemanticTextTable::MAX_ROWS,
            bytes: UiMountedSemanticTextTable::MAX_BYTES,
        };
        assert!(validate(maximum).is_ok());
        assert_eq!(
            add(maximum, TextCapacity { rows: 1, bytes: 0 }),
            Err(UiMountedProjectionDenial::SemanticTextCapacityExceeded)
        );
        assert_eq!(
            add(maximum, TextCapacity { rows: 0, bytes: 1 }),
            Err(UiMountedProjectionDenial::SemanticTextCapacityExceeded)
        );
        assert_eq!(
            subtract(maximum, 1, 1)
                .and_then(|capacity| { add(capacity, TextCapacity { rows: 1, bytes: 1 }) }),
            Ok(maximum)
        );
    }
}
