//! Declared application-aftermath contract entry point.

use super::{
    DeclaredCorrectionAuthority, DeclaredCorrectionMechanism, DeclaredReconciliationProcedure,
};

/// Portable declared aftermath contract for one mutation operation.
///
/// Callers declare correction authority and, unless authority is
/// `NotCorrectable`, exactly one correction mechanism. The four law-14 posture
/// names are not constructible here.
///
/// Whether the operation escapes into an external rail is deliberately *not* an
/// axis of this contract. The escaping lane is declared once, on the schema, by
/// the operation definition's external-effect slot, and installation derives the aftermath's
/// external posture from that one declaration. An aftermath that carried its own
/// posture could name a different rail than the outbox correlates to — or claim
/// no escape at all while the operation dispatched — and the reversibility guard
/// would read the claim instead of the lane (Q8.25-C1).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeclaredApplicationAftermathContract {
    authority: DeclaredCorrectionAuthority,
    mechanism: Option<DeclaredCorrectionMechanism>,
    reconciliation: Option<DeclaredReconciliationProcedure>,
}

impl DeclaredApplicationAftermathContract {
    /// Runtime-alone correction through a recorded inverse or compensation.
    pub const fn runtime_alone(mechanism: DeclaredCorrectionMechanism) -> Self {
        Self {
            authority: DeclaredCorrectionAuthority::RuntimeAlone,
            mechanism: Some(mechanism),
            reconciliation: None,
        }
    }

    /// Correction that requires an external owner or distinct actor.
    pub const fn runtime_with_external_owner(
        mechanism: DeclaredCorrectionMechanism,
        reconciliation: DeclaredReconciliationProcedure,
    ) -> Self {
        Self {
            authority: DeclaredCorrectionAuthority::RuntimeWithExternalOwner,
            mechanism: Some(mechanism),
            reconciliation: Some(reconciliation),
        }
    }

    /// Terminal non-correctable aftermath.
    pub const fn not_correctable() -> Self {
        Self {
            authority: DeclaredCorrectionAuthority::NotCorrectable,
            mechanism: None,
            reconciliation: None,
        }
    }

    pub const fn authority(&self) -> DeclaredCorrectionAuthority {
        self.authority
    }

    pub const fn mechanism(&self) -> Option<&DeclaredCorrectionMechanism> {
        self.mechanism.as_ref()
    }

    pub const fn reconciliation(&self) -> Option<&DeclaredReconciliationProcedure> {
        self.reconciliation.as_ref()
    }
}
