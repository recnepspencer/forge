use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

pub(in crate::logic::transaction::runtime::state::resource) fn resource_policy_resolution_signal_error(
    err: ResourcePolicyResolutionError,
) -> crate::data::error::SignalError {
    match err {
        ResourcePolicyResolutionError::UnknownPolicy { kind, name } => {
            crate::data::error::SignalError::invalid_input(format!(
                "unknown resource policy '{}' for {:?}",
                name.as_str(),
                kind
            ))
        }
        ResourcePolicyResolutionError::MissingDescriptor { kind, name } => {
            crate::data::error::SignalError::invalid_input(format!(
                "missing resource policy descriptor '{}' for {:?}",
                name.as_str(),
                kind
            ))
        }
        ResourcePolicyResolutionError::RegistryDigestDrift { expected, actual } => {
            crate::data::error::SignalError::invalid_input(format!(
                "resource policy registry digest drift during freeze: expected '{}', got '{}'",
                expected.as_str(),
                actual.as_str()
            ))
        }
        ResourcePolicyResolutionError::IncompatibleDescriptor {
            kind,
            name,
            version,
            compatibility_posture,
        } => crate::data::error::SignalError::invalid_input(format!(
            "incompatible resource policy descriptor '{}' for {:?} at version {}.{} with posture {:?}",
            name.as_str(),
            kind,
            version.major(),
            version.minor(),
            compatibility_posture
        )),
        ResourcePolicyResolutionError::MalformedDescriptor { kind, name, reason } => {
            crate::data::error::SignalError::invalid_input(format!(
                "malformed resource policy descriptor '{}' for {:?}: {}",
                name.as_str(),
                kind,
                reason
            ))
        }
        ResourcePolicyResolutionError::UnsupportedExecutablePolicy { kind, name, reason } => {
            crate::data::error::SignalError::invalid_input(format!(
                "resource policy descriptor '{}' for {:?} is not executable in the first ship runtime: {}",
                name.as_str(),
                kind,
                reason
            ))
        }
    }
}

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime) fn set_policy_registry(
        &mut self,
        policy_registry: FrozenResourcePolicyRegistry,
    ) {
        self.policy_registry = policy_registry;
    }

    pub(super) fn validated_policy_declaration(
        &self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<ValidatedResourcePolicyDeclaration, crate::data::error::SignalError> {
        ValidatedResourcePolicyDeclaration::from_declaration(declaration, &self.policy_registry)
            .map_err(resource_policy_resolution_signal_error)
    }

    pub fn validate_async_capability_declaration(
        &self,
        declaration: &ResourceNodeDeclaration,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ValidatedResourcePolicyDeclaration, crate::data::error::SignalError> {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.async_node_capability_validation_count += 1;
            telemetry.resource_policy_resolution_count += 1;
        }
        ValidatedResourcePolicyDeclaration::from_declaration(declaration, &self.policy_registry)
            .map_err(|err| {
                if let Some(telemetry) = telemetry {
                    telemetry.resource_policy_resolution_denial_count += 1;
                }
                resource_policy_resolution_signal_error(err)
            })
    }

    pub(in crate::logic::transaction::runtime) fn validate_resource_policy_declaration_without_async_accounting(
        &self,
        declaration: &ResourceNodeDeclaration,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<ValidatedResourcePolicyDeclaration, crate::data::error::SignalError> {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_policy_resolution_count += 1;
        }
        ValidatedResourcePolicyDeclaration::from_declaration(declaration, &self.policy_registry)
            .map_err(|err| {
                if let Some(telemetry) = telemetry {
                    telemetry.resource_policy_resolution_denial_count += 1;
                }
                resource_policy_resolution_signal_error(err)
            })
    }

    pub fn freeze_async_capability_declaration(
        &self,
        validated: &ValidatedResourcePolicyDeclaration,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<FrozenResourcePolicyDescriptorSet, crate::data::error::SignalError> {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.async_node_capability_freeze_count += 1;
        }
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            validated,
            &self.policy_registry,
        )
        .map_err(|err| {
            if let Some(telemetry) = telemetry {
                telemetry.resource_policy_resolution_denial_count += 1;
            }
            resource_policy_resolution_signal_error(err)
        })
    }

    pub(in crate::logic::transaction::runtime) fn freeze_resource_policy_declaration_without_async_accounting(
        &self,
        validated: &ValidatedResourcePolicyDeclaration,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> Result<FrozenResourcePolicyDescriptorSet, crate::data::error::SignalError> {
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            validated,
            &self.policy_registry,
        )
        .map_err(|err| {
            if let Some(telemetry) = telemetry {
                telemetry.resource_policy_resolution_denial_count += 1;
            }
            resource_policy_resolution_signal_error(err)
        })
    }

    pub fn lower_async_capability_bundle(
        &self,
        frozen: &FrozenResourcePolicyDescriptorSet,
        telemetry: Option<&mut ResourceTelemetry>,
    ) -> LoweredResourcePolicyBundle {
        if let Some(telemetry) = telemetry {
            telemetry.async_node_capability_bundle_lowering_count += 1;
        }
        LoweredResourcePolicyBundle::from_frozen_descriptors(frozen)
    }

    pub(in crate::logic::transaction::runtime) fn lower_resource_policy_bundle_without_async_accounting(
        &self,
        frozen: &FrozenResourcePolicyDescriptorSet,
    ) -> LoweredResourcePolicyBundle {
        LoweredResourcePolicyBundle::from_frozen_descriptors(frozen)
    }
}
