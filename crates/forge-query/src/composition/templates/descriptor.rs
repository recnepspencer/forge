use crate::authoring::{
    AuthoredQuery, AuthoredResultShape, CollectionFamily, CollectionResultShapeFamily,
    DetailFamily, DetailResultShapeFamily, QueryAuthoringFamily, ResultShapeAuthoringFamily,
};
use crate::composition::TemplateFamily;

use crate::composition::scopes::BasisScopeEvidence;

use super::slot::TemplateParameterSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryTemplateDescriptor<Q, S>
where
    Q: QueryAuthoringFamily,
    S: ResultShapeAuthoringFamily,
{
    family: TemplateFamily,
    query: AuthoredQuery<Q>,
    result_shape: AuthoredResultShape<S>,
    slots: Vec<TemplateParameterSlot>,
    basis_evidence: Option<BasisScopeEvidence>,
}

impl QueryTemplateDescriptor<DetailFamily, DetailResultShapeFamily> {
    pub fn detail(
        query: AuthoredQuery<DetailFamily>,
        result_shape: AuthoredResultShape<DetailResultShapeFamily>,
    ) -> Self {
        Self {
            family: TemplateFamily::DetailTemplate,
            query,
            result_shape,
            slots: Vec::new(),
            basis_evidence: None,
        }
    }
}

impl QueryTemplateDescriptor<CollectionFamily, CollectionResultShapeFamily> {
    pub fn collection(
        query: AuthoredQuery<CollectionFamily>,
        result_shape: AuthoredResultShape<CollectionResultShapeFamily>,
    ) -> Self {
        Self {
            family: TemplateFamily::CollectionTemplate,
            query,
            result_shape,
            slots: Vec::new(),
            basis_evidence: None,
        }
    }

    pub fn grouped_collection(
        query: AuthoredQuery<CollectionFamily>,
        result_shape: AuthoredResultShape<CollectionResultShapeFamily>,
    ) -> Self {
        Self {
            family: TemplateFamily::GroupedCollectionTemplate,
            query,
            result_shape,
            slots: Vec::new(),
            basis_evidence: None,
        }
    }
}

impl<Q, S> QueryTemplateDescriptor<Q, S>
where
    Q: QueryAuthoringFamily,
    S: ResultShapeAuthoringFamily,
{
    #[cfg(test)]
    pub(crate) fn with_family_for_test(
        family: TemplateFamily,
        query: AuthoredQuery<Q>,
        result_shape: AuthoredResultShape<S>,
    ) -> Self {
        Self {
            family,
            query,
            result_shape,
            slots: Vec::new(),
            basis_evidence: None,
        }
    }

    pub fn with_slot(mut self, slot: TemplateParameterSlot) -> Self {
        self.slots.push(slot);
        self
    }

    pub fn with_basis_evidence(mut self, evidence: BasisScopeEvidence) -> Self {
        self.basis_evidence = Some(evidence);
        self
    }

    pub fn family(&self) -> TemplateFamily {
        self.family
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        TemplateFamily,
        AuthoredQuery<Q>,
        AuthoredResultShape<S>,
        Vec<TemplateParameterSlot>,
        Option<BasisScopeEvidence>,
    ) {
        (
            self.family,
            self.query,
            self.result_shape,
            self.slots,
            self.basis_evidence,
        )
    }
}
