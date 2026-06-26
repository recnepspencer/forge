use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessResidueGrowthPolicy {
    current_count: usize,
    must_not_exceed_count: usize,
    previous_must_not_exceed_count: usize,
}

impl WorthGraphReadAccessResidueGrowthPolicy {
    pub fn admit(
        current_count: usize,
        must_not_exceed_count: usize,
        previous_must_not_exceed_count: usize,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        if current_count > must_not_exceed_count {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::ResidueCountExceedsCap,
            ));
        }
        if must_not_exceed_count > previous_must_not_exceed_count {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::ResidueGrowthRequiresCapUpdate,
            ));
        }
        Ok(Self {
            current_count,
            must_not_exceed_count,
            previous_must_not_exceed_count,
        })
    }

    pub const fn current_count(&self) -> usize {
        self.current_count
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub const fn previous_must_not_exceed_count(&self) -> usize {
        self.previous_must_not_exceed_count
    }
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
