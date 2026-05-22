#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SpatialBlockedCapability {
    MergeBoolean,
    SubtractBoolean,
    CutOpening,
    Join,
    HostAttach,
}

impl SpatialBlockedCapability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MergeBoolean => "merge_boolean",
            Self::SubtractBoolean => "subtract_boolean",
            Self::CutOpening => "cut_opening",
            Self::Join => "join",
            Self::HostAttach => "host_attach",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentCandidateAvailability {
    Available,
    Blocked(SpatialBlockedCapability),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SpatialIntentCapabilitySummary {
    supported: Vec<SpatialBlockedCapability>,
    blocked: Vec<SpatialBlockedCapability>,
}

impl SpatialIntentCapabilitySummary {
    pub(crate) fn new(
        supported: Vec<SpatialBlockedCapability>,
        blocked: Vec<SpatialBlockedCapability>,
    ) -> Self {
        Self { supported, blocked }
    }

    pub fn supported(&self) -> &[SpatialBlockedCapability] {
        &self.supported
    }

    pub fn blocked(&self) -> &[SpatialBlockedCapability] {
        &self.blocked
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpatialIntentCapabilitySet {
    merge_boolean: bool,
    subtract_boolean: bool,
    cut_opening: bool,
    join: bool,
    host_attach: bool,
}

impl SpatialIntentCapabilitySet {
    pub fn blocked_defaults() -> Self {
        Self::default()
    }

    pub fn with_merge_boolean(self) -> Self {
        Self {
            merge_boolean: true,
            ..self
        }
    }

    pub fn with_subtract_boolean(self) -> Self {
        Self {
            subtract_boolean: true,
            ..self
        }
    }

    pub fn with_cut_opening(self) -> Self {
        Self {
            cut_opening: true,
            ..self
        }
    }

    pub fn with_join(self) -> Self {
        Self { join: true, ..self }
    }

    pub fn with_host_attach(self) -> Self {
        Self {
            host_attach: true,
            ..self
        }
    }

    pub fn availability_for(
        &self,
        capability: Option<SpatialBlockedCapability>,
    ) -> SpatialIntentCandidateAvailability {
        match capability {
            None => SpatialIntentCandidateAvailability::Available,
            Some(capability) if self.supports(capability) => {
                SpatialIntentCandidateAvailability::Available
            }
            Some(capability) => SpatialIntentCandidateAvailability::Blocked(capability),
        }
    }

    pub fn summary(&self) -> SpatialIntentCapabilitySummary {
        let all = [
            SpatialBlockedCapability::MergeBoolean,
            SpatialBlockedCapability::SubtractBoolean,
            SpatialBlockedCapability::CutOpening,
            SpatialBlockedCapability::Join,
            SpatialBlockedCapability::HostAttach,
        ];
        let (supported, blocked): (Vec<_>, Vec<_>) = all
            .into_iter()
            .partition(|capability| self.supports(*capability));
        SpatialIntentCapabilitySummary::new(supported, blocked)
    }

    fn supports(&self, capability: SpatialBlockedCapability) -> bool {
        match capability {
            SpatialBlockedCapability::MergeBoolean => self.merge_boolean,
            SpatialBlockedCapability::SubtractBoolean => self.subtract_boolean,
            SpatialBlockedCapability::CutOpening => self.cut_opening,
            SpatialBlockedCapability::Join => self.join,
            SpatialBlockedCapability::HostAttach => self.host_attach,
        }
    }
}
