use std::sync::Arc;

use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

use super::{
    UiCollectionCompleteness, UiCollectionProjectionFactReceipt, UiPresentProjection,
    UiProjectionAvailability, UiProjectionFactReceipt, UiProjectionFactStopKind,
    UiProjectionRetainedActivityKind, UiProjectionUnavailableKind, UiScalarProjectionFactReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionInputRevision {
    inner: Arc<UiProjectionInputRevisionInner>,
}

#[derive(Debug, Eq, PartialEq)]
struct UiProjectionInputRevisionInner {
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
    rows: Box<[UiProjectionInputCollectionRow]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionOptionReference {
    owner_revision: UiProjectionInputRevision,
    query_row_identity: Arc<WorthQueryEvidenceIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionInputCollectionRow {
    option: UiProjectionOptionReference,
    selected_values: Box<[Arc<str>]>,
}

impl UiProjectionInputRevision {
    fn from_fact(fact: &UiProjectionFactReceipt) -> Self {
        Self {
            inner: Arc::new(UiProjectionInputRevisionInner {
                projection: fact.projection_identity().clone(),
                observation_order: fact.observation_order(),
                query_world: fact.query_world_identity().clone(),
                binding: fact.binding_identity().clone(),
                source_generation: fact.source_generation_identity().clone(),
                result_generation: fact.result_generation_identity().clone(),
            }),
        }
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

    pub fn rows(&self) -> &[UiProjectionInputCollectionRow] {
        &self.rows
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
    pub fn option(&self) -> &UiProjectionOptionReference {
        &self.option
    }

    pub fn selected_values(&self) -> &[Arc<str>] {
        &self.selected_values
    }
}

impl UiScalarProjectionFactReceipt {
    pub fn intent_input_reference(&self) -> UiProjectionInputFactReference {
        let revision = UiProjectionInputRevision::from_fact(self.core());
        let (posture, value) = scalar_input(self.availability());
        UiProjectionInputFactReference::Scalar(Arc::new(UiScalarProjectionInputFact {
            revision,
            posture,
            value,
        }))
    }
}

impl UiCollectionProjectionFactReceipt {
    pub fn intent_input_reference(&self) -> UiProjectionInputFactReference {
        let revision = UiProjectionInputRevision::from_fact(self.core());
        let (posture, completeness, rows) = collection_input(self.availability(), &revision);
        UiProjectionInputFactReference::Collection(Arc::new(UiCollectionProjectionInputFact {
            revision,
            posture,
            completeness,
            rows,
        }))
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

fn collection_input(
    availability: &UiProjectionAvailability<super::UiCollectionProjectionValue>,
    revision: &UiProjectionInputRevision,
) -> (
    UiProjectionInputPosture,
    Option<UiCollectionCompleteness>,
    Box<[UiProjectionInputCollectionRow]>,
) {
    match availability {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => (
            UiProjectionInputPosture::Current,
            Some(value.completeness()),
            collection_rows(value, revision),
        ),
        UiProjectionAvailability::Present(UiPresentProjection::RetainedStale {
            value,
            activity,
        }) => (
            UiProjectionInputPosture::RetainedStale(activity.kind()),
            Some(value.completeness()),
            collection_rows(value, revision),
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
    revision: &UiProjectionInputRevision,
) -> Box<[UiProjectionInputCollectionRow]> {
    value
        .rows()
        .iter()
        .map(|row| UiProjectionInputCollectionRow {
            option: UiProjectionOptionReference::query_issued(
                revision.clone(),
                row.row().query_row_identity().clone(),
            ),
            selected_values: row
                .selected_values()
                .iter()
                .map(|value| Arc::from(value.as_str()))
                .collect(),
        })
        .collect()
}
