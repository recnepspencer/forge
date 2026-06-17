use super::{
    ForgeQueryLowerRuntimeBoundaryEnvelope, LiveViewDeclarationAdmissionBoundaryReceipt,
    SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
    WriteAuthorityExecutionReceipt,
};

pub trait ForgeQueryLowerRuntimeBoundaryEnvelopeSource: sealed::Sealed {
    fn lower_runtime_boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope;

    fn lower_runtime_boundary_source_kind(&self) -> &'static str;

    fn lower_runtime_boundary_source_identity(
        &self,
    ) -> &crate::evidence_identity::ForgeQueryEvidenceIdentity {
        self.lower_runtime_boundary_envelope().envelope_identity()
    }
}

impl ForgeQueryLowerRuntimeBoundaryEnvelopeSource for ForgeQueryLowerRuntimeBoundaryEnvelope {
    fn lower_runtime_boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        self
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "lower-runtime-boundary-envelope"
    }
}

impl ForgeQueryLowerRuntimeBoundaryEnvelopeSource for LiveViewDeclarationAdmissionBoundaryReceipt {
    fn lower_runtime_boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        self.boundary_envelope()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "live-view-declaration-admission-boundary-receipt"
    }
}

impl ForgeQueryLowerRuntimeBoundaryEnvelopeSource for WriteAuthorityExecutionReceipt {
    fn lower_runtime_boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        self.boundary_envelope()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "write-authority-execution-receipt"
    }
}

impl ForgeQueryLowerRuntimeBoundaryEnvelopeSource for SignalInvalidationBoundaryReceipt {
    fn lower_runtime_boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        self.boundary_envelope()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "signal-invalidation-boundary-receipt"
    }
}

impl ForgeQueryLowerRuntimeBoundaryEnvelopeSource for SubscriptionActivationBoundaryReceipt {
    fn lower_runtime_boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        self.boundary_envelope()
    }

    fn lower_runtime_boundary_source_kind(&self) -> &'static str {
        "subscription-activation-boundary-receipt"
    }
}

pub(crate) mod sealed {
    use super::{
        ForgeQueryLowerRuntimeBoundaryEnvelope, LiveViewDeclarationAdmissionBoundaryReceipt,
        SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
        WriteAuthorityExecutionReceipt,
    };

    pub trait Sealed {}

    impl Sealed for ForgeQueryLowerRuntimeBoundaryEnvelope {}
    impl Sealed for LiveViewDeclarationAdmissionBoundaryReceipt {}
    impl Sealed for WriteAuthorityExecutionReceipt {}
    impl Sealed for SignalInvalidationBoundaryReceipt {}
    impl Sealed for SubscriptionActivationBoundaryReceipt {}
}
