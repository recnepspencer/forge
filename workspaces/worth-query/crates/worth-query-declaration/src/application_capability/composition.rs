use crate::application_schema::ApplicationPolicyRef;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityRule {
    NotApplicable,
    Policy(String),
}

impl ApplicationCapabilityRule {
    pub const fn not_applicable() -> Self {
        Self::NotApplicable
    }

    pub fn policy<Schema, Policy>(policy: ApplicationPolicyRef<Schema, Policy>) -> Self {
        Self::Policy(policy.name().to_string())
    }

    pub fn policy_name(&self) -> Option<&str> {
        match self {
            Self::NotApplicable => None,
            Self::Policy(policy) => Some(policy),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityDecisionComposition {
    allow: ApplicationCapabilityRule,
    deny: ApplicationCapabilityRule,
    conflict: ApplicationCapabilityRule,
}

impl ApplicationCapabilityDecisionComposition {
    pub fn new(
        allow: ApplicationCapabilityRule,
        deny: ApplicationCapabilityRule,
        conflict: ApplicationCapabilityRule,
    ) -> Self {
        Self {
            allow,
            deny,
            conflict,
        }
    }

    pub const fn allow(&self) -> &ApplicationCapabilityRule {
        &self.allow
    }

    pub const fn deny(&self) -> &ApplicationCapabilityRule {
        &self.deny
    }

    pub const fn conflict(&self) -> &ApplicationCapabilityRule {
        &self.conflict
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityActorComposition {
    separation_of_duty: ApplicationCapabilityRule,
    distinct_actor: ApplicationCapabilityRule,
}

impl ApplicationCapabilityActorComposition {
    pub fn new(
        separation_of_duty: ApplicationCapabilityRule,
        distinct_actor: ApplicationCapabilityRule,
    ) -> Self {
        Self {
            separation_of_duty,
            distinct_actor,
        }
    }

    pub const fn separation_of_duty(&self) -> &ApplicationCapabilityRule {
        &self.separation_of_duty
    }

    pub const fn distinct_actor(&self) -> &ApplicationCapabilityRule {
        &self.distinct_actor
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityPropagationComposition {
    delegation: ApplicationCapabilityRule,
    disclosure: ApplicationCapabilityRule,
}

impl ApplicationCapabilityPropagationComposition {
    pub fn new(
        delegation: ApplicationCapabilityRule,
        disclosure: ApplicationCapabilityRule,
    ) -> Self {
        Self {
            delegation,
            disclosure,
        }
    }

    pub const fn delegation(&self) -> &ApplicationCapabilityRule {
        &self.delegation
    }

    pub const fn disclosure(&self) -> &ApplicationCapabilityRule {
        &self.disclosure
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityComposition {
    decision: ApplicationCapabilityDecisionComposition,
    actors: ApplicationCapabilityActorComposition,
    propagation: ApplicationCapabilityPropagationComposition,
}

impl ApplicationCapabilityComposition {
    pub fn new(
        decision: ApplicationCapabilityDecisionComposition,
        actors: ApplicationCapabilityActorComposition,
        propagation: ApplicationCapabilityPropagationComposition,
    ) -> Self {
        Self {
            decision,
            actors,
            propagation,
        }
    }

    pub const fn decision(&self) -> &ApplicationCapabilityDecisionComposition {
        &self.decision
    }

    pub const fn actors(&self) -> &ApplicationCapabilityActorComposition {
        &self.actors
    }

    pub const fn propagation(&self) -> &ApplicationCapabilityPropagationComposition {
        &self.propagation
    }
}
