use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::declaration::ResourceNodeDeclaration;
use super::policy::{
    ResourceCancellationPolicyDeclaration, ResourceDiagnosticsPolicyDeclaration,
    ResourceObservationPolicyDeclaration, ResourceOutputContinuityPolicyDeclaration,
    ResourcePolicyName, ResourceReplayPolicyDeclaration, ResourceRetentionPolicyDeclaration,
    ResourceRetryBudgetScope, ResourceRetryPolicyDeclaration,
    ResourceRevalidationPolicyDeclaration, ResourceStaleAfterPolicyDeclaration,
    ResourceSupersessionPolicyDeclaration, ResourceTimeoutPolicyDeclaration,
};
use super::summary::ResourceCostContractId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ResourcePolicyKind {
    Retry,
    Timeout,
    Cancellation,
    StaleAfter,
    Supersession,
    Revalidation,
    Observation,
    OutputContinuity,
    Retention,
    Diagnostics,
    Replay,
}

impl ResourcePolicyKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::Timeout => "timeout",
            Self::Cancellation => "cancellation",
            Self::StaleAfter => "stale-after",
            Self::Supersession => "supersession",
            Self::Revalidation => "revalidation",
            Self::Observation => "observation",
            Self::OutputContinuity => "output-continuity",
            Self::Retention => "retention",
            Self::Diagnostics => "diagnostics",
            Self::Replay => "replay",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyDescriptorId(u64);

impl ResourcePolicyDescriptorId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyVersion {
    major: u16,
    minor: u16,
}

impl ResourcePolicyVersion {
    pub const INITIAL: Self = Self { major: 1, minor: 0 };

    pub fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub fn major(self) -> u16 {
        self.major
    }

    pub fn minor(self) -> u16 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyDigest(String);

impl ResourcePolicyDigest {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourcePolicyCompatibilityPosture {
    ExactDescriptorMatch,
    CompatibleVersion,
    IncompatibleVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourcePolicySelectionBasis {
    BuiltInDefault,
    DeclaredBuiltIn,
    DeclaredName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourcePolicyDescriptor {
    id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: ResourcePolicyName,
    version: ResourcePolicyVersion,
    descriptor_digest: ResourcePolicyDigest,
    cost_contract: ResourceCostContractId,
    compatibility_posture: ResourcePolicyCompatibilityPosture,
}

impl ResourcePolicyDescriptor {
    pub(crate) fn new(
        id: ResourcePolicyDescriptorId,
        kind: ResourcePolicyKind,
        semantic_name: ResourcePolicyName,
        version: ResourcePolicyVersion,
        cost_contract: ResourceCostContractId,
        compatibility_posture: ResourcePolicyCompatibilityPosture,
    ) -> Self {
        let descriptor_digest = descriptor_digest(id, kind, &semantic_name, version, cost_contract);
        Self {
            id,
            kind,
            semantic_name,
            version,
            descriptor_digest,
            cost_contract,
            compatibility_posture,
        }
    }

    pub fn id(&self) -> ResourcePolicyDescriptorId {
        self.id
    }

    pub fn kind(&self) -> ResourcePolicyKind {
        self.kind
    }

    pub fn semantic_name(&self) -> &ResourcePolicyName {
        &self.semantic_name
    }

    pub fn version(&self) -> ResourcePolicyVersion {
        self.version
    }

    pub fn descriptor_digest(&self) -> &ResourcePolicyDigest {
        &self.descriptor_digest
    }

    pub fn cost_contract(&self) -> ResourceCostContractId {
        self.cost_contract
    }

    pub fn compatibility_posture(&self) -> ResourcePolicyCompatibilityPosture {
        self.compatibility_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePolicyRegistryFreezeReport {
    descriptor_count: usize,
    id_index_width: usize,
    kind_name_index_width: usize,
    registry_digest: ResourcePolicyDigest,
}

impl ResourcePolicyRegistryFreezeReport {
    pub(crate) fn new(
        descriptor_count: usize,
        id_index_width: usize,
        kind_name_index_width: usize,
        registry_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            descriptor_count,
            id_index_width,
            kind_name_index_width,
            registry_digest,
        }
    }

    pub fn descriptor_count(&self) -> usize {
        self.descriptor_count
    }

    pub fn id_index_width(&self) -> usize {
        self.id_index_width
    }

    pub fn kind_name_index_width(&self) -> usize {
        self.kind_name_index_width
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedResourcePolicyReference {
    descriptor_id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: ResourcePolicyName,
    selection_basis: ResourcePolicySelectionBasis,
    parameter_digest: ResourcePolicyDigest,
}

impl ValidatedResourcePolicyReference {
    fn new(
        descriptor: ResourcePolicyDescriptor,
        selection_basis: ResourcePolicySelectionBasis,
        parameter_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            descriptor_id: descriptor.id(),
            kind: descriptor.kind(),
            semantic_name: descriptor.semantic_name().clone(),
            selection_basis,
            parameter_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn kind(&self) -> ResourcePolicyKind {
        self.kind
    }

    pub fn semantic_name(&self) -> &ResourcePolicyName {
        &self.semantic_name
    }

    pub fn selection_basis(&self) -> ResourcePolicySelectionBasis {
        self.selection_basis
    }

    pub fn parameter_digest(&self) -> &ResourcePolicyDigest {
        &self.parameter_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrozenResourcePolicyDescriptor {
    descriptor: ResourcePolicyDescriptor,
    selection_basis: ResourcePolicySelectionBasis,
    parameter_digest: ResourcePolicyDigest,
    frozen_digest: ResourcePolicyDigest,
}

impl FrozenResourcePolicyDescriptor {
    fn from_validated_reference(
        reference: &ValidatedResourcePolicyReference,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        let descriptor = registry
            .resolve_by_id(reference.descriptor_id())
            .cloned()
            .ok_or_else(|| ResourcePolicyResolutionError::MissingDescriptor {
                kind: reference.kind(),
                name: reference.semantic_name().clone(),
            })?;
        let frozen_digest = frozen_policy_descriptor_digest(
            &descriptor,
            reference.selection_basis(),
            reference.parameter_digest(),
        );
        Ok(Self {
            descriptor,
            selection_basis: reference.selection_basis(),
            parameter_digest: reference.parameter_digest().clone(),
            frozen_digest,
        })
    }

    pub fn descriptor(&self) -> &ResourcePolicyDescriptor {
        &self.descriptor
    }

    pub fn selection_basis(&self) -> ResourcePolicySelectionBasis {
        self.selection_basis
    }

    pub fn parameter_digest(&self) -> &ResourcePolicyDigest {
        &self.parameter_digest
    }

    pub fn frozen_digest(&self) -> &ResourcePolicyDigest {
        &self.frozen_digest
    }

    pub fn resolved_digest(&self) -> &ResourcePolicyDigest {
        &self.frozen_digest
    }
}

pub type ResourceResolvedPolicy = FrozenResourcePolicyDescriptor;

impl ResourcePolicySelectionBasis {
    fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInDefault => "built-in-default",
            Self::DeclaredBuiltIn => "declared-built-in",
            Self::DeclaredName => "declared-name",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedResourcePolicyDeclaration {
    declaration: ResourceNodeDeclaration,
    retry: ValidatedResourcePolicyReference,
    timeout: ValidatedResourcePolicyReference,
    cancellation: ValidatedResourcePolicyReference,
    stale_after: ValidatedResourcePolicyReference,
    supersession: ValidatedResourcePolicyReference,
    revalidation: ValidatedResourcePolicyReference,
    observation: ValidatedResourcePolicyReference,
    output_continuity: ValidatedResourcePolicyReference,
    retention: ValidatedResourcePolicyReference,
    diagnostics: ValidatedResourcePolicyReference,
    replay: ValidatedResourcePolicyReference,
    registry_digest: ResourcePolicyDigest,
}

impl ValidatedResourcePolicyDeclaration {
    pub(crate) fn from_declaration(
        declaration: &ResourceNodeDeclaration,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        Ok(Self {
            declaration: declaration.clone(),
            retry: registry.resolve_retry(declaration)?,
            timeout: registry.resolve_timeout(declaration.timeout_policy())?,
            cancellation: registry.resolve_cancellation(declaration.cancellation_policy())?,
            stale_after: registry.resolve_stale_after(declaration.stale_after_policy())?,
            supersession: registry.resolve_supersession(declaration.supersession_policy())?,
            revalidation: registry.resolve_revalidation(declaration.revalidation_policy())?,
            observation: registry.resolve_observation(declaration.observation_policy())?,
            output_continuity: registry
                .resolve_output_continuity(declaration.output_continuity_policy())?,
            retention: registry.resolve_retention(declaration.retention_policy())?,
            diagnostics: registry.resolve_diagnostics(declaration.diagnostics_policy())?,
            replay: registry.resolve_replay(declaration.replay_policy())?,
            registry_digest: registry.registry_digest().clone(),
        })
    }

    pub fn declaration(&self) -> &ResourceNodeDeclaration {
        &self.declaration
    }

    pub fn retry(&self) -> &ValidatedResourcePolicyReference {
        &self.retry
    }

    pub fn timeout(&self) -> &ValidatedResourcePolicyReference {
        &self.timeout
    }

    pub fn cancellation(&self) -> &ValidatedResourcePolicyReference {
        &self.cancellation
    }

    pub fn stale_after(&self) -> &ValidatedResourcePolicyReference {
        &self.stale_after
    }

    pub fn supersession(&self) -> &ValidatedResourcePolicyReference {
        &self.supersession
    }

    pub fn revalidation(&self) -> &ValidatedResourcePolicyReference {
        &self.revalidation
    }

    pub fn observation(&self) -> &ValidatedResourcePolicyReference {
        &self.observation
    }

    pub fn output_continuity(&self) -> &ValidatedResourcePolicyReference {
        &self.output_continuity
    }

    pub fn retention(&self) -> &ValidatedResourcePolicyReference {
        &self.retention
    }

    pub fn diagnostics(&self) -> &ValidatedResourcePolicyReference {
        &self.diagnostics
    }

    pub fn replay(&self) -> &ValidatedResourcePolicyReference {
        &self.replay
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrozenResourcePolicyDescriptorSet {
    retry: FrozenResourcePolicyDescriptor,
    timeout: FrozenResourcePolicyDescriptor,
    cancellation: FrozenResourcePolicyDescriptor,
    stale_after: FrozenResourcePolicyDescriptor,
    supersession: FrozenResourcePolicyDescriptor,
    revalidation: FrozenResourcePolicyDescriptor,
    observation: FrozenResourcePolicyDescriptor,
    output_continuity: FrozenResourcePolicyDescriptor,
    retention: FrozenResourcePolicyDescriptor,
    diagnostics: FrozenResourcePolicyDescriptor,
    replay: FrozenResourcePolicyDescriptor,
    registry_digest: ResourcePolicyDigest,
}

impl FrozenResourcePolicyDescriptorSet {
    pub(crate) fn from_validated_declaration(
        validated: &ValidatedResourcePolicyDeclaration,
        registry: &FrozenResourcePolicyRegistry,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        if validated.registry_digest() != registry.registry_digest() {
            return Err(ResourcePolicyResolutionError::RegistryDigestDrift {
                expected: validated.registry_digest().clone(),
                actual: registry.registry_digest().clone(),
            });
        }
        Ok(Self {
            retry: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.retry(),
                registry,
            )?,
            timeout: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.timeout(),
                registry,
            )?,
            cancellation: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.cancellation(),
                registry,
            )?,
            stale_after: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.stale_after(),
                registry,
            )?,
            supersession: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.supersession(),
                registry,
            )?,
            revalidation: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.revalidation(),
                registry,
            )?,
            observation: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.observation(),
                registry,
            )?,
            output_continuity: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.output_continuity(),
                registry,
            )?,
            retention: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.retention(),
                registry,
            )?,
            diagnostics: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.diagnostics(),
                registry,
            )?,
            replay: FrozenResourcePolicyDescriptor::from_validated_reference(
                validated.replay(),
                registry,
            )?,
            registry_digest: validated.registry_digest().clone(),
        })
    }

    pub fn retry(&self) -> &FrozenResourcePolicyDescriptor {
        &self.retry
    }
    pub fn timeout(&self) -> &FrozenResourcePolicyDescriptor {
        &self.timeout
    }
    pub fn cancellation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.cancellation
    }
    pub fn stale_after(&self) -> &FrozenResourcePolicyDescriptor {
        &self.stale_after
    }
    pub fn supersession(&self) -> &FrozenResourcePolicyDescriptor {
        &self.supersession
    }
    pub fn revalidation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.revalidation
    }
    pub fn observation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.observation
    }
    pub fn output_continuity(&self) -> &FrozenResourcePolicyDescriptor {
        &self.output_continuity
    }
    pub fn retention(&self) -> &FrozenResourcePolicyDescriptor {
        &self.retention
    }
    pub fn diagnostics(&self) -> &FrozenResourcePolicyDescriptor {
        &self.diagnostics
    }
    pub fn replay(&self) -> &FrozenResourcePolicyDescriptor {
        &self.replay
    }
    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoweredResourcePolicyBundle {
    retry: FrozenResourcePolicyDescriptor,
    timeout: FrozenResourcePolicyDescriptor,
    cancellation: FrozenResourcePolicyDescriptor,
    stale_after: FrozenResourcePolicyDescriptor,
    supersession: FrozenResourcePolicyDescriptor,
    revalidation: FrozenResourcePolicyDescriptor,
    observation: FrozenResourcePolicyDescriptor,
    output_continuity: FrozenResourcePolicyDescriptor,
    retention: FrozenResourcePolicyDescriptor,
    diagnostics: FrozenResourcePolicyDescriptor,
    replay: FrozenResourcePolicyDescriptor,
    registry_digest: ResourcePolicyDigest,
    bundle_digest: ResourcePolicyDigest,
}

impl LoweredResourcePolicyBundle {
    pub(crate) fn from_frozen_descriptors(frozen: &FrozenResourcePolicyDescriptorSet) -> Self {
        let retry = frozen.retry().clone();
        let timeout = frozen.timeout().clone();
        let cancellation = frozen.cancellation().clone();
        let stale_after = frozen.stale_after().clone();
        let supersession = frozen.supersession().clone();
        let revalidation = frozen.revalidation().clone();
        let observation = frozen.observation().clone();
        let output_continuity = frozen.output_continuity().clone();
        let retention = frozen.retention().clone();
        let diagnostics = frozen.diagnostics().clone();
        let replay = frozen.replay().clone();
        let bundle_digest = bundle_digest(&[
            &retry,
            &timeout,
            &cancellation,
            &stale_after,
            &supersession,
            &revalidation,
            &observation,
            &output_continuity,
            &retention,
            &diagnostics,
            &replay,
        ]);
        Self {
            retry,
            timeout,
            cancellation,
            stale_after,
            supersession,
            revalidation,
            observation,
            output_continuity,
            retention,
            diagnostics,
            replay,
            registry_digest: frozen.registry_digest().clone(),
            bundle_digest,
        }
    }

    pub fn retry(&self) -> &FrozenResourcePolicyDescriptor {
        &self.retry
    }

    pub fn timeout(&self) -> &FrozenResourcePolicyDescriptor {
        &self.timeout
    }

    pub fn cancellation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.cancellation
    }

    pub fn stale_after(&self) -> &FrozenResourcePolicyDescriptor {
        &self.stale_after
    }

    pub fn supersession(&self) -> &FrozenResourcePolicyDescriptor {
        &self.supersession
    }

    pub fn revalidation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.revalidation
    }

    pub fn observation(&self) -> &FrozenResourcePolicyDescriptor {
        &self.observation
    }

    pub fn output_continuity(&self) -> &FrozenResourcePolicyDescriptor {
        &self.output_continuity
    }

    pub fn retention(&self) -> &FrozenResourcePolicyDescriptor {
        &self.retention
    }

    pub fn diagnostics(&self) -> &FrozenResourcePolicyDescriptor {
        &self.diagnostics
    }

    pub fn replay(&self) -> &FrozenResourcePolicyDescriptor {
        &self.replay
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }

    pub fn bundle_digest(&self) -> &ResourcePolicyDigest {
        &self.bundle_digest
    }
}

pub type ResourceResolvedPolicyBundle = LoweredResourcePolicyBundle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePolicyRegistration {
    id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: ResourcePolicyName,
    version: ResourcePolicyVersion,
    cost_contract: ResourceCostContractId,
    compatibility_posture: ResourcePolicyCompatibilityPosture,
}

impl ResourcePolicyRegistration {
    pub fn new(
        id: ResourcePolicyDescriptorId,
        kind: ResourcePolicyKind,
        semantic_name: ResourcePolicyName,
        version: ResourcePolicyVersion,
        cost_contract: ResourceCostContractId,
        compatibility_posture: ResourcePolicyCompatibilityPosture,
    ) -> Self {
        Self {
            id,
            kind,
            semantic_name,
            version,
            cost_contract,
            compatibility_posture,
        }
    }

    pub fn id(&self) -> ResourcePolicyDescriptorId {
        self.id
    }

    pub fn kind(&self) -> ResourcePolicyKind {
        self.kind
    }

    pub fn semantic_name(&self) -> &ResourcePolicyName {
        &self.semantic_name
    }

    pub fn version(&self) -> ResourcePolicyVersion {
        self.version
    }

    pub fn cost_contract(&self) -> ResourceCostContractId {
        self.cost_contract
    }

    pub fn compatibility_posture(&self) -> ResourcePolicyCompatibilityPosture {
        self.compatibility_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePolicyRegistryError {
    DuplicateId(ResourcePolicyDescriptorId),
    DuplicateName {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
    },
    MalformedDescriptor {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
        reason: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePolicyResolutionError {
    UnknownPolicy {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
    },
    MissingDescriptor {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
    },
    RegistryDigestDrift {
        expected: ResourcePolicyDigest,
        actual: ResourcePolicyDigest,
    },
    IncompatibleDescriptor {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
        version: ResourcePolicyVersion,
        compatibility_posture: ResourcePolicyCompatibilityPosture,
    },
    MalformedDescriptor {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
        reason: &'static str,
    },
    UnsupportedExecutablePolicy {
        kind: ResourcePolicyKind,
        name: ResourcePolicyName,
        reason: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenResourcePolicyRegistry {
    descriptors: Vec<ResourcePolicyDescriptor>,
    index_by_id: BTreeMap<ResourcePolicyDescriptorId, usize>,
    index_by_kind_name: BTreeMap<(ResourcePolicyKind, ResourcePolicyName), usize>,
    freeze_report: ResourcePolicyRegistryFreezeReport,
}

impl FrozenResourcePolicyRegistry {
    pub fn new(
        registrations: Vec<ResourcePolicyRegistration>,
    ) -> Result<Self, ResourcePolicyRegistryError> {
        let mut descriptors = Vec::with_capacity(registrations.len());
        let mut index_by_id = BTreeMap::new();
        let mut index_by_kind_name = BTreeMap::new();

        for registration in registrations {
            validate_policy_registration_name(registration.kind(), registration.semantic_name())
                .map_err(|(kind, name, reason)| {
                    ResourcePolicyRegistryError::MalformedDescriptor { kind, name, reason }
                })?;
            let descriptor = ResourcePolicyDescriptor::new(
                registration.id(),
                registration.kind(),
                registration.semantic_name().clone(),
                registration.version(),
                registration.cost_contract(),
                registration.compatibility_posture(),
            );
            let index = descriptors.len();
            if index_by_id.insert(descriptor.id(), index).is_some() {
                return Err(ResourcePolicyRegistryError::DuplicateId(descriptor.id()));
            }
            let kind_name = (descriptor.kind(), descriptor.semantic_name().clone());
            if index_by_kind_name
                .insert(kind_name.clone(), index)
                .is_some()
            {
                let (kind, name) = kind_name;
                return Err(ResourcePolicyRegistryError::DuplicateName { kind, name });
            }
            descriptors.push(descriptor);
        }

        let registry_digest = registry_digest(&descriptors);
        let freeze_report = ResourcePolicyRegistryFreezeReport::new(
            descriptors.len(),
            index_by_id.len(),
            index_by_kind_name.len(),
            registry_digest,
        );

        Ok(Self {
            descriptors,
            index_by_id,
            index_by_kind_name,
            freeze_report,
        })
    }

    pub fn built_in() -> Self {
        Self::new(built_in_policy_registrations()).expect("built-in resource policy registry")
    }

    pub fn resolve_by_name(
        &self,
        kind: ResourcePolicyKind,
        name: &ResourcePolicyName,
    ) -> Option<&ResourcePolicyDescriptor> {
        self.index_by_kind_name
            .get(&(kind, name.clone()))
            .and_then(|index| self.descriptors.get(*index))
    }

    pub fn resolve_by_id(
        &self,
        id: ResourcePolicyDescriptorId,
    ) -> Option<&ResourcePolicyDescriptor> {
        self.index_by_id
            .get(&id)
            .and_then(|index| self.descriptors.get(*index))
    }

    pub fn descriptor_count(&self) -> usize {
        self.descriptors.len()
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        self.freeze_report.registry_digest()
    }

    pub fn freeze_report(&self) -> &ResourcePolicyRegistryFreezeReport {
        &self.freeze_report
    }

    fn resolve_named(
        &self,
        kind: ResourcePolicyKind,
        name: &ResourcePolicyName,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        validate_named_policy_name(kind, name)?;
        let descriptor = self.resolve_by_name(kind, name).cloned().ok_or_else(|| {
            ResourcePolicyResolutionError::UnknownPolicy {
                kind,
                name: name.clone(),
            }
        })?;
        ensure_compatible_descriptor(kind, &descriptor)?;
        Ok(ValidatedResourcePolicyReference::new(
            descriptor,
            ResourcePolicySelectionBasis::DeclaredName,
            ResourcePolicyDigest::new(format!("named:{}", name.as_str())),
        ))
    }

    fn built_in_policy(
        &self,
        kind: ResourcePolicyKind,
        name: &str,
        selection_basis: ResourcePolicySelectionBasis,
        parameter_digest: ResourcePolicyDigest,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        let descriptor = self
            .resolve_by_name(kind, &ResourcePolicyName::new(name))
            .cloned()
            .ok_or_else(|| ResourcePolicyResolutionError::MissingDescriptor {
                kind,
                name: ResourcePolicyName::new(name),
            })?;
        ensure_compatible_descriptor(kind, &descriptor)?;
        Ok(ValidatedResourcePolicyReference::new(
            descriptor,
            selection_basis,
            parameter_digest,
        ))
    }

    fn resolve_retry(
        &self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        let policy = declaration.retry_policy();
        let max_attempts = declaration.retry_max_attempts();
        let max_jitter = declaration.retry_deterministic_jitter();
        let retry_budget_scope = declaration.retry_budget_scope();
        let retry_budget_limit = declaration.retry_budget_limit();
        Ok(match policy {
            ResourceRetryPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                retry_parameter_digest(
                    "disabled",
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ),
            )?,
            ResourceRetryPolicyDeclaration::FixedDelay { delay }
            | ResourceRetryPolicyDeclaration::RuntimeBackoff { delay } => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.fixed-delay",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                retry_parameter_digest(
                    &format!("fixed-delay:{}", delay.get()),
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ),
            )?,
            ResourceRetryPolicyDeclaration::ExponentialBackoff {
                initial_delay,
                multiplier,
            } => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.exponential-backoff",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                retry_parameter_digest(
                    &format!("exponential-backoff:{}:{}", initial_delay.get(), multiplier),
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ),
            )?,
            ResourceRetryPolicyDeclaration::CappedExponentialBackoff {
                initial_delay,
                multiplier,
                max_delay,
            } => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.capped-exponential-backoff",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                retry_parameter_digest(
                    &format!(
                        "capped-exponential-backoff:{}:{}:{}",
                        initial_delay.get(),
                        multiplier,
                        max_delay.get()
                    ),
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ),
            )?,
            ResourceRetryPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Retry, name)?
            }
        })
    }

    fn resolve_timeout(
        &self,
        policy: &ResourceTimeoutPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceTimeoutPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("timeout:disabled"),
            )?,
            ResourceTimeoutPolicyDeclaration::TransactionInheritedDeadline => self
                .built_in_policy(
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.transaction-inherited-deadline",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("timeout:transaction-inherited-deadline"),
                )?,
            ResourceTimeoutPolicyDeclaration::RuntimeInheritedDeadline => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.runtime-inherited-deadline",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("timeout:runtime-inherited-deadline"),
            )?,
            ResourceTimeoutPolicyDeclaration::PerAttemptTimeout { timeout }
            | ResourceTimeoutPolicyDeclaration::FixedTimeout { timeout }
            | ResourceTimeoutPolicyDeclaration::RuntimeTimeout { timeout } => self
                .built_in_policy(
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    timeout_parameter_digest("fixed-timeout", *timeout),
                )?,
            ResourceTimeoutPolicyDeclaration::TotalRequestLifetimeTimeout { timeout } => self
                .built_in_policy(
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.total-request-lifetime-timeout",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    timeout_parameter_digest("total-request-lifetime-timeout", *timeout),
                )?,
            ResourceTimeoutPolicyDeclaration::ProgressHeartbeatExtension {
                timeout,
                heartbeat_extension,
            } => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.progress-heartbeat-extension",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new(format!(
                    "timeout:progress-heartbeat-extension:{}:{}",
                    timeout.get(),
                    heartbeat_extension.get()
                )),
            )?,
            ResourceTimeoutPolicyDeclaration::TerminalTimeout { timeout } => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.terminal-timeout",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                timeout_parameter_digest("terminal-timeout", *timeout),
            )?,
            ResourceTimeoutPolicyDeclaration::RevalidationEligibleTimeout { timeout } => self
                .built_in_policy(
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.revalidation-eligible-timeout",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    timeout_parameter_digest("revalidation-eligible-timeout", *timeout),
                )?,
            ResourceTimeoutPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Timeout, name)?
            }
        })
    }

    fn resolve_cancellation(
        &self,
        policy: &ResourceCancellationPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceCancellationPolicyDeclaration::RuntimeDenialOnly => self.built_in_policy(
                ResourcePolicyKind::Cancellation,
                "signal.resource.cancellation.runtime-denial-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("cancellation:runtime-denial-only"),
            )?,
            ResourceCancellationPolicyDeclaration::BestEffortHostSignalAndRuntimeDenial => self
                .built_in_policy(
                    ResourcePolicyKind::Cancellation,
                    "signal.resource.cancellation.best-effort-host-signal-and-runtime-denial",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "cancellation:best-effort-host-signal-and-runtime-denial",
                    ),
                )?,
            ResourceCancellationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Cancellation, name)?
            }
        })
    }

    fn resolve_stale_after(
        &self,
        policy: &ResourceStaleAfterPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceStaleAfterPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::StaleAfter,
                "signal.resource.stale-after.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("stale-after:disabled"),
            )?,
            ResourceStaleAfterPolicyDeclaration::RuntimeStaleAfter { stale_after } => self
                .built_in_policy(
                    ResourcePolicyKind::StaleAfter,
                    "signal.resource.stale-after.runtime-stale-after",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(format!(
                        "stale-after:runtime-stale-after:{}",
                        stale_after.get()
                    )),
                )?,
            ResourceStaleAfterPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::StaleAfter, name)?
            }
        })
    }

    fn resolve_supersession(
        &self,
        policy: &ResourceSupersessionPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceSupersessionPolicyDeclaration::NewGenerationSupersedesPrior => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.new-generation-supersedes-prior",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new("supersession:new-generation-supersedes-prior"),
                )?,
            ResourceSupersessionPolicyDeclaration::OverlappingGenerationRetainsOldHostWork => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.overlapping-generation-retains-old-host-work",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "supersession:overlapping-generation-retains-old-host-work",
                    ),
                )?,
            ResourceSupersessionPolicyDeclaration::OverlappingGenerationCancelsOldHostWork => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.overlapping-generation-cancels-old-host-work",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "supersession:overlapping-generation-cancels-old-host-work",
                    ),
                )?,
            ResourceSupersessionPolicyDeclaration::IntentEquivalentCoalescesToActive => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.intent-equivalent-coalesces-to-active",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("supersession:intent-equivalent-coalesces-to-active"),
                )?,
            ResourceSupersessionPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Supersession, name)?
            }
        })
    }

    fn resolve_revalidation(
        &self,
        policy: &ResourceRevalidationPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceRevalidationPolicyDeclaration::ExplicitIntentOnly => self.built_in_policy(
                ResourcePolicyKind::Revalidation,
                "signal.resource.revalidation.explicit-intent-only",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("revalidation:explicit-intent-only"),
            )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-active-handle-forced"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilled => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-stale-after-fulfilled",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-stale-after-fulfilled",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilledOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-stale-after-fulfilled-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-stale-after-fulfilled-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChange => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-dependency-change",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-dependency-change"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-dependency-change-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-dependency-change-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemand => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-observer-demand",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-observer-demand"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemandOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-observer-demand-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-observer-demand-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemand => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-dependency-change-or-observer-demand",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemandOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-dependency-change-or-observer-demand-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalState => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-terminal-state",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-terminal-state"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalStateOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-terminal-state-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-terminal-state-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycle => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-fulfilled-lifecycle",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-fulfilled-lifecycle"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycleOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-fulfilled-lifecycle-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-fulfilled-lifecycle-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Revalidation, name)?
            }
        })
    }

    fn resolve_observation(
        &self,
        policy: &ResourceObservationPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceObservationPolicyDeclaration::LifecycleOnly => self.built_in_policy(
                ResourcePolicyKind::Observation,
                "signal.resource.observation.lifecycle-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("observation:lifecycle-only"),
            )?,
            ResourceObservationPolicyDeclaration::LifecycleAndOutput => self.built_in_policy(
                ResourcePolicyKind::Observation,
                "signal.resource.observation.lifecycle-and-output",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("observation:lifecycle-and-output"),
            )?,
            ResourceObservationPolicyDeclaration::LifecycleOutputAndDeniedCompletion => self
                .built_in_policy(
                    ResourcePolicyKind::Observation,
                    "signal.resource.observation.lifecycle-output-and-denied-completion",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "observation:lifecycle-output-and-denied-completion",
                    ),
                )?,
            ResourceObservationPolicyDeclaration::LifecycleOutputAndRetrySchedule => self
                .built_in_policy(
                    ResourcePolicyKind::Observation,
                    "signal.resource.observation.lifecycle-output-and-retry-schedule",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "observation:lifecycle-output-and-retry-schedule",
                    ),
                )?,
            ResourceObservationPolicyDeclaration::LifecycleOutputAndDeniedCompletionAndRetrySchedule => self
                .built_in_policy(
                    ResourcePolicyKind::Observation,
                    "signal.resource.observation.lifecycle-output-and-denied-completion-and-retry-schedule",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "observation:lifecycle-output-and-denied-completion-and-retry-schedule",
                    ),
                )?,
            ResourceObservationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Observation, name)?
            }
        })
    }

    fn resolve_output_continuity(
        &self,
        policy: &ResourceOutputContinuityPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceOutputContinuityPolicyDeclaration::PreserveLifecycleOutputSeparation => self
                .built_in_policy(
                    ResourcePolicyKind::OutputContinuity,
                    "signal.resource.output-continuity.preserve-lifecycle-output-separation",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "output-continuity:preserve-lifecycle-output-separation",
                    ),
                )?,
            ResourceOutputContinuityPolicyDeclaration::HideWhilePending => self.built_in_policy(
                ResourcePolicyKind::OutputContinuity,
                "signal.resource.output-continuity.hide-while-pending",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("output-continuity:hide-while-pending"),
            )?,
            ResourceOutputContinuityPolicyDeclaration::HideAfterRejection => self.built_in_policy(
                ResourcePolicyKind::OutputContinuity,
                "signal.resource.output-continuity.hide-after-rejection",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("output-continuity:hide-after-rejection"),
            )?,
            ResourceOutputContinuityPolicyDeclaration::HideAfterTimeout => self.built_in_policy(
                ResourcePolicyKind::OutputContinuity,
                "signal.resource.output-continuity.hide-after-timeout",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("output-continuity:hide-after-timeout"),
            )?,
            ResourceOutputContinuityPolicyDeclaration::HideAfterCancellation => self
                .built_in_policy(
                    ResourcePolicyKind::OutputContinuity,
                    "signal.resource.output-continuity.hide-after-cancellation",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("output-continuity:hide-after-cancellation"),
                )?,
            ResourceOutputContinuityPolicyDeclaration::HideAfterSupersession => self
                .built_in_policy(
                    ResourcePolicyKind::OutputContinuity,
                    "signal.resource.output-continuity.hide-after-supersession",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("output-continuity:hide-after-supersession"),
                )?,
            ResourceOutputContinuityPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::OutputContinuity, name)?
            }
        })
    }

    fn resolve_retention(
        &self,
        policy: &ResourceRetentionPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceRetentionPolicyDeclaration::RetainAllTransitions => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.retain-all-transitions",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:retain-all-transitions"),
            )?,
            ResourceRetentionPolicyDeclaration::RetainOperationalLifecycleSummary => self
                .built_in_policy(
                    ResourcePolicyKind::Retention,
                    "signal.resource.retention.terminal-summaries-only",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new("retention:terminal-summaries-only"),
                )?,
            ResourceRetentionPolicyDeclaration::TerminalSummariesOnly => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.terminal-summaries-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:terminal-summaries-only"),
            )?,
            ResourceRetentionPolicyDeclaration::CompactSuperseded => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.compact-superseded",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:compact-superseded"),
            )?,
            ResourceRetentionPolicyDeclaration::CompactCancelled => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.compact-cancelled",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:compact-cancelled"),
            )?,
            ResourceRetentionPolicyDeclaration::CompactTimedOut => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.compact-timed-out",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:compact-timed-out"),
            )?,
            ResourceRetentionPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Retention, name)?
            }
        })
    }

    fn resolve_diagnostics(
        &self,
        policy: &ResourceDiagnosticsPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceDiagnosticsPolicyDeclaration::RetainedOnly => self.built_in_policy(
                ResourcePolicyKind::Diagnostics,
                "signal.resource.diagnostics.retained-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("diagnostics:retained-only"),
            )?,
            ResourceDiagnosticsPolicyDeclaration::BudgetedExpansion {
                max_replay_reconstruction_width,
            } => self.built_in_policy(
                ResourcePolicyKind::Diagnostics,
                "signal.resource.diagnostics.budgeted-expansion",
                if *max_replay_reconstruction_width == u32::MAX {
                    ResourcePolicySelectionBasis::BuiltInDefault
                } else {
                    ResourcePolicySelectionBasis::DeclaredBuiltIn
                },
                diagnostics_parameter_digest(
                    "budgeted-expansion",
                    *max_replay_reconstruction_width,
                    None,
                ),
            )?,
            ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                max_replay_reconstruction_width,
                max_forensic_reconstruction_width,
            } => self.built_in_policy(
                ResourcePolicyKind::Diagnostics,
                "signal.resource.diagnostics.forensic-expansion-budget",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                diagnostics_parameter_digest(
                    "forensic-expansion-budget",
                    *max_replay_reconstruction_width,
                    Some(*max_forensic_reconstruction_width),
                ),
            )?,
            ResourceDiagnosticsPolicyDeclaration::DenyColdExpansion => self.built_in_policy(
                ResourcePolicyKind::Diagnostics,
                "signal.resource.diagnostics.deny-cold-expansion",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("diagnostics:deny-cold-expansion"),
            )?,
            ResourceDiagnosticsPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Diagnostics, name)?
            }
        })
    }

    fn resolve_replay(
        &self,
        policy: &ResourceReplayPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceReplayPolicyDeclaration::IdenticalOnly => self.built_in_policy(
                ResourcePolicyKind::Replay,
                "signal.resource.replay.identical-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("replay:identical-only"),
            )?,
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansion => self.built_in_policy(
                ResourcePolicyKind::Replay,
                "signal.resource.replay.compatible-parameter-expansion",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("replay:compatible-parameter-expansion"),
            )?,
            ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowing => self.built_in_policy(
                ResourcePolicyKind::Replay,
                "signal.resource.replay.compatible-retention-narrowing",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("replay:compatible-retention-narrowing"),
            )?,
            ResourceReplayPolicyDeclaration::CompatibleDiagnosticsRichnessChange => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-diagnostics-richness-change",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("replay:compatible-diagnostics-richness-change"),
                )?,
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndRetentionNarrowing => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "replay:compatible-parameter-expansion-and-retention-narrowing",
                    ),
                )?,
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndDiagnosticsRichnessChange => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-parameter-expansion-and-diagnostics-richness-change",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "replay:compatible-parameter-expansion-and-diagnostics-richness-change",
                    ),
                )?,
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing-and-diagnostics-richness-change",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "replay:compatible-parameter-expansion-and-retention-narrowing-and-diagnostics-richness-change",
                    ),
                )?,
            ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-retention-narrowing-and-diagnostics-richness-change",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "replay:compatible-retention-narrowing-and-diagnostics-richness-change",
                    ),
                )?,
            ResourceReplayPolicyDeclaration::DenyOnUnknownOrMissing => self.built_in_policy(
                ResourcePolicyKind::Replay,
                "signal.resource.replay.deny-on-unknown-or-missing",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("replay:deny-on-unknown-or-missing"),
            )?,
            ResourceReplayPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Replay, name)?
            }
        })
    }
}

pub(crate) fn built_in_policy_registrations() -> Vec<ResourcePolicyRegistration> {
    let rows = [
        (
            0,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.disabled",
            5,
        ),
        (
            1,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.fixed-delay",
            5,
        ),
        (
            14,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.exponential-backoff",
            5,
        ),
        (
            15,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.capped-exponential-backoff",
            5,
        ),
        (
            2,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.disabled",
            4,
        ),
        (
            20,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.transaction-inherited-deadline",
            4,
        ),
        (
            21,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.runtime-inherited-deadline",
            4,
        ),
        (
            3,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.fixed-timeout",
            4,
        ),
        (
            16,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.total-request-lifetime-timeout",
            4,
        ),
        (
            17,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.progress-heartbeat-extension",
            4,
        ),
        (
            18,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.terminal-timeout",
            4,
        ),
        (
            19,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.revalidation-eligible-timeout",
            4,
        ),
        (
            4,
            ResourcePolicyKind::Cancellation,
            "signal.resource.cancellation.runtime-denial-only",
            3,
        ),
        (
            5,
            ResourcePolicyKind::Cancellation,
            "signal.resource.cancellation.best-effort-host-signal-and-runtime-denial",
            3,
        ),
        (
            6,
            ResourcePolicyKind::StaleAfter,
            "signal.resource.stale-after.disabled",
            7,
        ),
        (
            7,
            ResourcePolicyKind::StaleAfter,
            "signal.resource.stale-after.runtime-stale-after",
            7,
        ),
        (
            8,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.new-generation-supersedes-prior",
            1,
        ),
        (
            22,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.overlapping-generation-retains-old-host-work",
            1,
        ),
        (
            23,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.overlapping-generation-cancels-old-host-work",
            1,
        ),
        (
            24,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.intent-equivalent-coalesces-to-active",
            1,
        ),
        (
            9,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-intent-only",
            7,
        ),
        (
            25,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-active-handle-forced",
            7,
        ),
        (
            26,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-stale-after-fulfilled",
            7,
        ),
        (
            27,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-stale-after-fulfilled-or-active-handle-forced",
            7,
        ),
        (
            28,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-dependency-change",
            7,
        ),
        (
            29,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-dependency-change-or-active-handle-forced",
            7,
        ),
        (
            30,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-observer-demand",
            7,
        ),
        (
            31,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-observer-demand-or-active-handle-forced",
            7,
        ),
        (
            36,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand",
            7,
        ),
        (
            37,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand-or-active-handle-forced",
            7,
        ),
        (
            32,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-terminal-state",
            7,
        ),
        (
            33,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-terminal-state-or-active-handle-forced",
            7,
        ),
        (
            34,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-fulfilled-lifecycle",
            7,
        ),
        (
            35,
            ResourcePolicyKind::Revalidation,
            "signal.resource.revalidation.explicit-or-fulfilled-lifecycle-or-active-handle-forced",
            7,
        ),
        (
            10,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-only",
            7,
        ),
        (
            11,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-and-output",
            7,
        ),
        (
            39,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-output-and-denied-completion",
            7,
        ),
        (
            40,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-output-and-retry-schedule",
            7,
        ),
        (
            41,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-output-and-denied-completion-and-retry-schedule",
            7,
        ),
        (
            12,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.preserve-lifecycle-output-separation",
            7,
        ),
        (
            38,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-while-pending",
            7,
        ),
        (
            45,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-after-rejection",
            7,
        ),
        (
            42,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-after-timeout",
            7,
        ),
        (
            43,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-after-cancellation",
            7,
        ),
        (
            44,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-after-supersession",
            7,
        ),
        (
            13,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.terminal-summaries-only",
            7,
        ),
        (
            46,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.retain-all-transitions",
            7,
        ),
        (
            47,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.compact-superseded",
            7,
        ),
        (
            48,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.compact-cancelled",
            7,
        ),
        (
            49,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.compact-timed-out",
            7,
        ),
        (
            50,
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.retained-only",
            8,
        ),
        (
            51,
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.budgeted-expansion",
            8,
        ),
        (
            58,
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.forensic-expansion-budget",
            8,
        ),
        (
            52,
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.deny-cold-expansion",
            8,
        ),
        (
            53,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.identical-only",
            21,
        ),
        (
            59,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.compatible-parameter-expansion",
            21,
        ),
        (
            54,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.compatible-retention-narrowing",
            21,
        ),
        (
            55,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.compatible-diagnostics-richness-change",
            21,
        ),
        (
            61,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing",
            21,
        ),
        (
            62,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.compatible-parameter-expansion-and-diagnostics-richness-change",
            21,
        ),
        (
            56,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.compatible-retention-narrowing-and-diagnostics-richness-change",
            21,
        ),
        (
            60,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing-and-diagnostics-richness-change",
            21,
        ),
        (
            57,
            ResourcePolicyKind::Replay,
            "signal.resource.replay.deny-on-unknown-or-missing",
            21,
        ),
    ];

    rows.into_iter()
        .map(|(id, kind, name, contract)| {
            ResourcePolicyRegistration::new(
                ResourcePolicyDescriptorId::new(id),
                kind,
                ResourcePolicyName::new(name),
                ResourcePolicyVersion::INITIAL,
                ResourceCostContractId::new(contract),
                ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
            )
        })
        .collect()
}

fn descriptor_digest(
    id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: &ResourcePolicyName,
    version: ResourcePolicyVersion,
    cost_contract: ResourceCostContractId,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "resource-policy-descriptor:{}:{}:{}:{}.{}:{}",
        id.get(),
        kind.as_str(),
        semantic_name.as_str(),
        version.major(),
        version.minor(),
        cost_contract.get()
    ))
}

fn validate_named_policy_name(
    kind: ResourcePolicyKind,
    name: &ResourcePolicyName,
) -> Result<(), ResourcePolicyResolutionError> {
    validate_policy_registration_name(kind, name).map_err(|(kind, name, reason)| {
        ResourcePolicyResolutionError::MalformedDescriptor { kind, name, reason }
    })?;
    Ok(())
}

fn validate_policy_registration_name(
    kind: ResourcePolicyKind,
    name: &ResourcePolicyName,
) -> Result<(), (ResourcePolicyKind, ResourcePolicyName, &'static str)> {
    if name.as_str().trim().is_empty() {
        return Err((kind, name.clone(), "resource policy name must not be empty"));
    }
    Ok(())
}

fn ensure_compatible_descriptor(
    kind: ResourcePolicyKind,
    descriptor: &ResourcePolicyDescriptor,
) -> Result<(), ResourcePolicyResolutionError> {
    if descriptor.compatibility_posture() == ResourcePolicyCompatibilityPosture::IncompatibleVersion
    {
        return Err(ResourcePolicyResolutionError::IncompatibleDescriptor {
            kind,
            name: descriptor.semantic_name().clone(),
            version: descriptor.version(),
            compatibility_posture: descriptor.compatibility_posture(),
        });
    }
    Ok(())
}

fn registry_digest(descriptors: &[ResourcePolicyDescriptor]) -> ResourcePolicyDigest {
    let mut rows = descriptors
        .iter()
        .map(|descriptor| {
            format!(
                "{}:{}:{}:{}",
                descriptor.id().get(),
                descriptor.kind().as_str(),
                descriptor.semantic_name().as_str(),
                descriptor.descriptor_digest().as_str()
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    ResourcePolicyDigest::new(format!("resource-policy-registry:{}", rows.join("|")))
}

fn bundle_digest(policies: &[&FrozenResourcePolicyDescriptor]) -> ResourcePolicyDigest {
    let joined = policies
        .iter()
        .map(|policy| policy.frozen_digest().as_str())
        .collect::<Vec<_>>()
        .join("|");
    ResourcePolicyDigest::new(format!("resource-policy-bundle:{joined}"))
}

fn frozen_policy_descriptor_digest(
    descriptor: &ResourcePolicyDescriptor,
    selection_basis: ResourcePolicySelectionBasis,
    parameter_digest: &ResourcePolicyDigest,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "frozen-resource-policy:{}:{}:{}",
        descriptor.descriptor_digest().as_str(),
        selection_basis.as_str(),
        parameter_digest.as_str()
    ))
}

fn retry_parameter_digest(
    base: &str,
    max_attempts: Option<u32>,
    max_jitter: Option<crate::data::temporal::TemporalDuration>,
    retry_budget_scope: Option<ResourceRetryBudgetScope>,
    retry_budget_limit: Option<u32>,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "retry:{}:max-attempts:{}:deterministic-jitter:{}:retry-budget-scope:{}:retry-budget-limit:{}",
        base,
        max_attempts
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unbounded".to_owned()),
        max_jitter
            .map(|value| value.get().to_string())
            .unwrap_or_else(|| "none".to_owned()),
        retry_budget_scope
            .map(|value| value.as_str().to_owned())
            .unwrap_or_else(|| "none".to_owned()),
        retry_budget_limit
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unbounded".to_owned())
    ))
}

fn timeout_parameter_digest(
    family: &'static str,
    timeout: crate::data::temporal::TemporalDuration,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!("timeout:{family}:{}", timeout.get()))
}

fn diagnostics_parameter_digest(
    family: &'static str,
    max_replay_reconstruction_width: u32,
    max_forensic_reconstruction_width: Option<u32>,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "diagnostics:{family}:max-replay-reconstruction-width:{max_replay_reconstruction_width}:max-forensic-reconstruction-width:{}",
        max_forensic_reconstruction_width
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_owned())
    ))
}
