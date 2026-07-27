use std::marker::PhantomData;

pub trait WritePosture {
    const WRITABLE: bool;
}

pub trait WritableCapability: WritePosture {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadOnly;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadWrite;

impl WritableCapability for ReadWrite {}

impl WritePosture for ReadOnly {
    const WRITABLE: bool = false;
}

impl WritePosture for ReadWrite {
    const WRITABLE: bool = true;
}

pub trait EqualityPosture {
    const QUERYABLE: bool;
}

pub trait EqualityCapable: EqualityPosture {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoEqualityPredicate;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EqualityPredicate;

impl EqualityCapable for EqualityPredicate {}

impl EqualityPosture for NoEqualityPredicate {
    const QUERYABLE: bool = false;
}

impl EqualityPosture for EqualityPredicate {
    const QUERYABLE: bool = true;
}

/// Compile-time declaration that a field marker may be written by an operation
/// marker. This is descriptive only; installed operation authority is still
/// required later.
pub trait OperationWrites<Operation> {}

/// Compile-time declaration that an entity marker may be created by an
/// operation marker.
pub trait OperationCreates<Operation> {}

pub trait CreatableBy<Operation>: OperationCreates<Operation> {}

impl<Entity, Operation> CreatableBy<Operation> for Entity where Entity: OperationCreates<Operation> {}

pub trait OperationDeletes<Operation> {}

pub trait OperationLinks<Operation> {}

pub trait OperationUnlinks<Operation> {}

/// Compile-time declaration that an effect marker may be emitted by an
/// operation marker.
pub trait OperationEmits<Operation> {}

pub trait ApplicationCurrencyMarker<DomainCurrency> {
    const NAME: &'static str;
}

pub trait ApplicationFieldCurrency {
    const NAME: Option<&'static str>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoApplicationCurrency;

impl ApplicationFieldCurrency for NoApplicationCurrency {
    const NAME: Option<&'static str> = None;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclaredApplicationCurrency<Marker, DomainCurrency>(
    PhantomData<fn() -> (Marker, DomainCurrency)>,
);

impl<Marker, DomainCurrency> ApplicationFieldCurrency
    for DeclaredApplicationCurrency<Marker, DomainCurrency>
where
    Marker: ApplicationCurrencyMarker<DomainCurrency>,
{
    const NAME: Option<&'static str> = Some(Marker::NAME);
}
