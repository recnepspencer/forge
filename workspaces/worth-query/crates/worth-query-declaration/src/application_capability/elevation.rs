use super::{
    ApplicationCapabilityElevationLifecycleDefinition, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityValidityDefinition,
    ApplicationCapabilityValueBinding,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityElevationStates {
    requested: ApplicationCapabilityValueBinding,
    approved: ApplicationCapabilityValueBinding,
    active: ApplicationCapabilityValueBinding,
    expired: ApplicationCapabilityValueBinding,
    revoked: ApplicationCapabilityValueBinding,
    review_required: ApplicationCapabilityValueBinding,
    reviewed: ApplicationCapabilityValueBinding,
}

impl ApplicationCapabilityElevationStates {
    pub fn new(
        requested: ApplicationCapabilityValueBinding,
        approved: ApplicationCapabilityValueBinding,
        active: ApplicationCapabilityValueBinding,
        expired: ApplicationCapabilityValueBinding,
        revoked: ApplicationCapabilityValueBinding,
        review_required: ApplicationCapabilityValueBinding,
        reviewed: ApplicationCapabilityValueBinding,
    ) -> Self {
        Self {
            requested,
            approved,
            active,
            expired,
            revoked,
            review_required,
            reviewed,
        }
    }

    pub const fn requested(&self) -> &ApplicationCapabilityValueBinding {
        &self.requested
    }

    pub const fn approved(&self) -> &ApplicationCapabilityValueBinding {
        &self.approved
    }

    pub const fn active(&self) -> &ApplicationCapabilityValueBinding {
        &self.active
    }

    pub const fn expired(&self) -> &ApplicationCapabilityValueBinding {
        &self.expired
    }

    pub const fn revoked(&self) -> &ApplicationCapabilityValueBinding {
        &self.revoked
    }

    pub const fn review_required(&self) -> &ApplicationCapabilityValueBinding {
        &self.review_required
    }

    pub const fn reviewed(&self) -> &ApplicationCapabilityValueBinding {
        &self.reviewed
    }

    pub fn values(&self) -> [&ApplicationCapabilityValueBinding; 7] {
        [
            &self.requested,
            &self.approved,
            &self.active,
            &self.expired,
            &self.revoked,
            &self.review_required,
            &self.reviewed,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityMandatoryReviewDefinition {
    relation: ApplicationCapabilityRelationBinding,
    identity: ApplicationCapabilityFieldBinding,
    reviewer: ApplicationCapabilityRelationBinding,
    status: ApplicationCapabilityFieldBinding,
    required: ApplicationCapabilityValueBinding,
    completed: ApplicationCapabilityValueBinding,
}

impl ApplicationCapabilityMandatoryReviewDefinition {
    pub fn new(
        relation: ApplicationCapabilityRelationBinding,
        identity: ApplicationCapabilityFieldBinding,
        reviewer: ApplicationCapabilityRelationBinding,
        status: ApplicationCapabilityFieldBinding,
        required: ApplicationCapabilityValueBinding,
        completed: ApplicationCapabilityValueBinding,
    ) -> Self {
        Self {
            relation,
            identity,
            reviewer,
            status,
            required,
            completed,
        }
    }

    pub const fn relation(&self) -> &ApplicationCapabilityRelationBinding {
        &self.relation
    }

    pub const fn identity(&self) -> &ApplicationCapabilityFieldBinding {
        &self.identity
    }

    pub const fn reviewer(&self) -> &ApplicationCapabilityRelationBinding {
        &self.reviewer
    }

    pub const fn status(&self) -> &ApplicationCapabilityFieldBinding {
        &self.status
    }

    pub const fn required(&self) -> &ApplicationCapabilityValueBinding {
        &self.required
    }

    pub const fn completed(&self) -> &ApplicationCapabilityValueBinding {
        &self.completed
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityElevationDefinition {
    identity: ApplicationCapabilityFieldBinding,
    reason: ApplicationCapabilityFieldBinding,
    status: ApplicationCapabilityFieldBinding,
    states: ApplicationCapabilityElevationStates,
    validity: ApplicationCapabilityValidityDefinition,
    requester: ApplicationCapabilityRelationBinding,
    approver: ApplicationCapabilityRelationBinding,
    grant: ApplicationCapabilityRelationBinding,
    lifecycle: ApplicationCapabilityElevationLifecycleDefinition,
    review: ApplicationCapabilityMandatoryReviewDefinition,
}

impl ApplicationCapabilityElevationDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        identity: ApplicationCapabilityFieldBinding,
        reason: ApplicationCapabilityFieldBinding,
        status: ApplicationCapabilityFieldBinding,
        states: ApplicationCapabilityElevationStates,
        validity: ApplicationCapabilityValidityDefinition,
        requester: ApplicationCapabilityRelationBinding,
        approver: ApplicationCapabilityRelationBinding,
        grant: ApplicationCapabilityRelationBinding,
        lifecycle: ApplicationCapabilityElevationLifecycleDefinition,
        review: ApplicationCapabilityMandatoryReviewDefinition,
    ) -> Self {
        Self {
            identity,
            reason,
            status,
            states,
            validity,
            requester,
            approver,
            grant,
            lifecycle,
            review,
        }
    }

    pub const fn identity(&self) -> &ApplicationCapabilityFieldBinding {
        &self.identity
    }

    pub const fn reason(&self) -> &ApplicationCapabilityFieldBinding {
        &self.reason
    }

    pub const fn status(&self) -> &ApplicationCapabilityFieldBinding {
        &self.status
    }

    pub const fn states(&self) -> &ApplicationCapabilityElevationStates {
        &self.states
    }

    pub const fn validity(&self) -> &ApplicationCapabilityValidityDefinition {
        &self.validity
    }

    pub const fn requester(&self) -> &ApplicationCapabilityRelationBinding {
        &self.requester
    }

    pub const fn approver(&self) -> &ApplicationCapabilityRelationBinding {
        &self.approver
    }

    pub const fn grant(&self) -> &ApplicationCapabilityRelationBinding {
        &self.grant
    }

    pub const fn lifecycle(&self) -> &ApplicationCapabilityElevationLifecycleDefinition {
        &self.lifecycle
    }

    pub const fn review(&self) -> &ApplicationCapabilityMandatoryReviewDefinition {
        &self.review
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityElevationRule {
    NotApplicable,
    Governed(ApplicationCapabilityElevationDefinition),
}

impl ApplicationCapabilityElevationRule {
    pub const fn not_applicable() -> Self {
        Self::NotApplicable
    }

    pub const fn governed(definition: ApplicationCapabilityElevationDefinition) -> Self {
        Self::Governed(definition)
    }

    pub const fn definition(&self) -> Option<&ApplicationCapabilityElevationDefinition> {
        match self {
            Self::NotApplicable => None,
            Self::Governed(definition) => Some(definition),
        }
    }
}
