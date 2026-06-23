#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopRoleOutcomeBoundaryCounters {
    reconstructed_loops_consumed: usize,
    born_loops_consumed: usize,
    role_outcomes_emitted: usize,
    containment_postures_emitted: usize,
    preserved_role_outcomes: usize,
    single_source_born_role_outcomes: usize,
    ambiguous_role_outcomes: usize,
    contradictory_role_outcomes: usize,
    missing_role_evidence_outcomes: usize,
}

impl PlanarBooleanLoopRoleOutcomeBoundaryCounters {
    pub(crate) fn consumed_reconstructed_loop(&mut self) {
        self.reconstructed_loops_consumed += 1;
    }

    pub(crate) fn consumed_born_loop(&mut self) {
        self.born_loops_consumed += 1;
    }

    pub(crate) fn emitted_role_outcome(
        &mut self,
        kind: crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopRoleOutcomeKind,
    ) {
        self.role_outcomes_emitted += 1;
        match kind {
            crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole => {
                self.preserved_role_outcomes += 1;
            }
            crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopRoleOutcomeKind::SingleSourceBornLoopRoleDerivedFromEvidence => {
                self.single_source_born_role_outcomes += 1;
            }
            crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopRoleOutcomeKind::BornLoopRoleAmbiguous => {
                self.ambiguous_role_outcomes += 1;
            }
            crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopRoleOutcomeKind::ContradictorySourceRoleEvidence => {
                self.contradictory_role_outcomes += 1;
            }
            crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopRoleOutcomeKind::MissingSourceRoleEvidence => {
                self.missing_role_evidence_outcomes += 1;
            }
        }
    }

    pub(crate) fn emitted_containment_posture(&mut self) {
        self.containment_postures_emitted += 1;
    }

    pub fn reconstructed_loops_consumed(self) -> usize {
        self.reconstructed_loops_consumed
    }

    pub fn born_loops_consumed(self) -> usize {
        self.born_loops_consumed
    }

    pub fn role_outcomes_emitted(self) -> usize {
        self.role_outcomes_emitted
    }

    pub fn containment_postures_emitted(self) -> usize {
        self.containment_postures_emitted
    }

    pub fn preserved_role_outcomes(self) -> usize {
        self.preserved_role_outcomes
    }

    pub fn single_source_born_role_outcomes(self) -> usize {
        self.single_source_born_role_outcomes
    }

    pub fn ambiguous_role_outcomes(self) -> usize {
        self.ambiguous_role_outcomes
    }

    pub fn contradictory_role_outcomes(self) -> usize {
        self.contradictory_role_outcomes
    }

    pub fn missing_role_evidence_outcomes(self) -> usize {
        self.missing_role_evidence_outcomes
    }
}
