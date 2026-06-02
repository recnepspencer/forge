#[cfg(test)]
mod boundary_tests;
mod query_native_boundary;

pub use query_native_boundary::{
    prepare_primitive_construction_query_admitted_handoff,
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    prepare_primitive_construction_query_envelope, prepare_primitive_construction_query_handoff,
    prepare_primitive_construction_query_receipt, TopologyConstructionQueryAdmittedHandoffError,
    TopologyConstructionQueryEnvelopeError, TopologyConstructionQueryFactKind,
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryFactRow,
    TopologyConstructionQueryHandoffError, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryMutationSurface, TopologyConstructionQueryReadSurface,
    TopologyConstructionQueryReceiptError, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis, TopologyPrimitiveConstructionQueryEnvelope,
    TopologyPrimitiveConstructionQueryHandoff, TopologyPrimitiveConstructionQueryReceipt,
};
