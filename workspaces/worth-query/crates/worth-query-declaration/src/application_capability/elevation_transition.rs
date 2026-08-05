use std::marker::PhantomData;
use std::time::Duration;

use super::{
    ApplicationCapabilityEntitySelector, ApplicationCapabilityRequestProjection,
    ApplicationCapabilityValueBinding, ErasedApplicationCapabilityEntitySelector,
};

/// Application-owned projection of one exact lifecycle-request input.
///
/// The projection describes proposed meaning only. Query selects the installed
/// lifecycle role, samples time, binds the authenticated requester, and decides
/// whether the proposed upper bound is lawful.
pub trait ApplicationCapabilityElevationRequest<Schema, Operation> {
    type Scope;
    type Context;

    fn elevation_request(
        &self,
    ) -> Result<
        ApplicationCapabilityElevationRequestProjection<Schema, Self::Scope, Self::Context>,
        ApplicationCapabilityElevationRequestProjectionDenial,
    >;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationCapabilityElevationRequestProjectionDenial {
    subject: String,
}

impl ApplicationCapabilityElevationRequestProjectionDenial {
    pub fn input_variant(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
        }
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

/// Exact proposed upper bound and durable identities for one elevation request.
///
/// Status, actors, and timestamps are deliberately absent. They are governed
/// facts derived by Query rather than caller-authored request dimensions.
pub struct ApplicationCapabilityElevationRequestProjection<Schema, Scope, Context> {
    target: ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
    grant: ErasedApplicationCapabilityEntitySelector,
    elevation_key: String,
    elevation_identity: ApplicationCapabilityValueBinding,
    review_key: String,
    review_identity: ApplicationCapabilityValueBinding,
    reason: ApplicationCapabilityValueBinding,
    duration: Duration,
    _schema: PhantomData<fn() -> Schema>,
}

impl<Schema, Scope, Context>
    ApplicationCapabilityElevationRequestProjection<Schema, Scope, Context>
{
    pub fn new<Grant>(
        target: ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
        grant: ApplicationCapabilityEntitySelector<Schema, Grant>,
        elevation_key: impl Into<String>,
        elevation_identity: ApplicationCapabilityValueBinding,
        review_key: impl Into<String>,
        review_identity: ApplicationCapabilityValueBinding,
        reason: ApplicationCapabilityValueBinding,
        duration: Duration,
    ) -> Result<Self, ApplicationCapabilityElevationRequestProjectionDenial> {
        let elevation_key = elevation_key.into();
        let review_key = review_key.into();
        if !valid_entity_key(&elevation_key) || !valid_entity_key(&review_key) {
            return Err(
                ApplicationCapabilityElevationRequestProjectionDenial::input_variant(
                    "elevation request entity key",
                ),
            );
        }
        Ok(Self {
            target,
            grant: grant.erase(),
            elevation_key,
            elevation_identity,
            review_key,
            review_identity,
            reason,
            duration,
            _schema: PhantomData,
        })
    }

    pub const fn target(&self) -> &ApplicationCapabilityRequestProjection<Schema, Scope, Context> {
        &self.target
    }

    pub const fn grant(&self) -> &ErasedApplicationCapabilityEntitySelector {
        &self.grant
    }

    pub fn elevation_key(&self) -> &str {
        &self.elevation_key
    }

    pub const fn elevation_identity(&self) -> &ApplicationCapabilityValueBinding {
        &self.elevation_identity
    }

    pub fn review_key(&self) -> &str {
        &self.review_key
    }

    pub const fn review_identity(&self) -> &ApplicationCapabilityValueBinding {
        &self.review_identity
    }

    pub const fn reason(&self) -> &ApplicationCapabilityValueBinding {
        &self.reason
    }

    pub const fn duration(&self) -> Duration {
        self.duration
    }
}

fn valid_entity_key(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && value.len() <= 512
        && !value.chars().any(char::is_control)
}
