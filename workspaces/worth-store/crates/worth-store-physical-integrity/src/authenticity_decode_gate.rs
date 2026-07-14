use crate::{
    AuthenticityPolicyDecodeCounters, AuthenticityRequiredDecodeCounters, IntegrityCheckedFrame,
    IntegrityCheckedPage, IntegrityCheckedPhysicalFormKind, LogicalDecodeGate,
    PreDecodePhysicalDenial, PreDecodePhysicalDenialKind, ProtectedPhysicalByteView,
};
use worth_store_physical_format::PhysicalAuthenticityIdentity;
use worth_store_security::{
    StoreAuthenticityCheckDenial, StoreAuthenticityCheckDenialKind, StoreAuthenticityResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticityRequiredPhysicalDecodeGate<'lease> {
    gate: LogicalDecodeGate<'lease>,
    form_kind: IntegrityCheckedPhysicalFormKind,
    authenticity_result: StoreAuthenticityResult<PhysicalAuthenticityIdentity>,
    counters: AuthenticityRequiredDecodeCounters,
}

impl<'lease> AuthenticityRequiredPhysicalDecodeGate<'lease> {
    pub fn admit_page(
        checked: IntegrityCheckedPage<'lease>,
        authenticity: Result<
            StoreAuthenticityResult<PhysicalAuthenticityIdentity>,
            StoreAuthenticityCheckDenial,
        >,
    ) -> Result<Self, PreDecodePhysicalDenial> {
        let expected_identity = authenticity_physical_identity(checked.gate_evidence().identity());
        admit_checked_form(
            checked.checked_bytes(),
            checked.logical_decode_gate(),
            checked.kind(),
            checked.counters(),
            expected_identity,
            authenticity,
        )
    }

    pub fn admit_frame(
        checked: IntegrityCheckedFrame<'lease>,
        authenticity: Result<
            StoreAuthenticityResult<PhysicalAuthenticityIdentity>,
            StoreAuthenticityCheckDenial,
        >,
    ) -> Result<Self, PreDecodePhysicalDenial> {
        let expected_identity = authenticity_physical_identity(checked.gate_evidence().identity());
        admit_checked_form(
            checked.checked_bytes(),
            checked.logical_decode_gate(),
            checked.kind(),
            checked.counters(),
            expected_identity,
            authenticity,
        )
    }

    pub const fn logical_decode_gate(&self) -> LogicalDecodeGate<'lease> {
        self.gate
    }

    pub const fn form_kind(&self) -> IntegrityCheckedPhysicalFormKind {
        self.form_kind
    }

    pub const fn authenticity_result(
        &self,
    ) -> StoreAuthenticityResult<PhysicalAuthenticityIdentity> {
        self.authenticity_result
    }

    pub const fn counters(&self) -> AuthenticityRequiredDecodeCounters {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticityPolicyPhysicalDecodeGate<'lease> {
    gate: LogicalDecodeGate<'lease>,
    form_kind: IntegrityCheckedPhysicalFormKind,
    authenticity_result: Option<StoreAuthenticityResult<PhysicalAuthenticityIdentity>>,
    counters: AuthenticityPolicyDecodeCounters,
}

impl<'lease> AuthenticityPolicyPhysicalDecodeGate<'lease> {
    pub fn admit_page(
        checked: IntegrityCheckedPage<'lease>,
        authenticity: Result<
            StoreAuthenticityResult<PhysicalAuthenticityIdentity>,
            StoreAuthenticityCheckDenial,
        >,
    ) -> Result<Self, PreDecodePhysicalDenial> {
        let expected_identity = authenticity_physical_identity(checked.gate_evidence().identity());
        admit_policy_checked_form(
            checked.checked_bytes(),
            checked.logical_decode_gate(),
            checked.kind(),
            checked.counters(),
            expected_identity,
            authenticity,
        )
    }

    pub fn admit_frame(
        checked: IntegrityCheckedFrame<'lease>,
        authenticity: Result<
            StoreAuthenticityResult<PhysicalAuthenticityIdentity>,
            StoreAuthenticityCheckDenial,
        >,
    ) -> Result<Self, PreDecodePhysicalDenial> {
        let expected_identity = authenticity_physical_identity(checked.gate_evidence().identity());
        admit_policy_checked_form(
            checked.checked_bytes(),
            checked.logical_decode_gate(),
            checked.kind(),
            checked.counters(),
            expected_identity,
            authenticity,
        )
    }

    pub const fn logical_decode_gate(&self) -> LogicalDecodeGate<'lease> {
        self.gate
    }

    pub const fn form_kind(&self) -> IntegrityCheckedPhysicalFormKind {
        self.form_kind
    }

    pub const fn authenticity_result(
        &self,
    ) -> Option<StoreAuthenticityResult<PhysicalAuthenticityIdentity>> {
        self.authenticity_result
    }

    pub const fn counters(&self) -> AuthenticityPolicyDecodeCounters {
        self.counters
    }
}

fn admit_checked_form<'lease>(
    view: ProtectedPhysicalByteView<'lease>,
    gate: LogicalDecodeGate<'lease>,
    form_kind: IntegrityCheckedPhysicalFormKind,
    integrity_counters: crate::PreDecodeAdmissionCounters,
    expected_identity: PhysicalAuthenticityIdentity,
    authenticity: Result<
        StoreAuthenticityResult<PhysicalAuthenticityIdentity>,
        StoreAuthenticityCheckDenial,
    >,
) -> Result<AuthenticityRequiredPhysicalDecodeGate<'lease>, PreDecodePhysicalDenial> {
    match authenticity {
        Ok(result) => {
            if result.physical_identity() != expected_identity {
                return Err(PreDecodePhysicalDenial::after_checksum(
                    PreDecodePhysicalDenialKind::AuthenticityResultPhysicalIdentityMismatch,
                    view,
                ));
            }
            Ok(AuthenticityRequiredPhysicalDecodeGate {
                gate,
                form_kind,
                counters: AuthenticityRequiredDecodeCounters::admitted(
                    integrity_counters,
                    result.counters(),
                ),
                authenticity_result: result,
            })
        }
        Err(denial) => Err(PreDecodePhysicalDenial::after_checksum(
            PreDecodePhysicalDenialKind::AuthenticityRequiredPhysicalDenial,
            view,
        )
        .with_authenticity_denial(denial)),
    }
}

fn admit_policy_checked_form<'lease>(
    view: ProtectedPhysicalByteView<'lease>,
    gate: LogicalDecodeGate<'lease>,
    form_kind: IntegrityCheckedPhysicalFormKind,
    integrity_counters: crate::PreDecodeAdmissionCounters,
    expected_identity: PhysicalAuthenticityIdentity,
    authenticity: Result<
        StoreAuthenticityResult<PhysicalAuthenticityIdentity>,
        StoreAuthenticityCheckDenial,
    >,
) -> Result<AuthenticityPolicyPhysicalDecodeGate<'lease>, PreDecodePhysicalDenial> {
    match authenticity {
        Ok(result) => {
            if result.physical_identity() != expected_identity {
                return Err(PreDecodePhysicalDenial::after_checksum(
                    PreDecodePhysicalDenialKind::AuthenticityResultPhysicalIdentityMismatch,
                    view,
                ));
            }
            Ok(AuthenticityPolicyPhysicalDecodeGate {
                gate,
                form_kind,
                counters: AuthenticityPolicyDecodeCounters::new(
                    integrity_counters,
                    result.counters(),
                ),
                authenticity_result: Some(result),
            })
        }
        Err(denial) if denial.kind() == StoreAuthenticityCheckDenialKind::ResultNotRequired => {
            Ok(AuthenticityPolicyPhysicalDecodeGate {
                gate,
                form_kind,
                counters: AuthenticityPolicyDecodeCounters::new(
                    integrity_counters,
                    denial.counters(),
                ),
                authenticity_result: None,
            })
        }
        Err(denial) => Err(PreDecodePhysicalDenial::after_checksum(
            PreDecodePhysicalDenialKind::AuthenticityRequiredPhysicalDenial,
            view,
        )
        .with_authenticity_denial(denial)),
    }
}

fn authenticity_physical_identity(
    identity: &crate::LogicalDecodeGateIdentity,
) -> PhysicalAuthenticityIdentity {
    PhysicalAuthenticityIdentity::new(
        identity.header_kind(),
        identity.locality(),
        identity.checked_byte_count(),
        identity.checksum_value(),
        identity.checksum_algorithm(),
    )
}
