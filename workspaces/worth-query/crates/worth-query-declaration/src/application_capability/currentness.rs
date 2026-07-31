use super::{ApplicationCapabilityFieldBinding, ApplicationCapabilityValueBinding};
use worth_foundational::facade::ScalarAspectType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityValidityTimeline {
    UnixEpochSeconds,
    UnixEpochMilliseconds,
}

impl ApplicationCapabilityValidityTimeline {
    pub const fn scalar_family(self) -> ScalarAspectType {
        match self {
            Self::UnixEpochSeconds | Self::UnixEpochMilliseconds => ScalarAspectType::UInt64,
        }
    }

    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::UnixEpochSeconds => "unix-epoch-seconds",
            Self::UnixEpochMilliseconds => "unix-epoch-milliseconds",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityValidityDefinition {
    timeline: ApplicationCapabilityValidityTimeline,
    not_before: ApplicationCapabilityFieldBinding,
    not_after: ApplicationCapabilityFieldBinding,
}

impl ApplicationCapabilityValidityDefinition {
    pub fn new(
        timeline: ApplicationCapabilityValidityTimeline,
        not_before: ApplicationCapabilityFieldBinding,
        not_after: ApplicationCapabilityFieldBinding,
    ) -> Self {
        Self {
            timeline,
            not_before,
            not_after,
        }
    }

    pub const fn timeline(&self) -> ApplicationCapabilityValidityTimeline {
        self.timeline
    }

    pub fn not_before(&self) -> &ApplicationCapabilityFieldBinding {
        &self.not_before
    }

    pub fn not_after(&self) -> &ApplicationCapabilityFieldBinding {
        &self.not_after
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityWorkflowDefinition {
    grant: ApplicationCapabilityFieldBinding,
    resource: ApplicationCapabilityFieldBinding,
}

impl ApplicationCapabilityWorkflowDefinition {
    pub fn new(
        grant: ApplicationCapabilityFieldBinding,
        resource: ApplicationCapabilityFieldBinding,
    ) -> Self {
        Self { grant, resource }
    }

    pub fn grant(&self) -> &ApplicationCapabilityFieldBinding {
        &self.grant
    }

    pub fn resource(&self) -> &ApplicationCapabilityFieldBinding {
        &self.resource
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityCurrentnessDefinition {
    active_status: ApplicationCapabilityValueBinding,
    workflow: ApplicationCapabilityWorkflowDefinition,
    validity: ApplicationCapabilityValidityDefinition,
}

impl ApplicationCapabilityCurrentnessDefinition {
    pub fn new(
        active_status: ApplicationCapabilityValueBinding,
        workflow: ApplicationCapabilityWorkflowDefinition,
        validity: ApplicationCapabilityValidityDefinition,
    ) -> Self {
        Self {
            active_status,
            workflow,
            validity,
        }
    }

    pub fn active_status(&self) -> &ApplicationCapabilityValueBinding {
        &self.active_status
    }

    pub const fn workflow(&self) -> &ApplicationCapabilityWorkflowDefinition {
        &self.workflow
    }

    pub const fn validity(&self) -> &ApplicationCapabilityValidityDefinition {
        &self.validity
    }
}
