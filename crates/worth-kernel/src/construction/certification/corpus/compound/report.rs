use crate::construction::certification::corpus::compound::schema::{
    PrimitiveConstructionCompoundAuthoringOrderRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
    PrimitiveConstructionCompoundRow,
};
use crate::construction::digest::digest_owned_parts;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundAdversarialSiegeReport {
    rows: Vec<PrimitiveConstructionCompoundRow>,
    authoring_order_rows: Vec<PrimitiveConstructionCompoundAuthoringOrderRow>,
    report_digest: String,
}

impl PrimitiveConstructionCompoundAdversarialSiegeReport {
    pub fn new(
        rows: Vec<PrimitiveConstructionCompoundRow>,
        authoring_order_rows: Vec<PrimitiveConstructionCompoundAuthoringOrderRow>,
    ) -> Self {
        let mut parts = rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>();
        parts.extend(
            authoring_order_rows
                .iter()
                .map(|row| row.row_digest().to_string()),
        );
        Self {
            rows,
            authoring_order_rows,
            report_digest: digest_owned_parts(&parts),
        }
    }
    pub fn rows(&self) -> &[PrimitiveConstructionCompoundRow] {
        &self.rows
    }
    pub fn authoring_order_rows(&self) -> &[PrimitiveConstructionCompoundAuthoringOrderRow] {
        &self.authoring_order_rows
    }
    pub fn authoring_order_parity_verified(&self) -> bool {
        self.authoring_order_rows
            .iter()
            .all(|row| row.parity_verified())
    }
    pub fn row_for(&self, scenario_id: &str) -> Option<&PrimitiveConstructionCompoundRow> {
        self.rows
            .iter()
            .find(|row| row.scenario_id() == scenario_id)
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundMotionParityReport {
    rows: Vec<PrimitiveConstructionCompoundMotionParityRow>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionCompoundMotionParityReport {
    pub fn new(
        rows: Vec<PrimitiveConstructionCompoundMotionParityRow>,
        parity_verified: bool,
    ) -> Self {
        let mut parts = rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>();
        parts.push(parity_verified.to_string());
        Self {
            rows,
            parity_verified,
            report_digest: digest_owned_parts(&parts),
        }
    }
    pub fn rows(&self) -> &[PrimitiveConstructionCompoundMotionParityRow] {
        &self.rows
    }
    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundGrazingBoundaryReport {
    rows: Vec<PrimitiveConstructionCompoundGrazingBoundaryRow>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionCompoundGrazingBoundaryReport {
    pub fn new(
        rows: Vec<PrimitiveConstructionCompoundGrazingBoundaryRow>,
        parity_verified: bool,
    ) -> Self {
        let mut parts = rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>();
        parts.push(parity_verified.to_string());
        Self {
            rows,
            parity_verified,
            report_digest: digest_owned_parts(&parts),
        }
    }
    pub fn rows(&self) -> &[PrimitiveConstructionCompoundGrazingBoundaryRow] {
        &self.rows
    }
    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
