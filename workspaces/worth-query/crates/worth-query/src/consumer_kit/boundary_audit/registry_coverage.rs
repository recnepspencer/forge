use crate::hard_prohibition_registry;
use crate::WorthQueryProhibitedSeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WorthQueryBoundaryAuditCoverageMechanism {
    SealedByVisibility,
    AstMethodNameResolved,
}

impl WorthQueryBoundaryAuditCoverageMechanism {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SealedByVisibility => "sealed-by-visibility",
            Self::AstMethodNameResolved => "ast-method-name-resolved",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundaryAuditCoverageRow {
    seam: WorthQueryProhibitedSeam,
    mechanism: WorthQueryBoundaryAuditCoverageMechanism,
    audit_required: bool,
}

impl WorthQueryBoundaryAuditCoverageRow {
    const fn new(
        seam: WorthQueryProhibitedSeam,
        mechanism: WorthQueryBoundaryAuditCoverageMechanism,
        audit_required: bool,
    ) -> Self {
        Self {
            seam,
            mechanism,
            audit_required,
        }
    }

    pub fn seam(&self) -> WorthQueryProhibitedSeam {
        self.seam
    }

    pub fn seam_key(&self) -> &'static str {
        self.seam.key()
    }

    pub fn mechanism(&self) -> WorthQueryBoundaryAuditCoverageMechanism {
        self.mechanism
    }

    pub fn audit_required(&self) -> bool {
        self.audit_required
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundaryAuditCoverage {
    rows: Vec<WorthQueryBoundaryAuditCoverageRow>,
}

impl WorthQueryBoundaryAuditCoverage {
    fn new(rows: Vec<WorthQueryBoundaryAuditCoverageRow>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[WorthQueryBoundaryAuditCoverageRow] {
        &self.rows
    }

    pub fn row(
        &self,
        seam: WorthQueryProhibitedSeam,
    ) -> Option<&WorthQueryBoundaryAuditCoverageRow> {
        self.rows.iter().find(|row| row.seam() == seam)
    }
}

pub fn hard_prohibition_boundary_audit_coverage() -> WorthQueryBoundaryAuditCoverage {
    WorthQueryBoundaryAuditCoverage::new(
        hard_prohibition_registry()
            .rows()
            .iter()
            .map(|row| {
                WorthQueryBoundaryAuditCoverageRow::new(
                    row.seam(),
                    WorthQueryBoundaryAuditCoverageMechanism::AstMethodNameResolved,
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
