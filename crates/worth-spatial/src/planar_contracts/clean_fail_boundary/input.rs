use crate::planar_contracts::admission::PlanarAdmissionRow;
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;

use super::{PlanarCleanFailAction, PlanarCleanFailClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarDirtyInputKind {
    SelfIntersectingLoop,
    NonManifoldWire,
    ThinWall,
    OrientationInconsistency,
}

impl PlanarDirtyInputKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfIntersectingLoop => "self-intersecting-loop",
            Self::NonManifoldWire => "non-manifold-wire",
            Self::ThinWall => "thin-wall",
            Self::OrientationInconsistency => "orientation-inconsistency",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarOpenInputKind {
    HalfSpaceGroup,
    OpenPlanarDomain,
}

impl PlanarOpenInputKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HalfSpaceGroup => "half-space-group",
            Self::OpenPlanarDomain => "open-planar-domain",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarCleanFailSourceDetail {
    Dirty(PlanarDirtyInputKind),
    UnboundedOrOpen(PlanarOpenInputKind),
}

impl PlanarCleanFailSourceDetail {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dirty(kind) => kind.as_str(),
            Self::UnboundedOrOpen(kind) => kind.as_str(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarCleanFailInput {
    class: PlanarCleanFailClass,
    action: PlanarCleanFailAction,
    source_digest: String,
    source_detail: PlanarCleanFailSourceDetail,
    stable_topology_identity: Option<String>,
    transform_posture_digest: Option<String>,
    admission_row: Option<PlanarAdmissionRow>,
}

impl PlanarCleanFailInput {
    pub fn dirty_planar_loop(source_digest: impl Into<String>) -> Self {
        Self::dirty_input(PlanarDirtyInputKind::SelfIntersectingLoop, source_digest)
    }

    pub fn non_manifold_wire(source_digest: impl Into<String>) -> Self {
        Self::dirty_input(PlanarDirtyInputKind::NonManifoldWire, source_digest)
    }

    pub fn thin_wall(source_digest: impl Into<String>) -> Self {
        Self::dirty_input(PlanarDirtyInputKind::ThinWall, source_digest)
    }

    pub fn orientation_inconsistency(source_digest: impl Into<String>) -> Self {
        Self::dirty_input(
            PlanarDirtyInputKind::OrientationInconsistency,
            source_digest,
        )
    }

    pub fn unbounded_half_space(source_digest: impl Into<String>) -> Self {
        Self::open_input(PlanarOpenInputKind::HalfSpaceGroup, source_digest)
    }

    pub fn open_planar_domain(source_digest: impl Into<String>) -> Self {
        Self::open_input(PlanarOpenInputKind::OpenPlanarDomain, source_digest)
    }

    pub fn with_topology_identity(mut self, identity: impl Into<String>) -> Self {
        self.stable_topology_identity = Some(identity.into());
        self
    }

    pub fn with_transform_posture(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.transform_posture_digest = Some(receipt.retained_motion_digest().to_string());
        self
    }

    pub fn with_admission_row(mut self, row: PlanarAdmissionRow) -> Self {
        self.admission_row = Some(row);
        self
    }

    pub fn class(&self) -> PlanarCleanFailClass {
        self.class
    }

    pub fn action(&self) -> PlanarCleanFailAction {
        self.action
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn source_detail(&self) -> &str {
        self.source_detail.as_str()
    }

    pub fn source_detail_kind(&self) -> PlanarCleanFailSourceDetail {
        self.source_detail
    }

    pub fn dirty_input_kind(&self) -> Option<PlanarDirtyInputKind> {
        match self.source_detail {
            PlanarCleanFailSourceDetail::Dirty(kind) => Some(kind),
            PlanarCleanFailSourceDetail::UnboundedOrOpen(_) => None,
        }
    }

    pub fn open_input_kind(&self) -> Option<PlanarOpenInputKind> {
        match self.source_detail {
            PlanarCleanFailSourceDetail::Dirty(_) => None,
            PlanarCleanFailSourceDetail::UnboundedOrOpen(kind) => Some(kind),
        }
    }

    pub fn stable_topology_identity(&self) -> Option<&str> {
        self.stable_topology_identity.as_deref()
    }

    pub fn transform_posture_digest(&self) -> Option<&str> {
        self.transform_posture_digest.as_deref()
    }

    pub fn admission_row(&self) -> Option<&PlanarAdmissionRow> {
        self.admission_row.as_ref()
    }

    fn dirty_input(kind: PlanarDirtyInputKind, source_digest: impl Into<String>) -> Self {
        Self {
            class: PlanarCleanFailClass::DirtyInput,
            action: PlanarCleanFailAction::InspectWithoutRepair,
            source_digest: source_digest.into(),
            source_detail: PlanarCleanFailSourceDetail::Dirty(kind),
            stable_topology_identity: None,
            transform_posture_digest: None,
            admission_row: None,
        }
    }

    fn open_input(kind: PlanarOpenInputKind, source_digest: impl Into<String>) -> Self {
        Self {
            class: PlanarCleanFailClass::UnboundedOrOpen,
            action: PlanarCleanFailAction::ClassifyWithoutBoundedConversion,
            source_digest: source_digest.into(),
            source_detail: PlanarCleanFailSourceDetail::UnboundedOrOpen(kind),
            stable_topology_identity: None,
            transform_posture_digest: None,
            admission_row: None,
        }
    }
}
