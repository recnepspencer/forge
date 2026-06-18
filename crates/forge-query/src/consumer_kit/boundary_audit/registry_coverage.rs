use crate::hard_prohibition_registry;
use crate::ForgeQueryProhibitedSeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ForgeQueryBoundaryAuditCoverageMechanism {
    SealedByVisibility,
    AstMethodNameResolved,
}

impl ForgeQueryBoundaryAuditCoverageMechanism {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SealedByVisibility => "sealed-by-visibility",
            Self::AstMethodNameResolved => "ast-method-name-resolved",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditCoverageRow {
    seam: ForgeQueryProhibitedSeam,
    mechanism: ForgeQueryBoundaryAuditCoverageMechanism,
    audit_required: bool,
}

impl ForgeQueryBoundaryAuditCoverageRow {
    const fn new(
        seam: ForgeQueryProhibitedSeam,
        mechanism: ForgeQueryBoundaryAuditCoverageMechanism,
        audit_required: bool,
    ) -> Self {
        Self {
            seam,
            mechanism,
            audit_required,
        }
    }

    pub fn seam(&self) -> ForgeQueryProhibitedSeam {
        self.seam
    }

    pub fn seam_key(&self) -> &'static str {
        self.seam.key()
    }

    pub fn mechanism(&self) -> ForgeQueryBoundaryAuditCoverageMechanism {
        self.mechanism
    }

    pub fn audit_required(&self) -> bool {
        self.audit_required
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditCoverage {
    rows: Vec<ForgeQueryBoundaryAuditCoverageRow>,
}

impl ForgeQueryBoundaryAuditCoverage {
    fn new(rows: Vec<ForgeQueryBoundaryAuditCoverageRow>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[ForgeQueryBoundaryAuditCoverageRow] {
        &self.rows
    }

    pub fn row(
        &self,
        seam: ForgeQueryProhibitedSeam,
    ) -> Option<&ForgeQueryBoundaryAuditCoverageRow> {
        self.rows.iter().find(|row| row.seam() == seam)
    }
}

pub fn hard_prohibition_boundary_audit_coverage() -> ForgeQueryBoundaryAuditCoverage {
    ForgeQueryBoundaryAuditCoverage::new(
        hard_prohibition_registry()
            .rows()
            .iter()
            .map(|row| {
                ForgeQueryBoundaryAuditCoverageRow::new(
                    row.seam(),
                    ForgeQueryBoundaryAuditCoverageMechanism::AstMethodNameResolved,
                    true,
                )
            })
            .collect(),
    )
}

#[cfg(test)]
pub(crate) fn assert_boundary_audit_coverage_matches_registry() {
    let registry = hard_prohibition_registry();
    let coverage = hard_prohibition_boundary_audit_coverage();
    assert_eq!(coverage.rows().len(), registry.rows().len());
    for row in registry.rows() {
        assert!(
            coverage.row(row.seam()).is_some(),
            "{} must have hard-prohibition boundary audit coverage",
            row.seam_key()
        );
    }
}
