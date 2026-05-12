use forge_query::facade::{
    BasisAuthorityPosture, BasisNormalizationCounters, BasisOperationLaneRequest,
    BasisTenantSchemaPosture, NormalizedBasisFamily, NormalizedBasisIntent, RawBasisSourcePath,
};

fn main() {
    let _ = NormalizedBasisIntent {
        raw_basis_intent_digest: String::new(),
        canonical_digest: String::new(),
        family: NormalizedBasisFamily::CurrentHead,
        authority_posture: BasisAuthorityPosture::RuntimeBackedCurrentHead,
        operation_lane: BasisOperationLaneRequest::Observation,
        tenant_scope: None,
        policy_scope: None,
        tenant_schema_posture: BasisTenantSchemaPosture::Unscoped,
        source_path: RawBasisSourcePath::DirectLifecycleConstructor,
        normalized_label: String::new(),
        counters: BasisNormalizationCounters {
            raw_intent_width: 1,
            normalized_family_count: 1,
            source_path_count: 1,
            rejection_width: 0,
        },
    };
}
