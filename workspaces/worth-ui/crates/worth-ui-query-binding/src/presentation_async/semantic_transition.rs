use std::collections::{HashMap, HashSet};

use worth_ui_host_contract::{
    UiHostPresentationLineageIdentity, UiHostSurfaceIdentity, UiMountedFrameIdentity,
    UiMountedPaintCommandIdentity, UiSemanticSurfaceIdentity,
};

use super::{
    semantic_registry::{
        partitions::{
            currentness_partition, partition_for_empty, partition_for_mechanic,
            partition_for_removed_mechanic, PresentationPinPartitionIndex,
        },
        PresentationSemanticPublication,
    },
    WorthUiPresentationMechanicBasis, WorthUiPresentationRequestBasis,
    WorthUiPresentationSemanticChange,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) struct PresentationLineageKey {
    semantic_surface: UiSemanticSurfaceIdentity,
    host_lineage: UiHostPresentationLineageIdentity,
}

#[derive(Clone)]
pub(super) struct RetainedPresentationSemanticState {
    frame: UiMountedFrameIdentity,
    predecessor: Option<UiMountedFrameIdentity>,
    host_surface: UiHostSurfaceIdentity,
    dpi_milli: u32,
    mechanics: HashMap<UiMountedPaintCommandIdentity, WorthUiPresentationMechanicBasis>,
    pins: HashSet<super::WorthUiPresentationPinBasis>,
}

pub(super) struct PresentationSemanticTransition {
    successor: RetainedPresentationSemanticState,
    removed_mechanics: Box<[WorthUiPresentationMechanicBasis]>,
    pending_publications: Box<[PresentationSemanticPublication]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PresentationSemanticTransitionDenial {
    MissingBaseline,
    StalePredecessor,
    ForeignHostSurface,
    UnknownRemovedMechanic,
    UnknownReleasedPin,
}

impl PresentationLineageKey {
    pub(super) fn from_basis(basis: &WorthUiPresentationRequestBasis) -> Self {
        Self {
            semantic_surface: basis.semantic_surface(),
            host_lineage: basis.host_lineage(),
        }
    }
}

impl PresentationSemanticTransition {
    pub(super) fn plan(
        current: Option<&RetainedPresentationSemanticState>,
        basis: &WorthUiPresentationRequestBasis,
    ) -> Result<Self, PresentationSemanticTransitionDenial> {
        validate_predecessor(current, basis)?;
        Self::build(current, basis)
    }

    pub(super) fn plan_reconstruction(
        unresolved: &RetainedPresentationSemanticState,
        basis: &WorthUiPresentationRequestBasis,
    ) -> Result<Self, PresentationSemanticTransitionDenial> {
        if !basis.complete() || basis.predecessor() != unresolved.predecessor {
            return Err(PresentationSemanticTransitionDenial::StalePredecessor);
        }
        Self::build(Some(unresolved), basis)
    }

    fn build(
        current: Option<&RetainedPresentationSemanticState>,
        basis: &WorthUiPresentationRequestBasis,
    ) -> Result<Self, PresentationSemanticTransitionDenial> {
        let mut removed_mechanics = if basis.complete() {
            current
                .map(|state| state.mechanics.values().cloned().collect())
                .unwrap_or_default()
        } else {
            Vec::with_capacity(basis.removed_mechanics().len())
        };
        let mut mechanics = if basis.complete() {
            HashMap::new()
        } else {
            current
                .map(|state| state.mechanics.clone())
                .ok_or(PresentationSemanticTransitionDenial::MissingBaseline)?
        };
        for removed in basis.removed_mechanics() {
            if basis.complete() {
                if current.is_none_or(|state| !state.mechanics.contains_key(removed)) {
                    return Err(PresentationSemanticTransitionDenial::UnknownRemovedMechanic);
                }
                continue;
            }
            let Some(removed) = mechanics.remove(removed) else {
                return Err(PresentationSemanticTransitionDenial::UnknownRemovedMechanic);
            };
            removed_mechanics.push(removed);
        }
        for mechanic in basis.mechanics() {
            mechanics.insert(mechanic.mechanic(), mechanic.clone());
        }
        let prior_pins = current.map(|state| &state.pins);
        for released in basis.pin_releases() {
            if prior_pins.is_none_or(|pins| !pins.contains(released)) {
                return Err(PresentationSemanticTransitionDenial::UnknownReleasedPin);
            }
        }
        let successor = RetainedPresentationSemanticState {
            frame: basis.mounted_frame(),
            predecessor: basis.predecessor(),
            host_surface: basis.host_surface(),
            dpi_milli: basis.dpi_milli(),
            mechanics,
            pins: basis.binding_pins().iter().copied().collect(),
        };
        let mut pending_publications = changed_publications(current, &successor, basis);
        pending_publications.push(PresentationSemanticPublication::new(
            WorthUiPresentationSemanticChange::Currentness,
            vec![currentness_partition(basis)],
        ));
        Ok(Self {
            successor,
            removed_mechanics: removed_mechanics.into_boxed_slice(),
            pending_publications: pending_publications.into_boxed_slice(),
        })
    }

    pub(super) fn successor(&self) -> &RetainedPresentationSemanticState {
        &self.successor
    }

    pub(super) fn pending_publications(&self) -> &[PresentationSemanticPublication] {
        &self.pending_publications
    }

    pub(super) fn removed_mechanics(&self) -> &[WorthUiPresentationMechanicBasis] {
        &self.removed_mechanics
    }
}

impl RetainedPresentationSemanticState {
    pub(super) fn mechanics(
        &self,
    ) -> &HashMap<UiMountedPaintCommandIdentity, WorthUiPresentationMechanicBasis> {
        &self.mechanics
    }

    pub(super) const fn dpi_milli(&self) -> u32 {
        self.dpi_milli
    }
}

fn validate_predecessor(
    current: Option<&RetainedPresentationSemanticState>,
    basis: &WorthUiPresentationRequestBasis,
) -> Result<(), PresentationSemanticTransitionDenial> {
    let Some(current) = current else {
        if basis.predecessor().is_some() || !basis.complete() {
            return Err(PresentationSemanticTransitionDenial::MissingBaseline);
        }
        return Ok(());
    };
    if basis.predecessor() != Some(current.frame) {
        return Err(PresentationSemanticTransitionDenial::StalePredecessor);
    }
    if basis.host_surface() != current.host_surface {
        return Err(PresentationSemanticTransitionDenial::ForeignHostSurface);
    }
    Ok(())
}

fn changed_publications(
    current: Option<&RetainedPresentationSemanticState>,
    successor: &RetainedPresentationSemanticState,
    basis: &WorthUiPresentationRequestBasis,
) -> Vec<PresentationSemanticPublication> {
    let current_mechanics = current.map(|state| &state.mechanics);
    let mut publications = Vec::with_capacity(5);
    append_mechanic_publication(
        &mut publications,
        WorthUiPresentationSemanticChange::Content,
        current_mechanics,
        &successor.mechanics,
        basis,
        |mechanic| (mechanic.content_generation(), mechanic.content().to_owned()),
    );
    append_mechanic_publication(
        &mut publications,
        WorthUiPresentationSemanticChange::Width,
        current_mechanics,
        &successor.mechanics,
        basis,
        |mechanic| {
            (
                mechanic.layout_width(),
                mechanic.layout_request(),
                mechanic.layout(),
            )
        },
    );
    append_mechanic_publication(
        &mut publications,
        WorthUiPresentationSemanticChange::PaintValue,
        current_mechanics,
        &successor.mechanics,
        basis,
        |mechanic| {
            mechanic
                .paint_spans()
                .iter()
                .map(|span| (span.identity(), span.foreground()))
                .collect::<Vec<_>>()
        },
    );
    append_mechanic_publication(
        &mut publications,
        WorthUiPresentationSemanticChange::PaintBoundary,
        current_mechanics,
        &successor.mechanics,
        basis,
        |mechanic| {
            mechanic
                .paint_spans()
                .iter()
                .map(|span| (span.identity(), span.original_range()))
                .collect::<Vec<_>>()
        },
    );
    let dpi_changed = current.is_none_or(|state| state.dpi_milli != successor.dpi_milli);
    let mut dpi_targets = changed_mechanics(
        current_mechanics,
        &successor.mechanics,
        WorthUiPresentationMechanicBasis::text_scale,
    );
    if dpi_changed {
        dpi_targets = successor.mechanics.values().collect();
    }
    if current.is_none() && successor.mechanics.is_empty() {
        publications.push(PresentationSemanticPublication::new(
            WorthUiPresentationSemanticChange::Dpi,
            vec![partition_for_empty(
                basis,
                WorthUiPresentationSemanticChange::Dpi,
            )],
        ));
    } else if !dpi_targets.is_empty() {
        publications.push(publication_for_targets(
            WorthUiPresentationSemanticChange::Dpi,
            dpi_targets,
            basis,
            &successor.mechanics,
        ));
    }
    publications
}

fn append_mechanic_publication<T: PartialEq>(
    publications: &mut Vec<PresentationSemanticPublication>,
    change: WorthUiPresentationSemanticChange,
    current: Option<&HashMap<UiMountedPaintCommandIdentity, WorthUiPresentationMechanicBasis>>,
    successor: &HashMap<UiMountedPaintCommandIdentity, WorthUiPresentationMechanicBasis>,
    basis: &WorthUiPresentationRequestBasis,
    select: impl Fn(&WorthUiPresentationMechanicBasis) -> T,
) {
    let targets = changed_mechanics(current, successor, select);
    if current.is_none() && successor.is_empty() {
        publications.push(PresentationSemanticPublication::new(
            change,
            vec![partition_for_empty(basis, change)],
        ));
    } else if !targets.is_empty() {
        publications.push(publication_for_targets(change, targets, basis, successor));
    }
}

fn changed_mechanics<'a, T: PartialEq>(
    current: Option<&'a HashMap<UiMountedPaintCommandIdentity, WorthUiPresentationMechanicBasis>>,
    successor: &'a HashMap<UiMountedPaintCommandIdentity, WorthUiPresentationMechanicBasis>,
    select: impl Fn(&WorthUiPresentationMechanicBasis) -> T,
) -> Vec<&'a WorthUiPresentationMechanicBasis> {
    let Some(current) = current else {
        return successor.values().collect();
    };
    let mut targets = successor
        .iter()
        .filter_map(|(identity, candidate)| {
            current
                .get(identity)
                .is_none_or(|prior| select(prior) != select(candidate))
                .then_some(candidate)
        })
        .collect::<Vec<_>>();
    targets.extend(
        current
            .iter()
            .filter(|(identity, _)| !successor.contains_key(identity))
            .map(|(_, mechanic)| mechanic),
    );
    targets
}

fn publication_for_targets(
    change: WorthUiPresentationSemanticChange,
    targets: Vec<&WorthUiPresentationMechanicBasis>,
    basis: &WorthUiPresentationRequestBasis,
    successor: &HashMap<UiMountedPaintCommandIdentity, WorthUiPresentationMechanicBasis>,
) -> PresentationSemanticPublication {
    let pin_index = PresentationPinPartitionIndex::from_basis(basis);
    let mut unique = HashSet::with_capacity(targets.len());
    let partitions = targets
        .into_iter()
        .map(|mechanic| {
            if successor.contains_key(&mechanic.mechanic()) {
                partition_for_mechanic(basis, mechanic, change, &pin_index)
            } else {
                partition_for_removed_mechanic(basis, mechanic, change, &pin_index)
            }
        })
        .filter(|partition| unique.insert(partition.clone()))
        .collect();
    PresentationSemanticPublication::new(change, partitions)
}
