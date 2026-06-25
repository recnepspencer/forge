use forge_query::facade::runtime::ForgeQueryGraphObligationKind;

use super::WorthUiQueryGraphExecutionRow;
use crate::runtime::query_graph::WorthUiQueryGraphObligationSemantic;

impl WorthUiQueryGraphExecutionRow {
    pub fn semantic(&self) -> WorthUiQueryGraphObligationSemantic {
        self.semantic
    }

    pub fn canonical_kind(&self) -> ForgeQueryGraphObligationKind {
        self.canonical_kind
    }

    pub fn support_lane(&self) -> &str {
        &self.support_lane
    }

    pub fn support_status(&self) -> &str {
        &self.support_status
    }

    pub fn execution_status(&self) -> &str {
        &self.execution_status
    }

    pub fn rule_identity_digest(&self) -> &str {
        &self.rule_identity_digest
    }

    pub fn registration_digest(&self) -> &str {
        &self.registration_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
