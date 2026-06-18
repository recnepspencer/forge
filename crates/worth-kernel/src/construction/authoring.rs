use forge_query::facade::consumer_kit::{
    project_workspace_support_snapshot, ForgeQuerySupportPinningError,
};
use forge_query::facade::ForgeQueryWorkspace;

use super::query_support_pins::primitive_construction_query_support_pins;

#[derive(Debug)]
pub enum PrimitiveConstructionQueryEntryError {
    SupportPinning(ForgeQuerySupportPinningError),
}

impl From<ForgeQuerySupportPinningError> for PrimitiveConstructionQueryEntryError {
    fn from(value: ForgeQuerySupportPinningError) -> Self {
        Self::SupportPinning(value)
    }
}

impl std::fmt::Display for PrimitiveConstructionQueryEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SupportPinning(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryEntryError {}

pub(crate) fn require_primitive_construction_query_authority(
    workspace: &ForgeQueryWorkspace,
) -> Result<(), PrimitiveConstructionQueryEntryError> {
    let snapshot = project_workspace_support_snapshot(workspace);
    let report = primitive_construction_query_support_pins()?.evaluate_snapshot(&snapshot)?;
    report.assert_satisfied()?;
    Ok(())
}
