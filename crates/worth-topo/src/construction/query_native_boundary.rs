mod admission;
mod admitted_handoff;
mod birth_synopsis;
mod compose_execution;
mod envelope;
mod handoff;
mod receipt;
mod surface_vocab;

pub use admission::{
    prepare_primitive_construction_query_envelope, prepare_primitive_construction_query_handoff,
    prepare_primitive_construction_query_receipt, TopologyConstructionQueryEnvelopeError,
    TopologyConstructionQueryHandoffError, TopologyConstructionQueryReceiptError,
};
pub use admitted_handoff::{
    prepare_primitive_construction_query_admitted_handoff,
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    TopologyConstructionQueryAdmittedHandoffError,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
};
pub use birth_synopsis::{
    TopologyPrimitiveConstructionBirthFamily, TopologyPrimitiveConstructionQueryBirthSynopsis,
};
pub(crate) use compose_execution::topology_primitive_construction_birth_layout_violation_registration;
pub use compose_execution::{
    run_primitive_construction_birth_declared_touched_basis_compose,
    topology_primitive_construction_birth_graph_authority_proof,
    topology_primitive_construction_birth_graph_obligation_registration,
    TopologyPrimitiveConstructionBirthComposeEvidence,
    TopologyPrimitiveConstructionBirthComposeExecution,
    TopologyPrimitiveConstructionBirthComposeExecutionError,
    TopologyPrimitiveConstructionBirthComposeProgram,
    TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
    TopologyPrimitiveConstructionBirthGraphAuthorityProof,
    TopologyPrimitiveConstructionBirthMaterializationCoverage,
    TopologyPrimitiveConstructionBirthSelectedObligationRow,
    TopologyPrimitiveConstructionBirthTopologyKind,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};
pub use envelope::TopologyPrimitiveConstructionQueryEnvelope;
pub use handoff::TopologyPrimitiveConstructionQueryHandoff;
pub use receipt::TopologyPrimitiveConstructionQueryReceipt;
pub use surface_vocab::{
    TopologyConstructionQueryFactKind, TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryFactRow, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryMutationSurface, TopologyConstructionQueryReadSurface,
};

const REQUIRED_QUERY_FAMILIES: [forge_query::facade::ForgeQueryRuntimeFacadeFamily; 2] = [
    forge_query::facade::ForgeQueryRuntimeFacadeFamily::Write,
    forge_query::facade::ForgeQueryRuntimeFacadeFamily::Inspect,
];

fn digest_parts(parts: &[String]) -> String {
    worth_primitives::truth_digest_parts(
        worth_primitives::TruthDigestScope::ArtifactIdentity,
        parts,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        TopologyConstructionQueryFactKind, TopologyConstructionQueryFactProvenance,
        TopologyConstructionQueryInspectionSurface, TopologyConstructionQueryMutationSurface,
        TopologyConstructionQueryReadSurface, TopologyPrimitiveConstructionQueryEnvelope,
        TopologyPrimitiveConstructionQueryReceipt,
    };
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

    #[test]
    fn query_native_receipt_bundles_query_mutation_and_inspection_posture() {
        let receipt = TopologyPrimitiveConstructionQueryReceipt::new_for_tests(
            "shell-birth",
            "planar_shell_with_hole_body",
            7,
            7,
            2,
            0,
            1,
            1,
            1,
        );

        assert_eq!(
            receipt.receipt_name(),
            "worth-topo.query-native-construction-receipt"
        );
        assert_eq!(receipt.source_birth_digest(), "shell-birth");
        assert_eq!(
            receipt.topology_birth_class(),
            "planar_shell_with_hole_body"
        );
        assert_eq!(
            receipt.mutation_surface(),
            TopologyConstructionQueryMutationSurface::ComposeGraph
        );
        assert_eq!(
            receipt.required_query_families(),
            &[
                ForgeQueryRuntimeFacadeFamily::Write,
                ForgeQueryRuntimeFacadeFamily::Inspect,
            ]
        );
        assert_eq!(
            receipt.read_surface(),
            TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
        );
        assert_eq!(
            receipt.inspection_surface(),
            TopologyConstructionQueryInspectionSurface::InspectReceipt
        );
        assert_eq!(
            receipt.fact_provenance(),
            TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption
        );
        assert_eq!(
            receipt
                .row_for(TopologyConstructionQueryFactKind::LoopMembership)
                .expect("loop row")
                .fact_count(),
            2
        );
        assert_eq!(
            receipt
                .row_for(TopologyConstructionQueryFactKind::BodyMembership)
                .expect("body row")
                .fact_count(),
            1
        );
        assert!(!receipt.receipt_digest().is_empty());
        assert!(!receipt.fact_digest().is_empty());
    }

    #[test]
    fn query_native_envelope_keeps_birth_truth_on_the_topology_boundary() {
        let receipt = TopologyPrimitiveConstructionQueryReceipt::new_for_tests(
            "orthotope-birth",
            "closed_orthotope_body",
            8,
            12,
            6,
            0,
            6,
            1,
            1,
        );
        let envelope = TopologyPrimitiveConstructionQueryEnvelope::new_for_tests(
            "orthotope-birth",
            "closed_orthotope_body",
            receipt,
        );

        assert_eq!(
            envelope.envelope_name(),
            "worth-topo.query-native-construction-envelope"
        );
        assert_eq!(envelope.source_birth_digest(), "orthotope-birth");
        assert_eq!(envelope.topology_birth_class(), "closed_orthotope_body");
        assert!(!envelope.receipt_digest().is_empty());
        assert_eq!(
            envelope.read_surface(),
            TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
        );
        assert!(!envelope.envelope_digest().is_empty());
    }

    #[test]
    fn query_native_admitted_handoff_can_lower_directly_from_birth_synopsis() {
        let synopsis = super::TopologyPrimitiveConstructionQueryBirthSynopsis::new(
            super::TopologyPrimitiveConstructionBirthFamily::WireBody,
            worth_primitives::PrimitiveConstructionFamilyContractRegistry::contract_for(
                &worth_primitives::PrimitiveWitnessDescriptor::WireBody { edge_count: 8 },
            ),
            "wire-birth".to_string(),
            "wire-birth-digest".to_string(),
            "planar_wire_body".to_string(),
            8,
            8,
            1,
            1,
            0,
            0,
            1,
        );

        let admitted = super::prepare_primitive_construction_query_admitted_handoff_from_synopsis(
            &synopsis,
            "completeness",
            "mapping",
            1,
            1,
        )
        .expect("admitted handoff");

        assert_eq!(admitted.source_birth_digest(), "wire-birth-digest");
        assert_eq!(
            admitted.topology_query_envelope().source_birth_digest(),
            "wire-birth-digest"
        );
        assert!(!admitted.admitted_handoff_digest().is_empty());
    }
}
