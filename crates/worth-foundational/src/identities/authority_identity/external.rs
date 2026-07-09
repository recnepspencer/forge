use std::fmt;
use std::marker::PhantomData;

use worth_proof::{AuthorityMarker, AuthorityWitness};

use super::current::FoundationalAdmittedIdentityValue;
use super::markers::FoundationalIdentityKind;

pub struct FoundationalExternalIdentityToken<Value, Kind>
where
    Kind: FoundationalIdentityKind,
{
    value: Value,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Value, Kind> FoundationalExternalIdentityToken<Value, Kind>
where
    Kind: FoundationalIdentityKind,
{
    pub fn new(value: Value) -> Self {
        Self {
            value,
            _kind: PhantomData,
        }
    }

    pub const fn value(&self) -> &Value {
        &self.value
    }

    pub fn into_value(self) -> Value {
        self.value
    }

    pub fn admit_with_authority<Authority>(
        self,
        authority: AuthorityWitness<Authority>,
    ) -> FoundationalAdmittedIdentityValue<Value, Authority, Kind>
    where
        Authority: AuthorityMarker,
    {
        FoundationalAdmittedIdentityValue::admit(self.value, authority)
    }
}

impl<Value, Kind> Clone for FoundationalExternalIdentityToken<Value, Kind>
where
    Value: Clone,
    Kind: FoundationalIdentityKind,
{
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<Value, Kind> fmt::Debug for FoundationalExternalIdentityToken<Value, Kind>
where
    Value: fmt::Debug,
    Kind: FoundationalIdentityKind,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FoundationalExternalIdentityToken")
            .field("value", &self.value)
            .finish_non_exhaustive()
    }
}

impl<Value, Kind> PartialEq for FoundationalExternalIdentityToken<Value, Kind>
where
    Value: PartialEq,
    Kind: FoundationalIdentityKind,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<Value, Kind> Eq for FoundationalExternalIdentityToken<Value, Kind>
where
    Value: Eq,
    Kind: FoundationalIdentityKind,
{
}
