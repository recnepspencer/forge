use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopClassifiedProductKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionLedgerRow {
    ledger_row_identity: String,
    canonical_loop_identity: String,
    tracked_loop_identity: String,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    source_loop_identities: Vec<String>,
    source_face_identities: Vec<String>,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    island_identities: Vec<String>,
    role_outcome_identity: String,
    degenerate_outcome_identity: String,
    propagated_persistent_name_identities: Vec<String>,
    propagated_signature_identities: Vec<String>,
    decision_identities: Vec<String>,
}

impl PlanarBooleanLoopReconstructionLedgerRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        ledger_row_identity: String,
        canonical_loop_identity: String,
        tracked_loop_identity: String,
        loop_kind: PlanarBooleanLoopClassifiedProductKind,
        source_loop_identities: Vec<String>,
        source_face_identities: Vec<String>,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        island_identities: Vec<String>,
        role_outcome_identity: String,
        degenerate_outcome_identity: String,
        propagated_persistent_name_identities: Vec<String>,
        propagated_signature_identities: Vec<String>,
        decision_identities: Vec<String>,
    ) -> Self {
        Self {
            ledger_row_identity,
            canonical_loop_identity,
            tracked_loop_identity,
            loop_kind,
            source_loop_identities,
            source_face_identities,
            fragment_identities,
            split_vertex_identities,
            island_identities,
            role_outcome_identity,
            degenerate_outcome_identity,
            propagated_persistent_name_identities,
            propagated_signature_identities,
            decision_identities,
        }
    }

    pub fn ledger_row_identity(&self) -> &str {
        &self.ledger_row_identity
    }

    pub fn canonical_loop_identity(&self) -> &str {
        &self.canonical_loop_identity
    }

    pub fn tracked_loop_identity(&self) -> &str {
        &self.tracked_loop_identity
    }

    pub fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn source_face_identities(&self) -> &[String] {
        &self.source_face_identities
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }

    pub fn island_identities(&self) -> &[String] {
        &self.island_identities
    }

    pub fn role_outcome_identity(&self) -> &str {
        &self.role_outcome_identity
    }

    pub fn degenerate_outcome_identity(&self) -> &str {
        &self.degenerate_outcome_identity
    }

    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }

    pub fn propagated_signature_identities(&self) -> &[String] {
        &self.propagated_signature_identities
    }

    pub fn decision_identities(&self) -> &[String] {
        &self.decision_identities
    }
}
