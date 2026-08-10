use super::super::super::certification::HostileExpectation;
use super::{PreviewCertificationRow, PreviewRejectionRow};

impl PreviewCertificationRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    pub fn has_hostile_coverage(&self) -> bool {
        match self.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_shape_digest
                        == self.hostile_lane.result_shape_digest
                    && self.control_lane.preview_session_identity
                        == self.hostile_lane.preview_session_identity
                    && self.control_lane.binding_digest == self.hostile_lane.binding_digest
                    && self.control_lane.preview_execution_digest
                        == self.hostile_lane.preview_execution_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
                    && self.control_lane.binding_digest == self.parity_lane.binding_digest
                    && self.control_lane.preview_execution_digest
                        == self.parity_lane.preview_execution_digest
                    && self.control_lane.preview_live_digest
                        == self.hostile_lane.preview_live_digest
                    && self.control_lane.preview_live_digest == self.parity_lane.preview_live_digest
                    && self.control_lane.preview_live_subscription_digest
                        == self.hostile_lane.preview_live_subscription_digest
                    && self.control_lane.preview_live_subscription_digest
                        == self.parity_lane.preview_live_subscription_digest
                    && self.control_lane.preview_live_family
                        == self.hostile_lane.preview_live_family
                    && self.control_lane.preview_live_family == self.parity_lane.preview_live_family
            }
            HostileExpectation::DistinctFromControl => {
                ((self.control_lane.evaluation_class != self.hostile_lane.evaluation_class
                    || self.control_lane.binding_digest != self.hostile_lane.binding_digest
                    || self.control_lane.preview_execution_digest
                        != self.hostile_lane.preview_execution_digest)
                    || (self.control_lane.preview_live_digest.is_some()
                        && self.control_lane.preview_live_digest
                            != self.hostile_lane.preview_live_digest))
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
                    && self.control_lane.binding_digest == self.parity_lane.binding_digest
                    && self.control_lane.preview_execution_digest
                        == self.parity_lane.preview_execution_digest
                    && self.control_lane.preview_live_digest == self.parity_lane.preview_live_digest
            }
        }
    }
}

impl PreviewRejectionRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
    }

    pub fn has_hostile_coverage(&self) -> bool {
        self.control_lane.query_digest == self.parity_lane.query_digest
            && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
            && self.control_lane.binding_digest == self.parity_lane.binding_digest
            && self.control_lane.preview_execution_digest
                == self.parity_lane.preview_execution_digest
    }
}
