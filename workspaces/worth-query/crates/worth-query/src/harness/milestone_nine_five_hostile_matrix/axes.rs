#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MilestoneNineFivePerturbationClass {
    NamedScopeTableRetainedDerivedParity,
    TemplateDetailLiveArtifactParity,
    RetainedVsLiveProjectionContractDistinctness,
    GroupedViewFamilyPreservedReuseDistinctness,
    GroupedOrdinaryVsPreservedReuseDistinctness,
    PublicBridgeBootstrapFixedUnderTemplateComposition,
    GroupedPreservedReuseBasisErasureDenied,
    InspectorTargetPreservedReuseDowncastDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFiveFailureClass {
    PreservedReuseDriftDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFiveCompositionAxis {
    Direct,
    NamedScopeExpansion,
    TemplateInstantiation,
    BasisAwareComposition,
}

impl MilestoneNineFiveCompositionAxis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::NamedScopeExpansion => "named_scope_expansion",
            Self::TemplateInstantiation => "template_instantiation",
            Self::BasisAwareComposition => "basis_aware_composition",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFiveViewAxis {
    Detail,
    Table,
    KanbanGrouped,
}

impl MilestoneNineFiveViewAxis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Detail => "detail",
            Self::Table => "table",
            Self::KanbanGrouped => "kanban_grouped",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFiveProjectionAxis {
    RetainedDerivedArtifactBinding,
    LiveArtifactBinding,
    RelationalGroupedProjection,
}

impl MilestoneNineFiveProjectionAxis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RetainedDerivedArtifactBinding => "retained_derived_artifact_binding",
            Self::LiveArtifactBinding => "live_artifact_binding",
            Self::RelationalGroupedProjection => "relational_grouped_projection",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFiveReuseAxis {
    Ordinary,
    FuturePreserving,
}

impl MilestoneNineFiveReuseAxis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::FuturePreserving => "future_preserving",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFiveBootstrapAxis {
    RuntimeBackedDefaultFacade,
    PublicBridgeReadBootstrapContract,
}

impl MilestoneNineFiveBootstrapAxis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeBackedDefaultFacade => "runtime_backed_default_facade",
            Self::PublicBridgeReadBootstrapContract => "public_bridge_read_bootstrap_contract",
        }
    }
}
