use super::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionDenial,
    BackendCapabilityAdmissionRequest, BackendCapabilityKind, BackendCapabilitySupportPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalBackendCapabilityAdmissionAuthority {
    _sealed: (),
}

impl PhysicalBackendCapabilityAdmissionAuthority {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn store_owned() -> Self {
        Self { _sealed: () }
    }

    pub fn admit_backend_capability(
        self,
        request: BackendCapabilityAdmissionRequest,
    ) -> Result<AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionDenial> {
        self.reject_base_staleness(request)?;
        Ok(AdmittedBackendCapabilityWitness::new(
            request.profile(),
            request.evidence_class(),
            request.support(),
            request.media_assumptions(),
            request.rebind_triggers(),
            request.confidence_limits(),
        ))
    }

    fn reject_base_staleness(
        self,
        request: BackendCapabilityAdmissionRequest,
    ) -> Result<(), BackendCapabilityAdmissionDenial> {
        let base = BackendCapabilityKind::BufferedFile;
        match request.support().posture(base) {
            BackendCapabilitySupportPosture::Supported => Ok(()),
            BackendCapabilitySupportPosture::Unsupported => {
                Err(BackendCapabilityAdmissionDenial::UnsupportedCapability {
                    kind: base,
                    posture: BackendCapabilitySupportPosture::Unsupported,
                })
            }
            BackendCapabilitySupportPosture::Unavailable => {
                Err(BackendCapabilityAdmissionDenial::UnavailableCapability {
                    kind: base,
                    posture: BackendCapabilitySupportPosture::Unavailable,
                })
            }
            BackendCapabilitySupportPosture::Unknown => {
                Err(BackendCapabilityAdmissionDenial::UnknownCapability {
                    kind: base,
                    posture: BackendCapabilitySupportPosture::Unknown,
                })
            }
            BackendCapabilitySupportPosture::Stale => {
                Err(BackendCapabilityAdmissionDenial::StaleCapability {
                    kind: base,
                    posture: BackendCapabilitySupportPosture::Stale,
                })
            }
            BackendCapabilitySupportPosture::RebindRequired => {
                Err(BackendCapabilityAdmissionDenial::RebindRequired {
                    kind: base,
                    triggers: request.rebind_triggers(),
                })
            }
        }
    }
}
