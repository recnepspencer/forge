use crate::declaration::{UiAspectName, UiDeclarationIdentity};
use crate::graph::{
    UiGraphGeneration, UiGraphLookup, UiGraphLookupReceipt, UiGraphNodeIdentity,
    UiGraphParticipationAxis, UiMountedReceiptIdentity,
};

use super::UiGraphEvidenceRef;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphInspectionTarget {
    GraphNode(UiGraphNodeIdentity),
    TopologyNode(UiGraphNodeIdentity),
    DeclarationInstances(UiDeclarationIdentity),
    ParentChild(UiGraphNodeIdentity),
    SlotOccupancy {
        parent_node_identity: UiGraphNodeIdentity,
        slot_name: Box<str>,
    },
    PageParticipation {
        page_node_identity: UiGraphNodeIdentity,
        axis: UiGraphParticipationAxis,
    },
    PublishedAspect(UiAspectName),
    ConsumedAspect(UiAspectName),
    MountedReceipt(UiMountedReceiptIdentity),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiGraphInspectionTargetKind {
    GraphNode,
    TopologyNode,
    DeclarationInstances,
    ParentChild,
    SlotOccupancy,
    PageParticipation,
    PublishedAspect,
    ConsumedAspect,
    MountedReceipt,
}

impl UiGraphInspectionTarget {
    pub const fn kind(&self) -> UiGraphInspectionTargetKind {
        match self {
            Self::GraphNode(_) => UiGraphInspectionTargetKind::GraphNode,
            Self::TopologyNode(_) => UiGraphInspectionTargetKind::TopologyNode,
            Self::DeclarationInstances(_) => UiGraphInspectionTargetKind::DeclarationInstances,
            Self::ParentChild(_) => UiGraphInspectionTargetKind::ParentChild,
            Self::SlotOccupancy { .. } => UiGraphInspectionTargetKind::SlotOccupancy,
            Self::PageParticipation { .. } => UiGraphInspectionTargetKind::PageParticipation,
            Self::PublishedAspect(_) => UiGraphInspectionTargetKind::PublishedAspect,
            Self::ConsumedAspect(_) => UiGraphInspectionTargetKind::ConsumedAspect,
            Self::MountedReceipt(_) => UiGraphInspectionTargetKind::MountedReceipt,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphInspection<T> {
    generation: UiGraphGeneration,
    target: UiGraphInspectionTarget,
    lookup: UiGraphLookup<T>,
    evidence_refs: Box<[UiGraphEvidenceRef]>,
}

impl<T> UiGraphInspection<T> {
    pub(crate) fn new(
        generation: UiGraphGeneration,
        target: UiGraphInspectionTarget,
        lookup: UiGraphLookup<T>,
        evidence_refs: Vec<UiGraphEvidenceRef>,
    ) -> Self {
        Self {
            generation,
            target,
            lookup,
            evidence_refs: evidence_refs.into_boxed_slice(),
        }
    }

    pub const fn generation(&self) -> UiGraphGeneration {
        self.generation
    }

    pub fn target(&self) -> &UiGraphInspectionTarget {
        &self.target
    }

    pub fn lookup_receipt(&self) -> UiGraphLookupReceipt {
        self.lookup.receipt()
    }

    pub fn value(&self) -> T
    where
        T: Clone,
    {
        self.lookup.value()
    }

    pub fn value_ref(&self) -> &T {
        self.lookup.value_ref()
    }

    pub fn evidence_refs(&self) -> &[UiGraphEvidenceRef] {
        &self.evidence_refs
    }
}
