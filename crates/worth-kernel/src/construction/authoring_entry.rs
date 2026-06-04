use crate::construction::authoring::PrimitiveConstructionQueryEntryError;
use crate::construction::outcome::{
    prepare_primitive_construction_outcome, PrimitiveConstructionPreparedOutcome,
};
use crate::construction::result::{
    prepare_primitive_construction_result, PreparedPrimitiveConstructionResult,
};
use crate::construction::{PrimitiveConstructionFamily, PrimitiveConstructionIntent};

#[derive(Clone, Debug)]
pub struct PrimitiveConstructionAuthoringEntry {
    intent: PrimitiveConstructionIntent,
}

impl PrimitiveConstructionAuthoringEntry {
    pub(crate) fn new(intent: PrimitiveConstructionIntent) -> Self {
        Self { intent }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.intent.family()
    }

    pub fn prepare_result(
        self,
    ) -> Result<PreparedPrimitiveConstructionResult, PrimitiveConstructionQueryEntryError> {
        prepare_primitive_construction_result(self.intent)
            .map_err(PrimitiveConstructionQueryEntryError::Result)
    }

    pub fn prepare_outcome(self) -> PrimitiveConstructionPreparedOutcome {
        prepare_primitive_construction_outcome(self.intent)
    }
}
