use worth_query_host::facade::domain::{
    self, AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy,
    AspectIdentity, AspectKey, AspectMask, FieldDeclaration, FieldKey, FieldRequirement,
    ProjectionMask, ScalarAspectType, StructAspectShape,
};

pub(super) fn curve_contract() -> AspectContract {
    struct_contract(
        "CurveFacts",
        3,
        [
            ("CurvePartitionField", ScalarAspectType::String),
            ("CurveDetailField", ScalarAspectType::String),
            ("CurveZeroRateField", ScalarAspectType::UInt64),
        ],
    )
}

pub(super) fn volatility_contract() -> AspectContract {
    scalar_contract(
        "VolatilityFacts",
        8,
        "VolatilitySurfaceField",
        ScalarAspectType::UInt64,
    )
}

pub(super) fn audit_contract() -> AspectContract {
    scalar_contract("AuditFacts", 2, "AuditLabelField", ScalarAspectType::String)
}

pub(super) fn price_contract() -> AspectContract {
    scalar_contract("PriceFacts", 6, "QuoteMidField", ScalarAspectType::UInt64)
}

pub(super) fn risk_contract() -> AspectContract {
    scalar_contract("RiskFacts", 7, "RiskValueField", ScalarAspectType::UInt64)
}

pub(in crate::financial_runtime_world) fn portfolio_contract() -> AspectContract {
    struct_contract(
        "PortfolioFacts",
        5,
        [
            ("PortfolioValueField", ScalarAspectType::UInt64),
            ("PortfolioDeskField", ScalarAspectType::String),
            ("PortfolioRankField", ScalarAspectType::UInt64),
        ],
    )
}

pub(super) fn field_mask(field: &'static str) -> AspectMask<ProjectionMask> {
    AspectMask::new([domain::CanonicalFieldPath::single(
        FieldKey::new(field).unwrap(),
    )])
}

pub(super) fn projection_contract(
    contract: AspectContract,
    field: &'static str,
) -> domain::WorthQueryOperationNativeProjectionContract {
    domain::WorthQueryOperationNativeProjectionContract::new(contract, field_mask(field)).unwrap()
}

fn scalar_contract(
    aspect: &'static str,
    identity: u64,
    field: &'static str,
    family: ScalarAspectType,
) -> AspectContract {
    struct_contract(aspect, identity, [(field, family)])
}

fn struct_contract<const N: usize>(
    aspect: &'static str,
    identity: u64,
    fields: [(&'static str, ScalarAspectType); N],
) -> AspectContract {
    AspectContract::struct_aspect(
        AspectKey::new(aspect).unwrap(),
        AspectIdentity(identity),
        AspectContractRevision(1),
        StructAspectShape::new(fields.into_iter().map(|(field, family)| {
            FieldDeclaration::new(
                FieldKey::new(field).unwrap(),
                family,
                FieldRequirement::Required,
                AbsenceLaw::Required,
                AspectEvolutionPolicy::AdditiveFieldsAllowed,
            )
            .unwrap()
        }))
        .unwrap(),
    )
}
