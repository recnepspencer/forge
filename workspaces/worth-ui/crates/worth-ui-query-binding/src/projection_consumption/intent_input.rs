use std::sync::Arc;

use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

use super::{
    UiCollectionCompleteness, UiCollectionProjectionChange, UiCollectionProjectionDelivery,
    UiCollectionProjectionFactReceipt, UiCollectionProjectionRowReference, UiPresentProjection,
    UiProjectionAvailability, UiProjectionFactReceipt, UiProjectionFactStopKind,
    UiProjectionRetainedActivityKind, UiProjectionUnavailableKind, UiScalarProjectionFactReceipt,
};

#[path = "intent_input/collection_catalog.rs"]
mod collection_catalog;
#[path = "intent_input/collection_transition.rs"]
mod collection_transition;
#[path = "intent_input/transition_work.rs"]
mod transition_work;

use collection_catalog::UiProjectionInputCollectionCatalog;
pub use collection_transition::UiProjectionInputFactTransition;
pub use transition_work::UiProjectionInputTransitionWork;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiProjectionInputSlot(u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionInputRevision {
    inner: Arc<UiProjectionInputRevisionInner>,
}

#[derive(Debug, Eq, PartialEq)]
struct UiProjectionInputRevisionInner {
    slot: UiProjectionInputSlot,
    projection: crate::WorthUiQueryViewIdentity,
    observation_order: u64,
    query_world: WorthQueryEvidenceIdentity,
    binding: WorthQueryEvidenceIdentity,
    source_generation: WorthQueryEvidenceIdentity,
    result_generation: WorthQueryEvidenceIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionInputPosture {
    Current,
    RetainedStale(UiProjectionRetainedActivityKind),
    Unavailable(UiProjectionUnavailableKind),
    Stopped(UiProjectionFactStopKind),
    TransitionStopped(UiProjectionInputTransitionStopKind),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionInputTransitionStopKind {
    MissingPredecessor,
    ProjectionChanged,
    WrongShape,
    PredecessorNotCurrent,
    MalformedPatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiProjectionInputFactReference {
    Scalar(Arc<UiScalarProjectionInputFact>),
    Collection(Arc<UiCollectionProjectionInputFact>),
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiScalarProjectionInputFact {
    revision: UiProjectionInputRevision,
    posture: UiProjectionInputPosture,
    value: Option<Arc<str>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCollectionProjectionInputFact {
    revision: UiProjectionInputRevision,
    posture: UiProjectionInputPosture,
    completeness: Option<UiCollectionCompleteness>,
    catalog: Option<UiProjectionInputCollectionCatalog>,
    transition_work: UiProjectionInputTransitionWork,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionOptionReference {
    owner_revision: UiProjectionInputRevision,
    query_row_identity: Arc<WorthQueryEvidenceIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionInputCollectionRow {
    row: UiCollectionProjectionRowReference,
    selected_values: Box<[Arc<str>]>,
}

impl UiProjectionInputRevision {
    fn from_fact(slot: UiProjectionInputSlot, fact: &UiProjectionFactReceipt) -> Self {
        Self {
            inner: Arc::new(UiProjectionInputRevisionInner {
                slot,
                projection: fact.projection_identity().clone(),
                observation_order: fact.observation_order(),
                query_world: fact.query_world_identity().clone(),
                binding: fact.binding_identity().clone(),
                source_generation: fact.source_generation_identity().clone(),
                result_generation: fact.result_generation_identity().clone(),
            }),
        }
    }

    pub fn slot(&self) -> UiProjectionInputSlot {
        self.inner.slot
    }

    pub fn projection_identity(&self) -> &crate::WorthUiQueryViewIdentity {
        &self.inner.projection
    }

    pub fn observation_order(&self) -> u64 {
        self.inner.observation_order
    }

    pub fn query_world_identity_for_reporting(&self) -> &str {
        self.inner.query_world.terminal_projection_for_reporting()
    }

    pub fn binding_identity_for_reporting(&self) -> &str {
        self.inner.binding.terminal_projection_for_reporting()
    }

    pub fn source_generation_for_reporting(&self) -> &str {
        self.inner
            .source_generation
            .terminal_projection_for_reporting()
    }

    pub fn result_generation_for_reporting(&self) -> &str {
        self.inner
            .result_generation
            .terminal_projection_for_reporting()
    }

    pub(super) fn has_same_projection_owner(&self, other: &Self) -> bool {
        self.inner.slot == other.inner.slot
            && self.inner.projection == other.inner.projection
            && self.inner.query_world == other.inner.query_world
            && self.inner.binding == other.inner.binding
            && self.inner.observation_order < other.inner.observation_order
    }
}

impl UiProjectionInputFactReference {
    pub fn revision(&self) -> &UiProjectionInputRevision {
        match self {
            Self::Scalar(fact) => fact.revision(),
            Self::Collection(fact) => fact.revision(),
        }
    }

    pub fn posture(&self) -> UiProjectionInputPosture {
        match self {
            Self::Scalar(fact) => fact.posture(),
            Self::Collection(fact) => fact.posture(),
        }
    }
}

impl UiScalarProjectionInputFact {
    pub fn revision(&self) -> &UiProjectionInputRevision {
        &self.revision
    }

    pub fn posture(&self) -> UiProjectionInputPosture {
        self.posture
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn value_reference(&self) -> Option<Arc<str>> {
        self.value.as_ref().map(Arc::clone)
    }
}

impl UiCollectionProjectionInputFact {
    pub fn revision(&self) -> &UiProjectionInputRevision {
        &self.revision
    }

    pub fn posture(&self) -> UiProjectionInputPosture {
        self.posture
    }

    pub fn completeness(&self) -> Option<UiCollectionCompleteness> {
        self.completeness
    }

    pub fn row_count(&self) -> usize {
        self.catalog.as_ref().map_or(0, |catalog| catalog.len())
    }

    pub fn current_option(
        &self,
        row: &UiCollectionProjectionRowReference,
    ) -> Option<UiProjectionOptionReference> {
        if self.posture != UiProjectionInputPosture::Current {
            return None;
        }
        let catalog = self.catalog.as_ref()?;
        let (retained, _) = catalog.row(row.query_row_identity());
        retained.map(|retained| {
            UiProjectionOptionReference::query_issued(
                self.revision.clone(),
                retained.row().query_row_identity().clone(),
            )
        })
    }

    pub fn transition_work(&self) -> UiProjectionInputTransitionWork {
        self.transition_work
    }
}

impl UiProjectionOptionReference {
    fn query_issued(
        owner_revision: UiProjectionInputRevision,
        query_row_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            owner_revision,
            query_row_identity: Arc::new(query_row_identity),
        }
    }

    pub fn owner_revision(&self) -> &UiProjectionInputRevision {
        &self.owner_revision
    }

    pub fn identity_for_reporting(&self) -> &str {
        self.query_row_identity.terminal_projection_for_reporting()
    }
}

impl UiProjectionInputCollectionRow {
    pub fn row(&self) -> &UiCollectionProjectionRowReference {
        &self.row
    }

    pub fn selected_values(&self) -> &[Arc<str>] {
        &self.selected_values
    }
}

impl UiScalarProjectionFactReceipt {
    pub fn intent_input_transition(
        &self,
        slot: UiProjectionInputSlot,
    ) -> UiProjectionInputFactTransition {
        let revision = UiProjectionInputRevision::from_fact(slot, self.core());
        let (posture, value) = scalar_input(self.availability());
        UiProjectionInputFactTransition::replace(UiProjectionInputFactReference::Scalar(Arc::new(
            UiScalarProjectionInputFact {
                revision,
                posture,
                value,
            },
        )))
    }
}

impl UiCollectionProjectionFactReceipt {
    pub fn intent_input_transition(
        &self,
        slot: UiProjectionInputSlot,
    ) -> UiProjectionInputFactTransition {
        collection_transition::from_fact(self, slot)
    }
}

impl UiProjectionInputSlot {
    pub(crate) fn from_index(index: usize) -> Option<Self> {
        u32::try_from(index).ok().map(Self)
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

fn scalar_input(
    availability: &UiProjectionAvailability<super::UiNativeTextValue>,
) -> (UiProjectionInputPosture, Option<Arc<str>>) {
    match availability {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => (
            UiProjectionInputPosture::Current,
            Some(Arc::from(value.as_str())),
        ),
        UiProjectionAvailability::Present(UiPresentProjection::RetainedStale {
            value,
            activity,
        }) => (
            UiProjectionInputPosture::RetainedStale(activity.kind()),
            Some(Arc::from(value.as_str())),
        ),
        UiProjectionAvailability::Unavailable(receipt) => {
            (UiProjectionInputPosture::Unavailable(receipt.kind()), None)
        }
        UiProjectionAvailability::Stopped(receipt) => {
            (UiProjectionInputPosture::Stopped(receipt.kind()), None)
        }
    }
}

pub(super) fn collection_input(
    availability: &UiProjectionAvailability<super::UiCollectionProjectionValue>,
) -> (
    UiProjectionInputPosture,
    Option<UiCollectionCompleteness>,
    Box<[UiProjectionInputCollectionRow]>,
) {
    match availability {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => (
            UiProjectionInputPosture::Current,
            Some(value.completeness()),
            collection_rows(value),
        ),
        UiProjectionAvailability::Present(UiPresentProjection::RetainedStale {
            value,
            activity,
        }) => (
            UiProjectionInputPosture::RetainedStale(activity.kind()),
            Some(value.completeness()),
            collection_rows(value),
        ),
        UiProjectionAvailability::Unavailable(receipt) => (
            UiProjectionInputPosture::Unavailable(receipt.kind()),
            None,
            Box::default(),
        ),
        UiProjectionAvailability::Stopped(receipt) => (
            UiProjectionInputPosture::Stopped(receipt.kind()),
            None,
            Box::default(),
        ),
    }
}

fn collection_rows(
    value: &super::UiCollectionProjectionValue,
) -> Box<[UiProjectionInputCollectionRow]> {
    value
        .rows()
        .iter()
        .map(|row| UiProjectionInputCollectionRow {
            row: row.row().clone(),
            selected_values: row
                .selected_values()
                .iter()
                .map(|value| Arc::from(value.as_str()))
                .collect(),
        })
        .collect()
}
