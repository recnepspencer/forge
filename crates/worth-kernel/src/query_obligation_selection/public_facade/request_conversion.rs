use topology::facade::{
    TopologyDeclaredTouchedGraphBasisProof, TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
};
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryTouchDescriptor;

use super::error::QueryGraphObligationSelectionFacadeError;
use super::request::QueryGraphObligationSelectionRequest;

pub trait IntoQueryGraphObligationSelectionRequest:
    private::SealedQueryGraphObligationSelectionInput
{
    fn into_query_graph_obligation_selection_request(
        self,
    ) -> Result<QueryGraphObligationSelectionRequest, QueryGraphObligationSelectionFacadeError>;
}

impl IntoQueryGraphObligationSelectionRequest for QueryGraphObligationSelectionRequest {
    fn into_query_graph_obligation_selection_request(
        self,
    ) -> Result<QueryGraphObligationSelectionRequest, QueryGraphObligationSelectionFacadeError>
    {
        Ok(self)
    }
}

impl IntoQueryGraphObligationSelectionRequest for &TopologyDeclaredTouchedGraphBasisProof {
    fn into_query_graph_obligation_selection_request(
        self,
    ) -> Result<QueryGraphObligationSelectionRequest, QueryGraphObligationSelectionFacadeError>
    {
        QueryGraphObligationSelectionRequest::from_topology_touched_basis(self)
    }
}

impl IntoQueryGraphObligationSelectionRequest
    for &TopologyPrimitiveConstructionBirthDeclaredTouchedBasis
{
    fn into_query_graph_obligation_selection_request(
        self,
    ) -> Result<QueryGraphObligationSelectionRequest, QueryGraphObligationSelectionFacadeError>
    {
        QueryGraphObligationSelectionRequest::from_primitive_construction_touched_basis(self)
    }
}

impl IntoQueryGraphObligationSelectionRequest for &SpatialEvidenceQueryTouchDescriptor {
    fn into_query_graph_obligation_selection_request(
        self,
    ) -> Result<QueryGraphObligationSelectionRequest, QueryGraphObligationSelectionFacadeError>
    {
        QueryGraphObligationSelectionRequest::from_spatial_descriptor(self)
    }
}

mod private {
    use topology::facade::{
        TopologyDeclaredTouchedGraphBasisProof,
        TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
    };
    use worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryTouchDescriptor;

    use super::QueryGraphObligationSelectionRequest;

    pub trait SealedQueryGraphObligationSelectionInput {}

    impl SealedQueryGraphObligationSelectionInput for QueryGraphObligationSelectionRequest {}
    impl SealedQueryGraphObligationSelectionInput for &TopologyDeclaredTouchedGraphBasisProof {}
    impl SealedQueryGraphObligationSelectionInput
        for &TopologyPrimitiveConstructionBirthDeclaredTouchedBasis
    {
    }
    impl SealedQueryGraphObligationSelectionInput for &SpatialEvidenceQueryTouchDescriptor {}
}
