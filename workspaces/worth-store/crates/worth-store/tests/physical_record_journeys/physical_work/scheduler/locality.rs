use tempfile::tempdir;
use worth_foundational::{aspects, FoundationalPerformanceWorkClass};
use worth_proof::TransitionOutcome;
use worth_store::{
    aspect_native::{StoreAspectAuthorityInput, StoreAspectBoundaryFact},
    physical_runtime::{
        PhysicalReadWorkRequest, PhysicalSchedulerDemand, PhysicalWorkProfileDeclaration,
        PhysicalWorkScope, PhysicalWorkSemanticBasis,
    },
};
use worth_store_io_scheduler::QueueLocalityRelation;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{
    super::fixture::{
        admitted_contract, security_scope, serving_from_initialization_with_work_profile,
        validated_value,
    },
    policy_receipt_for, ready_read_work, secure_demand,
};

#[test]
fn multi_artifact_store_scope_reaches_scheduler_with_exact_structural_relations() {
    let root = tempdir().unwrap();
    let (contract, identity, admission, witness) = admitted_contract(41);
    let state = match aspects()
        .authoritative_state()
        .admit([validated_value(&contract, "multi-artifact-locality")])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state should admit: {outcome:?}"),
    };
    let fact = StoreAspectBoundaryFact::from_admitted_state(
        identity,
        StoreAspectAuthorityInput::new(state, witness),
    )
    .unwrap();
    let basis = PhysicalWorkSemanticBasis::projection(fact, admission.clone()).unwrap();
    let security = security_scope(witness);
    let profile = PhysicalWorkProfileDeclaration::new(security, [admission]).unwrap();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let before = serving.media_counters();

    let batch = scheduler_admit(
        &serving,
        PhysicalReadWorkRequest::new(
            PhysicalWorkScope::batch([
                coordinate(RecordArtifactFile::BootstrapCatalog, 0, 8),
                coordinate(RecordArtifactFile::RootManifest { generation: 1 }, 32, 8),
            ])
            .unwrap(),
            basis.clone(),
            security,
        )
        .unwrap(),
    );
    let overlapping = scheduler_admit(
        &serving,
        PhysicalReadWorkRequest::new(
            PhysicalWorkScope::one(coordinate(
                RecordArtifactFile::RootManifest { generation: 1 },
                36,
                8,
            )),
            basis.clone(),
            security,
        )
        .unwrap(),
    );
    let adjacent = scheduler_admit(
        &serving,
        PhysicalReadWorkRequest::new(
            PhysicalWorkScope::batch([
                coordinate(RecordArtifactFile::BootstrapCatalog, 8, 8),
                coordinate(
                    RecordArtifactFile::Segment {
                        segment: 3,
                        generation: 1,
                    },
                    0,
                    8,
                ),
            ])
            .unwrap(),
            basis,
            security,
        )
        .unwrap(),
    );

    let batch_locality = batch.queue_plan().grouping_basis().locality().unwrap();
    let overlapping_locality = overlapping
        .queue_plan()
        .grouping_basis()
        .locality()
        .unwrap();
    let adjacent_locality = adjacent.queue_plan().grouping_basis().locality().unwrap();
    assert_eq!(
        batch_locality.relation(overlapping_locality),
        QueueLocalityRelation::OverlappingOrInterleaved
    );
    assert_eq!(
        batch_locality.relation(adjacent_locality),
        QueueLocalityRelation::Adjacent
    );
    assert_eq!(batch.intent().scope().coordinates().len(), 2);
    assert_eq!(serving.media_counters(), before);
    drop((batch, overlapping, adjacent));
    serving.close();
}

fn scheduler_admit(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
    request: PhysicalReadWorkRequest,
) -> worth_store::physical_runtime::ResourceAdmittedPhysicalWork {
    let ready = ready_read_work(serving, request);
    let demand = PhysicalSchedulerDemand::foreground(
        ready,
        super::super::reserved_buffered_file_read(serving),
        None,
    )
    .unwrap();
    let work = demand.queue_work();
    let backend_requirement = work.backend_requirement();
    let requested_budget = work.requested_budget();
    let backend = serving
        .admit_physical_scheduler_capability(backend_requirement)
        .unwrap();
    let demand = secure_demand(demand, &backend);
    serving
        .admit_physical_scheduler_demand(
            demand,
            &backend,
            policy_receipt_for(
                requested_budget,
                0,
                FoundationalPerformanceWorkClass::AuthoritativeRead,
            ),
        )
        .unwrap()
}

fn coordinate(artifact: RecordArtifactFile, offset: u64, length: u32) -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(artifact, offset, length).unwrap()
}
