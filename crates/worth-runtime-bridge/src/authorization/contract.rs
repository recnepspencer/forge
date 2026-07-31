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
pub enum BridgeAuthorizationPathEffect {
    Allow,
    Deny,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationPathContract {
    identity: [u8; 32],
    effect: BridgeAuthorizationPathEffect,
}

impl BridgeAuthorizationPathContract {
    pub const fn new(identity: [u8; 32], effect: BridgeAuthorizationPathEffect) -> Self {
        Self { identity, effect }
    }

    pub const fn identity(&self) -> &[u8; 32] {
        &self.identity
    }

    pub const fn effect(&self) -> BridgeAuthorizationPathEffect {
        self.effect
    }
}

pub struct BridgeAuthorizationInstallationRequest {
    pub(crate) correspondence: BridgeAuthorizationCorrespondenceIdentity,
    pub(crate) binding_identity: ApplicationSchemaBindingIdentity,
    pub(crate) ability: String,
    pub(crate) scope_entity: String,
    pub(crate) policy: String,
    pub(crate) paths: Vec<BridgeAuthorizationPathContract>,
}

impl BridgeAuthorizationInstallationRequest {
    pub fn new(
        installed_policy_identity: &CanonicalDigestId,
        binding_identity: ApplicationSchemaBindingIdentity,
        ability: impl Into<String>,
        scope_entity: impl Into<String>,
        policy: impl Into<String>,
        paths: impl IntoIterator<Item = BridgeAuthorizationPathContract>,
    ) -> Self {
        Self {
            correspondence: BridgeAuthorizationCorrespondenceIdentity::from_installed_policy(
                installed_policy_identity,
            ),
            binding_identity,
            ability: ability.into(),
            scope_entity: scope_entity.into(),
            policy: policy.into(),
            paths: paths.into_iter().collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationPathObservation {
    identity: [u8; 32],
    effect: BridgeAuthorizationPathEffect,
    matched: bool,
    exhaustive: bool,
    dependencies: BridgeAuthorizationDependencyCardinality,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationDependencyCardinality {
    pub entities: usize,
    pub relations: usize,
    pub adjacency_lists: usize,
    pub fields: usize,
}

impl BridgeAuthorizationPathObservation {
    pub const fn new(
        identity: [u8; 32],
        effect: BridgeAuthorizationPathEffect,
        matched: bool,
        exhaustive: bool,
        dependencies: BridgeAuthorizationDependencyCardinality,
    ) -> Self {
        Self {
            identity,
            effect,
            matched,
            exhaustive,
            dependencies,
        }
    }

    pub(crate) const fn identity(&self) -> &[u8; 32] {
        &self.identity
    }

    pub(crate) const fn effect(&self) -> BridgeAuthorizationPathEffect {
        self.effect
    }

    pub(crate) const fn matched(&self) -> bool {
        self.matched
    }

    pub(crate) const fn exhaustive(&self) -> bool {
        self.exhaustive
    }

    pub(crate) const fn entity_dependencies(&self) -> usize {
        self.dependencies.entities
    }

    pub(crate) const fn relation_dependencies(&self) -> usize {
        self.dependencies.relations
    }

    pub(crate) const fn field_dependencies(&self) -> usize {
        self.dependencies.fields
    }

    pub(crate) const fn adjacency_dependencies(&self) -> usize {
        self.dependencies.adjacency_lists
    }
}

pub struct BridgeAuthorizationObservation {
    pub(crate) correspondence: BridgeAuthorizationCorrespondenceIdentity,
    pub(crate) dependency_identity: [u8; 32],
    pub(crate) paths: Vec<BridgeAuthorizationPathObservation>,
}

impl BridgeAuthorizationObservation {
    pub fn new(
        correspondence: BridgeAuthorizationCorrespondenceIdentity,
        dependency_identity: [u8; 32],
        paths: impl IntoIterator<Item = BridgeAuthorizationPathObservation>,
    ) -> Self {
        Self {
            correspondence,
            dependency_identity,
            paths: paths.into_iter().collect(),
        }
    }
}
