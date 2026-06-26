use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::relations::TopologyRelationKind;

use super::family::DerivedTopologyProductFamilyRecordInput;
use super::{
    current_derived_invalidation_family_catalog, DerivedInvalidationFamilyCatalog,
    DerivedInvalidationFamilyCatalogError, DerivedTopologyConsumedGraphFacts,
    DerivedTopologyDiagnosticPosture, DerivedTopologyInvalidationPredicate,
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyProductFamilyRecord, DerivedTopologyQueryReceiptPosture,
    DerivedTopologySpatialEvidencePosture, DerivedTopologySupportPosture,
    DerivedTopologyUpdatePosture,
};
use crate::derived_topology::invalidation_plan::inventory::{
    current_derived_invalidation_authority_inventory,
    DerivedInvalidationAuthorityInventoryCloseout, DerivedInvalidationPhaseTwoSeed,
};
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    test_basis_from_parts, LoopEndpointKind, TopologyDeclaredTouchedGraphBasis,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopEndpointFamily, TopologyTouchedAspect,
    TopologyTouchedGraphBasis, TopologyTouchedOperatingWorld,
};

pub(super) fn current_catalog() -> DerivedInvalidationFamilyCatalog {
    current_derived_invalidation_family_catalog(phase_two_seed()).unwrap()
}

pub(super) fn phase_two_seed() -> DerivedInvalidationPhaseTwoSeed {
    let inventory = current_derived_invalidation_authority_inventory();
    DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .unwrap()
        .phase_two_seed()
        .clone()
}

pub(super) fn synthetic_basis_from_relation_kind(
    relation_kind: TopologyRelationKind,
) -> TopologyTouchedGraphBasis {
    test_basis_from_parts(
        Vec::new(),
        Vec::new(),
        vec![relation_kind],
        Vec::new(),
        Vec::new(),
    )
}

pub(super) fn loop_cycle_graph_facts() -> DerivedTopologyConsumedGraphFacts {
    DerivedTopologyConsumedGraphFacts::new(
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
    )
}

pub(super) fn rewire_loop_endpoint_declared_basis(
    endpoint: LoopEndpointKind,
    slot: u64,
) -> TopologyTouchedGraphBasis {
    let declaration = TopologyRewireLoopEndpointDeclaration::new(
        relation_id(slot),
        endpoint,
        entity_id(slot + 1),
        entity_id(slot + 2),
    );
    let sequence = declaration.clone().into_mutation_sequence();
    TopologyDeclaredTouchedGraphBasis::from_sequence(
        TopologyRewireLoopEndpointFamily::semantic_family_key(),
        declaration,
        &sequence,
        TopologyTouchedOperatingWorld::mainline(),
    )
    .expect("rewire loop endpoint declaration should lower to touched graph basis")
    .proof()
    .basis()
    .clone()
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}

pub(super) struct RecordInputBuilder {
    input: DerivedTopologyProductFamilyRecordInput,
}

impl RecordInputBuilder {
    pub(super) fn with_consumed_graph_facts(
        mut self,
        consumed_graph_facts: Option<DerivedTopologyConsumedGraphFacts>,
    ) -> Self {
        self.input.consumed_graph_facts = consumed_graph_facts;
        self
    }

    pub(super) fn with_invalidation_predicate(
        mut self,
        invalidation_predicate: Option<DerivedTopologyInvalidationPredicate>,
    ) -> Self {
        self.input.invalidation_predicate = invalidation_predicate;
        self
    }

    pub(super) fn with_query_receipt_posture(
        mut self,
        query_receipt_posture: Option<DerivedTopologyQueryReceiptPosture>,
    ) -> Self {
        self.input.query_receipt_posture = query_receipt_posture;
        self
    }

    pub(super) fn with_update_posture(
        mut self,
        update_posture: Option<DerivedTopologyUpdatePosture>,
    ) -> Self {
        self.input.update_posture = update_posture;
        self
    }

    pub(super) fn with_spatial_evidence_posture(
        mut self,
        spatial_evidence_posture: Option<DerivedTopologySpatialEvidencePosture>,
    ) -> Self {
        self.input.spatial_evidence_posture = spatial_evidence_posture;
        self
    }

    pub(super) fn with_legality_receipt_posture(
        mut self,
        legality_receipt_posture: Option<DerivedTopologyLegalityReceiptPosture>,
    ) -> Self {
        self.input.legality_receipt_posture = legality_receipt_posture;
        self
    }

    pub(super) fn with_diagnostic_posture(
        mut self,
        diagnostic_posture: Option<DerivedTopologyDiagnosticPosture>,
    ) -> Self {
        self.input.diagnostic_posture = diagnostic_posture;
        self
    }

    pub(super) fn with_support_posture(
        mut self,
        support_posture: Option<DerivedTopologySupportPosture>,
    ) -> Self {
        self.input.support_posture = support_posture;
        self
    }

    pub(super) fn build(
        self,
    ) -> Result<DerivedTopologyProductFamilyRecord, DerivedInvalidationFamilyCatalogError> {
        DerivedTopologyProductFamilyRecord::from_input(self.input)
    }
}

pub(super) fn loop_cycle_record_input(
    consumed_graph_facts: Option<DerivedTopologyConsumedGraphFacts>,
) -> RecordInputBuilder {
    RecordInputBuilder {
        input: DerivedTopologyProductFamilyRecordInput {
            identity: DerivedTopologyProductFamilyIdentity::LoopCycles,
            consumed_graph_facts,
            invalidation_predicate: Some(
                DerivedTopologyInvalidationPredicate::ConsumedGraphFactsIntersectTouchedClosure,
            ),
            update_posture: Some(DerivedTopologyUpdatePosture::IncrementalEligible),
            spatial_evidence_posture: Some(
                DerivedTopologySpatialEvidencePosture::NoSpatialEvidenceConsumed,
            ),
            query_receipt_posture: Some(
                DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
            ),
            legality_receipt_posture: Some(
                DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
            ),
            diagnostic_posture: Some(
                DerivedTopologyDiagnosticPosture::ProductFamilyWitnessRequired,
            ),
            support_posture: Some(DerivedTopologySupportPosture::QuerySupportRequired),
        },
    }
}
