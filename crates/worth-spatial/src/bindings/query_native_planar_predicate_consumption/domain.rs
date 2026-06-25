use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PredicateCertificateConsumptionQueryDomain;

impl ForgeQueryDomainEntryMarker for PredicateCertificateConsumptionQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.predicate_certificate_consumption"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPredicateCertificateConsumptionDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredicateCertificateConsumptionQueryWorld {
    identity: String,
}

impl PredicateCertificateConsumptionQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PredicateCertificateConsumptionQueryDomain>
    for PredicateCertificateConsumptionQueryWorld
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "worth.spatial.predicate_certificate_consumption.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PredicateCertificateConsumptionDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PredicateCertificateConsumptionQueryDomain>
    for PredicateCertificateConsumptionDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PredicateCertificateConsumptionValidator"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.predicate_consumption.topology_basis",
                "geometry.predicate_consumption.movement_rotation",
                "geometry.predicate_consumption.local_frame",
                "geometry.predicate_consumption.predicate_receipts",
                "geometry.predicate_consumption.consumer_receipts",
            ],
            &[
                "geometry.predicate_consumption.certified_rows",
                "geometry.predicate_consumption.precision_metadata",
                "geometry.predicate_consumption.substitute_rejection",
            ],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}
