use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};
use topology::facade::{
    DerivedInvalidationTouchedClosure, EntityId, LoopSuccessorKind, PartitionId, RelationId,
    TopologyLoopSuccessorRewireMember, TopologyRewireLoopSuccessorProgramDeclaration,
};
use topology::touched_graph_conflict::{
    admit_topology_conflict_family_declaration, admit_topology_conflict_family_identity,
    TopologyConflictDiagnosticWitness, TopologyConflictFamilyCatalog,
    TopologyConflictFamilyCatalogCloseout, TopologyConflictFamilyDeclarationInput,
    TopologyConflictFamilyIdentityAuthority, TopologyConflictLocalityAuthorityRequirement,
    TopologyConflictPriorProofPosture, TopologyConflictSelectionProductPosture,
};
use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::replay_family_catalog::current_spatial_replay_family_catalog;
use worth_spatial::facade::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    boolean_event_ledger_spatial_boundary_fixture,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_boolean_event_ledger_request,
    prepare_spatial_replay_semantic_graph_request, BooleanEventLedgerRollbackRequest,
    SpatialReplaySemanticGraphPreparationRequest,
};
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;
use worth_spatial::touched_graph_conflict::{
    admit_spatial_conflict_family_declaration, admit_spatial_conflict_family_identity,
    SpatialConflictDiagnosticWitness, SpatialConflictFamilyCatalog,
    SpatialConflictFamilyCatalogCloseout, SpatialConflictFamilyDeclarationInput,
    SpatialConflictFamilyIdentityAuthority, SpatialConflictLocalityAuthorityRequirement,
    SpatialConflictPriorProofPosture, SpatialConflictSelectionProductPosture,
};

#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support/ordinary_topology_undo_support.rs"]
mod ordinary_topology_undo_support;
#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod replay_support;

pub(crate) struct SpatialEvidenceRouteFixture {
    authority: SpatialGeometryEvidenceTouchAuthority,
    workload_handoff: EvidenceLookupConsumedWorkloadHandoff,
    execution_receipt: EvidenceLookupExecutionReceipt,
}

// These helpers stay test-only because the ordinary owner catalogs intentionally
// expose only `RequiresFamilySelection`. The tests still build real owner
// declarations and close them through the real owner closeout types.
pub(crate) fn owner_backed_topology_closeout_with_aspect_routing_posture(
    routing_posture: ConflictRoutingPosture,
) -> TopologyConflictFamilyCatalogCloseout {
    TopologyConflictFamilyCatalogCloseout::close(TopologyConflictFamilyCatalog::new(vec![
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::aspect_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::Aspect,
            secondary_overlap_category: Some(ConflictOverlapCategory::Locality),
            routing_posture,
            prior_proof_posture: TopologyConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: TopologyConflictDiagnosticWitness::TouchedClosureDigest,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::validator_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::Validator,
            secondary_overlap_category: None,
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: TopologyConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: TopologyConflictDiagnosticWitness::ValidatorFamilyDigest,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::replay_boundary_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::ReplayUndo,
            secondary_overlap_category: Some(ConflictOverlapCategory::Transaction),
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture:
                TopologyConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
            diagnostic_witness: TopologyConflictDiagnosticWitness::ReplayBoundaryScope,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
    ]))
    .expect("custom topology closeout")
}

pub(crate) fn owner_backed_topology_closeout_with_replay_prior_proof_posture(
    prior_proof_posture: TopologyConflictPriorProofPosture,
) -> TopologyConflictFamilyCatalogCloseout {
    TopologyConflictFamilyCatalogCloseout::close(TopologyConflictFamilyCatalog::new(vec![
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::aspect_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::Aspect,
            secondary_overlap_category: Some(ConflictOverlapCategory::Locality),
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: TopologyConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: TopologyConflictDiagnosticWitness::TouchedClosureDigest,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::validator_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::Validator,
            secondary_overlap_category: None,
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: TopologyConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: TopologyConflictDiagnosticWitness::ValidatorFamilyDigest,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::replay_boundary_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::ReplayUndo,
            secondary_overlap_category: Some(ConflictOverlapCategory::Transaction),
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture,
            diagnostic_witness: TopologyConflictDiagnosticWitness::ReplayBoundaryScope,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
    ]))
    .expect("custom topology replay-prior-proof closeout")
}

pub(crate) fn owner_backed_spatial_closeout_with_evidence_routing_posture(
    routing_posture: ConflictRoutingPosture,
) -> SpatialConflictFamilyCatalogCloseout {
    SpatialConflictFamilyCatalogCloseout::close(SpatialConflictFamilyCatalog::new(vec![
        admit_spatial_conflict_family_declaration(SpatialConflictFamilyDeclarationInput {
            identity: admit_spatial_conflict_family_identity(
                SpatialConflictFamilyIdentityAuthority::evidence_selection(),
            ),
            locality_authority_requirement:
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
            primary_overlap_category: ConflictOverlapCategory::Evidence,
            secondary_overlap_category: None,
            routing_posture,
            prior_proof_posture: SpatialConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: SpatialConflictDiagnosticWitness::EvidenceFamilyDigest,
            selection_product_posture:
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_spatial_conflict_family_declaration(SpatialConflictFamilyDeclarationInput {
            identity: admit_spatial_conflict_family_identity(
                SpatialConflictFamilyIdentityAuthority::replay_boundary_selection(),
            ),
            locality_authority_requirement:
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
            primary_overlap_category: ConflictOverlapCategory::ReplayUndo,
            secondary_overlap_category: Some(ConflictOverlapCategory::Transaction),
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: SpatialConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
            diagnostic_witness: SpatialConflictDiagnosticWitness::ReplayBoundaryScope,
            selection_product_posture:
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
    ]))
    .expect("custom spatial closeout")
}

pub(crate) fn owner_backed_spatial_closeout_with_replay_prior_proof_posture(
    prior_proof_posture: SpatialConflictPriorProofPosture,
) -> SpatialConflictFamilyCatalogCloseout {
    SpatialConflictFamilyCatalogCloseout::close(SpatialConflictFamilyCatalog::new(vec![
        admit_spatial_conflict_family_declaration(SpatialConflictFamilyDeclarationInput {
            identity: admit_spatial_conflict_family_identity(
                SpatialConflictFamilyIdentityAuthority::evidence_selection(),
            ),
            locality_authority_requirement:
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
            primary_overlap_category: ConflictOverlapCategory::Evidence,
            secondary_overlap_category: None,
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: SpatialConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: SpatialConflictDiagnosticWitness::EvidenceFamilyDigest,
            selection_product_posture:
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_spatial_conflict_family_declaration(SpatialConflictFamilyDeclarationInput {
            identity: admit_spatial_conflict_family_identity(
                SpatialConflictFamilyIdentityAuthority::replay_boundary_selection(),
            ),
            locality_authority_requirement:
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
            primary_overlap_category: ConflictOverlapCategory::ReplayUndo,
            secondary_overlap_category: Some(ConflictOverlapCategory::Transaction),
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture,
            diagnostic_witness: SpatialConflictDiagnosticWitness::ReplayBoundaryScope,
            selection_product_posture:
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
    ]))
    .expect("custom spatial replay-prior-proof closeout")
}

pub(crate) fn ordinary_touched_closure(
    relation_slot: u64,
    source_slot: u64,
    target_slot: u64,
) -> DerivedInvalidationTouchedClosure {
    let declaration = TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        TopologyLoopSuccessorRewireMember::new(
            RelationId::new(PartitionId::main(), relation_slot, 1),
            LoopSuccessorKind::Next,
            EntityId::new(PartitionId::main(), source_slot, 1),
            EntityId::new(PartitionId::main(), target_slot, 1),
        ),
    ]);
    let proof = declaration
        .declared_touched_basis_proof(
            "topology.rewire_loop_successor_program",
            topology::facade::TopologyTouchedOperatingWorld::mainline(),
        )
        .expect("ordinary topology declaration lowers touched proof");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}

pub(crate) fn spatial_evidence_route_fixture(label: &'static str) -> SpatialEvidenceRouteFixture {
    let subject = replay_support::MetabossEventExtractionSubject::certify(label);
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let execution_receipt = subject
        .pair()
        .left()
        .workload()
        .require_boolean_event_ledger_lookup_execution_packet(subject.ledger())
        .expect("event-ledger lookup packet")
        .execution_receipt()
        .clone();
    let workload_handoff = completed_split_handoff
        .lookup_consumed_workload_handoff()
        .clone();
    SpatialEvidenceRouteFixture {
        authority,
        workload_handoff,
        execution_receipt,
    }
}

pub(crate) fn packet_backed_boundary(
    label: &'static str,
) -> crate::workload_composition::AdmittedBooleanSplitReplayUndoBoundary {
    let subject = replay_support::MetabossEventExtractionSubject::certify(label);
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff =
        replay_support::completed_split_handoff_for(&subject, &replay_subject);
    let topology_undo_support =
        ordinary_topology_undo_support::ordinary_traversal_views_undo_scope_support();
    let topology_undo_scope_product = topology_undo_support
        .lower_undo_scope_product()
        .expect("ordinary topology undo scope product");
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let event_ledger_lookup_packet = subject
        .pair()
        .left()
        .workload()
        .require_boolean_event_ledger_lookup_execution_packet(subject.ledger())
        .expect("event-ledger lookup packet");
    let request = prepare_spatial_replay_semantic_graph_request(
        SpatialReplaySemanticGraphPreparationRequest::new(
            boolean_event_ledger_spatial_boundary_fixture().replay_family_identity(),
            &authority,
            event_ledger_lookup_packet.execution_receipt(),
            completed_split_handoff.lookup_consumed_workload_handoff(),
        )
        .with_retained_replay_receipt(
            completed_split_handoff
                .completed_workload()
                .retained_replay(),
        ),
    )
    .expect("prepared replay request");
    let admitted = admit_prepared_spatial_replay_semantic_graph_input(
        &current_spatial_replay_family_catalog(),
        &request,
    )
    .expect("admitted replay request");
    let replay_scope =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("replay scope");
    let undo_scope = lower_spatial_undo_scope_product_from_boolean_event_ledger_request(
        BooleanEventLedgerRollbackRequest::new(
            &authority,
            event_ledger_lookup_packet.execution_receipt(),
            completed_split_handoff
                .completed_workload()
                .evidence_ledger()
                .stage_index(),
            completed_split_handoff.lookup_consumed_workload_handoff(),
        ),
    )
    .expect("undo scope");
    completed_split_handoff
        .admit_batch_execution_cluster()
        .expect("packet-backed split handoff admits batch execution cluster")
        .admit_boolean_split_replay_undo_boundary(
            crate::workload_composition::BooleanSplitReplayUndoBoundaryRequest::new(
                &topology_undo_scope_product,
                &replay_scope,
                &undo_scope,
            ),
        )
        .expect("packet-backed split handoff admits replay/undo boundary")
}

impl SpatialEvidenceRouteFixture {
    pub fn authority(&self) -> &SpatialGeometryEvidenceTouchAuthority {
        &self.authority
    }

    pub fn workload_handoff(&self) -> &EvidenceLookupConsumedWorkloadHandoff {
        &self.workload_handoff
    }

    pub fn execution_receipt(&self) -> &EvidenceLookupExecutionReceipt {
        &self.execution_receipt
    }
}
