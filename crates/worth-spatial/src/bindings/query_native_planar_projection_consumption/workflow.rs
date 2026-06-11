use crate::bindings::query_native_planar_projection_consumption::authoring::{
    projection_consumed_planar_facts_entry, ProjectionConsumedPlanarFactsCase,
    ProjectionConsumedPlanarFactsEntry,
};
use crate::bindings::query_native_planar_projection_consumption::domain::ProjectionConsumedPlanarFactsQueryDomain;
use crate::bindings::query_native_planar_projection_consumption::facts::{
    projection_consumed_planar_facts, ProjectionConsumedPlanarFactsFactError,
};
use crate::bindings::query_native_planar_projection_consumption::inspection::ProjectionConsumedPlanarFactsInspectionRow;
use crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt;
use crate::planar_contracts::projection_consumed_facts::{
    ProjectionConsumedPlanarFactsBasis, ProjectionConsumedPlanarFactsDenial,
    ProjectionConsumedPlanarFactsReceipt,
};
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProjectionConsumedPlanarFacts {
    builder:
        crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsBuilder,
}

impl ProjectionConsumedPlanarFacts {
    pub fn from_retained_planar_facts(receipt: RetainedPlanarFactsReceipt) -> Self {
        Self {
            builder: ProjectionConsumedPlanarFactsBasis::builder().retained_planar_facts(receipt),
        }
    }

    pub fn consume_bundle_projection_receipts<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = ProjectPointToCertifiedPlane2DReceipt>,
    {
        self.builder = self.builder.projection_receipts(receipts);
        self
    }

    pub fn materialize_as(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.materialization_basis(identity);
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a ProjectionConsumedPlanarFactsContracts<WC>,
    ) -> Result<ProjectionConsumedPlanarFactsPlan<'a, WC>, ProjectionConsumedPlanarFactsDenial>
    where
        WC: ForgeQueryDomainOperatingContext<ProjectionConsumedPlanarFactsQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry = projection_consumed_planar_facts_entry(
            ProjectionConsumedPlanarFactsCase::from_basis(basis),
        );
        Ok(ProjectionConsumedPlanarFactsPlan { entry, contracts })
    }
}

pub struct ProjectionConsumedPlanarFactsContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<ProjectionConsumedPlanarFactsQueryDomain>,
{
    projection_consumption_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<ProjectionConsumedPlanarFactsQueryDomain, WC>,
}

impl<WC> ProjectionConsumedPlanarFactsContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<ProjectionConsumedPlanarFactsQueryDomain>,
{
    pub fn new(
        projection_consumption_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            ProjectionConsumedPlanarFactsQueryDomain,
            WC,
        >,
    ) -> Self {
        Self {
            projection_consumption_handle,
        }
    }
}

pub struct ProjectionConsumedPlanarFactsPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<ProjectionConsumedPlanarFactsQueryDomain>,
{
    entry: ProjectionConsumedPlanarFactsEntry,
    contracts: &'a ProjectionConsumedPlanarFactsContracts<WC>,
}

impl<WC> ProjectionConsumedPlanarFactsPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<ProjectionConsumedPlanarFactsQueryDomain>,
{
    pub fn inspected_projection_consumption_rows(&self) -> usize {
        ProjectionConsumedPlanarFactsInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn consume(
        self,
    ) -> Result<ProjectionConsumedPlanarFactsReceipt, ProjectionConsumedPlanarFactsFactError> {
        projection_consumed_planar_facts(&self.entry, &self.contracts.projection_consumption_handle)
    }
}
