use core::num::NonZeroU64;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiHostFocusPlacementRequestIdentity(NonZeroU64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostFocusPlacementTarget {
    mounted_instance: crate::UiMountedInstanceIdentity,
    node_receipt: crate::UiMountedNodeReceiptIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostFocusPlacementRequest {
    identity: UiHostFocusPlacementRequestIdentity,
    protocol: crate::UiHostProtocolAgreement,
    host_session: u64,
    host_surface: crate::UiHostSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    presentation: crate::UiHostObservationPresentationBasis,
    target: UiHostFocusPlacementTarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostFocusPlacementRequestInput {
    pub identity: UiHostFocusPlacementRequestIdentity,
    pub protocol: crate::UiHostProtocolAgreement,
    pub host_session: u64,
    pub host_surface: crate::UiHostSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub presentation: crate::UiHostObservationPresentationBasis,
    pub target: UiHostFocusPlacementTarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostFocusPlacementRequestDenial {
    MissingHostSession,
    ForeignSurface,
    StaleBinding,
    TargetFrameMismatch,
    TargetInstanceMismatch,
}

pub type UiHostFocusPlacementDisposition =
    super::UiHostSolicitedEffectOutcome<UiHostFocusPlacementRejection>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostFocusPlacementRejection {
    Unsupported,
    ForeignSurface,
    StalePresentation,
    UnknownTarget,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostFocusPlacementAcknowledgement {
    request: UiHostFocusPlacementRequest,
    disposition: UiHostFocusPlacementDisposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostFocusPlacementObservation {
    request: UiHostFocusPlacementRequestIdentity,
    protocol: crate::UiHostProtocolAgreement,
    host_session: u64,
    host_surface: crate::UiHostSurfaceIdentity,
    presentation: crate::UiHostObservationPresentationBasis,
    observed_target: Option<UiHostFocusPlacementTarget>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostFocusPlacementObservationInput {
    pub request: UiHostFocusPlacementRequestIdentity,
    pub protocol: crate::UiHostProtocolAgreement,
    pub host_session: u64,
    pub host_surface: crate::UiHostSurfaceIdentity,
    pub presentation: crate::UiHostObservationPresentationBasis,
    pub observed_target: Option<UiHostFocusPlacementTarget>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostFocusPlacementObservationDenial {
    MissingHostSession,
    ForeignSurface,
    TargetFrameMismatch,
    TargetInstanceMismatch,
}

impl UiHostFocusPlacementRequestIdentity {
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.0.get()
    }
}

impl UiHostFocusPlacementTarget {
    pub const fn new(
        mounted_instance: crate::UiMountedInstanceIdentity,
        node_receipt: crate::UiMountedNodeReceiptIdentity,
    ) -> Self {
        Self {
            mounted_instance,
            node_receipt,
        }
    }

    pub const fn mounted_instance(self) -> crate::UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub const fn node_receipt(self) -> crate::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
}

impl UiHostFocusPlacementRequest {
    pub fn new(
        input: UiHostFocusPlacementRequestInput,
    ) -> Result<Self, UiHostFocusPlacementRequestDenial> {
        if input.host_session == 0 {
            return Err(UiHostFocusPlacementRequestDenial::MissingHostSession);
        }
        if input.presentation.host_surface() != input.host_surface {
            return Err(UiHostFocusPlacementRequestDenial::ForeignSurface);
        }
        if input.presentation.binding() != input.binding {
            return Err(UiHostFocusPlacementRequestDenial::StaleBinding);
        }
        match crate::mounted_target_coherence::validate_mounted_target_coherence(
            input.presentation,
            input.target.mounted_instance(),
            input.target.node_receipt(),
        ) {
            Ok(()) => {}
            Err(crate::mounted_target_coherence::UiMountedTargetCoherenceDenial::ForeignFrame) => {
                return Err(UiHostFocusPlacementRequestDenial::TargetFrameMismatch);
            }
            Err(
                crate::mounted_target_coherence::UiMountedTargetCoherenceDenial::ForeignInstance,
            ) => {
                return Err(UiHostFocusPlacementRequestDenial::TargetInstanceMismatch);
            }
        }
        Ok(Self {
            identity: input.identity,
            protocol: input.protocol,
            host_session: input.host_session,
            host_surface: input.host_surface,
            binding: input.binding,
            presentation: input.presentation,
            target: input.target,
        })
    }

    pub const fn identity(self) -> UiHostFocusPlacementRequestIdentity {
        self.identity
    }

    pub const fn protocol(self) -> crate::UiHostProtocolAgreement {
        self.protocol
    }

    pub const fn host_session(self) -> u64 {
        self.host_session
    }

    pub const fn host_surface(self) -> crate::UiHostSurfaceIdentity {
        self.host_surface
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn presentation(self) -> crate::UiHostObservationPresentationBasis {
        self.presentation
    }

    pub const fn target(self) -> UiHostFocusPlacementTarget {
        self.target
    }
}

impl UiHostFocusPlacementAcknowledgement {
    pub const fn settled(
        request: UiHostFocusPlacementRequest,
        disposition: UiHostFocusPlacementDisposition,
    ) -> Self {
        Self {
            request,
            disposition,
        }
    }

    pub const fn request(self) -> UiHostFocusPlacementRequest {
        self.request
    }

    pub const fn disposition(self) -> UiHostFocusPlacementDisposition {
        self.disposition
    }
}

impl UiHostFocusPlacementObservation {
    pub fn current(
        input: UiHostFocusPlacementObservationInput,
    ) -> Result<Self, UiHostFocusPlacementObservationDenial> {
        if input.host_session == 0 {
            return Err(UiHostFocusPlacementObservationDenial::MissingHostSession);
        }
        if input.presentation.host_surface() != input.host_surface {
            return Err(UiHostFocusPlacementObservationDenial::ForeignSurface);
        }
        if let Some(target) = input.observed_target {
            match crate::mounted_target_coherence::validate_mounted_target_coherence(
                input.presentation,
                target.mounted_instance(),
                target.node_receipt(),
            ) {
                Ok(()) => {}
                Err(crate::mounted_target_coherence::UiMountedTargetCoherenceDenial::ForeignFrame) => {
                    return Err(UiHostFocusPlacementObservationDenial::TargetFrameMismatch);
                }
                Err(crate::mounted_target_coherence::UiMountedTargetCoherenceDenial::ForeignInstance) => {
                    return Err(UiHostFocusPlacementObservationDenial::TargetInstanceMismatch);
                }
            }
        }
        Ok(Self {
            request: input.request,
            protocol: input.protocol,
            host_session: input.host_session,
            host_surface: input.host_surface,
            presentation: input.presentation,
            observed_target: input.observed_target,
        })
    }

    pub const fn request(self) -> UiHostFocusPlacementRequestIdentity {
        self.request
    }
    pub const fn protocol(self) -> crate::UiHostProtocolAgreement {
        self.protocol
    }
    pub const fn host_session(self) -> u64 {
        self.host_session
    }
    pub const fn host_surface(self) -> crate::UiHostSurfaceIdentity {
        self.host_surface
    }
    pub const fn presentation(self) -> crate::UiHostObservationPresentationBasis {
        self.presentation
    }
    pub const fn observed_target(self) -> Option<UiHostFocusPlacementTarget> {
        self.observed_target
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UiHostFocusPlacementAcknowledgement, UiHostFocusPlacementDisposition,
        UiHostFocusPlacementRequest, UiHostFocusPlacementRequestDenial,
        UiHostFocusPlacementRequestIdentity, UiHostFocusPlacementRequestInput,
        UiHostFocusPlacementTarget,
    };

    #[test]
    fn focus_placement_requires_exact_surface_binding_and_target_instance() {
        let surface = crate::UiHostSurfaceIdentity::mint_unbound().unwrap();
        let foreign_surface = crate::UiHostSurfaceIdentity::mint_unbound().unwrap();
        let binding = crate::UiSurfaceBindingGeneration::mint_unbound().unwrap();
        let frame = crate::UiMountedFrameIdentity::mint_unbound().unwrap();
        let instance = crate::UiMountedInstanceIdentity::mint_unbound().unwrap();
        let issuer = crate::UiMountedNodeReceiptIssuer::mint_for(frame).unwrap();
        let target = UiHostFocusPlacementTarget::new(instance, issuer.receipt_for(instance));
        let presentation = crate::UiHostObservationPresentationBasis::new(
            surface,
            frame,
            binding,
            crate::UiHostPresentationEpoch::issued_by_host(9),
        );
        let protocol = match crate::UiHostProtocolContract::current().negotiate() {
            crate::UiHostProtocolNegotiation::Compatible(protocol) => protocol,
            crate::UiHostProtocolNegotiation::Incompatible(_) => unreachable!(),
        };
        let input = UiHostFocusPlacementRequestInput {
            identity: UiHostFocusPlacementRequestIdentity::new(1).unwrap(),
            protocol,
            host_session: 4,
            host_surface: surface,
            binding,
            presentation,
            target,
        };
        assert!(UiHostFocusPlacementRequest::new(input).is_ok());
        assert_eq!(
            UiHostFocusPlacementRequest::new(UiHostFocusPlacementRequestInput {
                host_surface: foreign_surface,
                ..input
            }),
            Err(UiHostFocusPlacementRequestDenial::ForeignSurface)
        );
        assert_eq!(
            UiHostFocusPlacementRequest::new(UiHostFocusPlacementRequestInput {
                host_session: 0,
                ..input
            }),
            Err(UiHostFocusPlacementRequestDenial::MissingHostSession)
        );
        assert_eq!(
            UiHostFocusPlacementRequest::new(UiHostFocusPlacementRequestInput {
                binding: crate::UiSurfaceBindingGeneration::mint_unbound().unwrap(),
                ..input
            }),
            Err(UiHostFocusPlacementRequestDenial::StaleBinding)
        );

        let foreign_instance = crate::UiMountedInstanceIdentity::mint_unbound().unwrap();
        assert_eq!(
            UiHostFocusPlacementRequest::new(UiHostFocusPlacementRequestInput {
                target: UiHostFocusPlacementTarget::new(
                    foreign_instance,
                    issuer.receipt_for(instance),
                ),
                ..input
            }),
            Err(UiHostFocusPlacementRequestDenial::TargetInstanceMismatch)
        );

        let foreign_frame = crate::UiMountedFrameIdentity::mint_unbound().unwrap();
        let foreign_frame_issuer =
            crate::UiMountedNodeReceiptIssuer::mint_for(foreign_frame).unwrap();
        assert_eq!(
            UiHostFocusPlacementRequest::new(UiHostFocusPlacementRequestInput {
                target: UiHostFocusPlacementTarget::new(
                    instance,
                    foreign_frame_issuer.receipt_for(instance),
                ),
                ..input
            }),
            Err(UiHostFocusPlacementRequestDenial::TargetFrameMismatch)
        );

        let request = UiHostFocusPlacementRequest::new(input).unwrap();
        let acknowledgement = UiHostFocusPlacementAcknowledgement::settled(
            request,
            UiHostFocusPlacementDisposition::Applied,
        );
        assert_eq!(acknowledgement.request(), request);
        assert_eq!(acknowledgement.request().host_session(), 4);
        assert_eq!(acknowledgement.request().presentation(), presentation);
    }
}
