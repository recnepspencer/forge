use crate::storm_proof::ValidationAuthoringTruthFinalBossProof;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAuthoringTruthFinalBossVisibleSummary {
    heading: String,
    runtime_change_line: String,
    projection_counter_line: String,
    compile_boundary_line: String,
}

impl ValidationAuthoringTruthFinalBossVisibleSummary {
    pub fn from_proof(proof: &ValidationAuthoringTruthFinalBossProof) -> Self {
        Self {
            heading: "Authoring-truth final boss".to_owned(),
            runtime_change_line: format!(
                "authored_delta={} runtime_change={} visible_result={}",
                proof.authored_delta_digest(),
                proof.runtime_change().digest(),
                proof.visible_result_digest()
            ),
            projection_counter_line: format!(
                "projection counters: inspected={} intersections={} rebuilds={} preserved={} denied={} rebuilt={}",
                proof.projection_counters().inspected_projection_count(),
                proof.projection_counters().dependency_intersection_count(),
                proof.projection_counters().rebuild_attempt_count(),
                proof.projection_counters().preserved_frame_count(),
                proof.projection_counters().denied_frame_count(),
                proof.projection_counters().rebuilt_frame_count(),
            ),
            compile_boundary_line: format!(
                "compile boundary: posture={:?} changed={:?} hot_reloadable={:?} compile_required={:?}",
                proof.compile_boundary().posture(),
                proof.compile_boundary().changed_slice_ids(),
                proof.compile_boundary().hot_reloadable_slice_ids(),
                proof.compile_boundary().compile_required_slice_ids(),
            ),
        }
    }

    pub fn heading(&self) -> &str {
        &self.heading
    }
    pub fn runtime_change_line(&self) -> &str {
        &self.runtime_change_line
    }
    pub fn projection_counter_line(&self) -> &str {
        &self.projection_counter_line
    }
    pub fn compile_boundary_line(&self) -> &str {
        &self.compile_boundary_line
    }
}
