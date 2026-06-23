#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphTouchReadVerb {
    ObservesCollection,
    ObservesRelationKind,
    ObservesAspect,
    ExposesDerivedTopology,
    MaterializesDiagnostic,
    RequiresPolicyBasis,
    RetainsLiveSubscription,
    CrossesOperatingWorld,
    ReadsStaleBasisAllowed,
}

impl ForgeQueryGraphTouchReadVerb {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ObservesCollection => "observes-collection",
            Self::ObservesRelationKind => "observes-relation-kind",
            Self::ObservesAspect => "observes-aspect",
            Self::ExposesDerivedTopology => "exposes-derived-topology",
            Self::MaterializesDiagnostic => "materializes-diagnostic",
            Self::RequiresPolicyBasis => "requires-policy-basis",
            Self::RetainsLiveSubscription => "retains-live-subscription",
            Self::CrossesOperatingWorld => "crosses-operating-world",
            Self::ReadsStaleBasisAllowed => "reads-stale-basis-allowed",
        }
    }
}
