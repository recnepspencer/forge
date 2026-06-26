#[cfg(test)]
use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::row::WorthGraphReadAccessOwner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessCappedResidueRow {
    source_path: String,
    owner: WorthGraphReadAccessOwner,
    current_count: usize,
    must_not_exceed_count: usize,
    blocker: String,
    removal_trigger: String,
}

impl WorthGraphReadAccessCappedResidueRow {
    #[cfg(test)]
    pub(crate) fn builder() -> WorthGraphReadAccessCappedResidueBuilder {
        WorthGraphReadAccessCappedResidueBuilder::default()
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn owner(&self) -> WorthGraphReadAccessOwner {
        self.owner
    }

    pub const fn current_count(&self) -> usize {
        self.current_count
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessCappedResidueBuilder {
    source_path: Option<String>,
    owner: Option<WorthGraphReadAccessOwner>,
    current_count: Option<usize>,
    must_not_exceed_count: Option<usize>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
}

#[cfg(test)]
impl WorthGraphReadAccessCappedResidueBuilder {
    pub fn source_path(mut self, source_path: impl Into<String>) -> Self {
        self.source_path = Some(source_path.into());
        self
    }

    pub const fn owner(mut self, owner: WorthGraphReadAccessOwner) -> Self {
        self.owner = Some(owner);
        self
    }

    pub const fn current_count(mut self, current_count: usize) -> Self {
        self.current_count = Some(current_count);
        self
    }

    pub const fn must_not_exceed_count(mut self, must_not_exceed_count: usize) -> Self {
        self.must_not_exceed_count = Some(must_not_exceed_count);
        self
    }

    pub fn blocker(mut self, blocker: impl Into<String>) -> Self {
        self.blocker = Some(blocker.into());
        self
    }

    pub fn removal_trigger(mut self, removal_trigger: impl Into<String>) -> Self {
        self.removal_trigger = Some(removal_trigger.into());
        self
    }

    pub fn build(
        self,
    ) -> Result<WorthGraphReadAccessCappedResidueRow, WorthGraphReadAccessInventoryError> {
        let source_path = require_non_empty_string(
            self.source_path,
            WorthGraphReadAccessInventoryErrorKind::MissingSourcePath,
        )?;
        let owner = self
            .owner
            .ok_or_else(|| error(WorthGraphReadAccessInventoryErrorKind::MissingOwner))?;
        let current_count = self.current_count.ok_or_else(|| {
            error(WorthGraphReadAccessInventoryErrorKind::MissingResidueCurrentCount)
        })?;
        let must_not_exceed_count = self
            .must_not_exceed_count
            .ok_or_else(|| error(WorthGraphReadAccessInventoryErrorKind::MissingResidueCap))?;
        let blocker = require_non_empty_string(
            self.blocker,
            WorthGraphReadAccessInventoryErrorKind::MissingResidueBlocker,
        )?;
        let removal_trigger = require_non_empty_string(
            self.removal_trigger,
            WorthGraphReadAccessInventoryErrorKind::MissingResidueRemovalTrigger,
        )?;
        if current_count > must_not_exceed_count {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::ResidueCountExceedsCap,
            ));
        }

        Ok(WorthGraphReadAccessCappedResidueRow {
            source_path,
            owner,
            current_count,
            must_not_exceed_count,
            blocker,
            removal_trigger,
        })
    }
}

#[cfg(test)]
fn require_non_empty_string(
    value: Option<String>,
    error_kind: WorthGraphReadAccessInventoryErrorKind,
) -> Result<String, WorthGraphReadAccessInventoryError> {
    let value = value.ok_or_else(|| error(error_kind))?;
    if value.is_empty() {
        return Err(error(error_kind));
    }
    Ok(value)
}

#[cfg(test)]
const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
