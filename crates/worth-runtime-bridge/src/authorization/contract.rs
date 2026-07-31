use worth_foundational::facade::CanonicalDigestId;
use worth_query_installation::facade::ApplicationSchemaBindingIdentity;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BridgeAuthorizationCorrespondenceIdentity(pub(crate) [u8; 32]);

impl BridgeAuthorizationCorrespondenceIdentity {
    pub const fn from_installed_policy(identity: &CanonicalDigestId) -> Self {
        Self(*identity.bytes())
    }

    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeAuthorizationRuleEffect {
    Required,
    Prohibited,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationClauseContract {
    identity: [u8; 32],
}

impl BridgeAuthorizationClauseContract {
    pub const fn new(identity: [u8; 32]) -> Self {
        Self { identity }
    }

    pub const fn identity(&self) -> &[u8; 32] {
        &self.identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationRequirementContract {
    clauses: Vec<BridgeAuthorizationClauseContract>,
}

impl BridgeAuthorizationRequirementContract {
    pub fn any(clauses: impl IntoIterator<Item = BridgeAuthorizationClauseContract>) -> Self {
        Self {
            clauses: clauses.into_iter().collect(),
        }
    }

    pub fn clauses(&self) -> &[BridgeAuthorizationClauseContract] {
        &self.clauses
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationRuleContract {
    effect: BridgeAuthorizationRuleEffect,
    requirements: Vec<BridgeAuthorizationRequirementContract>,
}

impl BridgeAuthorizationRuleContract {
    pub fn all(
        effect: BridgeAuthorizationRuleEffect,
        requirements: impl IntoIterator<Item = BridgeAuthorizationRequirementContract>,
    ) -> Self {
        Self {
            effect,
            requirements: requirements.into_iter().collect(),
        }
    }

    pub const fn effect(&self) -> BridgeAuthorizationRuleEffect {
        self.effect
    }

    pub fn requirements(&self) -> &[BridgeAuthorizationRequirementContract] {
        &self.requirements
    }
}

pub struct BridgeAuthorizationInstallationRequest {
    pub(crate) correspondence: BridgeAuthorizationCorrespondenceIdentity,
    pub(crate) binding_identity: ApplicationSchemaBindingIdentity,
    pub(crate) ability: String,
    pub(crate) scope_entity: String,
    pub(crate) policy: String,
    pub(crate) rules: Vec<BridgeAuthorizationRuleContract>,
}

impl BridgeAuthorizationInstallationRequest {
    pub fn new(
        installed_policy_identity: &CanonicalDigestId,
        binding_identity: ApplicationSchemaBindingIdentity,
        ability: impl Into<String>,
        scope_entity: impl Into<String>,
        policy: impl Into<String>,
        rules: impl IntoIterator<Item = BridgeAuthorizationRuleContract>,
    ) -> Self {
        Self {
            correspondence: BridgeAuthorizationCorrespondenceIdentity::from_installed_policy(
                installed_policy_identity,
            ),
            binding_identity,
            ability: ability.into(),
            scope_entity: scope_entity.into(),
            policy: policy.into(),
            rules: rules.into_iter().collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationDependencyCardinality {
    pub entities: usize,
    pub relations: usize,
    pub adjacency_lists: usize,
    pub fields: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationClauseObservation {
    identity: [u8; 32],
    matched: bool,
    exhaustive: bool,
    dependencies: BridgeAuthorizationDependencyCardinality,
}

impl BridgeAuthorizationClauseObservation {
    pub const fn new(
        identity: [u8; 32],
        matched: bool,
        exhaustive: bool,
        dependencies: BridgeAuthorizationDependencyCardinality,
    ) -> Self {
        Self {
            identity,
            matched,
            exhaustive,
            dependencies,
        }
    }

    pub(crate) const fn identity(&self) -> &[u8; 32] {
        &self.identity
    }

    pub(crate) const fn matched(&self) -> bool {
        self.matched
    }

    pub(crate) const fn exhaustive(&self) -> bool {
        self.exhaustive
    }

    pub(crate) const fn dependencies(&self) -> BridgeAuthorizationDependencyCardinality {
        self.dependencies
    }
}

pub struct BridgeAuthorizationRequirementObservation {
    pub(crate) clauses: Vec<BridgeAuthorizationClauseObservation>,
}

impl BridgeAuthorizationRequirementObservation {
    pub fn any(clauses: impl IntoIterator<Item = BridgeAuthorizationClauseObservation>) -> Self {
        Self {
            clauses: clauses.into_iter().collect(),
        }
    }
}

pub struct BridgeAuthorizationRuleObservation {
    pub(crate) effect: BridgeAuthorizationRuleEffect,
    pub(crate) requirements: Vec<BridgeAuthorizationRequirementObservation>,
}

impl BridgeAuthorizationRuleObservation {
    pub fn all(
        effect: BridgeAuthorizationRuleEffect,
        requirements: impl IntoIterator<Item = BridgeAuthorizationRequirementObservation>,
    ) -> Self {
        Self {
            effect,
            requirements: requirements.into_iter().collect(),
        }
    }
}

pub struct BridgeAuthorizationObservation {
    pub(crate) correspondence: BridgeAuthorizationCorrespondenceIdentity,
    pub(crate) dependency_identity: [u8; 32],
    pub(crate) rules: Vec<BridgeAuthorizationRuleObservation>,
}

impl BridgeAuthorizationObservation {
    pub fn new(
        correspondence: BridgeAuthorizationCorrespondenceIdentity,
        dependency_identity: [u8; 32],
        rules: impl IntoIterator<Item = BridgeAuthorizationRuleObservation>,
    ) -> Self {
        Self {
            correspondence,
            dependency_identity,
            rules: rules.into_iter().collect(),
        }
    }
}
