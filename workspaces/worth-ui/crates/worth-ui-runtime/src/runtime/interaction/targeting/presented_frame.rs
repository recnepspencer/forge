use worth_ui_host_contract::{
    UiHostObservationPresentationBasis, UiHostSurfaceCoordinateSpace, UiHostSurfaceCoordinateUnit,
    UiHostSurfacePosition, UiHostSurfacePositionBasis, UiMountedCoordinateSpace,
    UiMountedHitTestMechanic, UI_HOST_SURFACE_POSITION_SUBPIXELS_PER_UNIT,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPresentedTargetFrameRelation {
    Current,
    Retained,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInteractionTargetingDenial {
    ExpiredPresentation,
    UnknownPresentation,
    BindingNotPresented,
    PresentationEpochMismatch,
    UnsupportedPositionBasis(UiHostSurfacePositionBasis),
    IncompatibleHitTestCoordinateSpace { row: UiMountedCoordinateSpace },
    NoTarget { hit_test_rows_considered: usize },
    AmbiguousHitTestOrder { rank: u32 },
    SurfaceNoLongerBound,
    BindingNoLongerCurrent,
    MountedInstanceNoLongerCurrent,
    MountedSurfaceAffinityChanged,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiPresentedInteractionTarget {
    presentation: UiHostObservationPresentationBasis,
    relation: UiPresentedTargetFrameRelation,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    hit_test_order: u32,
    semantic_digest: u64,
    hit_test_rows_considered: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPresentedInteractionTargetView {
    presentation: UiHostObservationPresentationBasis,
    relation: UiPresentedTargetFrameRelation,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    hit_test_order: u32,
    semantic_digest: u64,
    hit_test_rows_considered: usize,
}

pub(crate) fn resolve_presented_target(
    mounted: &crate::mounting::WorthUiMountedSessionState,
    presentation: UiHostObservationPresentationBasis,
    position: UiHostSurfacePosition,
) -> Result<UiPresentedInteractionTarget, UiInteractionTargetingDenial> {
    require_viewport_logical(position.basis())?;
    let basis = mounted
        .interaction_hit_test_basis(presentation)
        .map_err(map_presentation_denial)?;
    debug_assert_eq!(basis.presentation(), presentation);
    let relation = map_relation(basis.relation());
    let rows = basis.rows();
    let point = canonical_point(position);
    let mut selected: Option<UiMountedHitTestMechanic> = None;
    for row in rows {
        if row.bounds().coordinate_space() != UiMountedCoordinateSpace::Viewport {
            return Err(
                UiInteractionTargetingDenial::IncompatibleHitTestCoordinateSpace {
                    row: row.bounds().coordinate_space(),
                },
            );
        }
        if !contains(row.bounds(), point) || !contains(row.clip_bounds(), point) {
            continue;
        }
        if let Some(current) = selected {
            if current.order() == row.order() {
                return Err(UiInteractionTargetingDenial::AmbiguousHitTestOrder {
                    rank: row.order().rank(),
                });
            }
            if current.order() < row.order() {
                continue;
            }
        }
        selected = Some(*row);
    }
    let row = selected.ok_or(UiInteractionTargetingDenial::NoTarget {
        hit_test_rows_considered: rows.len(),
    })?;
    let current = mounted
        .admit_current_hit_target(row)
        .map_err(map_current_affinity_denial)?;
    Ok(seal_target(presentation, relation, current, rows.len()))
}

fn require_viewport_logical(
    basis: UiHostSurfacePositionBasis,
) -> Result<(), UiInteractionTargetingDenial> {
    let supported = basis.coordinate_space() == UiHostSurfaceCoordinateSpace::Viewport
        && basis.coordinate_unit() == UiHostSurfaceCoordinateUnit::LogicalPoint;
    supported
        .then_some(())
        .ok_or(UiInteractionTargetingDenial::UnsupportedPositionBasis(
            basis,
        ))
}

fn canonical_point(position: UiHostSurfacePosition) -> [f32; 2] {
    let scale = UI_HOST_SURFACE_POSITION_SUBPIXELS_PER_UNIT as f64;
    [
        (position.x_subpixels() as f64 / scale) as f32,
        (position.y_subpixels() as f64 / scale) as f32,
    ]
}

fn contains(bounds: worth_ui_host_contract::UiMountedCanonicalBox, point: [f32; 2]) -> bool {
    point[0] >= bounds.x()
        && point[0] < bounds.x() + bounds.width()
        && point[1] >= bounds.y()
        && point[1] < bounds.y() + bounds.height()
}

fn seal_target(
    presentation: UiHostObservationPresentationBasis,
    relation: UiPresentedTargetFrameRelation,
    current: crate::mounting::UiCurrentHitTarget,
    hit_test_rows_considered: usize,
) -> UiPresentedInteractionTarget {
    let row = current.row();
    UiPresentedInteractionTarget {
        presentation,
        relation,
        surface: row.surface(),
        binding: row.binding(),
        mounted_instance: row.mounted_instance(),
        node_receipt: row.node_receipt(),
        hit_test_order: row.order().rank(),
        semantic_digest: row.semantic_digest(),
        hit_test_rows_considered,
    }
}

fn map_current_affinity_denial(
    denial: crate::mounting::UiCurrentHitTargetAffinityDenial,
) -> UiInteractionTargetingDenial {
    match denial {
        crate::mounting::UiCurrentHitTargetAffinityDenial::SurfaceNoLongerBound => {
            UiInteractionTargetingDenial::SurfaceNoLongerBound
        }
        crate::mounting::UiCurrentHitTargetAffinityDenial::BindingNoLongerCurrent => {
            UiInteractionTargetingDenial::BindingNoLongerCurrent
        }
        crate::mounting::UiCurrentHitTargetAffinityDenial::MountedInstanceNoLongerCurrent => {
            UiInteractionTargetingDenial::MountedInstanceNoLongerCurrent
        }
        crate::mounting::UiCurrentHitTargetAffinityDenial::MountedSurfaceAffinityChanged => {
            UiInteractionTargetingDenial::MountedSurfaceAffinityChanged
        }
    }
}

fn map_relation(
    relation: crate::mounting::UiPresentedFrameBasisRelation,
) -> UiPresentedTargetFrameRelation {
    match relation {
        crate::mounting::UiPresentedFrameBasisRelation::Current => {
            UiPresentedTargetFrameRelation::Current
        }
        crate::mounting::UiPresentedFrameBasisRelation::Retained => {
            UiPresentedTargetFrameRelation::Retained
        }
    }
}

fn map_presentation_denial(
    denial: crate::mounting::UiPresentedFrameBasisDenial,
) -> UiInteractionTargetingDenial {
    match denial {
        crate::mounting::UiPresentedFrameBasisDenial::Expired => {
            UiInteractionTargetingDenial::ExpiredPresentation
        }
        crate::mounting::UiPresentedFrameBasisDenial::Unknown => {
            UiInteractionTargetingDenial::UnknownPresentation
        }
        crate::mounting::UiPresentedFrameBasisDenial::BindingNotPresented => {
            UiInteractionTargetingDenial::BindingNotPresented
        }
        crate::mounting::UiPresentedFrameBasisDenial::PresentationEpochMismatch => {
            UiInteractionTargetingDenial::PresentationEpochMismatch
        }
        crate::mounting::UiPresentedFrameBasisDenial::InstanceNotPresented
        | crate::mounting::UiPresentedFrameBasisDenial::NodeReceiptMismatch => {
            unreachable!("target lookup classifies only frame presentation evidence")
        }
    }
}

impl UiPresentedInteractionTarget {
    pub const fn view(&self) -> UiPresentedInteractionTargetView {
        UiPresentedInteractionTargetView {
            presentation: self.presentation,
            relation: self.relation,
            surface: self.surface,
            binding: self.binding,
            mounted_instance: self.mounted_instance,
            node_receipt: self.node_receipt,
            hit_test_order: self.hit_test_order,
            semantic_digest: self.semantic_digest,
            hit_test_rows_considered: self.hit_test_rows_considered,
        }
    }

    pub const fn presentation(&self) -> UiHostObservationPresentationBasis {
        self.presentation
    }

    pub const fn frame_relation(&self) -> UiPresentedTargetFrameRelation {
        self.relation
    }

    pub const fn surface(&self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub const fn binding(&self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn mounted_instance(&self) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub const fn node_receipt(&self) -> worth_ui_host_contract::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }

    pub const fn hit_test_order(&self) -> u32 {
        self.hit_test_order
    }

    pub const fn semantic_digest(&self) -> u64 {
        self.semantic_digest
    }

    pub const fn hit_test_rows_considered(&self) -> usize {
        self.hit_test_rows_considered
    }
}

impl UiPresentedInteractionTargetView {
    pub const fn presentation(self) -> UiHostObservationPresentationBasis {
        self.presentation
    }

    pub const fn frame_relation(self) -> UiPresentedTargetFrameRelation {
        self.relation
    }

    pub const fn surface(self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub const fn binding(self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn mounted_instance(self) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub const fn node_receipt(self) -> worth_ui_host_contract::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }

    pub const fn hit_test_order(self) -> u32 {
        self.hit_test_order
    }

    pub const fn semantic_digest(self) -> u64 {
        self.semantic_digest
    }

    pub const fn hit_test_rows_considered(self) -> usize {
        self.hit_test_rows_considered
    }
}

pub(crate) fn require_current_target(
    mounted: &crate::mounting::WorthUiMountedSessionState,
    target: UiPresentedInteractionTargetView,
) -> Result<(), UiInteractionTargetingDenial> {
    admit_current_target(mounted, target).map(|_| ())
}

pub(crate) fn admit_current_target(
    mounted: &crate::mounting::WorthUiMountedSessionState,
    target: UiPresentedInteractionTargetView,
) -> Result<crate::mounting::UiCurrentInteractionAffinity, UiInteractionTargetingDenial> {
    mounted
        .admit_current_interaction_affinity(crate::mounting::UiMountedInteractionAffinityInput {
            surface: target.surface(),
            binding: target.binding(),
            mounted_instance: target.mounted_instance(),
            node_receipt: target.node_receipt(),
        })
        .map_err(map_current_affinity_denial)
}

#[cfg(test)]
pub(crate) const fn interaction_target_view_for_test(
    presentation: UiHostObservationPresentationBasis,
    affinity: crate::mounting::UiMountedInteractionAffinityInput,
) -> UiPresentedInteractionTargetView {
    UiPresentedInteractionTargetView {
        presentation,
        relation: UiPresentedTargetFrameRelation::Current,
        surface: affinity.surface,
        binding: affinity.binding,
        mounted_instance: affinity.mounted_instance,
        node_receipt: affinity.node_receipt,
        hit_test_order: 0,
        semantic_digest: 1,
        hit_test_rows_considered: 1,
    }
}
