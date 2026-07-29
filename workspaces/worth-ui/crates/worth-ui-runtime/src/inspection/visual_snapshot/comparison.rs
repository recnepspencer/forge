use std::marker::PhantomData;

use worth_ui_inspection::UiVisualArtifactPolicy;

use super::{UiVisualSnapshotComparisonGrant, UiVisualSnapshotReceipt};

pub struct UiUnbudgetedVisualSnapshotComparisonRequest<
    'receipt,
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
> {
    basis: UiVisualSnapshotComparisonBasis<'receipt, Predecessor, Successor>,
}

pub struct UiVisualSnapshotComparisonRequest<
    'receipt,
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
> {
    basis: UiVisualSnapshotComparisonBasis<'receipt, Predecessor, Successor>,
    budget: worth_ui_inspection::UiVisualSnapshotComparisonBudget,
}

struct UiVisualSnapshotComparisonBasis<
    'receipt,
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
> {
    predecessor: &'receipt UiVisualSnapshotReceipt<Predecessor>,
    successor: &'receipt UiVisualSnapshotReceipt<Successor>,
    rebind: UiVisualRebindComparisonEvidence,
    predecessor_overlay_clear: Option<super::UiClearedVisualOverlayReceipt>,
    pixel_policy: worth_ui_inspection::UiVisualComparisonPixelPolicy,
    _invariant: PhantomData<&'receipt mut &'receipt ()>,
}

#[derive(Clone, Copy)]
pub(crate) struct UiVisualRebindComparisonEvidence {
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    successor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    continuity: worth_ui_inspection::UiVisualIdentityContinuity,
}

impl UiVisualRebindComparisonEvidence {
    pub(crate) const fn new(
        session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
        predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
        successor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
        continuity: worth_ui_inspection::UiVisualIdentityContinuity,
    ) -> Self {
        Self {
            session,
            predecessor,
            successor,
            continuity,
        }
    }
}

pub(crate) fn new_unbudgeted_comparison_request<'receipt, Predecessor, Successor>(
    predecessor: &'receipt UiVisualSnapshotReceipt<Predecessor>,
    successor: &'receipt UiVisualSnapshotReceipt<Successor>,
    rebind: UiVisualRebindComparisonEvidence,
) -> UiUnbudgetedVisualSnapshotComparisonRequest<'receipt, Predecessor, Successor>
where
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
{
    UiUnbudgetedVisualSnapshotComparisonRequest {
        basis: UiVisualSnapshotComparisonBasis {
            predecessor,
            successor,
            rebind,
            predecessor_overlay_clear: None,
            pixel_policy: worth_ui_inspection::UiVisualComparisonPixelPolicy::Omit,
            _invariant: PhantomData,
        },
    }
}

impl<'receipt, Predecessor, Successor>
    UiUnbudgetedVisualSnapshotComparisonRequest<'receipt, Predecessor, Successor>
where
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
{
    pub const fn with_pixel_observation(
        mut self,
        policy: worth_ui_inspection::UiVisualComparisonPixelPolicy,
    ) -> Self {
        self.basis.pixel_policy = policy;
        self
    }

    pub const fn through_cleared_predecessor_overlay(
        mut self,
        cleared: super::UiClearedVisualOverlayReceipt,
    ) -> Self {
        self.basis.predecessor_overlay_clear = Some(cleared);
        self
    }

    pub const fn with_budget(
        self,
        budget: worth_ui_inspection::UiVisualSnapshotComparisonBudget,
    ) -> UiVisualSnapshotComparisonRequest<'receipt, Predecessor, Successor> {
        UiVisualSnapshotComparisonRequest {
            basis: self.basis,
            budget,
        }
    }
}

pub(crate) fn compare_visual_snapshots<Predecessor, Successor, Reserve, Reservation>(
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    grant: &UiVisualSnapshotComparisonGrant,
    request: UiVisualSnapshotComparisonRequest<'_, Predecessor, Successor>,
    reserve: Reserve,
) -> worth_ui_inspection::UiVisualSnapshotComparisonOutcome
where
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
    Reserve:
        FnOnce() -> Result<Reservation, worth_ui_inspection::UiVisualSnapshotComparisonOutcome>,
{
    let basis = request.basis;
    let affinities = match validate_comparison_basis(session, grant, &basis) {
        Ok(affinities) => affinities,
        Err(outcome) => return outcome,
    };
    let required = basis
        .predecessor
        .visible_region_count()
        .saturating_add(basis.successor.visible_region_count());
    let configured = request.budget.maximum_structural_entries();
    if required > configured {
        return worth_ui_inspection::UiVisualSnapshotComparisonOutcome::Denied(
            worth_ui_inspection::UiVisualSnapshotComparisonDenial::budget_exceeded(
                configured, required,
            ),
        );
    }
    let _reservation = match reserve() {
        Ok(reservation) => reservation,
        Err(outcome) => return outcome,
    };
    let pixels = compare_retained_pixels(&basis);
    worth_ui_inspection::UiVisualSnapshotComparisonOutcome::Compared(assemble_comparison(
        &basis, affinities, required, pixels,
    ))
}

fn validate_comparison_basis<Predecessor, Successor>(
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    grant: &UiVisualSnapshotComparisonGrant,
    basis: &UiVisualSnapshotComparisonBasis<'_, Predecessor, Successor>,
) -> Result<
    [worth_ui_inspection::UiVisualSnapshotAffinity; 2],
    worth_ui_inspection::UiVisualSnapshotComparisonOutcome,
>
where
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
{
    use worth_ui_inspection::{
        UiVisualSnapshotComparisonIncompatibility as Incompatible,
        UiVisualSnapshotComparisonOutcome as Outcome,
    };
    if grant.session() != session
        || basis.predecessor.session_identity() != session
        || basis.successor.session_identity() != session
        || basis.rebind.session != session
    {
        return Err(Outcome::Incompatible(Incompatible::ForeignSession));
    }
    if grant.scope().disclosure() != basis.predecessor.disclosure()
        || grant.scope().disclosure() != basis.successor.disclosure()
    {
        return Err(Outcome::Incompatible(Incompatible::DisclosureMismatch));
    }
    let Some(publication_successor) = basis.rebind.successor else {
        return Err(Outcome::Incompatible(
            Incompatible::RebindHasNoMountedPublication,
        ));
    };
    let predecessor_affinity = basis.predecessor.affinity();
    let successor_affinity = basis.successor.affinity();
    let publication_predecessor = basis.rebind.predecessor;
    if publication_predecessor.map(|frame| frame.diagnostic_value())
        != Some(predecessor_affinity.frame())
    {
        let Some(cleared) = basis.predecessor_overlay_clear else {
            return Err(Outcome::Incompatible(
                Incompatible::PredecessorFrameMismatch,
            ));
        };
        if cleared.session() != session
            || cleared.base_snapshot() != basis.predecessor.identity()
            || cleared.base_frame().diagnostic_value() != predecessor_affinity.frame()
            || Some(cleared.cleared_frame()) != publication_predecessor
        {
            return Err(Outcome::Incompatible(
                Incompatible::PredecessorLineageMismatch,
            ));
        }
    }
    if publication_successor.diagnostic_value() != successor_affinity.frame() {
        return Err(Outcome::Incompatible(Incompatible::SuccessorFrameMismatch));
    }
    if predecessor_affinity.semantic_surface() != successor_affinity.semantic_surface()
        || predecessor_affinity.host_surface() != successor_affinity.host_surface()
    {
        return Err(Outcome::Incompatible(Incompatible::SurfaceMismatch));
    }
    Ok([predecessor_affinity, successor_affinity])
}

fn assemble_comparison<Predecessor, Successor>(
    basis: &UiVisualSnapshotComparisonBasis<'_, Predecessor, Successor>,
    affinities: [worth_ui_inspection::UiVisualSnapshotAffinity; 2],
    structural_entries: usize,
    pixels: (Option<bool>, usize),
) -> worth_ui_inspection::UiVisualSnapshotComparison
where
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
{
    let [predecessor_affinity, successor_affinity] = affinities;
    worth_ui_inspection::UiVisualSnapshotComparison::from_runtime_projection(
        worth_ui_inspection::UiVisualSnapshotComparisonInput {
            snapshots: [
                predecessor_affinity.snapshot(),
                successor_affinity.snapshot(),
            ],
            frames: [predecessor_affinity.frame(), successor_affinity.frame()],
            bindings: [
                predecessor_affinity.binding_generation(),
                successor_affinity.binding_generation(),
            ],
            continuity: basis.rebind.continuity,
            visible_regions: [
                basis.predecessor.visible_region_count(),
                basis.successor.visible_region_count(),
            ],
            retained_pixels_differ: pixels.0,
            cost: [structural_entries, pixels.1],
        },
    )
}

fn compare_retained_pixels<Predecessor, Successor>(
    basis: &UiVisualSnapshotComparisonBasis<'_, Predecessor, Successor>,
) -> (Option<bool>, usize)
where
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
{
    if basis.pixel_policy == worth_ui_inspection::UiVisualComparisonPixelPolicy::Omit {
        return (None, 0);
    }
    let Some(predecessor) = basis.predecessor.retained_pixel_artifact() else {
        return (None, 0);
    };
    let Some(successor) = basis.successor.retained_pixel_artifact() else {
        return (None, 0);
    };
    let predecessor_bytes = predecessor.bytes();
    let successor_bytes = successor.bytes();
    let examined = predecessor_bytes
        .len()
        .saturating_add(successor_bytes.len());
    (
        Some(
            predecessor.dimensions() != successor.dimensions()
                || predecessor.stride() != successor.stride()
                || predecessor.format() != successor.format()
                || predecessor.color_space() != successor.color_space()
                || predecessor_bytes != successor_bytes,
        ),
        examined,
    )
}
