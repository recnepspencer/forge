use crate::{
    AdmittedLayoutMaterialization, LayoutCoverageWitness, LayoutStrategyRegistrySnapshot,
    PhysicalArtifactFamily,
};

use super::{IndexMaintenanceFailureOutcome, IndexMaintenanceMode, IndexPublicationProtocol};

#[derive(Debug, Clone, Copy)]
pub struct LiveMaintenanceRequest<'a> {
    strategy: &'a LayoutStrategyRegistrySnapshot,
    materialization: &'a AdmittedLayoutMaterialization,
}

impl<'a> LiveMaintenanceRequest<'a> {
    pub const fn new(
        strategy: &'a LayoutStrategyRegistrySnapshot,
        materialization: &'a AdmittedLayoutMaterialization,
    ) -> Self {
        Self {
            strategy,
            materialization,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexLagWitness {
    family: PhysicalArtifactFamily,
    coverage: LayoutCoverageWitness,
    maintenance_mode: IndexMaintenanceMode,
}

impl IndexLagWitness {
    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }
    pub const fn coverage(&self) -> &LayoutCoverageWitness {
        &self.coverage
    }
    pub const fn maintenance_mode(&self) -> IndexMaintenanceMode {
        self.maintenance_mode
    }
}

macro_rules! define_maintenance_capability {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            family: PhysicalArtifactFamily,
            coverage: LayoutCoverageWitness,
        }

        impl $name {
            pub const fn family(&self) -> PhysicalArtifactFamily {
                self.family
            }
            pub const fn coverage(&self) -> &LayoutCoverageWitness {
                &self.coverage
            }
        }
    };
}

define_maintenance_capability!(RebuildOnlyMaintenanceCapability);
define_maintenance_capability!(LazyMaintenanceCapability);
define_maintenance_capability!(AdvisoryMaintenanceCapability);
define_maintenance_capability!(VerifierMaintenanceCapability);
define_maintenance_capability!(MigrationMaintenanceCapability);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredMaintenanceWitness {
    family: PhysicalArtifactFamily,
    coverage: LayoutCoverageWitness,
    reason: IndexMaintenanceFailureOutcome,
}

impl DeferredMaintenanceWitness {
    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }
    pub const fn coverage(&self) -> &LayoutCoverageWitness {
        &self.coverage
    }
    pub const fn reason(&self) -> IndexMaintenanceFailureOutcome {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveMaintenancePosture {
    Lagged(IndexLagWitness),
    RebuildOnly(RebuildOnlyMaintenanceCapability),
    Lazy(LazyMaintenanceCapability),
    Advisory(AdvisoryMaintenanceCapability),
    Verifier(VerifierMaintenanceCapability),
    Migration(MigrationMaintenanceCapability),
    Deferred(DeferredMaintenanceWitness),
}

impl LiveMaintenancePosture {
    pub const fn publication_protocol(&self) -> IndexPublicationProtocol {
        match self {
            Self::Lagged(_) => IndexPublicationProtocol::LsmManifestReplacement,
            Self::RebuildOnly(_) => IndexPublicationProtocol::DeferredUntilRebuild,
            Self::Lazy(_) => IndexPublicationProtocol::MaterializeOnDemand,
            Self::Advisory(_) => IndexPublicationProtocol::AdvisoryObservation,
            Self::Verifier(_) => IndexPublicationProtocol::VerificationObservation,
            Self::Migration(_) => IndexPublicationProtocol::MigrationCutover,
            Self::Deferred(_) => IndexPublicationProtocol::LsmManifestReplacement,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiveMaintenancePostureAdmission;

pub const fn live_maintenance_posture() -> LiveMaintenancePostureAdmission {
    LiveMaintenancePostureAdmission
}

impl LiveMaintenancePostureAdmission {
    pub fn classify(self, request: LiveMaintenanceRequest<'_>) -> LiveMaintenancePostureOutcome {
        let admitted_family = request.strategy.admitted_strategy().admitted_family();
        if admitted_family != request.materialization.family() {
            return LiveMaintenancePostureOutcome::denied(
                IndexMaintenanceFailureOutcome::FamilyBindingMismatch,
            );
        }
        let family = request.materialization.coverage().family();
        let coverage = request.materialization.coverage().clone();
        let mode = request.strategy.request().maintenance_mode();
        LiveMaintenancePostureOutcome::classified(match mode {
            IndexMaintenanceMode::SynchronousExact => {
                LiveMaintenancePosture::Deferred(DeferredMaintenanceWitness {
                    family,
                    coverage,
                    reason: IndexMaintenanceFailureOutcome::AwaitingExactPublication,
                })
            }
            IndexMaintenanceMode::AsynchronousLagged => {
                LiveMaintenancePosture::Lagged(IndexLagWitness {
                    family,
                    coverage,
                    maintenance_mode: mode,
                })
            }
            IndexMaintenanceMode::RebuildOnly => {
                LiveMaintenancePosture::RebuildOnly(RebuildOnlyMaintenanceCapability {
                    family,
                    coverage,
                })
            }
            IndexMaintenanceMode::LazyMaterializedOnDemand => {
                LiveMaintenancePosture::Lazy(LazyMaintenanceCapability { family, coverage })
            }
            IndexMaintenanceMode::AdvisoryOnly => {
                LiveMaintenancePosture::Advisory(AdvisoryMaintenanceCapability { family, coverage })
            }
            IndexMaintenanceMode::VerifierOnly => {
                LiveMaintenancePosture::Verifier(VerifierMaintenanceCapability { family, coverage })
            }
            IndexMaintenanceMode::MigrationOnly => {
                LiveMaintenancePosture::Migration(MigrationMaintenanceCapability {
                    family,
                    coverage,
                })
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LiveMaintenancePostureCase {
    Classified(Box<LiveMaintenancePosture>),
    Denied(IndexMaintenanceFailureOutcome),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveMaintenancePostureOutcome {
    case: LiveMaintenancePostureCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveMaintenancePostureView<'a> {
    Classified(&'a LiveMaintenancePosture),
    Denied(&'a IndexMaintenanceFailureOutcome),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiveMaintenancePostureCaseId(&'static str);

impl LiveMaintenancePostureCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn live_maintenance_posture_cases() -> impl Iterator<Item = LiveMaintenancePostureCaseId> {
    [
        LiveMaintenancePostureCaseId("layout.maintenance.posture.lagged"),
        LiveMaintenancePostureCaseId("layout.maintenance.posture.rebuild_only"),
        LiveMaintenancePostureCaseId("layout.maintenance.posture.lazy"),
        LiveMaintenancePostureCaseId("layout.maintenance.posture.advisory"),
        LiveMaintenancePostureCaseId("layout.maintenance.posture.verifier"),
        LiveMaintenancePostureCaseId("layout.maintenance.posture.migration"),
        LiveMaintenancePostureCaseId("layout.maintenance.posture.deferred"),
        LiveMaintenancePostureCaseId("layout.maintenance.posture.denied.family"),
    ]
    .into_iter()
}

impl LiveMaintenancePostureOutcome {
    fn classified(posture: LiveMaintenancePosture) -> Self {
        Self {
            case: LiveMaintenancePostureCase::Classified(Box::new(posture)),
        }
    }
    fn denied(denial: IndexMaintenanceFailureOutcome) -> Self {
        Self {
            case: LiveMaintenancePostureCase::Denied(denial),
        }
    }
    pub const fn view(&self) -> LiveMaintenancePostureView<'_> {
        match &self.case {
            LiveMaintenancePostureCase::Classified(posture) => {
                LiveMaintenancePostureView::Classified(posture)
            }
            LiveMaintenancePostureCase::Denied(denial) => {
                LiveMaintenancePostureView::Denied(denial)
            }
        }
    }
    pub fn case_id(&self) -> LiveMaintenancePostureCaseId {
        match &self.case {
            LiveMaintenancePostureCase::Classified(posture) => match posture.as_ref() {
                LiveMaintenancePosture::Lagged(_) => {
                    LiveMaintenancePostureCaseId("layout.maintenance.posture.lagged")
                }
                LiveMaintenancePosture::RebuildOnly(_) => {
                    LiveMaintenancePostureCaseId("layout.maintenance.posture.rebuild_only")
                }
                LiveMaintenancePosture::Lazy(_) => {
                    LiveMaintenancePostureCaseId("layout.maintenance.posture.lazy")
                }
                LiveMaintenancePosture::Advisory(_) => {
                    LiveMaintenancePostureCaseId("layout.maintenance.posture.advisory")
                }
                LiveMaintenancePosture::Verifier(_) => {
                    LiveMaintenancePostureCaseId("layout.maintenance.posture.verifier")
                }
                LiveMaintenancePosture::Migration(_) => {
                    LiveMaintenancePostureCaseId("layout.maintenance.posture.migration")
                }
                LiveMaintenancePosture::Deferred(_) => {
                    LiveMaintenancePostureCaseId("layout.maintenance.posture.deferred")
                }
            },
            LiveMaintenancePostureCase::Denied(_) => {
                LiveMaintenancePostureCaseId("layout.maintenance.posture.denied.family")
            }
        }
    }
    pub fn into_classified(self) -> Result<LiveMaintenancePosture, Self> {
        match self.case {
            LiveMaintenancePostureCase::Classified(posture) => Ok(*posture),
            case => Err(Self { case }),
        }
    }
}
