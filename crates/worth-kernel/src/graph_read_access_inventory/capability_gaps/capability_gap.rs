use super::super::inventory_lane::WorthGraphReadAccessInventoryRow;
use super::super::phase_six_closeout::{
    WorthGraphReadAccessInventoryRowContext, WorthGraphReadAccessInventoryRowIdentity,
    WorthGraphReadAccessPhaseSixError, WorthGraphReadAccessPhaseSixErrorKind,
};
use super::expected_denial::WorthGraphReadExpectedDenial;
use super::missing_query_capability::WorthGraphReadMissingQueryCapability;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadQueryAccessCapabilityGap {
    inventory_row_context: WorthGraphReadAccessInventoryRowContext,
    missing_capability: WorthGraphReadMissingQueryCapability,
    expected_denial: WorthGraphReadExpectedDenial,
    must_not_exceed_count: usize,
    blocker: String,
    removal_trigger: String,
}

impl WorthGraphReadQueryAccessCapabilityGap {
    pub fn for_inventory_row(
        row: &WorthGraphReadAccessInventoryRow,
    ) -> WorthGraphReadQueryAccessCapabilityGapBuilder {
        WorthGraphReadQueryAccessCapabilityGapBuilder {
            inventory_row_context: Some(WorthGraphReadAccessInventoryRowContext::from_row(row)),
            ..WorthGraphReadQueryAccessCapabilityGapBuilder::default()
        }
    }

    pub fn inventory_row_identity(&self) -> &WorthGraphReadAccessInventoryRowIdentity {
        self.inventory_row_context.identity()
    }

    pub fn inventory_row_context(&self) -> &WorthGraphReadAccessInventoryRowContext {
        &self.inventory_row_context
    }

    pub const fn missing_capability(&self) -> WorthGraphReadMissingQueryCapability {
        self.missing_capability
    }

    pub fn expected_denial(&self) -> &WorthGraphReadExpectedDenial {
        &self.expected_denial
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthGraphReadQueryAccessCapabilityGapBuilder {
    inventory_row_context: Option<WorthGraphReadAccessInventoryRowContext>,
    missing_capability: Option<WorthGraphReadMissingQueryCapability>,
    expected_denial: Option<WorthGraphReadExpectedDenial>,
    must_not_exceed_count: Option<usize>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
}

impl WorthGraphReadQueryAccessCapabilityGapBuilder {
    pub const fn missing_capability(
        mut self,
        missing_capability: WorthGraphReadMissingQueryCapability,
    ) -> Self {
        self.missing_capability = Some(missing_capability);
        self
    }

    pub const fn expected_denial(mut self, expected_denial: WorthGraphReadExpectedDenial) -> Self {
        self.expected_denial = Some(expected_denial);
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
    ) -> Result<WorthGraphReadQueryAccessCapabilityGap, WorthGraphReadAccessPhaseSixError> {
        Ok(WorthGraphReadQueryAccessCapabilityGap {
            inventory_row_context: self.inventory_row_context.ok_or_else(|| {
                error(WorthGraphReadAccessPhaseSixErrorKind::MissingInventoryRowIdentity)
            })?,
            missing_capability: self.missing_capability.ok_or_else(|| {
                error(WorthGraphReadAccessPhaseSixErrorKind::MissingQueryCapability)
            })?,
            expected_denial: self.expected_denial.ok_or_else(|| {
                error(WorthGraphReadAccessPhaseSixErrorKind::MissingExpectedDenialKind)
            })?,
            must_not_exceed_count: self.must_not_exceed_count.ok_or_else(|| {
                error(WorthGraphReadAccessPhaseSixErrorKind::MissingCapabilityGapCap)
            })?,
            blocker: require_non_empty(
                self.blocker,
                WorthGraphReadAccessPhaseSixErrorKind::MissingCapabilityGapBlocker,
            )?,
            removal_trigger: require_non_empty(
                self.removal_trigger,
                WorthGraphReadAccessPhaseSixErrorKind::MissingCapabilityGapRemovalTrigger,
            )?,
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
