#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupRouteMismatch {
    RouteAuthorityIdentity { expected: String, actual: String },
    RouteFamilyIdentity { expected: String, actual: String },
    RightRouteFamilyIdentity { expected: String, actual: String },
    StageReceiptFamilyIdentity { expected: String, actual: String },
    RightStageReceiptIdentity { expected: String, actual: String },
    SelectedPlanIdentity { expected: String, actual: String },
    RightLookupExecutionReceiptDigest { expected: String, actual: String },
    CompiledProductIdentity { expected: String, actual: String },
    EquivalencePolicyIdentity { expected: String, actual: String },
    SelectedEquivalenceFamilyIdentity { expected: String, actual: String },
    SelectedEquivalenceBasisIdentity { expected: String, actual: String },
    SelectedCompatibilityBasisIdentity { expected: String, actual: String },
    SelectedReuseBasisIdentity { expected: String, actual: String },
    TopologySupportDigest { expected: String, actual: String },
    QuerySupportDigest { expected: String, actual: String },
    RightAuthorityStageIndexIdentity { expected: String, actual: String },
}
