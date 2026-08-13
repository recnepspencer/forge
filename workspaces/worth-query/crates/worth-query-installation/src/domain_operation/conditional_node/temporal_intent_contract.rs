use std::marker::PhantomData;

use super::WorthQueryClockCoordinate;

type TemporalIntentMarker<Clock, Input> = fn(Input) -> Clock;

pub const MAX_TEMPORAL_INTENT_RECONSTRUCTION_ROWS: usize = 100_000;
pub const MAX_TEMPORAL_INTENT_QUERY_WORK: usize = 10_000_000;
pub const MAX_TEMPORAL_DUE_WAKES_PER_OBSERVATION: usize = 10_000;

/// Explicit work limits for reconstruction and ordinary due-wake fan-out.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryTemporalIntentBounds {
    maximum_reconstruction_rows: usize,
    maximum_query_work: usize,
    maximum_due_wakes_per_observation: usize,
}

impl WorthQueryTemporalIntentBounds {
    pub fn new(
        maximum_reconstruction_rows: usize,
        maximum_query_work: usize,
        maximum_due_wakes_per_observation: usize,
    ) -> Result<Self, &'static str> {
        let bounds = Self {
            maximum_reconstruction_rows,
            maximum_query_work,
            maximum_due_wakes_per_observation,
        };
        bounds.validate()?;
        Ok(bounds)
    }

    pub fn maximum_reconstruction_rows(self) -> usize {
        self.maximum_reconstruction_rows
    }

    pub fn maximum_query_work(self) -> usize {
        self.maximum_query_work
    }

    pub fn maximum_due_wakes_per_observation(self) -> usize {
        self.maximum_due_wakes_per_observation
    }

    fn validate(self) -> Result<(), &'static str> {
        if self.maximum_reconstruction_rows == 0
            || self.maximum_reconstruction_rows > MAX_TEMPORAL_INTENT_RECONSTRUCTION_ROWS
            || self.maximum_query_work == 0
            || self.maximum_query_work > MAX_TEMPORAL_INTENT_QUERY_WORK
            || self.maximum_due_wakes_per_observation == 0
            || self.maximum_due_wakes_per_observation > MAX_TEMPORAL_DUE_WAKES_PER_OBSERVATION
        {
            Err("invalid-temporal-intent-bounds")
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryTemporalIntentIdentity(String);

impl WorthQueryTemporalIntentIdentity {
    pub fn declare(identity: impl Into<String>) -> Result<Self, &'static str> {
        validated_intent_identity(identity).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryTemporalIntentIdempotencyRelation(String);

impl WorthQueryTemporalIntentIdempotencyRelation {
    pub fn declare(identity: impl Into<String>) -> Result<Self, &'static str> {
        validated_intent_identity(identity).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryTemporalIntentLifecycle {
    Active,
    Cancelled,
    Completed,
}

/// Typed durable-intent meaning reconstructed from one installed query row.
pub struct WorthQueryTemporalIntentCandidate<Clock, Input> {
    identity: WorthQueryTemporalIntentIdentity,
    revision: u64,
    due: WorthQueryClockCoordinate<Clock>,
    input: Input,
    idempotency: WorthQueryTemporalIntentIdempotencyRelation,
    lifecycle: WorthQueryTemporalIntentLifecycle,
    marker: PhantomData<TemporalIntentMarker<Clock, Input>>,
}

impl<Clock, Input> WorthQueryTemporalIntentCandidate<Clock, Input> {
    pub fn active(
        identity: WorthQueryTemporalIntentIdentity,
        revision: u64,
        due: WorthQueryClockCoordinate<Clock>,
        input: Input,
        idempotency: WorthQueryTemporalIntentIdempotencyRelation,
    ) -> Self {
        Self {
            identity,
            revision,
            due,
            input,
            idempotency,
            lifecycle: WorthQueryTemporalIntentLifecycle::Active,
            marker: PhantomData,
        }
    }

    pub fn cancelled(
        identity: WorthQueryTemporalIntentIdentity,
        revision: u64,
        due: WorthQueryClockCoordinate<Clock>,
        input: Input,
        idempotency: WorthQueryTemporalIntentIdempotencyRelation,
    ) -> Self {
        Self {
            identity,
            revision,
            due,
            input,
            idempotency,
            lifecycle: WorthQueryTemporalIntentLifecycle::Cancelled,
            marker: PhantomData,
        }
    }

    pub fn completed(
        identity: WorthQueryTemporalIntentIdentity,
        revision: u64,
        due: WorthQueryClockCoordinate<Clock>,
        input: Input,
        idempotency: WorthQueryTemporalIntentIdempotencyRelation,
    ) -> Self {
        Self {
            identity,
            revision,
            due,
            input,
            idempotency,
            lifecycle: WorthQueryTemporalIntentLifecycle::Completed,
            marker: PhantomData,
        }
    }

    pub fn identity(&self) -> &WorthQueryTemporalIntentIdentity {
        &self.identity
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn due(&self) -> WorthQueryClockCoordinate<Clock> {
        WorthQueryClockCoordinate::from_nanoseconds(self.due.nanoseconds())
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn idempotency(&self) -> &WorthQueryTemporalIntentIdempotencyRelation {
        &self.idempotency
    }

    pub fn lifecycle(&self) -> WorthQueryTemporalIntentLifecycle {
        self.lifecycle
    }

    pub fn into_input(self) -> Input {
        self.input
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryTemporalIntentProjectionFailureKind {
    MissingRequiredValue,
    InvalidIdentity,
    InvalidDueBasis,
    InvalidOperationInput,
    UnsupportedLifecycle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTemporalIntentProjectionFailure {
    kind: WorthQueryTemporalIntentProjectionFailureKind,
    detail: String,
}

impl WorthQueryTemporalIntentProjectionFailure {
    pub fn new(
        kind: WorthQueryTemporalIntentProjectionFailureKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryTemporalIntentProjectionFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Host mapper from the exact installed Relational query result into durable
/// temporal-intent meaning. It does not decide eligibility or schedule work.
pub trait WorthQueryTemporalIntentProjector<Node, Clock, QueryResult, Input>:
    Send + Sync + 'static
{
    const SEMANTIC_IDENTITY: &'static str;

    fn project(
        &self,
        row: &QueryResult,
    ) -> Result<
        WorthQueryTemporalIntentCandidate<Clock, Input>,
        WorthQueryTemporalIntentProjectionFailure,
    >;
}

fn validated_intent_identity(identity: impl Into<String>) -> Result<String, &'static str> {
    let identity = identity.into();
    if identity.is_empty()
        || identity.len() > 256
        || identity.trim() != identity
        || identity.chars().any(char::is_whitespace)
    {
        Err("invalid-temporal-intent-identity")
    } else {
        Ok(identity)
    }
}
