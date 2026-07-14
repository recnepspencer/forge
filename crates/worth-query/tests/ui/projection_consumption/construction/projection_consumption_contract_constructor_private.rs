use worth_query::facade::foundation::{BoundProjectionFactFamily, MaterializedProjectionContract, ProjectionContractSourcePosture, ProjectionContractSupportPosture, ProjectionSourceFamily, ProjectionSourceReferenceIdentity};

fn impossible<T>() -> T {
    panic!("fixture should fail before construction")
}

fn main() {
    let _ = MaterializedProjectionContract {
        declaration_digest: String::new(),
        eligibility_digest: String::new(),
        query_digest: None,
        basis_digest: None,
        result_digest: None,
        canonical_result_shape_digest: String::new(),
        narrowed_result_shape_digest: String::new(),
        authorized_projection_identity: String::new(),
        policy_digest: String::new(),
        tenant_schema_basis_digest: String::new(),
        source_family: ProjectionSourceFamily::QueryReadReceipt,
        source_posture: ProjectionContractSourcePosture::QueryOwnedReceiptSource,
        source_identity: String::new().into(),
        source_reference_identities: vec![impossible::<ProjectionSourceReferenceIdentity>()],
        fact_families: vec![impossible::<BoundProjectionFactFamily>()],
        support_posture: ProjectionContractSupportPosture::Admitted,
        contract_digest: String::new(),
    };
}
