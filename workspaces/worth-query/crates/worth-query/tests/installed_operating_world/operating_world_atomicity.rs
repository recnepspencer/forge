use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use worth_query::facade::domain;

use super::installed_operation_fixture::{
    mixed_mutation_workflow_runtime, GeometryDomain, MutationFamily, WorkflowMutation,
};

#[derive(Clone, Copy)]
struct RemoteGraph;
#[derive(Clone, Copy)]
struct SeparateCommit;

struct UncontactedProvider(Arc<AtomicUsize>);

impl domain::WorthQueryGraphParticipationProvider<RemoteGraph> for UncontactedProvider {
    fn observe(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        self.0.fetch_add(1, Ordering::Relaxed);
        Ok(call.completed("observe"))
    }

    fn project(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        self.0.fetch_add(1, Ordering::Relaxed);
        Ok(call.completed("project"))
    }

    fn touch_effect(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        self.0.fetch_add(1, Ordering::Relaxed);
        Ok(call.completed("touch"))
    }
}

impl domain::WorthQueryGraphCommitProvider<SeparateCommit> for UncontactedProvider {
    fn admit_commit(
        &self,
        call: &domain::WorthQueryGraphCommitCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        self.0.fetch_add(1, Ordering::Relaxed);
        Ok(call.completed("commit"))
    }
}

#[test]
fn primary_and_separate_mutation_requires_declared_compensation() {
    let contacts = Arc::new(AtomicUsize::new(0));
    let uncompensated = runtime(false, Arc::clone(&contacts), "mixed-uncompensated");
    let installed = uncompensated.domain(GeometryDomain).unwrap();
    let denial = match uncompensated
        .prepare_mutation_operating_world()
        .unwrap()
        .family(MutationFamily)
        .bind(&installed, WorkflowMutation)
    {
        Ok(_) => panic!("separate commit authority cannot cover primary graph mutation"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial.kind(),
        domain::WorthQueryOperationBindingDenialKind::CompensationUndeclared
    );
    assert_eq!(contacts.load(Ordering::Relaxed), 0);

    let compensated = runtime(true, Arc::clone(&contacts), "mixed-compensated");
    let installed = compensated.domain(GeometryDomain).unwrap();
    let bound = compensated
        .prepare_mutation_operating_world()
        .unwrap()
        .family(MutationFamily)
        .bind(&installed, WorkflowMutation)
        .unwrap();
    assert_eq!(
        bound.commit_posture(),
        domain::WorthQueryBoundCommitPosture::Compensated
    );
    assert_eq!(contacts.load(Ordering::Relaxed), 0);
}

fn runtime(
    compensated: bool,
    contacts: Arc<AtomicUsize>,
    name: &str,
) -> worth_query::facade::runtime::WorthQueryWorkspace {
    mixed_mutation_workflow_runtime::<RemoteGraph>(compensated)
        .graph_participation(atomic_definition())
        .atomic_graph_participation_provider(
            RemoteGraph,
            UncontactedProvider(Arc::clone(&contacts)),
            SeparateCommit,
        )
        .graph_commit_provider(SeparateCommit, UncontactedProvider(contacts))
        .workspace(name)
        .unwrap()
}

fn atomic_definition() -> domain::WorthQueryGraphParticipationDefinition<RemoteGraph> {
    domain::WorthQueryGraphParticipationDefinition::new(
        "remote-a",
        domain::WorthQueryGraphParticipationContract {
            observation: domain::WorthQueryGraphObservationPosture::Snapshot,
            projection: domain::WorthQueryGraphProjectionPosture::NativeProjection,
            mutation: domain::WorthQueryGraphMutationPosture::TouchAndEffect,
            identity: domain::WorthQueryGraphIdentityPosture::Opaque,
            locality: domain::WorthQueryGraphLocalityPosture::ExternalBoundary,
            budget: domain::WorthQueryGraphBudgetPosture::ExternalBoundary,
            commit: domain::WorthQueryGraphCommitPosture::AtomicAuthorityRequired,
            failure: domain::WorthQueryGraphFailureTopology::BoundaryFailure,
        },
    )
}
