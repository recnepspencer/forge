use crate::application::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryGroupedDeclarationPosture,
    ForgeQuerySignalCompatibilityPosture,
};

mod sealed {
    pub trait Sealed {}
}

pub trait ForgeQueryDeclarationPrimaryAuthorityTag:
    sealed::Sealed + Clone + Copy + std::fmt::Debug + Eq + PartialEq
{
    fn runtime_value() -> ForgeQueryDeclarationPrimaryAuthorityFamily;
}

pub trait ForgeQueryDeclarationSignalCompatibilityTag:
    sealed::Sealed + Clone + Copy + std::fmt::Debug + Eq + PartialEq
{
    fn runtime_value() -> ForgeQuerySignalCompatibilityPosture;
}

pub trait ForgeQueryDeclarationGroupedPostureTag:
    sealed::Sealed + Clone + Copy + std::fmt::Debug + Eq + PartialEq
{
    fn runtime_value() -> ForgeQueryGroupedDeclarationPosture;
}

pub trait ForgeQueryDeclarationSupportsRelationalTruth<D: ForgeQueryDomainEntryMarker>:
    ForgeQueryDeclarationFamilyMarker<D>
{
}
pub trait ForgeQueryDeclarationSupportsBridgeContinuation<D: ForgeQueryDomainEntryMarker>:
    ForgeQueryDeclarationFamilyMarker<D>
{
}
pub trait ForgeQueryDeclarationSupportsSignalCompatibility<D: ForgeQueryDomainEntryMarker>:
    ForgeQueryDeclarationFamilyMarker<D>
{
}
pub trait ForgeQueryDeclarationSupportsNeighborhoodGrouping<D: ForgeQueryDomainEntryMarker>:
    ForgeQueryDeclarationFamilyMarker<D>
{
}
pub trait ForgeQueryDeclarationSupportsBatchGrouping<D: ForgeQueryDomainEntryMarker>:
    ForgeQueryDeclarationFamilyMarker<D>
{
}

pub trait ForgeQueryNeighborhoodCapableGroupingTag: ForgeQueryDeclarationGroupedPostureTag {}
pub trait ForgeQueryBatchCapableGroupingTag: ForgeQueryDeclarationGroupedPostureTag {}

macro_rules! declare_tag {
    ($name:ident, $tag_trait:ident, $value_ty:ty, $value:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl sealed::Sealed for $name {}
        impl $tag_trait for $name {
            fn runtime_value() -> $value_ty {
                $value
            }
        }
    };
}

declare_tag!(
    ForgeQueryDescriptiveOnlyAuthority,
    ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDeclarationPrimaryAuthorityFamily::DescriptiveOnly
);
declare_tag!(
    ForgeQueryRelationalTruthAuthority,
    ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth
);
declare_tag!(
    ForgeQueryBridgeContinuationAuthority,
    ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDeclarationPrimaryAuthorityFamily::BridgeContinuation
);
declare_tag!(
    ForgeQueryMixedAuthority,
    ForgeQueryDeclarationPrimaryAuthorityTag,
    ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDeclarationPrimaryAuthorityFamily::MixedAuthority
);

declare_tag!(
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQueryDeclarationSignalCompatibilityTag,
    ForgeQuerySignalCompatibilityPosture,
    ForgeQuerySignalCompatibilityPosture::NotCompatible
);
declare_tag!(
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationSignalCompatibilityTag,
    ForgeQuerySignalCompatibilityPosture,
    ForgeQuerySignalCompatibilityPosture::Compatible
);
declare_tag!(
    ForgeQuerySignalDeferredPosture,
    ForgeQueryDeclarationSignalCompatibilityTag,
    ForgeQuerySignalCompatibilityPosture,
    ForgeQuerySignalCompatibilityPosture::Deferred
);

declare_tag!(
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationGroupedPostureTag,
    ForgeQueryGroupedDeclarationPosture,
    ForgeQueryGroupedDeclarationPosture::SingleOnly
);
declare_tag!(
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationGroupedPostureTag,
    ForgeQueryGroupedDeclarationPosture,
    ForgeQueryGroupedDeclarationPosture::NeighborhoodCapable
);
declare_tag!(
    ForgeQueryBatchCapableGrouping,
    ForgeQueryDeclarationGroupedPostureTag,
    ForgeQueryGroupedDeclarationPosture,
    ForgeQueryGroupedDeclarationPosture::BatchCapable
);
declare_tag!(
    ForgeQueryNeighborhoodAndBatchCapableGrouping,
    ForgeQueryDeclarationGroupedPostureTag,
    ForgeQueryGroupedDeclarationPosture,
    ForgeQueryGroupedDeclarationPosture::NeighborhoodAndBatchCapable
);

impl<D, F> ForgeQueryDeclarationSupportsRelationalTruth<D> for F
where
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
    F::PrimaryAuthority: ForgeQueryRelationalTruthCapableAuthorityTag,
{
}

impl<D, F> ForgeQueryDeclarationSupportsBridgeContinuation<D> for F
where
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
    F::PrimaryAuthority: ForgeQueryBridgeContinuationCapableAuthorityTag,
{
}

impl<D, F> ForgeQueryDeclarationSupportsSignalCompatibility<D> for F
where
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<
        D,
        SignalCompatibility = ForgeQuerySignalCompatiblePosture,
    >,
{
}

impl<D, F> ForgeQueryDeclarationSupportsNeighborhoodGrouping<D> for F
where
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
    F::GroupedPosture: ForgeQueryNeighborhoodCapableGroupingTag,
{
}

impl<D, F> ForgeQueryDeclarationSupportsBatchGrouping<D> for F
where
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
    F::GroupedPosture: ForgeQueryBatchCapableGroupingTag,
{
}

impl ForgeQueryNeighborhoodCapableGroupingTag for ForgeQueryNeighborhoodCapableGrouping {}
impl ForgeQueryNeighborhoodCapableGroupingTag for ForgeQueryNeighborhoodAndBatchCapableGrouping {}
impl ForgeQueryBatchCapableGroupingTag for ForgeQueryBatchCapableGrouping {}
impl ForgeQueryBatchCapableGroupingTag for ForgeQueryNeighborhoodAndBatchCapableGrouping {}

pub trait ForgeQueryRelationalTruthCapableAuthorityTag:
    ForgeQueryDeclarationPrimaryAuthorityTag
{
}

pub trait ForgeQueryBridgeContinuationCapableAuthorityTag:
    ForgeQueryDeclarationPrimaryAuthorityTag
{
}

impl ForgeQueryRelationalTruthCapableAuthorityTag for ForgeQueryRelationalTruthAuthority {}
impl ForgeQueryRelationalTruthCapableAuthorityTag for ForgeQueryMixedAuthority {}
impl ForgeQueryBridgeContinuationCapableAuthorityTag for ForgeQueryBridgeContinuationAuthority {}
impl ForgeQueryBridgeContinuationCapableAuthorityTag for ForgeQueryMixedAuthority {}
