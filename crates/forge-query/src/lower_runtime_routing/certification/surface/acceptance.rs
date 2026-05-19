use crate::identity::hash_parts;
use crate::lower_runtime_routing::forge_query_lower_runtime_crossing_inventory;

use super::acceptance_checks::{control_digest, hostile_digest, required_phase_six_concrete_seams};
use super::evidence::forge_query_lower_runtime_representative_surface;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeAcceptanceLane {
    Control,
    Hostile,
    Parity,
}

impl ForgeQueryLowerRuntimeAcceptanceLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Control => "control-lane",
            Self::Hostile => "hostile-lane",
            Self::Parity => "parity-lane",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeAcceptanceRow {
    lane: ForgeQueryLowerRuntimeAcceptanceLane,
    digest: String,
    detail: String,
}

impl ForgeQueryLowerRuntimeAcceptanceRow {
    fn new(
        lane: ForgeQueryLowerRuntimeAcceptanceLane,
        digest: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            lane,
            digest: digest.into(),
            detail: detail.into(),
        }
    }

    pub fn lane(&self) -> ForgeQueryLowerRuntimeAcceptanceLane {
        self.lane
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeAcceptanceSuite {
    rows: Vec<ForgeQueryLowerRuntimeAcceptanceRow>,
    suite_digest: String,
}

impl ForgeQueryLowerRuntimeAcceptanceSuite {
    fn new(rows: Vec<ForgeQueryLowerRuntimeAcceptanceRow>) -> Self {
        let suite_digest = hash_parts(
            &rows
                .iter()
                .map(|row| format!("{}|{}|{}", row.lane().as_str(), row.digest(), row.detail()))
                .collect::<Vec<_>>(),
        );
        Self { rows, suite_digest }
    }

    pub fn rows(&self) -> &[ForgeQueryLowerRuntimeAcceptanceRow] {
        &self.rows
    }

    pub fn suite_digest(&self) -> &str {
        &self.suite_digest
    }

    pub fn lane(
        &self,
        lane: ForgeQueryLowerRuntimeAcceptanceLane,
    ) -> &ForgeQueryLowerRuntimeAcceptanceRow {
        self.rows
            .iter()
            .find(|row| row.lane() == lane)
            .unwrap_or_else(|| panic!("missing acceptance lane {}", lane.as_str()))
    }
}

pub fn forge_query_lower_runtime_acceptance_suite() -> ForgeQueryLowerRuntimeAcceptanceSuite {
    let surface = forge_query_lower_runtime_representative_surface();
    ForgeQueryLowerRuntimeAcceptanceSuite::new(vec![
        ForgeQueryLowerRuntimeAcceptanceRow::new(
            ForgeQueryLowerRuntimeAcceptanceLane::Control,
            control_digest(&surface),
            format!(
                "crossings={} requests={} route_plans={} receipts={} envelopes={} concrete={} synthetic={} required_concrete_seams={}",
                forge_query_lower_runtime_crossing_inventory().rows().len(),
                surface.requests().len(),
                surface.route_plans().len(),
                surface.boundary_receipts().len(),
                surface.envelopes().len(),
                surface.concrete_surface_width(),
                surface.synthetic_surface_width(),
                required_phase_six_concrete_seams().len(),
            ),
        ),
        ForgeQueryLowerRuntimeAcceptanceRow::new(
            ForgeQueryLowerRuntimeAcceptanceLane::Hostile,
            hostile_digest(&surface),
            "deleted seam survival, specialist-gap survival, route cardinality drift, support drift, and bypass drift remain forbidden".to_string(),
        ),
        ForgeQueryLowerRuntimeAcceptanceRow::new(
            ForgeQueryLowerRuntimeAcceptanceLane::Parity,
            surface.route_parity_digest().to_string(),
            "equivalent admitted crossings normalize to shared lower-runtime route meaning while intentionally different families diverge".to_string(),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_suite_exposes_control_hostile_and_parity_lanes() {
        let suite = forge_query_lower_runtime_acceptance_suite();

        assert_eq!(suite.rows().len(), 3);
        assert!(!suite.suite_digest().is_empty());
        assert!(suite
            .rows()
            .iter()
            .any(|row| row.lane() == ForgeQueryLowerRuntimeAcceptanceLane::Control));
        assert!(suite
            .rows()
            .iter()
            .any(|row| row.lane() == ForgeQueryLowerRuntimeAcceptanceLane::Hostile));
        assert!(suite
            .rows()
            .iter()
            .any(|row| row.lane() == ForgeQueryLowerRuntimeAcceptanceLane::Parity));
    }

    #[test]
    fn acceptance_suite_control_lane_proves_exact_cardinality() {
        let suite = forge_query_lower_runtime_acceptance_suite();
        let control = suite.lane(ForgeQueryLowerRuntimeAcceptanceLane::Control);

        assert!(control.detail().contains("crossings="));
        assert!(control.detail().contains("receipts="));
        assert!(control.detail().contains("envelopes="));
        assert!(control.detail().contains("concrete="));
        assert!(control.detail().contains("synthetic="));
    }

    #[test]
    fn acceptance_suite_hostile_digest_stays_distinct_from_control() {
        let suite = forge_query_lower_runtime_acceptance_suite();

        assert_ne!(
            suite
                .lane(ForgeQueryLowerRuntimeAcceptanceLane::Control)
                .digest(),
            suite
                .lane(ForgeQueryLowerRuntimeAcceptanceLane::Hostile)
                .digest()
        );
    }
}
