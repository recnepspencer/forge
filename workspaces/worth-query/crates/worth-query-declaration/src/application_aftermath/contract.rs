//! Schema-affine aftermath authoring and portable installed meaning.

use crate::application_schema::AftermathAssociationAuthority;

use super::{
    DeclaredCorrectionAuthority, DeclaredCorrectionMechanism, DeclaredReconciliationProcedure,
    PortableCorrectionMechanism,
};

/// Schema-affine aftermath contract for one mutation operation.
///
/// `Schema` remains part of the type until the matching schema's operation
/// builder consumes this contract. This prevents a same-text field locus from
/// another schema from entering the operation association lane.
pub struct DeclaredApplicationAftermathContract<Schema> {
    authority: DeclaredCorrectionAuthority,
    mechanism: Option<DeclaredCorrectionMechanism<Schema>>,
    reconciliation: Option<DeclaredReconciliationProcedure>,
}

impl<Schema> Clone for DeclaredApplicationAftermathContract<Schema> {
    fn clone(&self) -> Self {
        Self {
            authority: self.authority,
            mechanism: self.mechanism.clone(),
            reconciliation: self.reconciliation.clone(),
        }
    }
}

impl<Schema> std::fmt::Debug for DeclaredApplicationAftermathContract<Schema> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DeclaredApplicationAftermathContract")
            .field("authority", &self.authority)
            .field("mechanism", &self.mechanism)
            .field("reconciliation", &self.reconciliation)
            .finish()
    }
}

impl<Schema> PartialEq for DeclaredApplicationAftermathContract<Schema> {
    fn eq(&self, other: &Self) -> bool {
        self.authority == other.authority
            && self.mechanism == other.mechanism
            && self.reconciliation == other.reconciliation
    }
}

impl<Schema> Eq for DeclaredApplicationAftermathContract<Schema> {}

impl<Schema> PartialOrd for DeclaredApplicationAftermathContract<Schema> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Schema> Ord for DeclaredApplicationAftermathContract<Schema> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.authority, &self.mechanism, &self.reconciliation).cmp(&(
            &other.authority,
            &other.mechanism,
            &other.reconciliation,
        ))
    }
}

impl<Schema> DeclaredApplicationAftermathContract<Schema> {
    pub const fn runtime_alone(mechanism: DeclaredCorrectionMechanism<Schema>) -> Self {
        Self {
            authority: DeclaredCorrectionAuthority::RuntimeAlone,
            mechanism: Some(mechanism),
            reconciliation: None,
        }
    }

    pub const fn runtime_with_external_owner(
        mechanism: DeclaredCorrectionMechanism<Schema>,
        reconciliation: DeclaredReconciliationProcedure,
    ) -> Self {
        Self {
            authority: DeclaredCorrectionAuthority::RuntimeWithExternalOwner,
            mechanism: Some(mechanism),
            reconciliation: Some(reconciliation),
        }
    }

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

    pub const fn mechanism(&self) -> Option<&DeclaredCorrectionMechanism<Schema>> {
        self.mechanism.as_ref()
    }

    pub const fn reconciliation(&self) -> Option<&DeclaredReconciliationProcedure> {
        self.reconciliation.as_ref()
    }

    pub(crate) fn associate_with_operation(
        self,
        _authority: AftermathAssociationAuthority<Schema>,
    ) -> PortableApplicationAftermathContract {
        PortableApplicationAftermathContract {
            authority: self.authority,
            mechanism: self
                .mechanism
                .map(DeclaredCorrectionMechanism::into_portable),
            reconciliation: self.reconciliation,
        }
    }
}

/// Portable, public-read aftermath meaning stored in a schema declaration.
///
/// Construction is owner-sealed: only the schema-affine operation builder can
/// mint this value. Installation consumers may inspect or clone the meaning,
/// but cannot author it positionally.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PortableApplicationAftermathContract {
    authority: DeclaredCorrectionAuthority,
    mechanism: Option<PortableCorrectionMechanism>,
    reconciliation: Option<DeclaredReconciliationProcedure>,
}

impl PortableApplicationAftermathContract {
    pub const fn authority(&self) -> DeclaredCorrectionAuthority {
        self.authority
    }

    pub const fn mechanism(&self) -> Option<&PortableCorrectionMechanism> {
        self.mechanism.as_ref()
    }

    pub const fn reconciliation(&self) -> Option<&DeclaredReconciliationProcedure> {
        self.reconciliation.as_ref()
    }
}
