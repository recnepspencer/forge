use std::path::PathBuf;

use crate::consumer_kit::boundary_audit::ForgeQueryBoundaryAuditError;
use crate::consumer_kit::consumer_residue::{
    forge_query_test_backend_residue_classes, query_consumer_residue_audit,
};

use super::report::ForgeQueryTestBackendResidueReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTestBackendResidueAudit {
    consumer_name: String,
    required_roots: Vec<PathBuf>,
}

pub fn query_test_backend_residue_audit(
    consumer_name: impl Into<String>,
) -> ForgeQueryTestBackendResidueAudit {
    ForgeQueryTestBackendResidueAudit {
        consumer_name: consumer_name.into(),
        required_roots: Vec::new(),
    }
}

impl ForgeQueryTestBackendResidueAudit {
    pub fn required_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.required_roots.push(root.into());
        self
    }

    pub fn evaluate(
        self,
    ) -> Result<ForgeQueryTestBackendResidueReport, ForgeQueryBoundaryAuditError> {
        let audit = self.required_roots.into_iter().fold(
            query_consumer_residue_audit(self.consumer_name)
                .with_class_filter(forge_query_test_backend_residue_classes()),
            |audit, root| audit.required_root(root),
        );
        audit
            .evaluate()
            .map(ForgeQueryTestBackendResidueReport::from_consumer_residue_report)
    }
}
