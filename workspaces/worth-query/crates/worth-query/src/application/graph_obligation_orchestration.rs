use crate::runtime::{
    WorthQueryAuthoritativeMutationObligationDispatch,
    WorthQueryAuthoritativeMutationObligationDispatchProjection,
    WorthQueryGraphObligationDenialProjection, WorthQueryGraphObligationDispatchContext,
    WorthQueryGraphObligationDispatchError, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationRegistrationCatalog,
    WorthQueryGraphObligationRegistrationDenial, WorthQueryGraphObligationSelection,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial,
    WorthQueryGraphTouchReadVerb,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationOrchestrationBoundary {
    DeclarationEntry,
    ContributionComposed,
}

impl WorthQueryGraphObligationOrchestrationBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationEntry => "declaration-entry",
            Self::ContributionComposed => "contribution-composed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphObligationOrchestrationDispatchError {
    MissingTouchCollection {
        boundary: WorthQueryGraphObligationOrchestrationBoundary,
    },
    TouchDescriptor(WorthQueryGraphTouchDescriptorDenial),
    Registration(WorthQueryGraphObligationRegistrationDenial),
    Dispatch(WorthQueryGraphObligationDispatchError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationOrchestrationDispatch {
    boundary: WorthQueryGraphObligationOrchestrationBoundary,
    operating_context_identity_digest: String,
    inner: WorthQueryAuthoritativeMutationObligationDispatch,
}

impl WorthQueryGraphObligationOrchestrationDispatch {
    fn new(
        boundary: WorthQueryGraphObligationOrchestrationBoundary,
        operating_context_identity_digest: impl Into<String>,
        inner: WorthQueryAuthoritativeMutationObligationDispatch,
    ) -> Self {
        Self {
            boundary,
            operating_context_identity_digest: operating_context_identity_digest.into(),
            inner,
        }
    }

    pub fn boundary(&self) -> WorthQueryGraphObligationOrchestrationBoundary {
        self.boundary
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn selection(&self) -> &WorthQueryGraphObligationSelection {
        self.inner.selection()
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.inner.envelope_digest()
    }

    pub fn dispatch_digest(&self) -> &str {
        self.inner.dispatch_digest()
    }

    pub fn blocking_denial_projection(&self) -> Option<WorthQueryGraphObligationDenialProjection> {
        self.inner.blocking_denial_projection()
    }

    pub fn evidence_projection(
        &self,
    ) -> WorthQueryAuthoritativeMutationObligationDispatchProjection {
        self.inner.evidence_projection()
    }
}

pub(crate) fn dispatch_graph_obligations_for_orchestration(
    boundary: WorthQueryGraphObligationOrchestrationBoundary,
    operating_context_identity_digest: &str,
    touch_descriptor: Option<
        Result<WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial>,
    >,
    touch_collection: Option<&'static str>,
    registrations: Vec<WorthQueryGraphObligationRegistration>,
) -> Result<
    Option<WorthQueryGraphObligationOrchestrationDispatch>,
    WorthQueryGraphObligationOrchestrationDispatchError,
> {
    if registrations.is_empty() {
        return Ok(None);
    }
    let descriptor = match touch_descriptor {
        Some(descriptor) => descriptor
            .map_err(WorthQueryGraphObligationOrchestrationDispatchError::TouchDescriptor)?,
        None => {
            let Some(touch_collection) = touch_collection else {
                return Err(
                    WorthQueryGraphObligationOrchestrationDispatchError::MissingTouchCollection {
                        boundary,
                    },
                );
            };
            WorthQueryGraphTouchDescriptor::read_family(
                touch_collection,
                [WorthQueryGraphTouchReadVerb::ExposesDerivedTopology],
            )
            .map_err(WorthQueryGraphObligationOrchestrationDispatchError::TouchDescriptor)?
        }
    };
    let catalog = WorthQueryGraphObligationRegistrationCatalog::from_registrations(registrations)
        .map_err(WorthQueryGraphObligationOrchestrationDispatchError::Registration)?;
    let index = crate::runtime::WorthQueryGraphObligationIndex::from_catalog(&catalog);
    let operating_world =
        WorthQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
    let context = orchestration_context(
        boundary,
        descriptor.descriptor_digest(),
        operating_world.descriptor_digest(),
    )
    .map_err(WorthQueryGraphObligationOrchestrationDispatchError::Dispatch)?;
    let selection = index.select_for_touch(&descriptor, &operating_world);
    let inner =
        WorthQueryAuthoritativeMutationObligationDispatch::from_selection(context, selection)
            .map_err(WorthQueryGraphObligationOrchestrationDispatchError::Dispatch)?;
    Ok(Some(WorthQueryGraphObligationOrchestrationDispatch::new(
        boundary,
        operating_context_identity_digest,
        inner,
    )))
}

fn orchestration_context(
    boundary: WorthQueryGraphObligationOrchestrationBoundary,
    touch_descriptor_digest: &str,
    operating_world_digest: &str,
) -> Result<WorthQueryGraphObligationDispatchContext, WorthQueryGraphObligationDispatchError> {
    match boundary {
        WorthQueryGraphObligationOrchestrationBoundary::DeclarationEntry => {
            WorthQueryGraphObligationDispatchContext::declaration_entry(
                touch_descriptor_digest,
                operating_world_digest,
            )
        }
        WorthQueryGraphObligationOrchestrationBoundary::ContributionComposed => {
            WorthQueryGraphObligationDispatchContext::contribution_composed(
                touch_descriptor_digest,
                operating_world_digest,
            )
        }
    }
}
