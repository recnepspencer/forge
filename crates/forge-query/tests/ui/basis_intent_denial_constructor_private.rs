use forge_query::facade::{
    BasisIntentDenial, BasisIntentDenialKind, BasisNormalizationCounters,
    BasisOperationLaneRequest, RawBasisSourcePath,
};

fn main() {
    let _ = BasisIntentDenial {
        raw_basis_intent_digest: String::new(),
        source_path: RawBasisSourcePath::DirectLifecycleConstructor,
        operation_lane: BasisOperationLaneRequest::Observation,
        kind: BasisIntentDenialKind::MalformedIdentifier {
            field: "tenant_scope",
        },
        counters: BasisNormalizationCounters {
            raw_intent_width: 1,
            normalized_family_count: 0,
            source_path_count: 1,
            rejection_width: 1,
        },
        failure_digest: String::new(),
    };
}
