use worth_query::facade::domain::WorthQueryDomainEvidenceBinding;

fn forge() -> WorthQueryDomainEvidenceBinding {
    WorthQueryDomainEvidenceBinding {
        operation_identity: "operation".into(),
        binding_identity: "binding".into(),
        run_identity: None,
        stage_identity: None,
        basis_identity: "basis".into(),
        execution_snapshot_identity: "snapshot".into(),
        output_occurrence_identity: "output".into(),
        execution_occurrence_identity: "occurrence".into(),
    }
}

fn main() {}
