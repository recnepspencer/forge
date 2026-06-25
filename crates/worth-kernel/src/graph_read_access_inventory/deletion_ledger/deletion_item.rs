use super::super::inventory_lane::WorthGraphReadAccessInventoryRow;
use super::super::phase_six_closeout::{
    WorthGraphReadAccessInventoryRowContext, WorthGraphReadAccessInventoryRowIdentity,
    WorthGraphReadAccessPhaseSixError, WorthGraphReadAccessPhaseSixErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeletionLedgerItem {
    inventory_row_context: WorthGraphReadAccessInventoryRowContext,
    deletion_trigger: String,
    blocker: Option<String>,
}

impl WorthGraphReadDeletionLedgerItem {
    pub fn for_inventory_row(
        row: &WorthGraphReadAccessInventoryRow,
    ) -> WorthGraphReadDeletionLedgerItemBuilder {
        WorthGraphReadDeletionLedgerItemBuilder {
            inventory_row_context: Some(WorthGraphReadAccessInventoryRowContext::from_row(row)),
            ..WorthGraphReadDeletionLedgerItemBuilder::default()
        }
    }

    pub fn inventory_row_identity(&self) -> &WorthGraphReadAccessInventoryRowIdentity {
        self.inventory_row_context.identity()
    }

    pub fn inventory_row_context(&self) -> &WorthGraphReadAccessInventoryRowContext {
        &self.inventory_row_context
    }

    pub fn deletion_trigger(&self) -> &str {
        &self.deletion_trigger
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthGraphReadDeletionLedgerItemBuilder {
    inventory_row_context: Option<WorthGraphReadAccessInventoryRowContext>,
    deletion_trigger: Option<String>,
    blocker: Option<String>,
}

impl WorthGraphReadDeletionLedgerItemBuilder {
    pub fn deletion_trigger(mut self, deletion_trigger: impl Into<String>) -> Self {
        self.deletion_trigger = Some(deletion_trigger.into());
        self
    }

    pub fn blocker(mut self, blocker: impl Into<String>) -> Self {
        self.blocker = Some(blocker.into());
        self
    }

    pub fn build(
        self,
    ) -> Result<WorthGraphReadDeletionLedgerItem, WorthGraphReadAccessPhaseSixError> {
        Ok(WorthGraphReadDeletionLedgerItem {
            inventory_row_context: self.inventory_row_context.ok_or_else(|| {
                error(WorthGraphReadAccessPhaseSixErrorKind::MissingInventoryRowIdentity)
            })?,
            deletion_trigger: require_non_empty(
                self.deletion_trigger,
                WorthGraphReadAccessPhaseSixErrorKind::MissingDeletionTrigger,
            )?,
            blocker: self.blocker,
        })
    }
}

fn require_non_empty(
    value: Option<String>,
    kind: WorthGraphReadAccessPhaseSixErrorKind,
) -> Result<String, WorthGraphReadAccessPhaseSixError> {
    let value = value.ok_or_else(|| error(kind))?;
    if value.is_empty() {
        return Err(error(kind));
    }
    Ok(value)
}

const fn error(kind: WorthGraphReadAccessPhaseSixErrorKind) -> WorthGraphReadAccessPhaseSixError {
    WorthGraphReadAccessPhaseSixError::new(kind)
}
