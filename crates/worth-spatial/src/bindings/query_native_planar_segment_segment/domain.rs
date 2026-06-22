use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedSegmentSegment2DQueryDomain;

impl ForgeQueryDomainEntryMarker for CertifiedSegmentSegment2DQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.certified_segment_segment_2d"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialCertifiedSegmentSegment2DDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedSegmentSegment2DQueryWorld {
    identity: String,
}

impl CertifiedSegmentSegment2DQueryWorld {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>
    for CertifiedSegmentSegment2DQueryWorld
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
            "worth.spatial.certified_segment_segment_2d.{}",
            self.identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedSegmentSegment2DDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<CertifiedSegmentSegment2DQueryDomain>
    for CertifiedSegmentSegment2DDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "CertifiedSegmentSegment2D"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "geometry.segment_segment_2d.first_segment_identity",
                "geometry.segment_segment_2d.second_segment_identity",
                "geometry.segment_segment_2d.topology_basis",
                "geometry.segment_segment_2d.contact_policy",
                "geometry.segment_segment_2d.local_frame_fact",
                "geometry.segment_segment_2d.movement_rotation",
                "geometry.segment_segment_2d.tolerance_policy",
                "geometry.segment_segment_2d.endpoint.0.projection_fact",
                "geometry.segment_segment_2d.endpoint.1.projection_fact",
                "geometry.segment_segment_2d.endpoint.2.projection_fact",
                "geometry.segment_segment_2d.endpoint.3.projection_fact",
            ],
            &[
                "geometry.segment_segment_2d.classification",
                "geometry.segment_segment_2d.orientation.0.predicate_fact",
                "geometry.segment_segment_2d.orientation.1.predicate_fact",
                "geometry.segment_segment_2d.orientation.2.predicate_fact",
                "geometry.segment_segment_2d.orientation.3.predicate_fact",
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
