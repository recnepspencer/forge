mod basis;

use basis::{
    UiIndexedVisualCaptureBasis, UiObservedHostVisualCaptureBasis, UiPinnedBasisAccess,
    UiPinnedVisualCaptureBasis, UiRequestedHostVisualCaptureBasis,
};

use basis::UiHostObservationWitness;
pub(crate) use basis::{UiValidatedHostVisualCapture, UiValidatedHostVisualCaptureInput};

pub(crate) struct UiVisualCaptureIntent<Target, Policy: worth_ui_inspection::UiVisualArtifactPolicy>
{
    settings: UiVisualCaptureSettings<Target, Policy>,
}

pub(crate) struct UiVisualCaptureSettings<
    Target,
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
> {
    deadline: Option<worth_ui_inspection::UiVisualCaptureDeadline>,
    disclosure: worth_ui_inspection::UiVisualInspectionDisclosure,
    _target: std::marker::PhantomData<fn() -> Target>,
    _policy: std::marker::PhantomData<Policy>,
}

pub(crate) trait UiVisualCapturePhase {
    type Basis;
}

pub(crate) enum UiAdmittedCapturePhase {}
pub(crate) enum UiPinnedCapturePhase {}
pub(crate) enum UiHostRequestedCapturePhase {}
pub(crate) enum UiHostObservedCapturePhase {}
pub(crate) enum UiIndexedCapturePhase {}

pub(crate) struct UiVisualCaptureStage<
    Target,
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
    Phase: UiVisualCapturePhase,
> {
    settings: UiVisualCaptureSettings<Target, Policy>,
    basis: Phase::Basis,
}

pub(crate) type UiAdmittedVisualCapture<Target, Policy> =
    UiVisualCaptureStage<Target, Policy, UiAdmittedCapturePhase>;
pub(crate) type UiPinnedVisualCapture<Target, Policy> =
    UiVisualCaptureStage<Target, Policy, UiPinnedCapturePhase>;
pub(crate) type UiRequestedHostVisualCapture<Target, Policy> =
    UiVisualCaptureStage<Target, Policy, UiHostRequestedCapturePhase>;
pub(crate) type UiObservedHostVisualCapture<Target, Policy> =
    UiVisualCaptureStage<Target, Policy, UiHostObservedCapturePhase>;
pub(crate) type UiIndexedVisualCapture<Target, Policy> =
    UiVisualCaptureStage<Target, Policy, UiIndexedCapturePhase>;

pub(crate) struct UiPinnedVisualCaptureInput {
    pub(crate) session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    pub(crate) capture_identity: u64,
    pub(crate) presentation: super::UiVisualSurfaceCaptureBasis,
    pub(crate) snapshot_lease: crate::mounting::UiMountedVisualSnapshotLease,
    pub(crate) visual_regions: crate::mounting::UiMountedVisualRegionBasis,
    pub(crate) identity_trace_basis: crate::mounting::UiMountedIdentityTraceBasis,
    pub(crate) registration: super::UiVisualCaptureRegistrationLease,
}

pub(crate) struct UiIndexedVisualCaptureParts<Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    pub(crate) disclosure: worth_ui_inspection::UiVisualInspectionDisclosure,
    pub(crate) capture_identity: u64,
    pub(crate) presentation: super::UiVisualSurfaceCaptureBasis,
    pub(crate) snapshot_lease: crate::mounting::UiMountedVisualSnapshotLease,
    pub(crate) identity_trace_basis: crate::mounting::UiMountedIdentityTraceBasis,
    pub(crate) registration: super::UiVisualCaptureRegistrationLease,
    pub(crate) host_request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    pub(crate) transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pub(crate) pixels: Option<worth_ui_inspection::UiVisualPixelArtifact>,
    pub(crate) visible_index: super::UiVisibleRegionIndex,
    pub(crate) hit_test_index: super::UiHitTestRegionIndex,
    pub(crate) spatial_cost: super::UiSpatialIndexBuildCost,
    pub(crate) _policy: std::marker::PhantomData<Policy>,
}

pub(crate) struct UiObservedHostVisualCaptureParts<Target, Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) requested: UiRequestedHostVisualCapture<Target, Policy>,
    observation: Option<worth_ui_host_contract::UiHostCaptureObservation>,
    observation_witness: UiHostObservationWitness,
}

impl UiVisualCapturePhase for UiAdmittedCapturePhase {
    type Basis = ();
}

impl UiVisualCapturePhase for UiPinnedCapturePhase {
    type Basis = UiPinnedVisualCaptureBasis;
}

impl UiVisualCapturePhase for UiHostRequestedCapturePhase {
    type Basis = UiRequestedHostVisualCaptureBasis;
}

impl UiVisualCapturePhase for UiHostObservedCapturePhase {
    type Basis = UiObservedHostVisualCaptureBasis;
}

impl UiVisualCapturePhase for UiIndexedCapturePhase {
    type Basis = UiIndexedVisualCaptureBasis;
}

impl<Target, Policy> UiVisualCaptureIntent<Target, Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) fn from_request(
        request: worth_ui_inspection::UiVisualSnapshotRequest<Target, Policy>,
    ) -> (Self, Target) {
        let (target, disclosure, _artifacts, deadline, _cancellation) = request.into_parts();
        (
            Self {
                settings: UiVisualCaptureSettings {
                    deadline,
                    disclosure,
                    _target: std::marker::PhantomData,
                    _policy: std::marker::PhantomData,
                },
            },
            target,
        )
    }

    pub(crate) const fn capture_deadline(
        &self,
    ) -> Option<worth_ui_inspection::UiVisualCaptureDeadline> {
        self.settings.deadline
    }

    pub(crate) fn admit(self) -> UiAdmittedVisualCapture<Target, Policy> {
        UiVisualCaptureStage {
            settings: self.settings,
            basis: (),
        }
    }
}

impl<Target, Policy> UiAdmittedVisualCapture<Target, Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) fn pin(
        self,
        input: UiPinnedVisualCaptureInput,
    ) -> UiPinnedVisualCapture<Target, Policy> {
        UiVisualCaptureStage {
            settings: self.settings,
            basis: UiPinnedVisualCaptureBasis {
                session: input.session,
                capture_identity: input.capture_identity,
                presentation: input.presentation,
                snapshot_lease: input.snapshot_lease,
                visual_regions: input.visual_regions,
                identity_trace_basis: input.identity_trace_basis,
                registration: input.registration,
            },
        }
    }
}

impl<Target, Policy> UiPinnedVisualCapture<Target, Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) fn request_host(
        self,
        host_request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> UiRequestedHostVisualCapture<Target, Policy> {
        UiVisualCaptureStage {
            settings: self.settings,
            basis: UiRequestedHostVisualCaptureBasis {
                pinned: self.basis,
                host_request,
            },
        }
    }
}

impl<Target, Policy> UiRequestedHostVisualCapture<Target, Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) const fn host_request(&self) -> worth_ui_host_contract::UiHostVisualCaptureRequest {
        self.basis.host_request
    }

    pub(crate) fn observe(
        self,
        observation: worth_ui_host_contract::UiHostCaptureObservation,
    ) -> UiObservedHostVisualCapture<Target, Policy> {
        UiVisualCaptureStage {
            settings: self.settings,
            basis: UiObservedHostVisualCaptureBasis {
                requested: self.basis,
                observation,
            },
        }
    }

    pub(crate) fn index_after_observation(
        self,
        observation_witness: UiHostObservationWitness,
        validated: UiValidatedHostVisualCapture,
    ) -> UiIndexedVisualCapture<Target, Policy> {
        UiVisualCaptureStage {
            settings: self.settings,
            basis: UiIndexedVisualCaptureBasis {
                requested: self.basis,
                _observation: observation_witness,
                validated,
            },
        }
    }
}

impl<Target, Policy> UiObservedHostVisualCapture<Target, Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) fn into_parts(self) -> UiObservedHostVisualCaptureParts<Target, Policy> {
        UiObservedHostVisualCaptureParts {
            requested: UiVisualCaptureStage {
                settings: self.settings,
                basis: self.basis.requested,
            },
            observation: Some(self.basis.observation),
            observation_witness: UiHostObservationWitness::issued_after_observation(),
        }
    }
}

impl<Target, Policy> UiIndexedVisualCapture<Target, Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) fn into_parts(self) -> UiIndexedVisualCaptureParts<Policy> {
        let requested = self.basis.requested;
        let pinned = requested.pinned;
        UiIndexedVisualCaptureParts {
            session: pinned.session,
            disclosure: self.settings.disclosure,
            capture_identity: pinned.capture_identity,
            presentation: pinned.presentation,
            snapshot_lease: pinned.snapshot_lease,
            identity_trace_basis: pinned.identity_trace_basis,
            registration: pinned.registration,
            host_request: requested.host_request,
            transform: self.basis.validated.transform,
            pixels: self.basis.validated.pixels,
            visible_index: self.basis.validated.visible_index,
            hit_test_index: self.basis.validated.hit_test_index,
            spatial_cost: self.basis.validated.spatial_cost,
            _policy: std::marker::PhantomData,
        }
    }
}

impl<Target, Policy> UiObservedHostVisualCaptureParts<Target, Policy>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
{
    pub(crate) fn take_observation(&mut self) -> worth_ui_host_contract::UiHostCaptureObservation {
        self.observation
            .take()
            .expect("host observation validation consumes the exact observation once")
    }

    pub(crate) fn index(
        self,
        validated: UiValidatedHostVisualCapture,
    ) -> UiIndexedVisualCapture<Target, Policy> {
        debug_assert!(
            self.observation.is_none(),
            "indexing follows consumption of the host observation"
        );
        self.requested
            .index_after_observation(self.observation_witness, validated)
    }
}

impl<Target, Policy, Phase> UiVisualCaptureStage<Target, Policy, Phase>
where
    Policy: worth_ui_inspection::UiVisualArtifactPolicy,
    Phase: UiVisualCapturePhase,
    Phase::Basis: UiPinnedBasisAccess,
{
    pub(crate) const fn capture_deadline(
        &self,
    ) -> Option<worth_ui_inspection::UiVisualCaptureDeadline> {
        self.settings.deadline
    }

    pub(crate) fn capture_identity(&self) -> u64 {
        self.basis.pinned().capture_identity
    }

    pub(crate) fn presentation(&self) -> super::UiVisualSurfaceCaptureBasis {
        self.basis.pinned().presentation
    }

    pub(crate) fn visual_regions(&self) -> &crate::mounting::UiMountedVisualRegionBasis {
        &self.basis.pinned().visual_regions
    }
}
