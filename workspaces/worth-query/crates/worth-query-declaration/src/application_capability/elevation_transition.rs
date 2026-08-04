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
    elevation_identity: ApplicationCapabilityValueBinding,
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
        elevation_identity: ApplicationCapabilityValueBinding,
        review_identity: ApplicationCapabilityValueBinding,
        reason: ApplicationCapabilityValueBinding,
        duration: Duration,
    ) -> Self {
        Self {
            target,
            grant: grant.erase(),
            elevation_identity,
            review_identity,
            reason,
            duration,
            _schema: PhantomData,
        }
    }

    pub const fn target(&self) -> &ApplicationCapabilityRequestProjection<Schema, Scope, Context> {
        &self.target
    }

    pub const fn grant(&self) -> &ErasedApplicationCapabilityEntitySelector {
        &self.grant
    }

    pub const fn elevation_identity(&self) -> &ApplicationCapabilityValueBinding {
        &self.elevation_identity
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
