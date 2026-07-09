use super::{
    LiveViewDeclarationAdmissionBoundaryReceipt, SignalInvalidationBoundaryReceipt,
    SubscriptionActivationBoundaryReceipt, WorthQueryLowerRuntimeBoundaryEnvelope,
    WriteAuthorityExecutionReceipt,
};

pub trait WorthQueryLowerRuntimeBoundaryEnvelopeSource: sealed::Sealed {
    fn lower_runtime_boundary_envelope(&self) -> &WorthQueryLowerRuntimeBoundaryEnvelope;

    fn lower_runtime_boundary_source_kind(&self) -> &'static str;

    fn lower_runtime_boundary_source_identity(
        &self,
    ) -> &crate::evidence_identity::WorthQueryEvidenceIdentity {
        self.lower_runtime_boundary_envelope().envelope_identity()
    }
}

impl WorthQueryLowerRuntimeBoundaryEnvelopeSource for WorthQueryLowerRuntimeBoundaryEnvelope {
    fn lower_runtime_boundary_envelope(&self) -> &WorthQueryLowerRuntimeBoundaryEnvelope {
        self
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "lower-runtime-boundary-envelope"
    }
}

impl WorthQueryLowerRuntimeBoundaryEnvelopeSource for LiveViewDeclarationAdmissionBoundaryReceipt {
    fn lower_runtime_boundary_envelope(&self) -> &WorthQueryLowerRuntimeBoundaryEnvelope {
        self.boundary_envelope()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "live-view-declaration-admission-boundary-receipt"
    }
}

impl WorthQueryLowerRuntimeBoundaryEnvelopeSource for WriteAuthorityExecutionReceipt {
    fn lower_runtime_boundary_envelope(&self) -> &WorthQueryLowerRuntimeBoundaryEnvelope {
        self.boundary_envelope()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "write-authority-execution-receipt"
    }
}

impl WorthQueryLowerRuntimeBoundaryEnvelopeSource for SignalInvalidationBoundaryReceipt {
    fn lower_runtime_boundary_envelope(&self) -> &WorthQueryLowerRuntimeBoundaryEnvelope {
        self.boundary_envelope()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "signal-invalidation-boundary-receipt"
    }
}

impl WorthQueryLowerRuntimeBoundaryEnvelopeSource for SubscriptionActivationBoundaryReceipt {
    fn lower_runtime_boundary_envelope(&self) -> &WorthQueryLowerRuntimeBoundaryEnvelope {
        self.boundary_envelope()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "subscription-activation-boundary-receipt"
    }
}

pub(crate) mod sealed {
    use super::{
        LiveViewDeclarationAdmissionBoundaryReceipt, SignalInvalidationBoundaryReceipt,
        SubscriptionActivationBoundaryReceipt, WorthQueryLowerRuntimeBoundaryEnvelope,
        WriteAuthorityExecutionReceipt,
    };

    pub trait Sealed {}

    impl Sealed for WorthQueryLowerRuntimeBoundaryEnvelope {}
    impl Sealed for LiveViewDeclarationAdmissionBoundaryReceipt {}
    impl Sealed for WriteAuthorityExecutionReceipt {}
    impl Sealed for SignalInvalidationBoundaryReceipt {}
    impl Sealed for SubscriptionActivationBoundaryReceipt {}
}
