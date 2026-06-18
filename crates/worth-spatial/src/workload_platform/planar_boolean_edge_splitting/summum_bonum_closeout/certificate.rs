use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::candidate_manifest::{
    closeout_candidate_manifest_rows, PlanarBooleanEdgeSplitCloseoutCandidateRow,
};
use super::counters::PlanarBooleanEdgeSplitSummumBonumCloseoutCounters as Counters;
use super::decision_localization::{
    closeout_decision_localization_rows, PlanarBooleanEdgeSplitCloseoutDecisionRow,
};
use super::denial::PlanarBooleanEdgeSplitSummumBonumCloseoutDenial as Denial;
use super::input::PlanarBooleanEdgeSplitSummumBonumCloseoutInput;
use super::source_edge_lineage::{
    closeout_source_edge_lineage_rows, PlanarBooleanEdgeSplitCloseoutLineageRow,
};
use super::validation::{validate_input, validate_rows};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitSummumBonumCloseout {
    closeout_identity: String,
    candidate_index_product_identity: String,
    candidate_index_plan_digest: String,
    decision_log_receipt_identity: String,
    split_ledger_receipt_identity: String,
    persistent_naming_receipt_identity: String,
    replay_parity_receipt_identity: String,
    downstream_consumption_identity: String,
    loop_reconstruction_consumption_identity: String,
    candidate_rows: Vec<PlanarBooleanEdgeSplitCloseoutCandidateRow>,
    lineage_rows: Vec<PlanarBooleanEdgeSplitCloseoutLineageRow>,
    decision_rows: Vec<PlanarBooleanEdgeSplitCloseoutDecisionRow>,
    counters: Counters,
}

impl PlanarBooleanEdgeSplitSummumBonumCloseout {
    pub fn certify(
        input: PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'_>,
    ) -> Result<Self, Denial> {
        validate_input(&input)?;
        let candidate_rows = closeout_candidate_manifest_rows(input.candidate_index());
        let lineage_rows = closeout_source_edge_lineage_rows(input);
        let decision_rows =
            closeout_decision_localization_rows(input.decision_log().decision_rows());
        validate_rows(&candidate_rows, &lineage_rows, &decision_rows)?;

        let counters = closeout_counters(input, &candidate_rows, &lineage_rows, &decision_rows);

        let closeout_identity = closeout_identity(
            input.candidate_index().product_identity(),
            input.decision_log().receipt_identity(),
            input.split_ledger().receipt_identity(),
            input.persistent_naming().receipt_identity(),
            input.replay_parity().receipt_identity(),
            input.downstream_consumption().consumption_identity(),
            input
                .loop_reconstruction_consumption()
                .consumption_identity(),
            &candidate_rows,
            &lineage_rows,
            &decision_rows,
        );
        Ok(Self {
            closeout_identity,
            candidate_index_product_identity: input
                .candidate_index()
                .product_identity()
                .to_string(),
            candidate_index_plan_digest: input.candidate_index().plan_digest().to_string(),
            decision_log_receipt_identity: input.decision_log().receipt_identity().to_string(),
            split_ledger_receipt_identity: input.split_ledger().receipt_identity().to_string(),
            persistent_naming_receipt_identity: input
                .persistent_naming()
                .receipt_identity()
                .to_string(),
            replay_parity_receipt_identity: input.replay_parity().receipt_identity().to_string(),
            downstream_consumption_identity: input
                .downstream_consumption()
                .consumption_identity()
                .to_string(),
            loop_reconstruction_consumption_identity: input
                .loop_reconstruction_consumption()
                .consumption_identity()
                .to_string(),
            candidate_rows,
            lineage_rows,
            decision_rows,
            counters,
        })
    }

    pub fn closeout_identity(&self) -> &str {
        &self.closeout_identity
    }
    pub fn candidate_index_product_identity(&self) -> &str {
        &self.candidate_index_product_identity
    }
    pub fn candidate_index_plan_digest(&self) -> &str {
        &self.candidate_index_plan_digest
    }
    pub fn decision_log_receipt_identity(&self) -> &str {
        &self.decision_log_receipt_identity
    }
    pub fn split_ledger_receipt_identity(&self) -> &str {
        &self.split_ledger_receipt_identity
    }
    pub fn persistent_naming_receipt_identity(&self) -> &str {
        &self.persistent_naming_receipt_identity
    }
    pub fn replay_parity_receipt_identity(&self) -> &str {
        &self.replay_parity_receipt_identity
    }
    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
    }
    pub fn loop_reconstruction_consumption_identity(&self) -> &str {
        &self.loop_reconstruction_consumption_identity
    }
    pub fn candidate_rows(&self) -> &[PlanarBooleanEdgeSplitCloseoutCandidateRow] {
        &self.candidate_rows
    }
    pub fn lineage_rows(&self) -> &[PlanarBooleanEdgeSplitCloseoutLineageRow] {
        &self.lineage_rows
    }
    pub fn decision_rows(&self) -> &[PlanarBooleanEdgeSplitCloseoutDecisionRow] {
        &self.decision_rows
    }
    pub fn counters(&self) -> Counters {
        self.counters
    }
    pub fn certifies_milestone_7_3_summum_bonum_closeout(&self) -> bool {
        !self.closeout_identity.is_empty()
            && !self.candidate_index_product_identity.is_empty()
            && !self.candidate_index_plan_digest.is_empty()
            && !self.decision_log_receipt_identity.is_empty()
            && !self.split_ledger_receipt_identity.is_empty()
            && !self.persistent_naming_receipt_identity.is_empty()
            && !self.replay_parity_receipt_identity.is_empty()
            && !self.downstream_consumption_identity.is_empty()
            && !self.loop_reconstruction_consumption_identity.is_empty()
            && self.counters.candidate_rows() == self.candidate_rows.len()
            && self.counters.lineage_rows() == self.lineage_rows.len()
            && self.counters.decision_rows() == self.decision_rows.len()
            && self.counters.persistent_name_rows() > 0
            && self.counters.replay_parity_rows() > 0
            && self.counters.split_ledger_chains() > 0
            && self.counters.downstream_consumptions() == 1
            && self.counters.loop_reconstruction_consumptions() == 1
    }
}

fn closeout_counters(
    input: PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'_>,
    candidate_rows: &[PlanarBooleanEdgeSplitCloseoutCandidateRow],
    lineage_rows: &[PlanarBooleanEdgeSplitCloseoutLineageRow],
    decision_rows: &[PlanarBooleanEdgeSplitCloseoutDecisionRow],
) -> Counters {
    let mut counters = Counters::default();
    counters.record_candidate_rows(candidate_rows.len());
    counters.record_lineage_rows(lineage_rows.len());
    counters.record_decision_rows(decision_rows.len());
    counters.record_persistent_name_rows(input.persistent_naming().persistent_name_rows().len());
    counters.record_replay_parity_rows(input.replay_parity().parity_rows().len());
    counters.record_split_ledger_chains(input.split_ledger().chain_identities().len());
    counters.record_downstream_consumption();
    counters.record_loop_reconstruction_consumption();
    counters.record_endpoint_noop_decisions(
        input
            .endpoint_boundary()
            .counters()
            .endpoint_noop_decisions(),
    );
    counters.record_micro_interval_policy_required(
        input
            .interval_subdivision()
            .counters()
            .micro_intervals_policy_required(),
    );
    counters.record_topology_products_emitted(
        input
            .overlap_chains()
            .counters()
            .topology_products_emitted(),
    );
    counters
}

fn closeout_identity(
    candidate_index_identity: &str,
    decision_log_identity: &str,
    split_ledger_identity: &str,
    persistent_naming_identity: &str,
    replay_parity_identity: &str,
    downstream_identity: &str,
    loop_identity: &str,
    candidate_rows: &[PlanarBooleanEdgeSplitCloseoutCandidateRow],
    lineage_rows: &[PlanarBooleanEdgeSplitCloseoutLineageRow],
    decision_rows: &[PlanarBooleanEdgeSplitCloseoutDecisionRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-edge-split-summum-bonum-closeout".to_string(),
        format!("candidate-index:{candidate_index_identity}"),
        format!("decision-log:{decision_log_identity}"),
        format!("split-ledger:{split_ledger_identity}"),
        format!("persistent-naming:{persistent_naming_identity}"),
        format!("replay-parity:{replay_parity_identity}"),
        format!("downstream:{downstream_identity}"),
        format!("loop:{loop_identity}"),
    ];
    parts.extend(candidate_rows.iter().map(|row| {
        format!(
            "candidate:{}:{}:{}",
            row.candidate_identity(),
            row.left_source_edge_identity(),
            row.right_source_edge_identity()
        )
    }));
    parts.extend(lineage_rows.iter().map(|row| {
        format!(
            "lineage:{}:{}:{}",
            row.source_edge_identity(),
            row.carrier_identity(),
            row.fragment_identities().join(",")
        )
    }));
    parts.extend(
        decision_rows
            .iter()
            .map(|row| format!("decision:{}", row.decision_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
