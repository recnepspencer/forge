use crate::construction::request::{PrimitiveConstructionFamily, PRIMITIVE_CONSTRUCTION_FAMILIES};
use crate::construction::tests::support::evidence_reports::sealed_report_identity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionFamilyCoverageStatus {
    AdmittedClosedSolid,
    AdmittedPlanarConstruction,
}

impl PrimitiveConstructionFamilyCoverageStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedClosedSolid => "admitted_closed_solid",
            Self::AdmittedPlanarConstruction => "admitted_planar_construction",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionFamilyCoverageRow {
    family: PrimitiveConstructionFamily,
    status: PrimitiveConstructionFamilyCoverageStatus,
    row_digest: String,
}

impl PrimitiveConstructionFamilyCoverageRow {
    fn new(
        family: PrimitiveConstructionFamily,
        status: PrimitiveConstructionFamilyCoverageStatus,
        reason: &'static str,
    ) -> Self {
        Self {
            family,
            status,
            row_digest: sealed_report_identity(
                "worth-kernel.construction.family-coverage",
                "family-coverage-row",
                |report| {
                    report
                        .shape_participating("family", family.as_str())?
                        .shape_participating("status", status.as_str())?
                        .value_participating("reason", reason)
                },
            ),
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn status(&self) -> PrimitiveConstructionFamilyCoverageStatus {
        self.status
    }

    pub(crate) fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionFamilyCoverageReport {
    rows: Vec<PrimitiveConstructionFamilyCoverageRow>,
}

impl PrimitiveConstructionFamilyCoverageReport {
    pub(crate) fn rows(&self) -> &[PrimitiveConstructionFamilyCoverageRow] {
        &self.rows
    }

    pub(crate) fn row_for(
        &self,
        family: PrimitiveConstructionFamily,
    ) -> Option<&PrimitiveConstructionFamilyCoverageRow> {
        self.rows.iter().find(|row| row.family() == family)
    }
}

pub(crate) fn primitive_construction_family_coverage_report(
) -> PrimitiveConstructionFamilyCoverageReport {
    let rows = PRIMITIVE_CONSTRUCTION_FAMILIES
        .into_iter()
        .map(|family| match family {
            PrimitiveConstructionFamily::SimplexSolid
            | PrimitiveConstructionFamily::Orthotope
            | PrimitiveConstructionFamily::RegularPrism
            | PrimitiveConstructionFamily::RegularPyramid => {
                PrimitiveConstructionFamilyCoverageRow::new(
                    family,
                    PrimitiveConstructionFamilyCoverageStatus::AdmittedClosedSolid,
                    "family is admitted through the canonical closed-solid phase chain in Phase 3",
                )
            }
            PrimitiveConstructionFamily::WireBody => PrimitiveConstructionFamilyCoverageRow::new(
                family,
                PrimitiveConstructionFamilyCoverageStatus::AdmittedPlanarConstruction,
                "family is admitted through the canonical planar wire-body phase chain in Phase 3",
            ),
            PrimitiveConstructionFamily::ShellWithHole => {
                PrimitiveConstructionFamilyCoverageRow::new(
                    family,
                    PrimitiveConstructionFamilyCoverageStatus::AdmittedPlanarConstruction,
                    "family is admitted through the canonical planar shell-with-hole phase chain in Phase 3",
                )
            }
        })
        .collect();
    PrimitiveConstructionFamilyCoverageReport { rows }
}
