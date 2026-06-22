use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanBornLoop,
    PlanarBooleanDegenerateLoopOutcome, PlanarBooleanLoopDecisionLog, PlanarBooleanLoopIdentityRow,
    PlanarBooleanLoopIslandPartitionRow, PlanarBooleanLoopPersistentNamePropagationRow,
    PlanarBooleanLoopRoleOutcome, PlanarBooleanLoopSubshapeSignatureRow,
};

use super::counters::PlanarBooleanLoopReconstructionLedgerCounters;
use super::input::PlanarBooleanLoopReconstructionLedgerInput;

pub(super) struct PlanarBooleanLoopReconstructionProductIndex<'a> {
    reconstructed_loops: BTreeMap<&'a str, &'a PlanarBooleanAdmittedReconstructedLoop>,
    born_loops: BTreeMap<&'a str, &'a PlanarBooleanBornLoop>,
    role_outcomes: BTreeMap<&'a str, &'a PlanarBooleanLoopRoleOutcome>,
    degenerate_outcomes: BTreeMap<&'a str, &'a PlanarBooleanDegenerateLoopOutcome>,
    island_rows: BTreeMap<&'a str, Vec<&'a PlanarBooleanLoopIslandPartitionRow>>,
    name_rows: BTreeMap<&'a str, Vec<&'a PlanarBooleanLoopPersistentNamePropagationRow>>,
    signature_rows: BTreeMap<&'a str, Vec<&'a PlanarBooleanLoopSubshapeSignatureRow>>,
    decision_log: &'a PlanarBooleanLoopDecisionLog,
}

impl<'a> PlanarBooleanLoopReconstructionProductIndex<'a> {
    pub(super) fn build(
        input: &PlanarBooleanLoopReconstructionLedgerInput<'a>,
        counters: &mut PlanarBooleanLoopReconstructionLedgerCounters,
    ) -> Self {
        let mut reconstructed_loops = BTreeMap::new();
        for row in input.reconstructed_loops().rows() {
            reconstructed_loops.insert(row.reconstructed_loop_identity(), row);
        }
        let mut born_loops = BTreeMap::new();
        for row in input.born_loops().rows() {
            born_loops.insert(row.born_loop_identity(), row);
        }
        let mut role_outcomes = BTreeMap::new();
        for row in input.role_outcomes().rows() {
            role_outcomes.insert(row.loop_identity(), row);
        }
        let mut degenerate_outcomes = BTreeMap::new();
        for row in input.degenerate_outcomes().rows() {
            degenerate_outcomes.insert(row.loop_identity(), row);
        }
        let mut island_rows: BTreeMap<&str, Vec<&PlanarBooleanLoopIslandPartitionRow>> =
            BTreeMap::new();
        for row in input.island_partition().rows() {
            for member_loop_identity in row.member_loop_identities() {
                island_rows
                    .entry(member_loop_identity.as_str())
                    .or_default()
                    .push(row);
            }
        }
        let mut name_rows: BTreeMap<&str, Vec<&PlanarBooleanLoopPersistentNamePropagationRow>> =
            BTreeMap::new();
        for row in input.persistent_name_map().rows() {
            counters.consumed_propagated_name_row();
            name_rows
                .entry(row.canonical_loop_identity())
                .or_default()
                .push(row);
        }
        let mut signature_rows: BTreeMap<&str, Vec<&PlanarBooleanLoopSubshapeSignatureRow>> =
            BTreeMap::new();
        for row in input.subshape_signature_map().rows() {
            counters.consumed_propagated_signature_row();
            signature_rows
                .entry(row.canonical_loop_identity())
                .or_default()
                .push(row);
        }
        for _ in input.decision_log().rows() {
            counters.consumed_decision_row();
        }
        Self {
            reconstructed_loops,
            born_loops,
            role_outcomes,
            degenerate_outcomes,
            island_rows,
            name_rows,
            signature_rows,
            decision_log: input.decision_log(),
        }
    }

    pub(super) fn reconstructed_loop(
        &self,
        tracked_loop_identity: &str,
    ) -> Option<&'a PlanarBooleanAdmittedReconstructedLoop> {
        self.reconstructed_loops.get(tracked_loop_identity).copied()
    }

    pub(super) fn born_loop(
        &self,
        tracked_loop_identity: &str,
    ) -> Option<&'a PlanarBooleanBornLoop> {
        self.born_loops.get(tracked_loop_identity).copied()
    }

    pub(super) fn role_outcome(
        &self,
        tracked_loop_identity: &str,
    ) -> Option<&'a PlanarBooleanLoopRoleOutcome> {
        self.role_outcomes.get(tracked_loop_identity).copied()
    }

    pub(super) fn degenerate_outcome(
        &self,
        tracked_loop_identity: &str,
    ) -> Option<&'a PlanarBooleanDegenerateLoopOutcome> {
        self.degenerate_outcomes.get(tracked_loop_identity).copied()
    }

    pub(super) fn island_identities(&self, tracked_loop_identity: &str) -> Vec<String> {
        self.island_rows
            .get(tracked_loop_identity)
            .into_iter()
            .flatten()
            .map(|row| row.island_identity().to_string())
            .collect()
    }

    pub(super) fn propagated_persistent_name_identities(
        &self,
        canonical_loop_identity: &str,
    ) -> Vec<String> {
        self.name_rows
            .get(canonical_loop_identity)
            .into_iter()
            .flatten()
            .map(|row| row.propagated_persistent_name_identity().to_string())
            .collect()
    }

    pub(super) fn propagated_signature_identities(
        &self,
        canonical_loop_identity: &str,
    ) -> Vec<String> {
        self.signature_rows
            .get(canonical_loop_identity)
            .into_iter()
            .flatten()
            .map(|row| row.propagated_signature_identity().to_string())
            .collect()
    }

    pub(super) fn decision_identities_for(&self, artifact_identity: &str) -> Vec<String> {
        self.decision_log
            .decisions_for_artifact(artifact_identity)
            .into_iter()
            .map(|row| row.decision_identity().to_string())
            .collect()
    }

    pub(super) fn identity_rows(
        input: &PlanarBooleanLoopReconstructionLedgerInput<'a>,
        counters: &mut PlanarBooleanLoopReconstructionLedgerCounters,
    ) -> &'a [PlanarBooleanLoopIdentityRow] {
        for _ in input.loop_identity_map().rows() {
            counters.consumed_identity_row();
        }
        input.loop_identity_map().rows()
    }
}
