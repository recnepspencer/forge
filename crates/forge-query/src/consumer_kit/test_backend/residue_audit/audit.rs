use std::path::PathBuf;

use crate::consumer_kit::boundary_audit::{
    ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditErrorKind,
};

use super::evidence::{
    derive_test_backend_residue_finding_identity, derive_test_backend_residue_report_identity,
};
use super::report::ForgeQueryTestBackendResidueReport;
use super::scanner::{normalize_path, scan_root};

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
        if self.consumer_name.trim().is_empty() {
            return Err(ForgeQueryBoundaryAuditError::new(
                ForgeQueryBoundaryAuditErrorKind::EmptyCrateName,
                "test backend residue audit consumer name must not be empty",
            ));
        }

        let mut audited_roots = Vec::new();
        let mut findings = Vec::new();
        let mut scanned_file_count = 0usize;
        for root in self.required_roots {
            if !root.exists() {
                return Err(ForgeQueryBoundaryAuditError::new(
                    ForgeQueryBoundaryAuditErrorKind::MissingRequiredRoot,
                    format!(
                        "required test backend residue root `{}` does not exist",
                        root.display()
                    ),
                ));
            }
            audited_roots.push(normalize_path(&root));
            scan_root(&root, &mut findings, &mut scanned_file_count)?;
        }
        findings.sort_by(|left, right| {
            left.source_path()
                .cmp(right.source_path())
                .then(left.matched_pattern().cmp(right.matched_pattern()))
        });
        let finding_identities = findings
            .iter()
            .map(derive_test_backend_residue_finding_identity)
            .collect::<Vec<_>>();
        let report_identity = derive_test_backend_residue_report_identity(
            &self.consumer_name,
            &audited_roots,
            scanned_file_count,
            &finding_identities,
        );

        Ok(ForgeQueryTestBackendResidueReport::sealed(
            self.consumer_name,
            audited_roots,
            findings,
            finding_identities,
            report_identity,
            scanned_file_count,
        ))
    }
}
