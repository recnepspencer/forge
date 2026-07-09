use worth_store::{
    BackendCapabilityMatrixRow, S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind,
    StoreBackendCapabilityTier,
};

fn main() {
    let _row = BackendCapabilityMatrixRow {
        row_id: S0ArtifactRowId::new("InjectedRow").unwrap(),
        subject_kind: S0ArtifactSubjectKind::Backend,
        subject_path_or_symbol: "backend:injected".into(),
        classification: "injected".into(),
        evidence_refs: Vec::new(),
        forbidden_claims: Vec::new(),
        deferred_s_sequences: Vec::new(),
        status: S0ArtifactRowStatus::Admitted,
        notes: "WORTHd".into(),
        capability_tier: StoreBackendCapabilityTier::PlatformGrade,
        valid_use: "WORTHd".into(),
        required_evidence_before_promotion: Vec::new(),
        known_semantic_guarantees: Vec::new(),
        known_physical_gaps: Vec::new(),
    };
}
