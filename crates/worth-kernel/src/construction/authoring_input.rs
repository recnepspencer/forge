use crate::construction::PrimitiveConstructionIntent;
use crate::spatial_intent::{
    AnchorMatchSpatialIntent, LiesOnSpatialIntent, MoveSpatialIntent, OffsetSpatialIntent,
    PointsTowardSpatialIntent, PrimitiveConstructionSpatialIntentError, ReorientSpatialIntent,
    RotateSpatialIntent,
};
use worth_spatial::facade::witness_catalog::SpatialWitnessCatalog;

mod sealed {
    pub trait PrimitiveConstructionAuthoringInputSealed {}
    pub trait PrimitiveConstructionCatalogAuthoringInputSealed {}
}

pub trait PrimitiveConstructionAuthoringInput:
    sealed::PrimitiveConstructionAuthoringInputSealed
{
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError>;
}

pub trait PrimitiveConstructionCatalogAuthoringInput:
    sealed::PrimitiveConstructionCatalogAuthoringInputSealed
{
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError>;
}

impl sealed::PrimitiveConstructionAuthoringInputSealed for PrimitiveConstructionIntent {}

impl PrimitiveConstructionAuthoringInput for PrimitiveConstructionIntent {
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        Ok(self)
    }
}

impl sealed::PrimitiveConstructionCatalogAuthoringInputSealed for PrimitiveConstructionIntent {}

impl PrimitiveConstructionCatalogAuthoringInput for PrimitiveConstructionIntent {
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        _catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        Ok(self)
    }
}

impl sealed::PrimitiveConstructionAuthoringInputSealed
    for MoveSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionAuthoringInput for MoveSpatialIntent<PrimitiveConstructionIntent> {
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish()
    }
}

impl sealed::PrimitiveConstructionCatalogAuthoringInputSealed
    for MoveSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionCatalogAuthoringInput for MoveSpatialIntent<PrimitiveConstructionIntent> {
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish_with_catalog(catalog)
    }
}

impl sealed::PrimitiveConstructionAuthoringInputSealed
    for OffsetSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionAuthoringInput for OffsetSpatialIntent<PrimitiveConstructionIntent> {
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish()
    }
}

impl sealed::PrimitiveConstructionCatalogAuthoringInputSealed
    for OffsetSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionCatalogAuthoringInput
    for OffsetSpatialIntent<PrimitiveConstructionIntent>
{
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish_with_catalog(catalog)
    }
}

impl sealed::PrimitiveConstructionAuthoringInputSealed
    for ReorientSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionAuthoringInput for ReorientSpatialIntent<PrimitiveConstructionIntent> {
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish()
    }
}

impl sealed::PrimitiveConstructionCatalogAuthoringInputSealed
    for ReorientSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionCatalogAuthoringInput
    for ReorientSpatialIntent<PrimitiveConstructionIntent>
{
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish_with_catalog(catalog)
    }
}

impl sealed::PrimitiveConstructionAuthoringInputSealed
    for RotateSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionAuthoringInput for RotateSpatialIntent<PrimitiveConstructionIntent> {
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish()
    }
}

impl sealed::PrimitiveConstructionCatalogAuthoringInputSealed
    for RotateSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionCatalogAuthoringInput
    for RotateSpatialIntent<PrimitiveConstructionIntent>
{
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish_with_catalog(catalog)
    }
}

impl sealed::PrimitiveConstructionAuthoringInputSealed
    for LiesOnSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionAuthoringInput for LiesOnSpatialIntent<PrimitiveConstructionIntent> {
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish()
    }
}

impl sealed::PrimitiveConstructionCatalogAuthoringInputSealed
    for LiesOnSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionCatalogAuthoringInput
    for LiesOnSpatialIntent<PrimitiveConstructionIntent>
{
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish_with_catalog(catalog)
    }
}

impl sealed::PrimitiveConstructionAuthoringInputSealed
    for PointsTowardSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionAuthoringInput
    for PointsTowardSpatialIntent<PrimitiveConstructionIntent>
{
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish()
    }
}

impl sealed::PrimitiveConstructionCatalogAuthoringInputSealed
    for PointsTowardSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionCatalogAuthoringInput
    for PointsTowardSpatialIntent<PrimitiveConstructionIntent>
{
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish_with_catalog(catalog)
    }
}

impl sealed::PrimitiveConstructionAuthoringInputSealed
    for AnchorMatchSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionAuthoringInput for AnchorMatchSpatialIntent<PrimitiveConstructionIntent> {
    fn lower_for_query_entry(
        self,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish()
    }
}

impl sealed::PrimitiveConstructionCatalogAuthoringInputSealed
    for AnchorMatchSpatialIntent<PrimitiveConstructionIntent>
{
}
impl PrimitiveConstructionCatalogAuthoringInput
    for AnchorMatchSpatialIntent<PrimitiveConstructionIntent>
{
    fn lower_for_query_entry_with_catalog<C: SpatialWitnessCatalog>(
        self,
        catalog: &C,
    ) -> Result<PrimitiveConstructionIntent, PrimitiveConstructionSpatialIntentError> {
        self.finish_with_catalog(catalog)
    }
}
