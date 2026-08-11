use crate::basis::ExecutionBasisIntent;
use crate::binding::{
    derive_binding_requirements, resolve_bindings, BindingResolution, BoundBindings,
    NonIdentityBindingMetadata,
};
use crate::validation::ValidatedQueryBundle;

use super::errors::PlanningError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningSemanticInputs {
    binding_resolution: Option<BindingResolution>,
    basis_intent: ExecutionBasisIntent,
}

impl PlanningSemanticInputs {
    pub(crate) fn new(
        binding_resolution: Option<BindingResolution>,
        basis_intent: ExecutionBasisIntent,
    ) -> Self {
        Self {
            binding_resolution,
            basis_intent,
        }
    }

    pub fn binding_resolution(&self) -> Option<&BindingResolution> {
        self.binding_resolution.as_ref()
    }

    pub fn basis_intent(&self) -> &ExecutionBasisIntent {
        &self.basis_intent
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningAmbientContext {
    metadata: Vec<NonIdentityBindingMetadata>,
}

impl PlanningAmbientContext {
    pub(crate) fn new(metadata: Vec<NonIdentityBindingMetadata>) -> Self {
        Self { metadata }
    }

    pub fn metadata(&self) -> &[NonIdentityBindingMetadata] {
        &self.metadata
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningRequestContext {
    semantic: PlanningSemanticInputs,
    ambient: PlanningAmbientContext,
}

impl PlanningRequestContext {
    pub(crate) fn new(semantic: PlanningSemanticInputs, ambient: PlanningAmbientContext) -> Self {
        Self { semantic, ambient }
    }

    pub fn semantic(&self) -> &PlanningSemanticInputs {
        &self.semantic
    }

    pub fn ambient(&self) -> &PlanningAmbientContext {
        &self.ambient
    }
}

pub fn planning_request_context_for_direct(
    bundle: &ValidatedQueryBundle,
    basis_intent: ExecutionBasisIntent,
) -> Result<PlanningRequestContext, PlanningError> {
    if !bundle.query().identity_bindings().is_empty() {
        return Err(PlanningError::MissingBindingResolutionForIdentityBoundQuery);
    }

    Ok(PlanningRequestContext::new(
        PlanningSemanticInputs::new(None, basis_intent),
        PlanningAmbientContext::new(Vec::new()),
    ))
}

pub fn planning_request_context_for_bound(
    bundle: &ValidatedQueryBundle,
    basis_intent: ExecutionBasisIntent,
    bindings: BoundBindings,
    ambient_metadata: Vec<NonIdentityBindingMetadata>,
) -> Result<PlanningRequestContext, PlanningError> {
    let requirements = derive_binding_requirements(bundle);
    let resolution = resolve_bindings(requirements, bindings).map_err(|error| {
        PlanningError::BindingResolutionFailed {
            failure_digest: error.failure_digest(),
        }
    })?;

    Ok(PlanningRequestContext::new(
        PlanningSemanticInputs::new(Some(resolution), basis_intent),
        PlanningAmbientContext::new(ambient_metadata),
    ))
}
