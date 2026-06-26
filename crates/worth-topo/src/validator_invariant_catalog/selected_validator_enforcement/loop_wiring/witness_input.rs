use forge_relational::facade::identity::EntityId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLoopWiringWitnessInput {
    selected_obligation_digest: String,
    loop_rows: Vec<WorthTopologyLoopWiringLoopWitnessRow>,
    half_edge_rows: Vec<WorthTopologyLoopWiringHalfEdgeWitnessRow>,
    input_digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLoopWiringLoopWitnessRow {
    loop_id: EntityId,
    half_edge_ids: Vec<EntityId>,
    row_digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLoopWiringHalfEdgeWitnessRow {
    half_edge_id: EntityId,
    loop_id: Option<EntityId>,
    next_half_edge_id: Option<EntityId>,
    prev_half_edge_id: Option<EntityId>,
    row_digest: String,
}

impl WorthTopologyLoopWiringWitnessInput {
    pub(in crate::validator_invariant_catalog) fn from_selected_obligation_and_rows(
        selected_obligation_digest: impl Into<String>,
        loop_rows: impl IntoIterator<Item = WorthTopologyLoopWiringLoopWitnessRow>,
        half_edge_rows: impl IntoIterator<Item = WorthTopologyLoopWiringHalfEdgeWitnessRow>,
    ) -> Self {
        let selected_obligation_digest = selected_obligation_digest.into();
        let loop_rows = loop_rows.into_iter().collect::<Vec<_>>();
        let half_edge_rows = half_edge_rows.into_iter().collect::<Vec<_>>();
        let input_digest = input_digest(&selected_obligation_digest, &loop_rows, &half_edge_rows);
        Self {
            selected_obligation_digest,
            loop_rows,
            half_edge_rows,
            input_digest,
        }
    }

    pub fn selected_obligation_digest(&self) -> &str {
        &self.selected_obligation_digest
    }

    pub fn loop_rows(&self) -> &[WorthTopologyLoopWiringLoopWitnessRow] {
        &self.loop_rows
    }

    pub fn half_edge_rows(&self) -> &[WorthTopologyLoopWiringHalfEdgeWitnessRow] {
        &self.half_edge_rows
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }
}

impl WorthTopologyLoopWiringLoopWitnessRow {
    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn new(
        loop_id: EntityId,
        half_edge_ids: Vec<EntityId>,
    ) -> Self {
        let row_digest = format!(
            "worth-topo-loop-wiring-loop-witness-row-v1|{:?}|{}",
            loop_id,
            half_edge_ids
                .iter()
                .map(|id| format!("{id:?}"))
                .collect::<Vec<_>>()
                .join(",")
        );
        Self {
            loop_id,
            half_edge_ids,
            row_digest,
        }
    }

    pub const fn loop_id(&self) -> EntityId {
        self.loop_id
    }

    pub fn half_edge_ids(&self) -> &[EntityId] {
        &self.half_edge_ids
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl WorthTopologyLoopWiringHalfEdgeWitnessRow {
    #[cfg(test)]
    pub(in crate::validator_invariant_catalog) fn new(
        half_edge_id: EntityId,
        loop_id: Option<EntityId>,
        next_half_edge_id: Option<EntityId>,
        prev_half_edge_id: Option<EntityId>,
    ) -> Self {
        let row_digest = format!(
            "worth-topo-loop-wiring-half-edge-witness-row-v1|{:?}|{:?}|{:?}|{:?}",
            half_edge_id, loop_id, next_half_edge_id, prev_half_edge_id
        );
        Self {
            half_edge_id,
            loop_id,
            next_half_edge_id,
            prev_half_edge_id,
            row_digest,
        }
    }

    pub const fn half_edge_id(&self) -> EntityId {
        self.half_edge_id
    }

    pub const fn loop_id(&self) -> Option<EntityId> {
        self.loop_id
    }

    pub const fn next_half_edge_id(&self) -> Option<EntityId> {
        self.next_half_edge_id
    }

    pub const fn prev_half_edge_id(&self) -> Option<EntityId> {
        self.prev_half_edge_id
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn input_digest(
    selected_obligation_digest: &str,
    loop_rows: &[WorthTopologyLoopWiringLoopWitnessRow],
    half_edge_rows: &[WorthTopologyLoopWiringHalfEdgeWitnessRow],
) -> String {
    let mut parts = vec![
        "worth-topo-loop-wiring-witness-input-v1".to_string(),
        format!("selected-obligation:{selected_obligation_digest}"),
    ];
    parts.extend(
        loop_rows
            .iter()
            .map(|row| format!("loop:{}", row.row_digest())),
    );
    parts.extend(
        half_edge_rows
            .iter()
            .map(|row| format!("half-edge:{}", row.row_digest())),
    );
    parts.join("|")
}
