use serde_json::Value;

use crate::aspects::AspectContract;
use crate::locators::BoundarySourceLocator;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonCompatibilityAspectInput {
    contract: AspectContract,
    source: BoundarySourceLocator,
    value: Value,
}

impl JsonCompatibilityAspectInput {
    pub fn new(contract: AspectContract, source: BoundarySourceLocator, value: Value) -> Self {
        Self {
            contract,
            source,
            value,
        }
    }

    pub fn contract(&self) -> &AspectContract {
        &self.contract
    }

    pub fn source(&self) -> &BoundarySourceLocator {
        &self.source
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
