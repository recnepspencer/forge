use worth_ui::facade::{WorthUiProjectionFamily, WorthUiProjectionRebindStatus};

use crate::storm_proof::{
    ValidationMixedReloadStormProjectionSurface, ValidationMixedReloadStormProof,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormVisibleSummary {
    heading: String,
    scenario_line: String,
    projection_counter_line: String,
    projection_rows: Vec<ValidationMixedReloadStormVisibleRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormVisibleRow {
    surface: ValidationMixedReloadStormProjectionSurface,
    projection_identity: String,
    projection_family: WorthUiProjectionFamily,
    status: WorthUiProjectionRebindStatus,
}

impl ValidationMixedReloadStormVisibleSummary {
    pub fn from_proof(storm: &ValidationMixedReloadStormProof) -> Self {
        let posture = storm.posture();
        let counters = storm.projection_counters();
        Self {
            heading: "Mixed reload storm".to_owned(),
            scenario_line: format!(
                "scenario={} activated={} equivalent={} denied={}",
                storm.scenario_digest(),
                posture.activated_step_count(),
                posture.equivalent_step_count(),
                posture.denied_step_count()
            ),
            projection_counter_line: format!(
                "projection counters: inspected={} intersections={} rebuilds={} preserved={} denied={} rebuilt={}",
                counters.inspected_projection_count(),
                counters.dependency_intersection_count(),
                counters.rebuild_attempt_count(),
                counters.preserved_frame_count(),
                counters.denied_frame_count(),
                counters.rebuilt_frame_count()
            ),
            projection_rows: storm
                .projection_roster()
                .rows()
                .iter()
                .map(|row| ValidationMixedReloadStormVisibleRow {
                    surface: row.surface(),
                    projection_identity: row.projection_identity().to_owned(),
                    projection_family: row.projection_family(),
                    status: row.status(),
                })
                .collect(),
        }
    }

    pub fn heading(&self) -> &str {
        &self.heading
    }

    pub fn scenario_line(&self) -> &str {
        &self.scenario_line
    }

    pub fn projection_counter_line(&self) -> &str {
        &self.projection_counter_line
    }

    pub fn projection_rows(&self) -> &[ValidationMixedReloadStormVisibleRow] {
        &self.projection_rows
    }
}

impl ValidationMixedReloadStormVisibleRow {
    pub fn surface(&self) -> ValidationMixedReloadStormProjectionSurface {
        self.surface
    }

    pub fn projection_identity(&self) -> &str {
        &self.projection_identity
    }

    pub fn projection_family(&self) -> WorthUiProjectionFamily {
        self.projection_family
    }

    pub fn status(&self) -> WorthUiProjectionRebindStatus {
        self.status
    }

    pub fn display_line(&self) -> String {
        format!(
            "{:?}|{}|{:?}|{:?}",
            self.surface, self.projection_identity, self.projection_family, self.status
        )
    }
}
