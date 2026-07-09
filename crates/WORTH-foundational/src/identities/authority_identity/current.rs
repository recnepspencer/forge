use std::fmt;
use std::marker::PhantomData;

use worth_proof::{AuthorityMarker, AuthorityWitness};

use super::markers::FoundationalIdentityKind;

pub struct FoundationalAdmittedIdentityValue<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    value: Value,
    _authority: PhantomData<fn() -> Authority>,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Value, Authority, Kind> FoundationalAdmittedIdentityValue<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    pub fn admit(value: Value, _authority: AuthorityWitness<Authority>) -> Self {
        Self {
            value,
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }

    pub const fn value(&self) -> &Value {
        &self.value
    }

    pub(super) fn into_value(self) -> Value {
        self.value
    }
}

impl<Value, Authority, Kind> fmt::Debug
    for FoundationalAdmittedIdentityValue<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FoundationalAdmittedIdentityValue")
            .field("authority", &std::any::type_name::<Authority>())
            .field("kind", &std::any::type_name::<Kind>())
            .field("value", &"<admitted-redacted>")
            .finish_non_exhaustive()
    }
}

pub struct FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    value: Value,
    _authority: PhantomData<fn() -> Authority>,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Value, Authority, Kind> FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    pub fn from_admitted(
        admitted: FoundationalAdmittedIdentityValue<Value, Authority, Kind>,
    ) -> Self {
        Self {
            value: admitted.into_value(),
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }

    pub fn readmit(
        revalidated: FoundationalRevalidatedIdentityValue<Value, Authority, Kind>,
    ) -> Self {
        Self {
            value: revalidated.into_value(),
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }

    pub const fn value(&self) -> &Value {
        &self.value
    }

    pub fn bridge_trust_boundary(
        self,
    ) -> FoundationalBoundaryBridgedIdentity<Value, Authority, Kind> {
        FoundationalBoundaryBridgedIdentity {
            value: self.value,
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }
}

impl<Value, Authority, Kind> Clone for FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Value: Clone,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }
}

impl<Value, Authority, Kind> fmt::Debug for FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FoundationalAuthorityIdentity")
            .field("authority", &std::any::type_name::<Authority>())
            .field("kind", &std::any::type_name::<Kind>())
            .field("value", &"<authority-redacted>")
            .finish_non_exhaustive()
    }
}

impl<Value, Authority, Kind> PartialEq for FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Value: PartialEq,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<Value, Authority, Kind> Eq for FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Value: Eq,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
}

pub struct FoundationalBoundaryBridgedIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    value: Value,
    _authority: PhantomData<fn() -> Authority>,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Value, Authority, Kind> FoundationalBoundaryBridgedIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    pub const fn value(&self) -> &Value {
        &self.value
    }

    pub fn revalidate_current_value(
        self,
        authority: AuthorityWitness<Authority>,
    ) -> FoundationalRevalidatedIdentityValue<Value, Authority, Kind> {
        FoundationalRevalidatedIdentityValue::revalidate(self.value, authority)
    }

    pub fn revalidate_replacement_value(
        self,
        value: Value,
        authority: AuthorityWitness<Authority>,
    ) -> FoundationalRevalidatedIdentityValue<Value, Authority, Kind> {
        let _boundary_observation = self;
        FoundationalRevalidatedIdentityValue::revalidate(value, authority)
    }
}

impl<Value, Authority, Kind> fmt::Debug
    for FoundationalBoundaryBridgedIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FoundationalBoundaryBridgedIdentity")
            .field("authority", &std::any::type_name::<Authority>())
            .field("kind", &std::any::type_name::<Kind>())
            .field("value", &"<boundary-bridged-redacted>")
            .finish_non_exhaustive()
    }
}

pub struct FoundationalRevalidatedIdentityValue<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    value: Value,
    _authority: PhantomData<fn() -> Authority>,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Value, Authority, Kind> FoundationalRevalidatedIdentityValue<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    fn revalidate(value: Value, _authority: AuthorityWitness<Authority>) -> Self {
        Self {
            value,
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }

    pub const fn value(&self) -> &Value {
        &self.value
    }

    fn into_value(self) -> Value {
        self.value
    }
}

impl<Value, Authority, Kind> fmt::Debug
    for FoundationalRevalidatedIdentityValue<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FoundationalRevalidatedIdentityValue")
            .field("authority", &std::any::type_name::<Authority>())
            .field("kind", &std::any::type_name::<Kind>())
            .field("value", &"<revalidated-redacted>")
            .finish_non_exhaustive()
    }
}
