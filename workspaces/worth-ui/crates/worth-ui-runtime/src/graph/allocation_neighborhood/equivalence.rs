#![cfg(test)]

use crate::evidence::UiAllocationNeighborhood;

pub(crate) fn equivalent_identity(
    left: &UiAllocationNeighborhood,
    right: &UiAllocationNeighborhood,
) -> bool {
    left.identity() == right.identity()
}
