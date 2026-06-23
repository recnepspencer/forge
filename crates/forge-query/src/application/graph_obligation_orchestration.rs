use crate::runtime::{
    ForgeQueryAuthoritativeMutationObligationDispatch,
    ForgeQueryAuthoritativeMutationObligationDispatchProjection,
    ForgeQueryGraphObligationDenialProjection, ForgeQueryGraphObligationDispatchContext,
    ForgeQueryGraphObligationDispatchError, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationRegistrationCatalog,
    ForgeQueryGraphObligationRegistrationDenial, ForgeQueryGraphObligationSelection,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial,
    ForgeQueryGraphTouchReadVerb,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationOrchestrationBoundary {
    DeclarationEntry,
    ContributionComposed,
}

impl ForgeQueryGraphObligationOrchestrationBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationEntry => "declaration-entry",
            Self::ContributionComposed => "contribution-composed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphObligationOrchestrationDispatchError {
    MissingTouchCollection {
        boundary: ForgeQueryGraphObligationOrchestrationBoundary,
    },
    TouchDescriptor(ForgeQueryGraphTouchDescriptorDenial),
    Registration(ForgeQueryGraphObligationRegistrationDenial),
    Dispatch(ForgeQueryGraphObligationDispatchError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationOrchestrationDispatch {
    boundary: ForgeQueryGraphObligationOrchestrationBoundary,
    operating_context_identity_digest: String,
    inner: ForgeQueryAuthoritativeMutationObligationDispatch,
}

impl ForgeQueryGraphObligationOrchestrationDispatch {
    fn new(
        boundary: ForgeQueryGraphObligationOrchestrationBoundary,
        operating_context_identity_digest: impl Into<String>,
        inner: ForgeQueryAuthoritativeMutationObligationDispatch,
    ) -> Self {
        Self {
            boundary,
            operating_context_identity_digest: operating_context_identity_digest.into(),
            inner,
        }
    }

    pub fn boundary(&self) -> ForgeQueryGraphObligationOrchestrationBoundary {
        self.boundary
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn selection(&self) -> &ForgeQueryGraphObligationSelection {
        self.inner.selection()
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.inner.envelope_digest()
    }

    pub fn dispatch_digest(&self) -> &str {
        self.inner.dispatch_digest()
    }

    pub fn blocking_denial_projection(&self) -> Option<ForgeQueryGraphObligationDenialProjection> {
        self.inner.blocking_denial_projection()
    }

    pub fn evidence_projection(
        &self,
    ) -> ForgeQueryAuthoritativeMutationObligationDispatchProjection {
        self.inner.evidence_projection()
    }
}

pub(crate) fn dispatch_graph_obligations_for_orchestration(
    boundary: ForgeQueryGraphObligationOrchestrationBoundary,
    operating_context_identity_digest: &str,
    touch_descriptor: Option<
        Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial>,
    >,
    touch_collection: Option<&'static str>,
    registrations: Vec<ForgeQueryGraphObligationRegistration>,
) -> Result<
    Option<ForgeQueryGraphObligationOrchestrationDispatch>,
    ForgeQueryGraphObligationOrchestrationDispatchError,
> {
    if registrations.is_empty() {
        return Ok(None);
    }
    let descriptor = match touch_descriptor {
        Some(descriptor) => descriptor
            .map_err(ForgeQueryGraphObligationOrchestrationDispatchError::TouchDescriptor)?,
        None => {
            let Some(touch_collection) = touch_collection else {
                return Err(
                    ForgeQueryGraphObligationOrchestrationDispatchError::MissingTouchCollection {
                        boundary,
                    },
                );
            };
            ForgeQueryGraphTouchDescriptor::read_family(
                touch_collection,
                [ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology],
            )
            .map_err(ForgeQueryGraphObligationOrchestrationDispatchError::TouchDescriptor)?
        }
    };
    let catalog = ForgeQueryGraphObligationRegistrationCatalog::from_registrations(registrations)
        .map_err(ForgeQueryGraphObligationOrchestrationDispatchError::Registration)?;
    let index = crate::runtime::ForgeQueryGraphObligationIndex::from_catalog(&catalog);
    let operating_world =
        ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
    let context = orchestration_context(
        boundary,
        descriptor.descriptor_digest(),
        operating_world.descriptor_digest(),
    )
    .map_err(ForgeQueryGraphObligationOrchestrationDispatchError::Dispatch)?;
    let selection = index.select_for_touch(&descriptor, &operating_world);
    let inner =
        ForgeQueryAuthoritativeMutationObligationDispatch::from_selection(context, selection)
            .map_err(ForgeQueryGraphObligationOrchestrationDispatchError::Dispatch)?;
    Ok(Some(ForgeQueryGraphObligationOrchestrationDispatch::new(
        boundary,
        operating_context_identity_digest,
        inner,
    )))
}

fn orchestration_context(
    boundary: ForgeQueryGraphObligationOrchestrationBoundary,
    touch_descriptor_digest: &str,
    operating_world_digest: &str,
) -> Result<ForgeQueryGraphObligationDispatchContext, ForgeQueryGraphObligationDispatchError> {
    match boundary {
        ForgeQueryGraphObligationOrchestrationBoundary::DeclarationEntry => {
            ForgeQueryGraphObligationDispatchContext::declaration_entry(
                touch_descriptor_digest,
                operating_world_digest,
            )
        }
        ForgeQueryGraphObligationOrchestrationBoundary::ContributionComposed => {
            ForgeQueryGraphObligationDispatchContext::contribution_composed(
                touch_descriptor_digest,
                operating_world_digest,
            )
        }
    }
}
