#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryConfigSectionFamily {
    Query,
    Relational,
    Signal,
    RuntimeBridge,
    Store,
}

impl WorthQueryConfigSectionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Relational => "relational",
            Self::Signal => "signal",
            Self::RuntimeBridge => "runtime_bridge",
            Self::Store => "store",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQuerySubsystemOwner {
    Query,
    Relational,
    Signal,
    RuntimeBridge,
    Store,
}

impl WorthQuerySubsystemOwner {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Relational => "relational",
            Self::Signal => "signal",
            Self::RuntimeBridge => "runtime_bridge",
            Self::Store => "store",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationAdmissionFailureClass {
    MissingRequiredSection,
    ContradictorySectionPosture,
    DeferredStoreBackedSection,
}

impl ConfigurationAdmissionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingRequiredSection => "missing_required_section",
            Self::ContradictorySectionPosture => "contradictory_section_posture",
            Self::DeferredStoreBackedSection => "deferred_store_backed_section",
        }
    }
}
