use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum GranularInvalidationScenario {
    CurveDetailToLiveRisk,
    SuppressedQuoteNoQueryPatch,
    OrderedPortfolioMembership,
    SharedLeaseDisclosureNoninterference,
    CorrespondenceRebindRestore,
    OpaqueRegionPlatformTwin,
}

impl GranularInvalidationScenario {
    pub(super) const ALL: [Self; 6] = [
        Self::CurveDetailToLiveRisk,
        Self::SuppressedQuoteNoQueryPatch,
        Self::OrderedPortfolioMembership,
        Self::SharedLeaseDisclosureNoninterference,
        Self::CorrespondenceRebindRestore,
        Self::OpaqueRegionPlatformTwin,
    ];

    pub(super) const fn name(self) -> &'static str {
        match self {
            Self::CurveDetailToLiveRisk => "curve_detail_to_live_risk",
            Self::SuppressedQuoteNoQueryPatch => "suppressed_quote_no_query_patch",
            Self::OrderedPortfolioMembership => "ordered_portfolio_membership",
            Self::SharedLeaseDisclosureNoninterference => "shared_lease_disclosure_noninterference",
            Self::CorrespondenceRebindRestore => "correspondence_rebind_restore",
            Self::OpaqueRegionPlatformTwin => "opaque_region_platform_twin",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclaredLocality {
    Unscoped,
    WholePartition(&'static str),
    ExactDetail(&'static str, &'static str),
}

impl DeclaredLocality {
    fn intersects(self, mutation: &GranularInvalidationMutation) -> bool {
        match self {
            Self::Unscoped => true,
            Self::WholePartition(partition) => partition == mutation.partition,
            Self::ExactDetail(partition, detail) => {
                partition == mutation.partition && Some(detail) == mutation.detail
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeclaredDependency {
    identity: &'static str,
    aspect: &'static str,
    locality: DeclaredLocality,
    relevant_field: &'static str,
    query_role: &'static str,
    maintenance_group: &'static str,
    consumer: &'static str,
    signal_tolerance: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GranularInvalidationWorldDefinition {
    scenario: GranularInvalidationScenario,
    dependencies: Vec<DeclaredDependency>,
}

impl GranularInvalidationWorldDefinition {
    pub(super) fn curve_detail_twins() -> Self {
        Self::for_scenario(GranularInvalidationScenario::CurveDetailToLiveRisk)
    }

    pub(super) fn for_scenario(scenario: GranularInvalidationScenario) -> Self {
        let dependencies = match scenario {
            GranularInvalidationScenario::CurveDetailToLiveRisk => curve_dependencies(false),
            GranularInvalidationScenario::OpaqueRegionPlatformTwin => curve_dependencies(true),
            GranularInvalidationScenario::SuppressedQuoteNoQueryPatch => vec![dependency(
                "quote-to-risk",
                "price",
                DeclaredLocality::Unscoped,
                "mid",
                "projected-value",
                "risk-value",
                "risk-live",
                5,
            )],
            GranularInvalidationScenario::OrderedPortfolioMembership => vec![
                dependency(
                    "portfolio-value",
                    "portfolio",
                    DeclaredLocality::Unscoped,
                    "pv",
                    "projected-value",
                    "portfolio-view",
                    "portfolio-live",
                    0,
                ),
                dependency(
                    "portfolio-membership",
                    "portfolio",
                    DeclaredLocality::Unscoped,
                    "desk",
                    "membership",
                    "portfolio-view",
                    "portfolio-live",
                    0,
                ),
                dependency(
                    "portfolio-order",
                    "portfolio",
                    DeclaredLocality::Unscoped,
                    "rank",
                    "ordering",
                    "portfolio-view",
                    "portfolio-live",
                    0,
                ),
                dependency(
                    "portfolio-window",
                    "portfolio",
                    DeclaredLocality::Unscoped,
                    "window",
                    "window",
                    "portfolio-view",
                    "portfolio-live",
                    0,
                ),
            ],
            GranularInvalidationScenario::SharedLeaseDisclosureNoninterference => vec![
                dependency(
                    "shared-public",
                    "curve",
                    DeclaredLocality::ExactDetail("usd-rates", "5y"),
                    "zero-rate",
                    "projected-value",
                    "shared-risk",
                    "public-consumer",
                    0,
                ),
                dependency(
                    "shared-governed",
                    "curve",
                    DeclaredLocality::ExactDetail("usd-rates", "5y"),
                    "zero-rate",
                    "disclosure",
                    "shared-risk",
                    "governed-consumer",
                    0,
                ),
            ],
            GranularInvalidationScenario::CorrespondenceRebindRestore => vec![dependency(
                "restored-risk",
                "curve",
                DeclaredLocality::ExactDetail("usd-rates", "5y"),
                "zero-rate",
                "projected-value",
                "restored-risk",
                "restored-consumer",
                0,
            )],
        };
        Self {
            scenario,
            dependencies,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GranularInvalidationMutation {
    aspect: &'static str,
    partition: &'static str,
    detail: Option<&'static str>,
    field: &'static str,
    magnitude: u64,
    current_authority: bool,
}

impl GranularInvalidationMutation {
    pub(super) fn curve_detail(partition: &'static str, detail: &'static str) -> Self {
        Self {
            aspect: "curve",
            partition,
            detail: Some(detail),
            field: "zero-rate",
            magnitude: 10,
            current_authority: true,
        }
    }

    pub(super) fn for_scenario(scenario: GranularInvalidationScenario) -> Self {
        match scenario {
            GranularInvalidationScenario::CurveDetailToLiveRisk
            | GranularInvalidationScenario::SharedLeaseDisclosureNoninterference
            | GranularInvalidationScenario::CorrespondenceRebindRestore => {
                Self::curve_detail("usd-rates", "5y")
            }
            GranularInvalidationScenario::OpaqueRegionPlatformTwin => Self {
                aspect: "opaque-a",
                partition: "region-7",
                detail: Some("scope-5"),
                field: "payload",
                magnitude: 10,
                current_authority: true,
            },
            GranularInvalidationScenario::SuppressedQuoteNoQueryPatch => Self {
                aspect: "price",
                partition: "quotes",
                detail: Some("instrument-17"),
                field: "mid",
                magnitude: 2,
                current_authority: true,
            },
            GranularInvalidationScenario::OrderedPortfolioMembership => Self {
                aspect: "portfolio",
                partition: "book-a",
                detail: Some("position-17"),
                field: "desk",
                magnitude: 1,
                current_authority: true,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CrossRuntimeInvalidationNecessityManifest {
    relational_changes: BTreeSet<String>,
    bridge_matches: BTreeSet<String>,
    signal_seeds: BTreeSet<String>,
    query_impacts: BTreeSet<String>,
    maintenance: BTreeSet<String>,
    deliveries: BTreeSet<String>,
    exclusions: BTreeSet<String>,
}

impl CrossRuntimeInvalidationNecessityManifest {
    pub(super) fn derive(
        world: &GranularInvalidationWorldDefinition,
        mutation: &GranularInvalidationMutation,
    ) -> Self {
        let relational_changes = BTreeSet::from([format!(
            "{}:{}:{}:{}",
            mutation.aspect,
            mutation.partition,
            mutation.detail.unwrap_or("*"),
            mutation.field
        )]);
        let mut manifest = Self {
            relational_changes,
            bridge_matches: BTreeSet::new(),
            signal_seeds: BTreeSet::new(),
            query_impacts: BTreeSet::new(),
            maintenance: BTreeSet::new(),
            deliveries: BTreeSet::new(),
            exclusions: BTreeSet::new(),
        };
        for dependency in &world.dependencies {
            let matches = mutation.current_authority
                && dependency.aspect == mutation.aspect
                && dependency.locality.intersects(mutation)
                && dependency.relevant_field == mutation.field;
            if !matches {
                manifest.exclusions.insert(dependency.identity.into());
                continue;
            }
            manifest.bridge_matches.insert(dependency.identity.into());
            if mutation.magnitude > dependency.signal_tolerance {
                manifest.signal_seeds.insert(dependency.identity.into());
                manifest
                    .query_impacts
                    .insert(format!("{}:{}", dependency.identity, dependency.query_role));
                manifest
                    .maintenance
                    .insert(dependency.maintenance_group.into());
                manifest.deliveries.insert(dependency.consumer.into());
            }
        }
        manifest
            .exclusions
            .insert(format!("scenario:{}", world.scenario.name()));
        manifest
    }

    pub(super) fn relational_changes(&self) -> &BTreeSet<String> {
        &self.relational_changes
    }
    pub(super) fn bridge_matches(&self) -> &BTreeSet<String> {
        &self.bridge_matches
    }
    pub(super) fn signal_seeds(&self) -> &BTreeSet<String> {
        &self.signal_seeds
    }
    pub(super) fn query_impacts(&self) -> &BTreeSet<String> {
        &self.query_impacts
    }
    pub(super) fn maintenance(&self) -> &BTreeSet<String> {
        &self.maintenance
    }
    pub(super) fn deliveries(&self) -> &BTreeSet<String> {
        &self.deliveries
    }
    pub(super) fn exclusions(&self) -> &BTreeSet<String> {
        &self.exclusions
    }
}

fn curve_dependencies(opaque: bool) -> Vec<DeclaredDependency> {
    let (aspect, partition, exact, sibling, field) = if opaque {
        ("opaque-a", "region-7", "scope-5", "scope-10", "payload")
    } else {
        ("curve", "usd-rates", "5y", "10y", "zero-rate")
    };
    vec![
        dependency(
            "risk-exact",
            aspect,
            DeclaredLocality::ExactDetail(partition, exact),
            field,
            "projected-value",
            "risk-exact",
            "risk-exact-consumer",
            0,
        ),
        dependency(
            "risk-partition",
            aspect,
            DeclaredLocality::WholePartition(partition),
            field,
            "projected-value",
            "risk-partition",
            "risk-partition-consumer",
            0,
        ),
        dependency(
            "risk-unscoped",
            aspect,
            DeclaredLocality::Unscoped,
            field,
            "projected-value",
            "risk-unscoped",
            "risk-unscoped-consumer",
            0,
        ),
        dependency(
            "risk-sibling",
            aspect,
            DeclaredLocality::ExactDetail(partition, sibling),
            field,
            "projected-value",
            "risk-sibling",
            "risk-sibling-consumer",
            0,
        ),
        dependency(
            "volatility",
            "volatility",
            DeclaredLocality::ExactDetail(partition, exact),
            field,
            "projected-value",
            "volatility",
            "vol-consumer",
            0,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn dependency(
    identity: &'static str,
    aspect: &'static str,
    locality: DeclaredLocality,
    relevant_field: &'static str,
    query_role: &'static str,
    maintenance_group: &'static str,
    consumer: &'static str,
    signal_tolerance: u64,
) -> DeclaredDependency {
    DeclaredDependency {
        identity,
        aspect,
        locality,
        relevant_field,
        query_role,
        maintenance_group,
        consumer,
        signal_tolerance,
    }
}
