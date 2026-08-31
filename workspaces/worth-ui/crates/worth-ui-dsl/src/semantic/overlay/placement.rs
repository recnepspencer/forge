use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiOverlayAnchor {
    SurfaceContent,
    Portal(super::UiPortalDeclarationId),
    Backdrop(super::UiBackdropIdentity),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiBackdropPlacement {
    AboveSurfaceContent,
    ImmediatelyBeforePortal(super::UiPortalDeclarationId),
    ImmediatelyAfterPortal(super::UiPortalDeclarationId),
    ImmediatelyBeforeBackdrop(super::UiBackdropIdentity),
    ImmediatelyAfterBackdrop(super::UiBackdropIdentity),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiOverlayRelationGraph {
    relations: Box<[UiOverlayRelation]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiOverlayRelationKind {
    Precedes,
    ImmediatelyPrecedes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiOverlayRelation {
    lower: UiOverlayAnchor,
    upper: UiOverlayAnchor,
    kind: UiOverlayRelationKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiOverlayRelationAdmissionDenial {
    BackdropCapacityExceeded,
    DuplicateParticipant,
    MissingAnchor,
    SelfRelation,
    Cycle,
    ConflictingImmediateAdjacency,
}

impl UiOverlayRelationGraph {
    pub fn admit(
        portals: impl IntoIterator<Item = super::UiPortalDeclarationId>,
        backdrops: impl IntoIterator<Item = (super::UiBackdropIdentity, UiBackdropPlacement)>,
    ) -> Result<Self, UiOverlayRelationAdmissionDenial> {
        let portals = portals.into_iter().collect::<Vec<_>>();
        let unique_portals = portals.iter().copied().collect::<BTreeSet<_>>();
        if unique_portals.len() != portals.len() {
            return Err(UiOverlayRelationAdmissionDenial::DuplicateParticipant);
        }
        let portals = unique_portals
            .into_iter()
            .map(UiOverlayAnchor::Portal)
            .collect::<BTreeSet<_>>();
        let backdrops = backdrops.into_iter().collect::<Vec<_>>();
        if backdrops.len() > crate::UI_APPEARANCE_BACKDROP_RELATION_CAPACITY {
            return Err(UiOverlayRelationAdmissionDenial::BackdropCapacityExceeded);
        }
        let mut nodes = BTreeSet::from([UiOverlayAnchor::SurfaceContent]);
        nodes.extend(portals.iter().copied());
        for (identity, _) in &backdrops {
            if !nodes.insert(UiOverlayAnchor::Backdrop(*identity)) {
                return Err(UiOverlayRelationAdmissionDenial::DuplicateParticipant);
            }
        }
        let mut edges = BTreeMap::<UiOverlayAnchor, BTreeSet<UiOverlayAnchor>>::new();
        let mut immediate_predecessors = BTreeMap::new();
        let mut immediate_successors = BTreeMap::new();
        let mut relations = Vec::new();
        for (identity, placement) in backdrops {
            let backdrop = UiOverlayAnchor::Backdrop(identity);
            let (before, after, kind) = relation(backdrop, placement);
            if before == after {
                return Err(UiOverlayRelationAdmissionDenial::SelfRelation);
            }
            if !nodes.contains(&before) || !nodes.contains(&after) {
                return Err(UiOverlayRelationAdmissionDenial::MissingAnchor);
            }
            if kind == UiOverlayRelationKind::ImmediatelyPrecedes
                && (immediate_successors.insert(before, after).is_some()
                    || immediate_predecessors.insert(after, before).is_some())
            {
                return Err(UiOverlayRelationAdmissionDenial::ConflictingImmediateAdjacency);
            }
            edges.entry(before).or_default().insert(after);
            relations.push(UiOverlayRelation {
                lower: before,
                upper: after,
                kind,
            });
        }
        ensure_acyclic(&nodes, &edges)?;
        relations.sort_by_key(|relation| (relation.lower, relation.upper));
        Ok(Self {
            relations: relations.into_boxed_slice(),
        })
    }

    pub fn relations(&self) -> &[UiOverlayRelation] {
        &self.relations
    }
}

impl UiOverlayRelation {
    pub const fn lower(self) -> UiOverlayAnchor {
        self.lower
    }
    pub const fn upper(self) -> UiOverlayAnchor {
        self.upper
    }
    pub const fn kind(self) -> UiOverlayRelationKind {
        self.kind
    }
}

impl UiBackdropPlacement {
    pub const fn portal_anchor(self) -> Option<super::UiPortalDeclarationId> {
        match self {
            Self::ImmediatelyBeforePortal(portal) | Self::ImmediatelyAfterPortal(portal) => {
                Some(portal)
            }
            Self::AboveSurfaceContent
            | Self::ImmediatelyBeforeBackdrop(_)
            | Self::ImmediatelyAfterBackdrop(_) => None,
        }
    }
}

fn relation(
    backdrop: UiOverlayAnchor,
    placement: UiBackdropPlacement,
) -> (UiOverlayAnchor, UiOverlayAnchor, UiOverlayRelationKind) {
    match placement {
        UiBackdropPlacement::AboveSurfaceContent => (
            UiOverlayAnchor::SurfaceContent,
            backdrop,
            UiOverlayRelationKind::Precedes,
        ),
        UiBackdropPlacement::ImmediatelyBeforePortal(portal) => (
            backdrop,
            UiOverlayAnchor::Portal(portal),
            UiOverlayRelationKind::ImmediatelyPrecedes,
        ),
        UiBackdropPlacement::ImmediatelyAfterPortal(portal) => (
            UiOverlayAnchor::Portal(portal),
            backdrop,
            UiOverlayRelationKind::ImmediatelyPrecedes,
        ),
        UiBackdropPlacement::ImmediatelyBeforeBackdrop(anchor) => (
            backdrop,
            UiOverlayAnchor::Backdrop(anchor),
            UiOverlayRelationKind::ImmediatelyPrecedes,
        ),
        UiBackdropPlacement::ImmediatelyAfterBackdrop(anchor) => (
            UiOverlayAnchor::Backdrop(anchor),
            backdrop,
            UiOverlayRelationKind::ImmediatelyPrecedes,
        ),
    }
}

fn ensure_acyclic(
    nodes: &BTreeSet<UiOverlayAnchor>,
    edges: &BTreeMap<UiOverlayAnchor, BTreeSet<UiOverlayAnchor>>,
) -> Result<(), UiOverlayRelationAdmissionDenial> {
    let mut incoming = nodes
        .iter()
        .map(|node| (*node, 0_usize))
        .collect::<BTreeMap<_, _>>();
    for successors in edges.values() {
        for successor in successors {
            *incoming.get_mut(successor).expect("validated node") += 1;
        }
    }
    let mut removed = BTreeSet::new();
    while removed.len() != nodes.len() {
        let available = incoming
            .iter()
            .filter_map(|(node, count)| (*count == 0 && !removed.contains(node)).then_some(*node))
            .collect::<Vec<_>>();
        if available.is_empty() {
            return Err(UiOverlayRelationAdmissionDenial::Cycle);
        }
        for selected in available {
            removed.insert(selected);
            if let Some(successors) = edges.get(&selected) {
                for successor in successors {
                    *incoming.get_mut(successor).expect("validated successor") -= 1;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relation_admission_rejects_cycles_and_preserves_partial_order() {
        let a = super::super::UiBackdropIdentity::new(1).unwrap();
        let b = super::super::UiBackdropIdentity::new(2).unwrap();
        assert_eq!(
            UiOverlayRelationGraph::admit(
                [],
                [
                    (a, UiBackdropPlacement::ImmediatelyBeforeBackdrop(b)),
                    (b, UiBackdropPlacement::ImmediatelyBeforeBackdrop(a))
                ]
            ),
            Err(UiOverlayRelationAdmissionDenial::Cycle)
        );
        assert_eq!(
            UiOverlayRelationGraph::admit(
                [],
                [
                    (a, UiBackdropPlacement::AboveSurfaceContent),
                    (b, UiBackdropPlacement::AboveSurfaceContent)
                ]
            )
            .unwrap()
            .relations()
            .len(),
            2
        );
        let portal = super::super::UiPortalDeclarationId::new(7).unwrap();
        assert_eq!(
            UiOverlayRelationGraph::admit([portal, portal], []),
            Err(UiOverlayRelationAdmissionDenial::DuplicateParticipant)
        );
        let c = super::super::UiBackdropIdentity::new(3).unwrap();
        assert_eq!(
            UiOverlayRelationGraph::admit(
                [portal],
                [
                    (a, UiBackdropPlacement::ImmediatelyBeforePortal(portal)),
                    (c, UiBackdropPlacement::ImmediatelyBeforePortal(portal)),
                ],
            ),
            Err(UiOverlayRelationAdmissionDenial::ConflictingImmediateAdjacency)
        );
    }

    #[test]
    fn relation_admission_enforces_the_backdrop_capacity() {
        let backdrops = (1..=4_097).map(|identity| {
            (
                super::super::UiBackdropIdentity::new(identity).unwrap(),
                UiBackdropPlacement::AboveSurfaceContent,
            )
        });
        assert_eq!(
            UiOverlayRelationGraph::admit([], backdrops),
            Err(UiOverlayRelationAdmissionDenial::BackdropCapacityExceeded)
        );
    }
}
