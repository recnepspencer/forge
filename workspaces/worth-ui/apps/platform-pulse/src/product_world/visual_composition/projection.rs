use super::PlatformPulseLogicalRect;

/// Product visibility policy for targets backed by runtime-service stories.
/// It carries no provider authority and cannot make an unavailable story real.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformPulseServiceStoryGate {
    RealServiceProviderInstalled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformPulseProductRegion {
    IdentityMasthead,
    EvidenceRail,
    ServiceStage,
    TruthfulStatusBand,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformPulseProductFactSource {
    ApplicationIdentity,
    NativeProcess,
    SourceGeneration,
    QueryProjection,
    IntentPosture,
    IntentProvider,
    NativePublication,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseProductRegionContract {
    region: PlatformPulseProductRegion,
    authored_identity: &'static str,
    sources: &'static [PlatformPulseProductFactSource],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseProductTargetContract {
    authored_identity: &'static str,
    admitted_action_identity: &'static str,
    reference_bounds: PlatformPulseLogicalRect,
    visibility: PlatformPulseServiceStoryGate,
}

const MASTHEAD_SOURCES: &[PlatformPulseProductFactSource] = &[
    PlatformPulseProductFactSource::ApplicationIdentity,
    PlatformPulseProductFactSource::NativeProcess,
];
const RAIL_SOURCES: &[PlatformPulseProductFactSource] = &[
    PlatformPulseProductFactSource::SourceGeneration,
    PlatformPulseProductFactSource::QueryProjection,
    PlatformPulseProductFactSource::IntentPosture,
    PlatformPulseProductFactSource::NativePublication,
];
const STAGE_SOURCES: &[PlatformPulseProductFactSource] = &[
    PlatformPulseProductFactSource::QueryProjection,
    PlatformPulseProductFactSource::IntentProvider,
    PlatformPulseProductFactSource::NativeProcess,
];
const STATUS_SOURCES: &[PlatformPulseProductFactSource] =
    &[PlatformPulseProductFactSource::NativePublication];

impl PlatformPulseProductRegionContract {
    pub const ALL: [Self; 4] = [
        Self::new(
            PlatformPulseProductRegion::IdentityMasthead,
            "platform.pulse.region.identity_masthead",
            MASTHEAD_SOURCES,
        ),
        Self::new(
            PlatformPulseProductRegion::EvidenceRail,
            "platform.pulse.region.evidence_rail",
            RAIL_SOURCES,
        ),
        Self::new(
            PlatformPulseProductRegion::ServiceStage,
            "platform.pulse.region.service_stage",
            STAGE_SOURCES,
        ),
        Self::new(
            PlatformPulseProductRegion::TruthfulStatusBand,
            "platform.pulse.region.truthful_status_band",
            STATUS_SOURCES,
        ),
    ];

    const fn new(
        region: PlatformPulseProductRegion,
        authored_identity: &'static str,
        sources: &'static [PlatformPulseProductFactSource],
    ) -> Self {
        Self {
            region,
            authored_identity,
            sources,
        }
    }

    pub const fn region(self) -> PlatformPulseProductRegion {
        self.region
    }

    pub const fn authored_identity(self) -> &'static str {
        self.authored_identity
    }

    pub const fn sources(self) -> &'static [PlatformPulseProductFactSource] {
        self.sources
    }
}

impl PlatformPulseProductTargetContract {
    pub const RUN_LIVE_ACTION: Self = Self {
        authored_identity: "platform.pulse.target.run_live_action",
        admitted_action_identity: "intent:platform.pulse.action.route:activate",
        reference_bounds: PlatformPulseLogicalRect::new(296, 416, 216, 48),
        visibility: PlatformPulseServiceStoryGate::RealServiceProviderInstalled,
    };

    pub const CONFIRM_LIVE_ACTION: Self = Self {
        authored_identity: "platform.pulse.target.confirm_live_action",
        admitted_action_identity: "intent:platform.pulse.action.route:confirm",
        reference_bounds: PlatformPulseLogicalRect::new(680, 176, 232, 72),
        visibility: PlatformPulseServiceStoryGate::RealServiceProviderInstalled,
    };

    pub const OPEN_SERVICE_DETAILS: Self = Self {
        authored_identity: "platform.pulse.target.open_service_details",
        admitted_action_identity: "intent:platform.pulse.portal.open.route:activate",
        reference_bounds: PlatformPulseLogicalRect::new(528, 416, 112, 48),
        visibility: PlatformPulseServiceStoryGate::RealServiceProviderInstalled,
    };

    pub const fn authored_identity(self) -> &'static str {
        self.authored_identity
    }

    pub const fn admitted_action_identity(self) -> &'static str {
        self.admitted_action_identity
    }

    pub const fn reference_bounds(self) -> PlatformPulseLogicalRect {
        self.reference_bounds
    }

    pub const fn visibility(self) -> PlatformPulseServiceStoryGate {
        self.visibility
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staged_targets_require_exact_admitted_action_identity_and_minimum_size() {
        for target in [
            PlatformPulseProductTargetContract::RUN_LIVE_ACTION,
            PlatformPulseProductTargetContract::OPEN_SERVICE_DETAILS,
            PlatformPulseProductTargetContract::CONFIRM_LIVE_ACTION,
        ] {
            let extent = target.reference_bounds().extent();
            assert!(extent[0] >= 32 && extent[1] >= 32);
            assert!(target.admitted_action_identity().starts_with("intent:"));
            assert_eq!(
                target.visibility(),
                PlatformPulseServiceStoryGate::RealServiceProviderInstalled
            );
        }
    }
}
