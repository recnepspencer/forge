use crate::application::{
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationPrimaryAuthorityFamily,
    WorthQueryDomainEntryMarker, WorthQueryGroupedDeclarationPosture,
    WorthQuerySignalCompatibilityPosture,
};

mod sealed {
    pub trait Sealed {}
}

pub trait WorthQueryDeclarationPrimaryAuthorityTag:
    sealed::Sealed + Clone + Copy + std::fmt::Debug + Eq + PartialEq
{
    fn runtime_value() -> WorthQueryDeclarationPrimaryAuthorityFamily;
}

pub trait WorthQueryDeclarationSignalCompatibilityTag:
    sealed::Sealed + Clone + Copy + std::fmt::Debug + Eq + PartialEq
{
    fn runtime_value() -> WorthQuerySignalCompatibilityPosture;
}

pub trait WorthQueryDeclarationGroupedPostureTag:
    sealed::Sealed + Clone + Copy + std::fmt::Debug + Eq + PartialEq
{
    fn runtime_value() -> WorthQueryGroupedDeclarationPosture;
}

pub trait WorthQueryDeclarationSupportsRelationalTruth<D: WorthQueryDomainEntryMarker>:
    WorthQueryDeclarationFamilyMarker<D>
{
}
pub trait WorthQueryDeclarationSupportsBridgeContinuation<D: WorthQueryDomainEntryMarker>:
    WorthQueryDeclarationFamilyMarker<D>
{
}
pub trait WorthQueryDeclarationSupportsSignalCompatibility<D: WorthQueryDomainEntryMarker>:
    WorthQueryDeclarationFamilyMarker<D>
{
}
pub trait WorthQueryDeclarationSupportsNeighborhoodGrouping<D: WorthQueryDomainEntryMarker>:
    WorthQueryDeclarationFamilyMarker<D>
{
}
pub trait WorthQueryDeclarationSupportsBatchGrouping<D: WorthQueryDomainEntryMarker>:
    WorthQueryDeclarationFamilyMarker<D>
{
}

pub trait WorthQueryNeighborhoodCapableGroupingTag: WorthQueryDeclarationGroupedPostureTag {}
pub trait WorthQueryBatchCapableGroupingTag: WorthQueryDeclarationGroupedPostureTag {}

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
    WorthQueryDescriptiveOnlyAuthority,
    WorthQueryDeclarationPrimaryAuthorityTag,
    WorthQueryDeclarationPrimaryAuthorityFamily,
    WorthQueryDeclarationPrimaryAuthorityFamily::DescriptiveOnly
);
declare_tag!(
    WorthQueryRelationalTruthAuthority,
    WorthQueryDeclarationPrimaryAuthorityTag,
    WorthQueryDeclarationPrimaryAuthorityFamily,
    WorthQueryDeclarationPrimaryAuthorityFamily::RelationalTruth
);
declare_tag!(
    WorthQueryBridgeContinuationAuthority,
    WorthQueryDeclarationPrimaryAuthorityTag,
    WorthQueryDeclarationPrimaryAuthorityFamily,
    WorthQueryDeclarationPrimaryAuthorityFamily::BridgeContinuation
);
declare_tag!(
    WorthQueryMixedAuthority,
    WorthQueryDeclarationPrimaryAuthorityTag,
    WorthQueryDeclarationPrimaryAuthorityFamily,
    WorthQueryDeclarationPrimaryAuthorityFamily::MixedAuthority
);

declare_tag!(
    WorthQuerySignalNotCompatiblePosture,
    WorthQueryDeclarationSignalCompatibilityTag,
    WorthQuerySignalCompatibilityPosture,
    WorthQuerySignalCompatibilityPosture::NotCompatible
);
declare_tag!(
    WorthQuerySignalCompatiblePosture,
    WorthQueryDeclarationSignalCompatibilityTag,
    WorthQuerySignalCompatibilityPosture,
    WorthQuerySignalCompatibilityPosture::Compatible
);
declare_tag!(
    WorthQuerySignalDeferredPosture,
    WorthQueryDeclarationSignalCompatibilityTag,
    WorthQuerySignalCompatibilityPosture,
    WorthQuerySignalCompatibilityPosture::Deferred
);

declare_tag!(
    WorthQuerySingleOnlyGrouping,
    WorthQueryDeclarationGroupedPostureTag,
    WorthQueryGroupedDeclarationPosture,
    WorthQueryGroupedDeclarationPosture::SingleOnly
);
declare_tag!(
    WorthQueryNeighborhoodCapableGrouping,
    WorthQueryDeclarationGroupedPostureTag,
    WorthQueryGroupedDeclarationPosture,
    WorthQueryGroupedDeclarationPosture::NeighborhoodCapable
);
declare_tag!(
    WorthQueryBatchCapableGrouping,
    WorthQueryDeclarationGroupedPostureTag,
    WorthQueryGroupedDeclarationPosture,
    WorthQueryGroupedDeclarationPosture::BatchCapable
);
declare_tag!(
    WorthQueryNeighborhoodAndBatchCapableGrouping,
    WorthQueryDeclarationGroupedPostureTag,
    WorthQueryGroupedDeclarationPosture,
    WorthQueryGroupedDeclarationPosture::NeighborhoodAndBatchCapable
);

impl<D, F> WorthQueryDeclarationSupportsRelationalTruth<D> for F
where
    D: WorthQueryDomainEntryMarker,
    F: WorthQueryDeclarationFamilyMarker<D>,
    F::PrimaryAuthority: WorthQueryRelationalTruthCapableAuthorityTag,
{
}

impl<D, F> WorthQueryDeclarationSupportsBridgeContinuation<D> for F
where
    D: WorthQueryDomainEntryMarker,
    F: WorthQueryDeclarationFamilyMarker<D>,
    F::PrimaryAuthority: WorthQueryBridgeContinuationCapableAuthorityTag,
{
}

impl<D, F> WorthQueryDeclarationSupportsSignalCompatibility<D> for F
where
    D: WorthQueryDomainEntryMarker,
    F: WorthQueryDeclarationFamilyMarker<
        D,
        SignalCompatibility = WorthQuerySignalCompatiblePosture,
    >,
{
}

impl<D, F> WorthQueryDeclarationSupportsNeighborhoodGrouping<D> for F
where
    D: WorthQueryDomainEntryMarker,
    F: WorthQueryDeclarationFamilyMarker<D>,
    F::GroupedPosture: WorthQueryNeighborhoodCapableGroupingTag,
{
}

impl<D, F> WorthQueryDeclarationSupportsBatchGrouping<D> for F
where
    D: WorthQueryDomainEntryMarker,
    F: WorthQueryDeclarationFamilyMarker<D>,
    F::GroupedPosture: WorthQueryBatchCapableGroupingTag,
{
}

impl WorthQueryNeighborhoodCapableGroupingTag for WorthQueryNeighborhoodCapableGrouping {}
impl WorthQueryNeighborhoodCapableGroupingTag for WorthQueryNeighborhoodAndBatchCapableGrouping {}
impl WorthQueryBatchCapableGroupingTag for WorthQueryBatchCapableGrouping {}
impl WorthQueryBatchCapableGroupingTag for WorthQueryNeighborhoodAndBatchCapableGrouping {}

pub trait WorthQueryRelationalTruthCapableAuthorityTag:
    WorthQueryDeclarationPrimaryAuthorityTag
{
}

pub trait WorthQueryBridgeContinuationCapableAuthorityTag:
    WorthQueryDeclarationPrimaryAuthorityTag
{
}

impl WorthQueryRelationalTruthCapableAuthorityTag for WorthQueryRelationalTruthAuthority {}
impl WorthQueryRelationalTruthCapableAuthorityTag for WorthQueryMixedAuthority {}
impl WorthQueryBridgeContinuationCapableAuthorityTag for WorthQueryBridgeContinuationAuthority {}
impl WorthQueryBridgeContinuationCapableAuthorityTag for WorthQueryMixedAuthority {}
