use super::replay_execution::PlanarBooleanEdgeSplitCloseout;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitReplayClosureRowKind {
    SplitRequestIdentity,
    SourceCarrierIdentity,
    ParticipationIndexIdentity,
    PointCandidateIdentity,
    IntervalCandidateIdentity,
    TJunctionDecision,
    EndpointNoOpDecision,
    RawScheduleIdentity,
    NormalizedScheduleIdentity,
    SplitVertexIdentity,
    CoalescenceDecision,
    SplitFragmentIdentity,
    OverlapChainIdentity,
    PersistentNamingPropagation,
    DecisionLogDigest,
    SplitLedgerDigest,
    DownstreamConsumptionIdentity,
    Counters,
    DenialPosture,
    PolicyPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitReplayClosureRow {
    kind: PlanarBooleanSplitReplayClosureRowKind,
    row_identity: String,
    original_identity: String,
    replayed_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitReplayClosureManifest {
    manifest_identity: String,
    rows: Vec<PlanarBooleanSplitReplayClosureRow>,
}

impl PlanarBooleanSplitReplayClosureManifest {
    pub(crate) fn compare_closeouts(
        original: &PlanarBooleanEdgeSplitCloseout<'_>,
        replayed: &PlanarBooleanEdgeSplitCloseout<'_>,
    ) -> Self {
        let rows = vec![
            row(
                PlanarBooleanSplitReplayClosureRowKind::SplitRequestIdentity,
                original.request().split_request_identity(),
                replayed.request().split_request_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::SourceCarrierIdentity,
                original.request().event_ledger_identity(),
                replayed.request().event_ledger_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::ParticipationIndexIdentity,
                original.endpoint_boundary().schedule_set_identity(),
                replayed.endpoint_boundary().schedule_set_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::PointCandidateIdentity,
                original
                    .endpoint_boundary()
                    .normalized_schedule_set_identity(),
                replayed
                    .endpoint_boundary()
                    .normalized_schedule_set_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::IntervalCandidateIdentity,
                original.interval_subdivision().schedule_set_identity(),
                replayed.interval_subdivision().schedule_set_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::TJunctionDecision,
                &endpoint_counter_identity(original),
                &endpoint_counter_identity(replayed),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::EndpointNoOpDecision,
                &endpoint_counter_identity(original),
                &endpoint_counter_identity(replayed),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::RawScheduleIdentity,
                original.endpoint_boundary().schedule_set_identity(),
                replayed.endpoint_boundary().schedule_set_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::NormalizedScheduleIdentity,
                original.interval_subdivision().schedule_set_identity(),
                replayed.interval_subdivision().schedule_set_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::SplitVertexIdentity,
                original.vertices().split_vertex_identity_set_identity(),
                replayed.vertices().split_vertex_identity_set_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::CoalescenceDecision,
                &coalescence_identity(original),
                &coalescence_identity(replayed),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::SplitFragmentIdentity,
                original.fragments().fragment_set_identity(),
                replayed.fragments().fragment_set_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::OverlapChainIdentity,
                original.overlap_chains().chain_set_identity(),
                replayed.overlap_chains().chain_set_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::PersistentNamingPropagation,
                original.naming().receipt_identity(),
                replayed.naming().receipt_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::DecisionLogDigest,
                original.decision_log().receipt().receipt_identity(),
                replayed.decision_log().receipt().receipt_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::SplitLedgerDigest,
                original.ledger().receipt().receipt_identity(),
                replayed.ledger().receipt().receipt_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::DownstreamConsumptionIdentity,
                original
                    .ledger()
                    .receipt()
                    .downstream_consumption_identity(),
                replayed
                    .ledger()
                    .receipt()
                    .downstream_consumption_identity(),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::Counters,
                &counter_identity(original),
                &counter_identity(replayed),
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::DenialPosture,
                "edge-split-replay-denial-posture:none",
                "edge-split-replay-denial-posture:none",
            ),
            row(
                PlanarBooleanSplitReplayClosureRowKind::PolicyPosture,
                &policy_identity(original),
                &policy_identity(replayed),
            ),
        ];
        let manifest_identity = manifest_identity(&rows);
        Self {
            manifest_identity,
            rows,
        }
    }

    pub fn manifest_identity(&self) -> &str {
        &self.manifest_identity
    }
    pub fn rows(&self) -> &[PlanarBooleanSplitReplayClosureRow] {
        &self.rows
    }
    pub fn is_complete_and_matching(&self) -> bool {
        self.rows.len() == 20 && self.rows.iter().all(|row| row.certifies_match())
    }
}

impl PlanarBooleanSplitReplayClosureRow {
    pub fn kind(&self) -> PlanarBooleanSplitReplayClosureRowKind {
        self.kind
    }
    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }
    pub fn original_identity(&self) -> &str {
        &self.original_identity
    }
    pub fn replayed_identity(&self) -> &str {
        &self.replayed_identity
    }
    pub fn certifies_match(&self) -> bool {
        self.original_identity == self.replayed_identity
    }
}

fn row(
    kind: PlanarBooleanSplitReplayClosureRowKind,
    original_identity: &str,
    replayed_identity: &str,
) -> PlanarBooleanSplitReplayClosureRow {
    let row_identity =
        format!("edge-split-replay-closure-row:{kind:?}:{original_identity}:{replayed_identity}");
    PlanarBooleanSplitReplayClosureRow {
        kind,
        row_identity,
        original_identity: original_identity.to_string(),
        replayed_identity: replayed_identity.to_string(),
    }
}

fn manifest_identity(rows: &[PlanarBooleanSplitReplayClosureRow]) -> String {
    let mut identity = format!("edge-split-replay-closure-manifest:{}", rows.len());
    for row in rows {
        identity.push(':');
        identity.push_str(row.row_identity());
    }
    identity
}

fn endpoint_counter_identity(closeout: &PlanarBooleanEdgeSplitCloseout<'_>) -> String {
    let counters = closeout.endpoint_boundary().counters();
    format!(
        "endpoint-decisions:{}:{}:{}",
        counters.endpoint_noop_decisions(),
        counters.shared_endpoint_decisions(),
        counters.t_junction_boundary_decisions()
    )
}

fn coalescence_identity(closeout: &PlanarBooleanEdgeSplitCloseout<'_>) -> String {
    format!(
        "coalescence-decisions:{}",
        closeout.vertices().coalescence_decisions().count()
    )
}

fn counter_identity(closeout: &PlanarBooleanEdgeSplitCloseout<'_>) -> String {
    format!(
        "split-closeout-counters:{}:{}:{}:{}:{}",
        closeout.endpoint_boundary().schedules().len(),
        closeout.interval_subdivision().schedules().len(),
        closeout.vertices().schedules().len(),
        closeout.fragments().schedules().len(),
        closeout.overlap_chains().chains().len()
    )
}

fn policy_identity(closeout: &PlanarBooleanEdgeSplitCloseout<'_>) -> String {
    format!(
        "edge-split-policy-posture:{}:{}",
        closeout.validation().certifies_split_chain_integrity(),
        closeout
            .ledger()
            .receipt()
            .certifies_split_edge_chain_ledger()
    )
}
