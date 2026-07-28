use std::marker::PhantomData;

mod sealed {
    pub trait ArtifactPolicy {}
    pub trait PixelArtifactPolicy: ArtifactPolicy {}
}

pub trait UiVisualArtifactPolicy: sealed::ArtifactPolicy + 'static {
    type CapturedPosture: UiVisualArtifactPolicy;
    const PIXELS_REQUESTED: bool;
    const PIXELS_REQUIRED: bool;
}

pub trait SealedPixelArtifactPolicy: UiVisualArtifactPolicy + sealed::PixelArtifactPolicy {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiGeometryOnly(PhantomData<fn() -> fn()>);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiPixelsOptional(PhantomData<fn() -> fn()>);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiPixelsRequired(PhantomData<fn() -> fn()>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualCaptureDeadline {
    tick: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualCaptureCancellation {
    diagnostic_identity: u64,
}

pub struct UiVisualSnapshotRequest<Target, ArtifactPolicy: UiVisualArtifactPolicy> {
    target: Target,
    disclosure: super::UiVisualInspectionDisclosure,
    artifacts: ArtifactPolicy,
    deadline: Option<UiVisualCaptureDeadline>,
    cancellation: Option<UiVisualCaptureCancellation>,
}

impl sealed::ArtifactPolicy for UiGeometryOnly {}
impl sealed::ArtifactPolicy for UiPixelsOptional {}
impl sealed::ArtifactPolicy for UiPixelsRequired {}
impl sealed::PixelArtifactPolicy for UiPixelsOptional {}
impl sealed::PixelArtifactPolicy for UiPixelsRequired {}

impl UiVisualArtifactPolicy for UiGeometryOnly {
    type CapturedPosture = Self;
    const PIXELS_REQUESTED: bool = false;
    const PIXELS_REQUIRED: bool = false;
}

impl UiVisualArtifactPolicy for UiPixelsOptional {
    type CapturedPosture = Self;
    const PIXELS_REQUESTED: bool = true;
    const PIXELS_REQUIRED: bool = false;
}

impl UiVisualArtifactPolicy for UiPixelsRequired {
    type CapturedPosture = Self;
    const PIXELS_REQUESTED: bool = true;
    const PIXELS_REQUIRED: bool = true;
}

impl SealedPixelArtifactPolicy for UiPixelsOptional {}
impl SealedPixelArtifactPolicy for UiPixelsRequired {}

impl UiVisualCaptureDeadline {
    pub const fn at_tick(tick: u64) -> Self {
        Self { tick }
    }

    pub const fn tick(self) -> u64 {
        self.tick
    }
}

impl UiVisualCaptureCancellation {
    pub const fn new(diagnostic_identity: u64) -> Self {
        Self {
            diagnostic_identity,
        }
    }

    pub const fn diagnostic_identity(self) -> u64 {
        self.diagnostic_identity
    }
}

impl<Target> UiVisualSnapshotRequest<Target, UiGeometryOnly> {
    pub fn for_frame(target: Target, disclosure: super::UiVisualInspectionDisclosure) -> Self {
        Self {
            target,
            disclosure,
            artifacts: UiGeometryOnly::default(),
            deadline: None,
            cancellation: None,
        }
    }

    pub fn for_local_development_unredacted_frame(target: Target) -> Self {
        Self::for_frame(
            target,
            super::UiVisualInspectionDisclosure::local_development_unredacted(),
        )
    }
}

impl<Target, Policy: UiVisualArtifactPolicy> UiVisualSnapshotRequest<Target, Policy> {
    pub fn artifacts<NextPolicy: UiVisualArtifactPolicy>(
        self,
        artifacts: NextPolicy,
    ) -> UiVisualSnapshotRequest<Target, NextPolicy> {
        UiVisualSnapshotRequest {
            target: self.target,
            disclosure: self.disclosure,
            artifacts,
            deadline: self.deadline,
            cancellation: self.cancellation,
        }
    }

    pub fn deadline(mut self, deadline: UiVisualCaptureDeadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn cancellation(mut self, cancellation: UiVisualCaptureCancellation) -> Self {
        self.cancellation = Some(cancellation);
        self
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub const fn disclosure(&self) -> super::UiVisualInspectionDisclosure {
        self.disclosure
    }

    pub const fn artifact_policy(&self) -> &Policy {
        &self.artifacts
    }

    pub const fn capture_deadline(&self) -> Option<UiVisualCaptureDeadline> {
        self.deadline
    }

    pub const fn cancellation_posture(&self) -> Option<UiVisualCaptureCancellation> {
        self.cancellation
    }

    pub fn into_parts(
        self,
    ) -> (
        Target,
        super::UiVisualInspectionDisclosure,
        Policy,
        Option<UiVisualCaptureDeadline>,
        Option<UiVisualCaptureCancellation>,
    ) {
        (
            self.target,
            self.disclosure,
            self.artifacts,
            self.deadline,
            self.cancellation,
        )
    }
}

impl UiGeometryOnly {
    pub const fn policy() -> Self {
        Self(PhantomData)
    }
}

impl UiPixelsOptional {
    pub const fn policy() -> Self {
        Self(PhantomData)
    }
}

impl UiPixelsRequired {
    pub const fn policy() -> Self {
        Self(PhantomData)
    }
}
