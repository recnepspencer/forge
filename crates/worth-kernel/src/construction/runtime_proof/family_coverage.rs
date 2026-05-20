use crate::construction::digest::digest_owned_parts;
use crate::construction::request::PrimitiveConstructionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionFamilyCoverageStatus {
    AdmittedClosedSolid,
    AdmittedPlanarConstruction,
}

impl PrimitiveConstructionFamilyCoverageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedClosedSolid => "admitted_closed_solid",
            Self::AdmittedPlanarConstruction => "admitted_planar_construction",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionFamilyCoverageRow {
    family: PrimitiveConstructionFamily,
    status: PrimitiveConstructionFamilyCoverageStatus,
    reason: &'static str,
    row_digest: String,
}

impl PrimitiveConstructionFamilyCoverageRow {
    fn new(
        family: PrimitiveConstructionFamily,
        status: PrimitiveConstructionFamilyCoverageStatus,
        reason: &'static str,
    ) -> Self {
        let parts = [
            family.as_str().to_string(),
            status.as_str().to_string(),
            reason.to_string(),
        ];
        Self {
            family,
            status,
            reason,
            row_digest: digest_owned_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn status(&self) -> PrimitiveConstructionFamilyCoverageStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionFamilyCoverageReport {
    rows: Vec<PrimitiveConstructionFamilyCoverageRow>,
    report_digest: String,
}

impl PrimitiveConstructionFamilyCoverageReport {
    pub fn rows(&self) -> &[PrimitiveConstructionFamilyCoverageRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        family: PrimitiveConstructionFamily,
    ) -> Option<&PrimitiveConstructionFamilyCoverageRow> {
        self.rows.iter().find(|row| row.family() == family)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn primitive_construction_family_coverage_report() -> PrimitiveConstructionFamilyCoverageReport
{
    let rows = PrimitiveConstructionFamily::ALL
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
        .collect::<Vec<_>>();
    let report_digest = digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    PrimitiveConstructionFamilyCoverageReport {
        rows,
        report_digest,
    }
}
