use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};

use super::super::stable_digest;
use super::cap_ledger_row::WorthGraphReadAccessPostureCapRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostureCapLedger {
    rows: Vec<WorthGraphReadAccessPostureCapRow>,
    ledger_digest: String,
}

impl WorthGraphReadAccessPostureCapLedger {
    pub(crate) fn current() -> Self {
        Self::from_rows(current_cap_rows())
    }

    #[cfg(test)]
    pub(crate) fn from_rows_for_tests(rows: Vec<WorthGraphReadAccessPostureCapRow>) -> Self {
        Self::from_rows(rows)
    }

    fn from_rows(rows: Vec<WorthGraphReadAccessPostureCapRow>) -> Self {
        let mut digest_parts = vec![
            "worth_graph_read_access_posture_cap_ledger_v1".to_string(),
            format!("row_count:{}", rows.len()),
        ];
        digest_parts.extend(
            rows.iter()
                .map(|row| format!("cap_row:{}", row.row_digest())),
        );
        Self {
            rows,
            ledger_digest: stable_digest(&digest_parts),
        }
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessPostureCapRow] {
        &self.rows
    }

    pub fn row_for_family(&self, family: &str) -> Option<&WorthGraphReadAccessPostureCapRow> {
        self.rows.iter().find(|row| row.family() == family)
    }

    pub fn covers_query_posture(&self, posture: &str) -> bool {
        self.row_for_family(posture).is_some()
    }

    pub fn covers_query_denial_kind(&self, denial_kind: &str) -> bool {
        self.row_for_family(denial_kind).is_some()
    }

    pub fn ledger_digest(&self) -> &str {
        &self.ledger_digest
    }
}

fn current_cap_rows() -> Vec<WorthGraphReadAccessPostureCapRow> {
    let mut rows = ForgeQueryGraphReadAccessAdmissionPosture::ALL
        .iter()
        .map(|posture| {
            WorthGraphReadAccessPostureCapRow::new(
                posture.as_str(),
                4096,
                "forge-query",
                "query_posture_family_observed",
                posture.as_str(),
                "Every forge-query graph-read admission posture must be represented before execution.",
                "Phase 4 may consume only capped posture families.",
            )
        })
        .collect::<Vec<_>>();

    rows.extend(ForgeQueryGraphReadAccessDenialKind::ALL.iter().map(|kind| {
        WorthGraphReadAccessPostureCapRow::new(
            kind.as_str(),
            4096,
            "forge-query",
            kind.as_str(),
            "denied_or_required_support_posture",
            "Every forge-query graph-read denial kind must be represented before receipts.",
            "Phase 4 may consume only capped denial families.",
        )
    }));

    rows.extend([
        WorthGraphReadAccessPostureCapRow::new(
            "admitted_plan_candidate",
            4096,
            "worth-kernel",
            "none",
            "admitted_plan_candidate",
            "Phase 2 admission candidates are metadata only until Phase 4 consumes them.",
            "Replace with forge-query admitted access plans in Phase 4.",
        ),
        WorthGraphReadAccessPostureCapRow::new(
            "required_support_posture",
            4096,
            "worth-kernel",
            "required_support_posture",
            "required_support_posture",
            "Generic required-support rows must stay capped until mapped to concrete query support.",
            "Replace generic support posture with exact forge-query posture.",
        ),
        WorthGraphReadAccessPostureCapRow::new(
            "carried_capability_gap",
            4096,
            "worth-kernel",
            "carried_capability_gap",
            "capability_gap_handoff",
            "Capability gaps may be carried but never executed as graph reads.",
            "Delete carried gap once the matching query family is adopted.",
        ),
        WorthGraphReadAccessPostureCapRow::new(
            "requirement_derivation_gap",
            5,
            "worth-kernel",
            "missing_query_requirement_derivation",
            "required_support_posture",
            "Milestone 7 currently exposes five declaration candidates whose requirement rows remain typed gaps.",
            "Delete this cap when forge-query derives requirement rows for every covered declaration candidate.",
        ),
        WorthGraphReadAccessPostureCapRow::new(
            "missing_query_read_family_artifact",
            4096,
            "worth-kernel",
            "missing_query_read_family_artifact",
            "declare_query_read_family_artifact",
            "A Worth requirement without a query read-family artifact cannot be admitted.",
            "Delete missing-artifact rows once the query read family is declared.",
        ),
    ]);

    rows
}
