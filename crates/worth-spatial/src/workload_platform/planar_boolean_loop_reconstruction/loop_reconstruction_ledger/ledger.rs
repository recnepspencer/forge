use super::counters::PlanarBooleanLoopReconstructionLedgerCounters;
use super::denial::{
    PlanarBooleanLoopReconstructionLedgerDenial,
    PlanarBooleanLoopReconstructionLedgerDenialKind as Kind,
};
use super::identity::{ledger_identity, ledger_row_identity};
use super::input::PlanarBooleanLoopReconstructionLedgerInput;
use super::product_index::PlanarBooleanLoopReconstructionProductIndex;
use super::receipt::PlanarBooleanLoopReconstructionLedgerReceipt;
use super::row::PlanarBooleanLoopReconstructionLedgerRow;
use super::validation::validate_input;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopClassifiedProductKind, PlanarBooleanLoopIdentityRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionLedger {
    ledger_identity: String,
    request_identity: String,
    selected_plan_digest: String,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    touched_closure_digest: String,
    overlap_identity_digests: Vec<String>,
    topology_query_posture_digest: String,
    spatial_query_posture_digest: String,
    residue_digest: String,
    source_firewall_digest: String,
    architecture_claim_digest: String,
    decision_log_identity: String,
    loop_identity_map_identity: String,
    persistent_name_map_identity: String,
    subshape_signature_map_identity: String,
    reconstructed_loop_set_identity: String,
    born_loop_set_identity: String,
    island_partition_identity: String,
    split_attribution_identity: String,
    role_outcome_set_identity: String,
    degenerate_outcome_set_identity: String,
    rows: Vec<PlanarBooleanLoopReconstructionLedgerRow>,
    counters: PlanarBooleanLoopReconstructionLedgerCounters,
}

impl PlanarBooleanLoopReconstructionLedger {
    pub fn assemble(
        input: PlanarBooleanLoopReconstructionLedgerInput<'_>,
    ) -> Result<
        (Self, PlanarBooleanLoopReconstructionLedgerReceipt),
        PlanarBooleanLoopReconstructionLedgerDenial,
    > {
        let mut counters = PlanarBooleanLoopReconstructionLedgerCounters::default();
        validate_input(input, &mut counters)?;
        let index = PlanarBooleanLoopReconstructionProductIndex::build(&input, &mut counters);
        let mut rows = Vec::new();
        for identity_row in
            PlanarBooleanLoopReconstructionProductIndex::identity_rows(&input, &mut counters)
        {
            rows.push(build_row(identity_row, &index, &mut counters)?);
        }
        rows.sort_by(|left, right| left.ledger_row_identity().cmp(right.ledger_row_identity()));
        let ledger = Self {
            ledger_identity: ledger_identity(
                input.request().request_identity(),
                input.decision_log().decision_log_identity(),
                &rows,
            ),
            request_identity: input.request().request_identity().to_string(),
            selected_plan_digest: input.request().selected_plan_digest().to_string(),
            selected_route_identity_digest: input
                .request()
                .selected_route_identity_digest()
                .to_string(),
            selected_family_identity: input.request().selected_family_identity().to_string(),
            selected_product_identity_digest: input
                .request()
                .selected_product_identity_digest()
                .to_string(),
            selected_witness_identity_digest: input
                .request()
                .selected_witness_identity_digest()
                .map(str::to_string),
            touched_closure_digest: input.request().touched_closure_digest().to_string(),
            overlap_identity_digests: input.request().overlap_identity_digests().to_vec(),
            topology_query_posture_digest: input
                .request()
                .topology_query_posture_digest()
                .to_string(),
            spatial_query_posture_digest: input
                .request()
                .spatial_query_posture_digest()
                .to_string(),
            residue_digest: input.request().residue_digest().to_string(),
            source_firewall_digest: input.request().source_firewall_digest().to_string(),
            architecture_claim_digest: input.request().architecture_claim_digest().to_string(),
            decision_log_identity: input.decision_log().decision_log_identity().to_string(),
            loop_identity_map_identity: input.loop_identity_map().map_identity().to_string(),
            persistent_name_map_identity: input.persistent_name_map().map_identity().to_string(),
            subshape_signature_map_identity: input
                .subshape_signature_map()
                .map_identity()
                .to_string(),
            reconstructed_loop_set_identity: input
                .reconstructed_loops()
                .reconstructed_loop_set_identity()
                .to_string(),
            born_loop_set_identity: input.born_loops().born_loop_set_identity().to_string(),
            island_partition_identity: input.island_partition().partition_identity().to_string(),
            split_attribution_identity: input
                .split_attribution()
                .attribution_identity()
                .to_string(),
            role_outcome_set_identity: input
                .role_outcomes()
                .role_outcome_set_identity()
                .to_string(),
            degenerate_outcome_set_identity: input
                .degenerate_outcomes()
                .degenerate_loop_outcome_set_identity()
                .to_string(),
            rows,
            counters,
        };
        let receipt = PlanarBooleanLoopReconstructionLedgerReceipt::from_ledger(&ledger);
        Ok((ledger, receipt))
    }

    pub fn receipt(&self) -> PlanarBooleanLoopReconstructionLedgerReceipt {
        PlanarBooleanLoopReconstructionLedgerReceipt::from_ledger(self)
    }

    pub fn ledger_identity(&self) -> &str {
        &self.ledger_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn decision_log_identity(&self) -> &str {
        &self.decision_log_identity
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub fn topology_query_posture_digest(&self) -> &str {
        &self.topology_query_posture_digest
    }

    pub fn spatial_query_posture_digest(&self) -> &str {
        &self.spatial_query_posture_digest
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn architecture_claim_digest(&self) -> &str {
        &self.architecture_claim_digest
    }

    pub fn loop_identity_map_identity(&self) -> &str {
        &self.loop_identity_map_identity
    }

    pub fn persistent_name_map_identity(&self) -> &str {
        &self.persistent_name_map_identity
    }

    pub fn subshape_signature_map_identity(&self) -> &str {
        &self.subshape_signature_map_identity
    }

    pub fn reconstructed_loop_set_identity(&self) -> &str {
        &self.reconstructed_loop_set_identity
    }

    pub fn born_loop_set_identity(&self) -> &str {
        &self.born_loop_set_identity
    }

    pub fn island_partition_identity(&self) -> &str {
        &self.island_partition_identity
    }

    pub fn split_attribution_identity(&self) -> &str {
        &self.split_attribution_identity
    }

    pub fn role_outcome_set_identity(&self) -> &str {
        &self.role_outcome_set_identity
    }

    pub fn degenerate_outcome_set_identity(&self) -> &str {
        &self.degenerate_outcome_set_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopReconstructionLedgerRow] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionLedgerCounters {
        self.counters
    }
}

fn build_row(
    identity_row: &PlanarBooleanLoopIdentityRow,
    index: &PlanarBooleanLoopReconstructionProductIndex<'_>,
    counters: &mut PlanarBooleanLoopReconstructionLedgerCounters,
) -> Result<PlanarBooleanLoopReconstructionLedgerRow, PlanarBooleanLoopReconstructionLedgerDenial> {
    let tracked_loop_identity = identity_row.tracked_loop_identity();
    let (source_face_identities, loop_kind) = if let Some(row) =
        index.reconstructed_loop(tracked_loop_identity)
    {
        (
            vec![row.source_face_identity().to_string()],
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
        )
    } else if let Some(_row) = index.born_loop(tracked_loop_identity) {
        (Vec::new(), PlanarBooleanLoopClassifiedProductKind::BornLoop)
    } else {
        counters.denied_missing_tracked_loop();
        return Err(PlanarBooleanLoopReconstructionLedgerDenial::new(
            Kind::MissingTrackedLoop,
            tracked_loop_identity,
            *counters,
            "loop reconstruction ledger requires every canonical loop to bind to a tracked loop product",
        ));
    };
    let role_outcome = index.role_outcome(tracked_loop_identity).ok_or_else(|| {
        counters.denied_missing_role_outcome();
        PlanarBooleanLoopReconstructionLedgerDenial::new(
            Kind::MissingRoleOutcome,
            tracked_loop_identity,
            *counters,
            "loop reconstruction ledger requires role evidence for every canonical loop",
        )
    })?;
    let degenerate_outcome = index
        .degenerate_outcome(tracked_loop_identity)
        .ok_or_else(|| {
            counters.denied_missing_degenerate_outcome();
            PlanarBooleanLoopReconstructionLedgerDenial::new(
                Kind::MissingDegenerateOutcome,
                tracked_loop_identity,
                *counters,
                "loop reconstruction ledger requires degeneracy posture for every canonical loop",
            )
        })?;
    let mut decision_identities = index.decision_identities_for(identity_row.row_identity());
    decision_identities.extend(index.decision_identities_for(tracked_loop_identity));
    decision_identities
        .extend(index.decision_identities_for(identity_row.canonical_loop_identity()));
    decision_identities.extend(index.decision_identities_for(role_outcome.role_outcome_identity()));
    decision_identities.extend(
        index.decision_identities_for(degenerate_outcome.degenerate_loop_outcome_identity()),
    );
    decision_identities.sort();
    decision_identities.dedup();
    if decision_identities.is_empty() {
        counters.denied_missing_decision_trace();
        return Err(PlanarBooleanLoopReconstructionLedgerDenial::new(
            Kind::MissingDecisionTrace,
            identity_row.canonical_loop_identity(),
            *counters,
            "loop reconstruction ledger requires typed decision-log trace coverage for every canonical loop",
        ));
    }
    let row = PlanarBooleanLoopReconstructionLedgerRow::new(
        ledger_row_identity(
            identity_row.canonical_loop_identity(),
            tracked_loop_identity,
        ),
        identity_row.canonical_loop_identity().to_string(),
        tracked_loop_identity.to_string(),
        loop_kind,
        identity_row.source_loop_identities().to_vec(),
        source_face_identities,
        identity_row.fragment_identities().to_vec(),
        identity_row.split_vertex_identities().to_vec(),
        index.island_identities(tracked_loop_identity),
        role_outcome.role_outcome_identity().to_string(),
        degenerate_outcome
            .degenerate_loop_outcome_identity()
            .to_string(),
        index.propagated_persistent_name_identities(identity_row.canonical_loop_identity()),
        index.propagated_signature_identities(identity_row.canonical_loop_identity()),
        decision_identities,
    );
    counters.emitted_ledger_row();
    Ok(row)
}
